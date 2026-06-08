use std::time::Duration;
use anyhow::Result;
use tokio::sync::mpsc;
use tokio::time::interval;

mod app;
mod event;
mod config;
mod action;
mod docker;
mod ui;
mod types;

use app::App;
use action::Action;
use event::{Event, event_loop};
//use event::{Event};
//use docker::stats::docker_poller;
//use docker::action_handler::action_handler;

#[tokio::main]
async fn main() -> Result<()> {
    let (event_tx, event_rx) = mpsc::channel::<Event>(100);
    let (action_tx, action_rx) = mpsc::channel::<Action>(100);
    let tx = event_tx.clone();
    tokio::spawn(async move {
        event_loop(tx).await;
    });
    let mut app = App::new(event_rx, action_tx);
    app.run().await

}