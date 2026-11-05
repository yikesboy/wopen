use crate::backend::Backend;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "wopen", version, about)]
pub struct Cli {
    pub url: String,

    #[arg(short = 'b', long = "browser")]
    pub browser: String,

    #[arg(short = 'm', long = "match")]
    pub match_patterns: Vec<String>,

    #[arg(long = "new-window-flag", default_value = "--new-window")]
    pub new_window_flag: String,

    #[arg(long = "backend", value_enum, default_value_t = Backend::Auto)]
    pub backend: Backend,
}
