extern crate gpt;
#[macro_use] extern crate text_io;

use gpt::completer::{Completer, GptCompleter};

use crossbeam::channel;
use crossbeam_channel::{Receiver, Sender};
use crossbeam_channel::{SendError, RecvError};
use std::thread;

// Server and world initiation.
// Two people in a room.
// One person says hello to the other.
// The server informs both people what happened.
// No state is updated.

struct PersonSender {
    name: String,
    sender: Sender<String>,
}

struct PersonReceiver {
    name: String,
    receiver: Receiver<String>,
}

impl PersonSender {
    pub fn send(&self, t: String) -> Result<(), SendError<String>> {
        println!("{} sees:", self.name);
        for line in t.split("\n") {
            println!("\t{}", line);
        }
        print!("\t{}: ", self.name);
        self.sender.send(t)
    }
}

impl PersonReceiver {
    pub fn recv(&self) -> Result<String, RecvError> {
        match self.receiver.recv() {
            Ok(msg) => {
                println!("{}: {}", self.name, msg);
                Ok(msg)
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}


fn person(name: String) -> (PersonSender, PersonReceiver) {
    let (server_tx, person_rx) = channel::unbounded();
    let (person_tx, server_rx) = channel::unbounded();

    thread::spawn(move || {
        let state = person_rx.recv().unwrap();

        let line: String = read!("{}\n");
        person_tx.send(line)
    });

    (PersonSender {name: name.clone(), sender: server_tx}, PersonReceiver {name, receiver: server_rx})
}

fn main() {
    let (person1_tx, person1_rx) = person("Person 1".to_string());
    let (person2_tx, person2_rx) = person("Person 2".to_string());

    person1_tx.send("You are in a room.".to_string());
    let response = person1_rx.recv().unwrap();
    let state = format!("You are in a room.\nA voice calls out \"{}\"", response);
    person2_tx.send(state);
    let person2_response = person2_rx.recv();

    // Initialize the GPT Completer
    let gpt_completer = GptCompleter;

    // Generate a completion for the prompt "Hello, my name is"
    let response = gpt_completer.complete("AI Chatbot: Hello, my name is ");
    println!("{}", response.expect("Hopefully this passed"));
}

