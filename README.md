# Mycelium

[![Build Status](https://github.com/jotaszewsky/mycelium/workflows/Rust/badge.svg?branch=0.1.x)](https://github.com/jotaszewsky/mycelium)

The `mycelium` repo is used to create connection between different
data sources. 

==========

by Karol<br>
<https://github.com/jotaszewsky>


## Usage

#### Help:

`-h, --help` Print help information <br>
`-V, --version` Print version information <br>

`connection` Open connection<br>
`help` Print help <br>
`multi-write` Define multiple output states <br>
`read` Define the input state <br>
`show` Show connections <br>
`write` Define the output state <br>
`apply` Define input and output state by yaml file <br>
`clear` Clear state <br>

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
`console`

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

![](https://64.media.tumblr.com/cecc281d4a592430b9d482b4fff19d9b/tumblr_plxi8amzQn1qdhps7o2_r1_540.gifv)