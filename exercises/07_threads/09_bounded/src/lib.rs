use std::error::Error;
use std::sync::mpsc::{channel, Receiver, Sender, sync_channel, SyncSender, TrySendError};

// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    fn send<T, C>(&self, command_provider: C, ) -> Result<T, TrySendError<String>>
    where
        C: FnOnce(Sender<T>) -> Command,
    {
        let (response_channel, response_receiver) = channel();
        let command = command_provider(response_channel);
        match self.sender.try_send(command) {
            Ok(_) => Ok(response_receiver.recv().unwrap()),
            Err(err) => Err(TrySendError::Full(err.to_string()))
        }
    }

    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, TrySendError<String>> {
        self.send(|response_channel| Command::Insert { draft, response_channel })
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, TrySendError<String>> {
        self.send(|response_channel| Command::Get { id, response_channel })
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient {
        sender
    }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                   draft,
                   response_channel,
               }) => {
                let id = store.add_ticket(draft);
                response_channel.send(id).unwrap()
            }
            Ok(Command::Get {
                   id,
                   response_channel,
               }) => {
                let ticket = store.get(id);
                response_channel.send(ticket.cloned()).unwrap()
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
