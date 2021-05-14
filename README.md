# StewardX CLI
![image](https://user-images.githubusercontent.com/18118125/117589232-04faef00-b131-11eb-9c2e-cd21578ab1c0.png)

*Logo converted to ASCII via [https://asciiart.club/](https://asciiart.club/)*

## The CLI to control [StewardX](https://github.com/gokayokyay/stewardx)
This is the official CLI to control [StewardX](https://github.com/gokayokyay/stewardx). Made with #[clap](https://crates.io/crates/clap).

### Getting started
If you're a linux x64 user, then you're in luck! Just download the precompiled binary or use the following utility:
```sh
$ curl https://raw.githubusercontent.com/gokayokyay/stewardx-cli/master/install.sh --output install.sh
$ chmod +x install.sh
$ ./install.sh
```
It'll handle everything for you, **note:** it's new and I'm pretty sure it won't work out for everyone.

If you're not, it's okay. Clone the repository and build it:
```sh
$ git clone https://github.com/gokayokyay/stewardx-cli
$ cd stewardx-cli
$ cargo build --release
```

When the build is completed, there'll be a file named `stewardx-cli` located in `target/release/` directory. For ease of use rename the file, create a directory to use the binary and append it to your profile, like:
```sh
$ mkdir -p $HOME/.stewardx
$ mv target/release/stewardx-cli $HOME/.stewardx/stxctl
$ echo "export PATH=\"\$PATH:/\$HOME/.stewardx\"" >> $HOME/.bashrc
$ source $HOME/.bashrc
```

Now you're ready!
Run:
```sh
$ stxctl -h
```

to see the help message.

### Usage
#### Install StewardX
To install StewardX, just run the `install` command, like so:
```sh
$ stxctl install
```

#### Start StewardX
To start the StewardX (daemonized) issue the `run` or `start` command.
```sh
$ stxctl run
# or
$ stxctl start
```

#### Stop StewardX
I guess you got it but here it is:
```sh
$ stxctl stop
```

#### Create a task
Okay here's the fun part. To create a task you need to run:
```sh
$ stxctl tasks create <task_type> [ARGS]
```

It looks complicated but it's not, let's create a hello world printer command task:
```sh
$ stxctl tasks create cmd -n "My first task" -c "echo Hello world!" -f Hook
```
Where
- -n is name of the task
- -c is the command to execute
- -f is the frequency of the task

After executing the command above you'll get an output like:
```sh
Task ID                              | Name             | Type     | Frequency       
---------------------------------------------------------------------------------
0558b548-d118-4c46-9309-e040b486a92b | first task       | CmdTask  | Hook        
```

Voila! You've created your first task.

#### Listing tasks
To list tasks
```sh
$ stxctl tasks list
```

To list active tasks
```sh
$ stxctl tasks active 
```

#### Executing and aborting tasks
To execute
```sh
$ stxctl tasks execute <id>
```

To abort
```sh
$ stxctl tasks abort <id>
```

and replace <id> with the id of the task.

#### Listing reports
To list last 10 reports:
```sh
$ stxctl reports latest
```

To list a specific task's reports
```sh
$ stxctl reports -t <id>
```

and replace <id> with your task's id.

To list a single report:
```sh
$ stxctl reports list <id>
```
where id is your report's id.

