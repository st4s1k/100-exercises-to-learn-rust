use crate::data::TicketDraft;
use crate::store::{TicketId, TicketStore};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, put};
use axum::{Json, Router};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};
use tracing_subscriber;

mod data;
mod store;

type SharedTicketStore = Arc<RwLock<TicketStore>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let store: SharedTicketStore = Arc::new(RwLock::new(TicketStore::new()));

    let app = Router::new()
        .route("/tickets/:ticket_id", get(get_ticket))
        .route("/tickets", post(create_ticket))
        .route("/tickets/:ticket_id", put(update_ticket))
        .with_state(store);

    let listener = match tokio::net::TcpListener::bind("127.0.0.1:3000").await {
        Ok(listener) => listener,
        Err(e) => {
            error!("Failed to bind to address: {}", e);
            return;
        }
    };

    if let Err(e) = axum::serve(listener, app).await {
        error!("Server error: {}", e);
    }
}

async fn get_ticket(
    State(store): State<SharedTicketStore>,
    Path(ticket_id): Path<u64>,
) -> (StatusCode, Response) {
    info!("Getting ticket for id: [{ticket_id}]");

    match store.read().await.get(TicketId(ticket_id)) {
        Some(ticket_arc) => {
            let ticket = ticket_arc.read().await.clone();
            (StatusCode::OK, Json(ticket).into_response())
        }
        None => {
            info!("Ticket not found for id: [{ticket_id}]");
            (StatusCode::NOT_FOUND, ().into_response())
        }
    }
}

async fn create_ticket(
    State(store): State<SharedTicketStore>,
    Json(draft): Json<TicketDraft>,
) -> (StatusCode, Response) {
    info!("Creating ticket: [{draft:?}]");

    let mut store_guard = store.write().await;
    let id = store_guard.add_ticket(draft);

    (StatusCode::CREATED, Json(*id).into_response())
}

async fn update_ticket(
    State(store): State<SharedTicketStore>,
    Path(ticket_id): Path<u64>,
    Json(draft): Json<TicketDraft>,
) -> (StatusCode, Response) {
    info!("Updating ticket with id: [{ticket_id}] with data: [{draft:?}]");

    let store_guard = store.write().await;
    if let Some(ticket_arc) = store_guard.get(TicketId(ticket_id)) {
        let mut ticket = ticket_arc.write().await;
        ticket.title = draft.title;
        ticket.description = draft.description;
        (StatusCode::OK, Json(ticket_id).into_response())
    } else {
        info!("Ticket not found for id: [{ticket_id}]");
        (StatusCode::NOT_FOUND, ().into_response())
    }
}
