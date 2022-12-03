# 📬 RMessenger
A simple `signal-esque` event system built in rust primarily made as tool for me to familiarize myself with the lanaguage, but also because I like building event based systems!

# Demo
To demo the project first make sure you have Rust installed, then clone the repo and run `cargo build` to install some dependencies (a uuid tool, and the Rust async lib) then `cargo run` to start the project.

# Usage
Usage is pretty simple, just instantiate a sender and tap it to begin subscribing to it's signals. Then send some data and watch your subscribers go!

```rust
mod messenger;
fn main() {
  // create a string sender
  let mut sender = messenger::Sender::new(String::from("EventOne"));

  // subscribe to our sender
  sender.tap(&|msg: &String| {
    println!("Sub1 got some data: {:?}", msg);
  });

  // tapping is also chainable
  sender
  .tap(&|msg: &String| {
    println!("Sub2 got some data: {:?}", msg);
  })
  .tap(&|msg: &String| {
    println!("Sub3 got some data: {:?}", msg);
  });

  // send some data 
  sender.send(&String::from("🦀🦀🦀"));
}
```
