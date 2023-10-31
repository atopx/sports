use clap::Parser;

/// A fast port scanner
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct ScanIp {
    /// The target to scan
    #[arg(long)]
    pub target: String,

    /// Number of concurrency to scan
    #[arg(short, long, default_value_t = 1024)]
    pub concurrency: usize,

    /// Scan all 65535 ports [default: false]
    #[arg(short, long, default_value_t = false)]
    pub full: bool,

    /// Seconds of connection timeout
    #[arg(short, long, default_value_t = 3)]
    pub timeout: u64,

    /// Display detailed information [default: false]
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
