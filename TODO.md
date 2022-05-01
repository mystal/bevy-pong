# TODO
## Features
* Use [iyes_loopless](https://lib.rs/iyes_loopless) for managing game states.
  * [x] Main menu
  * [ ] Settings
  * [ ] Rounds
* [ ] Expose tunables via a egui.
* [ ] Add simple sound effects.

### Juice
* [ ] Camera shake when a paddle hits the ball!
  * [GDC: Juicing Your Cameras with Math](https://www.youtube.com/watch?v=tu-Qe66AvtY)
* Try out [bevy_hanabi](https://lib.rs/bevy_hanabi/) for particle effects!
  * [ ] Trailing particles behind ball. More as it goes faster.
  * [ ] Sparks when ball hits paddle/wall. More as it goes faster.
* [ ] Try out [bevy_tweening](https://lib.rs/bevy_tweening/) for something?!

## Bugs
* [ ] Figure out how to fix up paths from absolute to relative in trunk's generated index.html.
* Clean up collision checks to prevent paddles from penetrating
  * [x] Walls
  * [ ] Ball
* [ ] Preload assets before starting the game. (Loading screen?)

# Done
* [x] Fix the ball resetting after someone scores.
* [x] Make paddles into kinematic bodies so they won't be pushed around by the ball.
* [x] Add my own rendering, make physics debug rendering optional.
* [x] Make ball spawn in random direction toward alternating players.
* [x] Show score.
* [x] Remove unused bevy features
* [x] Put a web build up on itch.io
  * [x] Figure out how to make web builds.
  * [x] Remove margin around canvas in itch iframe.
  * [x] Optimize size of wasm file.
