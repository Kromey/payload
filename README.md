# Payload

Another roguelike game... *IN SPAAAAACE!*

## Development

Getting Amethyst up and running on Ubuntu 18.04 required installing the following packages:

`sudo apt install libasound2-dev libx11-xcb-dev libssl-dev cmake libfreetype6-dev`

Amethyst's ["Getting Started"](https://www.amethyst.rs/book/latest/getting-started.html) only lists the first 3, but compilation of Amethyst crates and dependencies failed until the latter 2 were installed as well.
