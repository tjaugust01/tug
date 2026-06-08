use crossterm::event::{KeyCode, KeyEvent};
use crate::action::Action;
use crate::event::Event;
use crate::types::{ComposeStack, ContainerInfo, ImageInfo};
use tokio::sync::mpsc;
use crate::ui;
use anyhow::Result;

#[derive(PartialEq)]
pub enum Tab {
    Containers,
    Images,
    Compose,
}
pub struct App {
    event_rx: mpsc::Receiver<Event>,
    action_tx: mpsc::Sender<Action>,
    pub tab: Tab,                       // Current View
    pub containers: Vec<ContainerInfo>, // List of all containers
    pub images: Vec<ImageInfo>,         // List of all Images
    pub stacks: Vec<ComposeStack>,      // List off all docker compose Stacks
    pub selected: usize,
    pub log_lines: Vec<String>,
    pub quit: bool,
}

impl App {
    pub fn new(event_rx: mpsc::Receiver<Event>, action_tx: mpsc::Sender<Action>) -> Self {
        Self {
            event_rx,
            action_tx,
            tab: Tab::Containers,
            containers: vec![],
            images: vec![],
            stacks: vec![],
            selected: 0,
            log_lines: vec![],
            quit: false,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
            let mut terminal = ratatui::init();
            while !self.quit {
                match self.event_rx.recv().await {
                    Some(Event::Tick) => {
                        terminal.draw(|frame| ui::render(frame, self))?;
                }
                Some(Event::Key(key)) =>{
                    self.handle_key(key).await;
                }
                Some(Event::ContainersUpdated(list)) => self.containers = list,
                Some(Event::ImagesUpdated(list))     => self.images = list,
                Some(Event::StacksUpdated(list))     => self.stacks = list,
                Some(Event::StatsUpdated {id, stats}) => {
                    if let Some(c) = self.containers.iter_mut().find(|c| c.id == id) {
                        c.stats = Some(stats);
                    }
                }
                Some(Event::LogLine { line, .. }) => self.log_lines.push(line),
                None => break,
                _ => {}
            }
        }
        ratatui::restore();
        Ok(())
    }

    async fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q')      => self.quit = true,
            KeyCode::Tab            => self.next_tab().await,
            KeyCode::Down | KeyCode::Char('j') => self.select_next(),
            KeyCode::Up   | KeyCode::Char('k') => self.select_prev(),

            KeyCode::Char('s') => {
                let id = self.selected_container_id();
                self.send(Action::StopContainer(id)).await;
            }
            KeyCode::Char('r') => {
                let id = self.selected_container_id();
                self.send(Action::RestartContainer(id)).await;
            }
            KeyCode::Enter => {
                let id = self.selected_container_id();
                self.send(Action::TailLogs(id)).await;
            }
            _ => {}
        }
    }

    async fn next_tab(&mut self) {
        self.tab = match self.tab {
            Tab::Containers => Tab::Images,
            Tab::Images     => Tab::Compose,
            Tab::Compose    => Tab::Containers,
        };
        self.selected = 0;
    }

    async fn send(&self, action: Action) {
        let _ = self.action_tx.send(action).await;
    }

    fn select_next(&mut self) {
        let max = self.current_len();
        if max > 0 { self.selected = (self.selected + 1) % max; }
    }

    fn select_prev(&mut self) {
        if self.selected > 0 { self.selected -= 1; }
    }

    fn current_len(&self) -> usize {
        match self.tab {
            Tab::Containers => self.containers.len(),
            Tab::Images     => self.images.len(),
            Tab::Compose    => self.stacks.len(),
        }
    }

    fn selected_container_id(&self) -> String {
        self.containers
            .get(self.selected)
            .map(|c| c.id.clone())
            .unwrap_or_default()
    }
}
