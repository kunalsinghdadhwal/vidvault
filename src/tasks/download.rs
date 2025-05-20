use crate::args::DownloadParams;
use std::process::Command;
use youtube_dl::download_yt_dlp;

pub async fn run_download(args: DownloadParams) -> anyhow::Result<()> {
    let yl_dlp_path = download_yt_dlp(".").await?;
    let url = args.url.expect("URL was not provided by the user");

    if !yl_dlp_path.exists() {
        println!("yt-dlp was not found");
        return Ok(());
    }

    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
    let download_path = format!("downloads__{}", timestamp);

    println!("Starting download...");

    let output = Command::new(yl_dlp_path)
        .arg("-f")
        .arg("best")
        .arg("-o")
        .arg(download_path.clone())
        .arg(url)
        .output()
        .expect("Failed to execute yt-dlp");

    if output.status.success() {
        println!("Video downloaded successfully");
        println!(
            "Video saved to: {}",
            std::fs::canonicalize(download_path).unwrap().display()
        );
    } else {
        println!("Error downloading video");
        println!("Status: {}", output.status);
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
