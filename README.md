# Tapse

Very early alpha! Stuff's broken!

## Features

* Realtime chat and file sharing
* Single-binary for easy deployments

## Inspiration
Tapse's features were inspired by the now-abandoned [PirateBox](https://piratebox.cc/) project.

## Requirements:
* sqlx-cli: `cargo install sqlx-cli`
* npm or equivalent

## Setup:
* `cd frontend` & `pnpm install` & `pnpm build`
* `cargo run/build --release`


## TODO
* [ ] Add forum module
* [ ] Unit tests
* [ ] Fix multipart exploding when files are too big
* [ ] Sync files (randomly broke)
* [ ] Styling polish
* [ ] A logo!
## Known problems

* Bigger files (> 80MB) may not upload properly