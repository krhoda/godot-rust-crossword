# Godot 3.5 Rust Template

This template creates the bare minimum to get up and running with Godot 3.5 and Rust.

It is currently opinionated to be for 2D games and MacOs development environment, but can easily be adjusted. Because of this, if running on a mac, it's as simple as:

```
$ cargo generate krhoda/godot-3-lts-rust-template my-game
$ cd my-game/rustlib
$ cargo build
```
Then, run the editor, open the project and run the main scene, if you see "Hello from MainScene!" in the Editor's Output, all is well. The Rust library exposes a MainScene Class, the game imports it. This is shown by the "Hello from MainScene!" printed in the console when running the game.

If you're not on a Mac, you will need to alter the rust.gdnlib file to point to your architecture.

If someting doesn't make sense to you, rigerously studying these [two pages](https://godot-rust.github.io/gdnative-book/faq/configuration.html) of [documentation](https://godot-rust.github.io/gdnative-book/intro/hello-world.html) eventually got me to the point I made this repo as a template for myself.
