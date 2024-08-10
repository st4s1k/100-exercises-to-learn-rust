// TODO: Implement the patching functionality.
use crate::data::{Ticket, TicketDraft, TicketPatch};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient(SyncSender<Command>);

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, OverloadedError> {
        let (response_channel, response_receiver) = sync_channel(1);
        let command = Command::Insert { draft, response_channel };
        self.exchange(command, || response_receiver.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, OverloadedError> {
        let (response_channel, response_receiver) = sync_channel(1);
        let command = Command::Get { id, response_channel };
        self.exchange(command, || response_receiver.recv().unwrap())
    }

    pub fn update(&self, ticket_patch: TicketPatch) -> Result<(), OverloadedError> {
        let command = Command::Update { patch: ticket_patch };
        self.exchange(command, || ())
    }

    fn exchange<T, F>(&self, command: Command, response_provider: F) -> Result<T, OverloadedError>
    where
        F: FnOnce() -> T,
    {
        self.0
            .try_send(command)
            .map_err(|_| OverloadedError)?;
        Ok(response_provider())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("The store is overloaded")]
pub struct OverloadedError;

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient(sender)
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
    Update {
        patch: TicketPatch
    },
}

macro_rules! set {
    ($item:expr) => { (|value| $item = value) };
}

macro_rules! if_some {
    ($option:expr => $consumer:expr) => {
        if let Some(value) = $option { ($consumer)(value) }
    };
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert { draft, response_channel }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get { id, response_channel }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Ok(Command::Update { patch }) =>
                if_some!(store.get_mut(patch.id) => |ticket: &mut Ticket| {
                    if_some!(patch.title => set!(ticket.title));
                    if_some!(patch.description => set!(ticket.description));
                    if_some!(patch.status => set!(ticket.status));
                }),
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
