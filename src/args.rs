pub use clap::Parser;
pub use std::path::PathBuf;

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
        .literal(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
        .valid(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
        .invalid(
            anstyle::Style::new()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}

#[derive(Parser)]
#[clap(version, about = "file-watcher", styles=get_styles() )]
pub struct Args {
    /// File to watch
    pub path: PathBuf,

    /// Command to execute
    pub command: String,

    /// Check interval time
    #[clap(short, long, default_value = "2.5")]
    pub time: f32,

    /// Whether a deep check must be made by contents length
    #[clap(short, long)]
    pub length: bool,

    /// Whether a deep check must be made by contents sum
    #[clap(short, long)]
    pub sum: bool,

    /// Show command
    #[clap(short, long)]
    pub verbose: bool,

    /// Ignore errors when executing command and don't panic
    #[clap(short, long)]
    pub error_skip: bool,

    /// Interactive
    #[clap(short, long)]
    pub interactive: bool,
}
