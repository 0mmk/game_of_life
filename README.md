# Game of Life
#### Terminal text based game of life in Rust.

### Running
```sh
git clone https://github.com/0mmk/game_of_life
cd game_of_life
cargo run -- game.txt
```

### Configuration
`game.txt` can be configured like this.
```
[alive cell char]:[dead cell char]:[sleep in millisecond per cycle]
[alive cell char][dead cell char][alive cell char]...
...
...
```

An example with 500 millis step, `x` as alive, `-` as dead cells for glider would be
```
x:-:500
---x--
-x-x--
--xx--
------
```
