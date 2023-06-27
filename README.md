# CS Flash

This Project can be used to trigger external devices, \
while you are currently flash banged in Counter Strike.

This script presses the `f13` key, when a CSGO flashbang is detected.!

## Installation

You can download the executable from the [Release Page](https://github.com/Lukas412/cs_flash/releases/latest).

### Building from Source

You first need to install `cargo` itself.

Cargo is a tool for conveniently accessing a lot of the rust functionality from the command line.

It comes with the rust toolchain. \
The rust toolchain can be downloaded and installed from here:
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

The project can be directly installed with cargo.

    cargo install --git https://github.com/Lukas412/cs_flash.git

    cs_flash.exe

Or you can clone and run this project.

    git clone https://github.com/Lukas412/cs_flash.git
    
    cd cs_flash

    cargo run --release

## Usage

1. You first need to start CSGO, before running an executable compiled from this project. \
When no CSGO Window is detected the script will fail and display an appropriate error message.

2. Once this script is running, you can verify if it is working by directly looking into a flashbang in CSGO. \
It should print into the console, when the flashbang starts and when it ends.

3. While the flashbang is active the script is pressing the `f13` key. \
You can capture that keypress with an external program and trigger any action you like.


For example you could trigger a realy bright light, that stays on for as long as the flash bang is visible.
