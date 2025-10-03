use std::vec;

use anyhow::{self, Ok};

use inquire::{CustomType, Select, Text};

use crate::args::{Commands, DislodgeParams, DownloadParams, EmbedParams};

pub async fn enrich_arguments(args: Option<Commands>) -> anyhow::Result<Commands> {
    Ok(match args {
        Some(Commands::Embed(embed_args)) => {
            Commands::Embed(enrich_embed_params(embed_args).await?)
        }
        Some(Commands::Dislodge(dislodge_args)) => {
            Commands::Dislodge(enrich_dislodge_params(dislodge_args).await?)
        }
        Some(Commands::Download(download_args)) => {
            Commands::Download(enrich_download_params(download_args).await?)
        }
        None => {
            let options = vec!["Embed", "Dislodge", "Download"];

            let modes = Select::new("Pick what what you want to do with the program", options)
                .with_help_message("Embed: Create a video from files\nDownload: Dowload Files from Youtube\nDislodge: Return Files from an embedded video")
                .prompt()
                .unwrap();

            match modes {
                "Embed" => Commands::Embed(enrich_embed_params(EmbedParams::default()).await?),
                "Dislodge" => {
                    Commands::Dislodge(enrich_dislodge_params(DislodgeParams::default()).await?)
                }
                "Download" => {
                    Commands::Download(enrich_download_params(DownloadParams::default()).await?)
                }
                _ => unreachable!(),
            }
        }
    })
}

async fn enrich_embed_params(mut args: EmbedParams) -> anyhow::Result<EmbedParams> {
    if args.in_path.is_none() {
        let path = Text::new("Enter the path to the file you want to embed")
            .with_default("src/tests/test.txt")
            .prompt()
            .unwrap();

        args.in_path = Some(path);
    }

    if args.mode.is_none()
        && args.block_size.is_none()
        && args.threads.is_none()
        && args.fps.is_none()
        && args.resolution.is_none()
    {
        let presets = vec![
            "Optimal Compression Resistance",
            "Paranoid Compression Resistance",
            "Maximum Efficiency",
            "Custom",
        ];

        let preset = Select::new(
            "You can choose a preset or customize the parameters",
            presets.clone(),
        )
        .with_help_message(
            "Any amount of compression on Maximum Efficiency will corrupt all hopes and dreams",
        )
        .prompt()
        .unwrap();

        match preset {
            "Maximum Efficiency" => {
                args.preset = Some(crate::args::EmbedPreset::MaxEfficiency);
                return Ok(args);
            }
            "Optimal Compression Resistance" => {
                args.preset = Some(crate::args::EmbedPreset::Optimal);
                return Ok(args);
            }
            "Paranoid Compression Resistance" => {
                args.preset = Some(crate::args::EmbedPreset::Paranoid);
                return Ok(args);
            }
            _ => (),
        }
    }

    if args.mode.is_none() {
        let out_modes = vec!["Colored", "B/W (Binary)"];
        let out_mode = Select::new("Pick the mode of data embedding", out_modes.clone())
            .with_help_message("Colored mod is useless if video undergoes compression at any point, B/W is the best option")
            .prompt()
            .unwrap();

        args.mode = Some(match out_mode {
            "Colored" => crate::args::EmbedOutputMode::Colored,
            "B/W (Binary)" => crate::args::EmbedOutputMode::Binary,
            _ => unreachable!(),
        });
    }

    if args.block_size.is_none() {
        let size = CustomType::<i32>::new("What should be size of the blocks ?")
            .with_error_message("Please enter a valid number")
            .with_help_message("Bigger blocks mean less compression, smaller blocks mean more compression, 2-5 is recommmended")
            .with_default(2)
            .prompt()?;

        args.block_size = Some(size);
    }

    if args.threads.is_none() {
        let threads = CustomType::<i32>::new("How many threads do you want to use ?")
            .with_error_message("Please enter a valid number")
            .with_help_message("More threads mean faster processing")
            .with_default(8)
            .prompt()?;

        args.threads = Some(threads as usize);
    }

    if args.fps.is_none() {
        let fps = CustomType::<i32>::new("What should be the fps of the video ?")
            .with_error_message("Please enter a valid number")
            .with_help_message(
                "Decreasing fps may decrease chance of compression 10 is recommended",
            )
            .with_default(10)
            .prompt()
            .expect("Invalid fps");

        args.fps = Some(fps);
    }

    let resolutions = vec!["144p", "240p", "360p", "480p", "720p"];

    if args.resolution.is_none() {
        let resolution = Select::new("Pick the resolution of the video", resolutions.clone())
            .with_help_message(
                "Higher resolution means more data, but also more compression, 720p is recommended",
            )
            .prompt()
            .unwrap();
        args.resolution = Some(resolution.to_string());
    }

    Ok(args)
}

async fn enrich_download_params(mut args: DownloadParams) -> anyhow::Result<DownloadParams> {
    if args.url.is_none() {
        let url = Text::new("Enter the URL of the video you want to download")
            .with_help_message("You can use youtube-dl format strings")
            .prompt()
            .unwrap();

        args.url = Some(url);
    }

    Ok(args)
}

async fn enrich_dislodge_params(mut args: DislodgeParams) -> anyhow::Result<DislodgeParams> {
    if args.in_path.is_none() {
        let in_path = Text::new("Enter the path to your video file")
            .with_default("output.avi")
            .prompt()
            .unwrap();
        args.in_path = Some(in_path);
    }

    if args.out_path.is_none() {
        let out_path = Text::new("Enter the path to the output file")
            .with_help_message("Please include namde of the file and extension")
            .prompt()
            .unwrap();
        args.out_path = Some(out_path);
    }

    Ok(args)
}
