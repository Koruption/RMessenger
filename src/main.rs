use std::{thread, time::Duration};
mod messenger;

fn main() {
    let mut sender = messenger::Sender::new(String::from("EventOne"));
    sender
        .tap(&|m: &String| {
            println!("Message received: {:?}", m)
        })
        .tap(&|m: &String| {
            thread::sleep(Duration::from_millis(900));
            println!("I'm late to the party!: but my data is: {:?}", m)
        });

    let data_one = String::from("Hello, everyone!");
    let data_two = String::from("Hope, you're having a fine day 👐");
    
    thread::sleep(Duration::from_secs(1));
    sender.send(&data_one);
    
    thread::sleep(Duration::from_secs(1));
    sender.send(&data_two);


    sender.send(&String::from("Rust is cool 🦀!"));

}
