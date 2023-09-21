extern crate common;

use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Download { query: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Download { query } => {
            let mut res = common::search(query).await?;
            // println!("{:#?}", res);
            if res.songs.data.is_empty() {
                println!("No songs found!!");
            } else {
                let mut fzf = Command::new("fzf")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("Uable to execute fzf");

                if let Some(mut stdin) = fzf.stdin.take() {
                    res.songs.data.sort_by_key(|res| res.position);
                    for song in &res.songs.data {
                        writeln!(stdin, "{}: {} - {}", song.position, song.title, song.album)
                            .expect("Failed to write to FZF stdin");
                    }
                }

                if let Some(stdout) = fzf.stdout.take() {
                    let reader = BufReader::new(stdout);
                    for selected_result in reader.lines().flatten() {
                        if let Some((s, _)) = selected_result.split_once(':') {
                            if let Ok(pos) = s.parse::<u32>() {
                                if let Some(finalres) =
                                    res.songs.data.iter().find(|f| f.position == pos)
                                {
                                    let song = common::get_song(&finalres.id).await?;
                                    println!("{:#?}", song);
                                    let url =
                                        common::get_media_url(&song.encrypted_media_url, true)?;
                                    println!("Downloading: {} - {}", song.song, song.album);

                                    let mut response = reqwest::get(url).await?;

                                    let content_length =
                                        response.content_length().expect("File Invalid");

                                    let mut destination =
                                        File::create(format!("{}.mp3", song.song))
                                            .await
                                            .expect("Cannot create file");

                                    let progress_bar = ProgressBar::new(content_length);
                                    progress_bar.set_style(ProgressStyle::default_bar()
                                            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})").expect("???")
                                            .progress_chars("#>-"));

                                    while let Some(bytes) = response.chunk().await? {
                                        let bytes_read = bytes.len();
                                        destination.write_all(&bytes).await?;
                                        progress_bar.inc(bytes_read as u64);
                                    }
                                    progress_bar.finish();
                                }
                            }
                        }
                    }
                }

                let status = fzf.wait().expect("Failed to wait for FZF process");
                if !status.success() {
                    eprintln!("FZF process exited with non-zero status: {:?}", status);
                }
            }
        }
    }

    Ok(())
}
