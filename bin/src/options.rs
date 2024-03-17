use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct CliOptions {
    #[arg(short, long)]
    pub debug: Option<LogLevel>,

    #[arg(short, long, default_value_t = true)]
    pub interactive: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}
