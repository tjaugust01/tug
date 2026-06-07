pub enum Event {
    Tick,
    Key(crossterm::event::KeyEvent),
    ContainersUpdated(Vec<ContainerInfo>),
    StatsUpdated { id: String, stats: ContainerStats },
    LogLine { source: String, line: String },
    Error(String),
}