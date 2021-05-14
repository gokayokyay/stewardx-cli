# Got these from https://github.com/nvm-sh/nvm/blob/master/install.sh - thanks guys!

try_profile() {
  if [ -z "${1-}" ] || [ ! -f "${1}" ]; then
    return 1
  fi
  echo "${1}"
}

detect_profile() {
  if [ "${PROFILE-}" = '/dev/null' ]; then
    return
  fi

  if [ -n "${PROFILE}" ] && [ -f "${PROFILE}" ]; then
    echo "${PROFILE}"
    return
  fi

  local DETECTED_PROFILE
  DETECTED_PROFILE=''

  if [ -n "${BASH_VERSION-}" ]; then
    if [ -f "$HOME/.bashrc" ]; then
      DETECTED_PROFILE="$HOME/.bashrc"
    elif [ -f "$HOME/.bash_profile" ]; then
      DETECTED_PROFILE="$HOME/.bash_profile"
    fi
  elif [ -n "${ZSH_VERSION-}" ]; then
    DETECTED_PROFILE="$HOME/.zshrc"
  fi

  if [ -z "$DETECTED_PROFILE" ]; then
    for EACH_PROFILE in ".profile" ".bashrc" ".bash_profile" ".zshrc"
    do
      if DETECTED_PROFILE="$(try_profile "${HOME}/${EACH_PROFILE}")"; then
        break
      fi
    done
  fi

  if [ -n "$DETECTED_PROFILE" ]; then
    echo "$DETECTED_PROFILE"
  fi
}

DETECTED_PROFILE=detect_profile
echo "Creating directory $HOME/.stewardx"
mkdir -p $HOME/.stewardx
cd $HOME/.stewardx
echo "Downloading the latest binary..."

if ! command -v curl &> /dev/null
then
    echo "Couldn't find curl, please install it then try again. Exiting.";
    exit 1;
elif ! command -v xargs &> /dev/null
then
    echo "Couldn't find xargs, please install it then try again. Exiting.";
    exit 1;
elif ! command -v cut &> /dev/null
then
    echo "Couldn't find cut, please install it then try again. Exiting.";
    exit 1;
fi


NODE_ARCH=''
if [ "$(arch)" = "x86_64" ]; then
  NODE_ARCH="x64";
elif [ "$(arch)" = "i386" ]; then
  NODE_ARCH="x32";
elif [ "$(arch)" = "i686" ]; then
  NODE_ARCH="x32"
else
  NODE_ARCH="$(arch)"
fi

# curl -s https://api.github.com/repos/user/reponame/releases/latest | grep -E 'browser_download_url' | grep linux_amd64 | cut -d '"' -f 4 | wget -qi - 

curl -s https://api.github.com/repos/gokayokyay/stewardx-cli/releases/latest |
  grep -E 'browser_download_url' |
  grep "linux_$NODE_ARCH" |
  cut -d '"' -f 4 |
  xargs curl -fsLJO

RESULT=$?

if [ $RESULT != 0 ]; then
    echo "Couldn't download latest release, this may happen because prebuilt binaries are not available for your system. Please check instructions for building it yourself.";
    echo "Please open an issue at https://github.com/gokayokyay/stewardx-cli"
    exit 1;
fi

mv stewardx-cli* stxctl

echo "Downloaded latest binary for your system, now changing permissions..."
chmod +x stxctl
echo "It's now an executable, appending the path to your profile..."

echo "export PATH=\"\$PATH:/\$HOME/.stewardx\"" >> "$(detect_profile)"
echo ""
echo "Appended \"export PATH=\"\$PATH:/\$HOME/.stewardx\"\" to $(detect_profile). Now all you need to do is running:"
echo ""
echo "source $(detect_profile)"
echo ""
echo ""
echo "Then you can run \"stxctl install\" to install StewardX"
echo "And run \"stxctl start\" to start StewardX! Make sure you have STEWARDX_DATABASE_URL environment variable set."
echo "See you around!"
