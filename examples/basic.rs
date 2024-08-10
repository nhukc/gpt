extern crate gpt;
#[macro_use] extern crate text_io;

use gpt::completer::{Completer, GptCompleter};

use crossbeam::channel;
use crossbeam_channel::{Receiver, Sender};
use crossbeam_channel::{SendError, RecvError};
use std::thread;

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

fn npc(name: String) -> (PersonSender, PersonReceiver) {
    let (server_tx, person_rx) = channel::unbounded::<String>();
    let (person_tx, server_rx) = channel::unbounded();
    let name_clone = name.clone();
    thread::spawn(move || {
        let mut state = person_rx.recv().unwrap();
        state.push_str(&format!("[Your response here]
Write your response as if you were saying it aloud.
More general actions can be indicated by enclosing the action with *

For example, *I do nothing.*

{}: ", name));

        let gpt_completer = GptCompleter;
        let response = gpt_completer.complete(state).unwrap_or("*is unresponsive*".to_string());
        let lines: Vec<_> = response.split("\n").collect();
        let final_line = lines[lines.len() - 1];
        let final_line = final_line.replace(&format!("{}:  ", name), "");
        person_tx.send(final_line.to_string())
    });

    (PersonSender {name: name_clone.clone(), sender: server_tx}, PersonReceiver {name: name_clone.clone(), receiver: server_rx})
}

fn person(name: String) -> (PersonSender, PersonReceiver) {
    let (server_tx, person_rx) = channel::unbounded();
    let (person_tx, server_rx) = channel::unbounded();
    let name_clone = name.clone();
    thread::spawn(move || {
        let state = person_rx.recv().unwrap();

        print!("\t{}: ", name);
        let line: String = read!("{}\n");
        person_tx.send(line)
    });

    (PersonSender {name: name_clone.clone(), sender: server_tx}, PersonReceiver {name: name_clone.clone(), receiver: server_rx})
}

fn main() {
    let (person1_tx, person1_rx) = person("Person 1".to_string());
    let (person2_tx, person2_rx) = npc("Person 2".to_string());

    println!("There are two people in a room. Person 1 and Person 2.");
    println!("The two people do not know each other.");
    println!("The two people do not notice each other.");

    let state = "You are in a room".to_string();
    person1_tx.send(state);
    let response = person1_rx.recv().unwrap();

    let state = format!("You are in a room.\nA voice calls out \"{}\"", response);
    person2_tx.send(state);
    let response = person2_rx.recv();
}

