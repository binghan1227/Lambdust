use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Show unique ID after the variable name
    #[arg(short = 'u', long, default_value_t = false)]
    pub unique_id: bool,

    /// Maximum number of steps to trace
    #[arg(short = 't', long, default_value_t = 10)]
    pub trace: usize,

    /// Print each step
    #[arg(short = 'p', long, default_value_t = false)]
    pub print_step: bool,
}
