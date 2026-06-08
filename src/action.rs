pub enum Action {
    StartContainer(String),
    StopContainer(String),
    RestartContainer(String),
    TailLogs(String),
    StartStack(String),
    StopStack(String),
    RestartStack(String),
    RefreshAll,
    Quit,
}