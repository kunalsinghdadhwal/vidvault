use crate::args::Arguments;

pub mod dislodge;
pub mod download;
pub mod embed;

pub async fn run_by_args(args: Arguments) -> anyhow::Result<()> {
    match args.command.expect("Command was not provided by the user") {
        crate::args::Commands::Embed(args) => embed::run_embed(args).await,

        crate::args::Commands::Dislodge(args) => dislodge::run_dislodge(args).await,

        crate::args::Commands::Download(args) => download::run_download(args).await,
    }
}
