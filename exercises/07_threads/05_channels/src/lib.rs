use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use crate::data::TicketDraft;
use crate::store::TicketStore;

pub mod data;
pub mod store;

pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server the thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv_timeout(Duration::from_secs/* hehe "secs" */(5)) {
            Ok(cmd) => match cmd {
                Command::Insert(draft) => {
                    store.add_ticket(draft);
                }
            }
            Err(_) => {
                panic!("Tired of waiting! I'm going to sleep...")
            }
        }
    }
}
