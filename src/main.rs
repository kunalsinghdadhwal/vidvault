mod args;
mod ethcer;
mod settings;
mod source;
mod tasks;
mod timer;
mod ui;

use anyhow::Ok;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Welcome to the Video Embedding System");
    println!(
        "This system enables secure data transmission by converting files into a video format resistant to compression artifacts."
    );

    println!("\nUsage Instructions:");
    println!(
        "1. Prepare your files by archiving them into a single compressed format (e.g., ZIP)."
    );
    println!("2. Use the 'Embed' option to encode the archive into a video file.");
    println!("3. Transmit or store the generated video securely.");
    println!("4. Use the 'Download' option to retrieve the video file.");
    println!("5. Use the 'Dislodge' option to extract the original files from the encoded video.");
    println!("6. Ensure data integrity and verify successful extraction.\n");

    println!(
        "For optimal results, choose the appropriate encoding settings based on your security and efficiency requirements."
    );

    let mut args = args::Arguments::parse();

    let new_command = ui::enrich_arguments(args.command).await?;
    args.command = Some(new_command);

    tasks::run_by_args(args).await?;
    Ok(())
}
