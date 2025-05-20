use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    // Embed data into a video
    Embed(EmbedParams),

    // Download a video from a URL
    Download(DownloadParams),

    // Extract data from a video
    Dislodge(DislodgeParams),
}

#[derive(Debug, Clone, ValueEnum)]
pub enum EmbedPreset {
    // Optimal
    Optimal,

    // Compression Resistant
    Paranoid,

    // Fast Encoding and Small Size
    MaxEfficiency,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum EmbedOutputMode {
    Colored,

    Binary,
}

impl From<EmbedOutputMode> for crate::settings::OutputMode {
    fn from(mode: EmbedOutputMode) -> Self {
        match mode {
            EmbedOutputMode::Colored => Self::Color,
            EmbedOutputMode::Binary => Self::Binary,
        }
    }
}

#[derive(Args, Default, Debug)]
pub struct EmbedParams {
    #[arg(short, long)]
    pub in_path: Option<String>,

    #[arg(short, long)]
    pub preset: Option<EmbedPreset>,

    #[arg(long)]
    pub mode: Option<EmbedOutputMode>,

    #[arg(long)]
    pub block_size: Option<i32>,

    #[arg(long)]
    pub threads: Option<usize>,

    #[arg(long)]
    pub fps: Option<i32>,

    #[arg(long)]
    pub resolution: Option<String>,
}

#[derive(Args, Default)]
pub struct DownloadParams {
    #[arg(short, long)]
    pub url: Option<String>,
}

#[derive(Args, Default)]
pub struct DislodgeParams {
    #[arg(short, long)]
    pub in_path: Option<String>,

    #[arg(short, long)]
    pub out_path: Option<String>,
}
