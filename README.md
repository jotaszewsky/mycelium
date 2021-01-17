# Mycelium

[![Build Status](https://github.com/Jotaszewsky/mycelium/workflows/Rust/badge.svg?branch=0.1.x)](https://github.com/Jotaszewsky/mycelium)

The `mycelium.technokreacja.io` repo is used to create connection between different
data sources. 

==========

by Karol Hrusza <khrusza@gmail.com><br>
<https://github.com/Jotaszewsky>


## Usage

#### Help:

`-h, --help` Prints help information <br>
`-V, --version` Prints version information <br>

`connection` Opening the connection by using mycelium <br>
`help` Prints this message or the help of the given subcommand(s) <br>
`multi-write` Defines the state of multiple storage sources <br>
`read` Defines the state of the reader source <br>
`show` Show mycelium connections <br>
`write` Defines the state of the storage source <br>
`apply` Defines states for read and write by yaml file <br>

#### Quick Start:

Set the input and output states for the mycelium network connection.
Set input: <br>
`mycelium read amqp --url amqp://localhost:5602 --queue default` <br>
Set output: <br>
`mycelium write amqp --url amqp://localhost:5602 --queue target` <br>
If you want to check the settings of the mycelium network connection, please enter: <br>
`mycelium show` <br>
Create mycelium network connection: <br>
`mycelium connection` to stop connection `Ctrl+c` <br>
For more commands use `mycelium help` <br>

#### Sources:
The list of currently implemented sources for the Mycelium.

##### Input
`amqp`
`file`

##### Output
`amqp`
`file`
`console`

## Installation and compilation

##### Compilation

`docker-compose exec cargo cargo build --release`

##### Test

`docker-compose exec cargo cargo test`

## Origins
Mycelium uses the cargo dependencies. I am not the author of the cargo dependency code.
However, I thank their creators for the useful tool that has allowed me to create the mycelium.

## Licensing
MIT