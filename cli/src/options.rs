use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct CliOptions {
    #[arg(short, long)]
    pub debug: Option<LogLevel>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
}
