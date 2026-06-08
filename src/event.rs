use std::time::Duration;
use tokio::sync::mpsc;
use crate::types::{ComposeStack, ContainerInfo, ContainerStats, ImageInfo};
use crossterm::event::{EventStream, Event as CrosstermEvent};
use tokio::time::interval;
use futures::StreamExt;

pub enum Event {
    Tick,
    Key(crossterm::event::KeyEvent),
    ContainersUpdated(Vec<ContainerInfo>),
    ImagesUpdated(Vec<ImageInfo>),
    StacksUpdated(Vec<ComposeStack>),
    StatsUpdated { id: String, stats: ContainerStats },
    LogLine { source: String, line: String },
    Error(String),
}
pub async fn event_loop(tx: mpsc::Sender<Event>) {
    let mut reader = EventStream::new();
    let mut tick = interval(Duration::from_millis(250));

    loop {
        tokio::select! {
            _ = tick.tick() => {
                let _ = tx.send(Event::Tick).await;
            }
            Some(Ok(event)) = reader.next() => {
                if let CrosstermEvent::Key(key) = event {
                    let _ = tx.send(Event::Key(key)).await;
                }
            }
        }
    }
}