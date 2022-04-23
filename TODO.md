* [x] Fix the ball resetting after someone scores.
* [x] Make paddles into kinematic bodies so they won't be pushed around by the ball.
* [x] Add my own rendering, make physics debug rendering optional.
* [x] Make ball spawn in random direction toward alternating players.
* [x] Show score.
* [ ] Use [iyes_loopless](https://lib.rs/iyes_loopless) for managing game states.
  * Rounds, menus, settings
* [x] Put a web build up on itch.io
  * [x] Figure out how to make web builds.
  * [ ] Figure out how to fix up paths from absolute to relative in trunk's generated index.html.
  * [x] Remove margin around canvas in itch iframe.
  * [x] Optimize size of wasm file.
* Clean up collision checks to prevent paddles from penetrating
  * [x] Walls
  * [ ] Ball
* [ ] Expose tunables via a egui.
* [ ] Preload assets before starting the game. (Loading screen?)
* [ ] Try out [bevy_hanabi](https://lib.rs/bevy_hanabi/) for particle effects!
* [ ] Try out [bevy_tweening](https://lib.rs/bevy_tweening/) for something?!
* [x] Remove unused bevy features
