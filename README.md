# esp8266-rust-template

The template uses interruption to control built-in LED with built-in button press.

## How to use

```shell
cargo generate https://github.com/knightpp/esp8266-rust-template cargo
```

## Prerequisites

### Install (cargo)

Install xtensa lx106 gcc compiler and the following tools:

```shell
cargo install cargo-generate
cargo install ldproxy
cargo install espflash
cargo install cargo-espflash # Optional
```

### Install (Archlinux)

```shell
pacman -Syu cargo-generate ldproxy espflash cargo-espflash
paru -Syu xtensa-lx106-elf-gcc-bin
```

## Run/Flash

```shell
cargo run
```
