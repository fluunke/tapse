# Tapse

Early alpha! Stuff's broken!


## Demo

![Tapse Demo](https://raw.githubusercontent.com/fluunke/tapse/master/assets/tapse.gif)

## Features

* Realtime chat and file sharing with multiple rooms
* Single-binary for easy deployments

## Inspiration
Tapse's features were inspired by the [PirateBox](https://piratebox.cc/) project.

## Requirements:
* pnpm (npm might work, untested)

## Setup:
* `cd frontend` & `pnpm install` & `pnpm build`
* `cargo run/build --release`

## Known problems

* Can't upload files larger than 1GB due to SQLite limits