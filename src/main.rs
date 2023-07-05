use std::path::Path;

use crate::{
    args::{Cli, Commands},
    chunk::Chunk,
    png::Png,
};
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod png;

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Print { file }) => {
            let png = read_png(file)?;
            println!("{}", png);
        }
        Some(Commands::Decode { file, chunk_type }) => {
            let png = read_png(file)?;
            println!(
                "Secret message: {:?}",
                png.chunk_by_type(&chunk_type.to_string())
                    .and_then(|chunk| chunk.data_as_string().ok())
            )
        }
        Some(Commands::Remove { file, chunk_type }) => {
            let mut png = read_png(file)?;
            let removed_chunk = png.remove_chunk(&chunk_type.to_string())?;
            write(file, &png)?;
            println!(
                "Removed {} with message: {}",
                removed_chunk.chunk_type(),
                removed_chunk.data_as_string()?
            );
        }
        Some(Commands::Encode {
            file,
            chunk_type,
            message,
            output_file,
        }) => {
            let mut png = read_png(file)?;
            png.append_chunk(Chunk::new(chunk_type.clone(), message.as_bytes().to_vec()));
            write(output_file.as_ref().unwrap_or(file), &png)?;
        }
        Some(Commands::Detect { file }) => {
            let png = read_png(file)?;
            png.chunks()
                .iter()
                .filter_map(|chunk| chunk.data_as_string().ok())
                .filter(|data| {
                    !data.trim().is_empty()
                        && (data
                            .trim()
                            .chars()
                            .filter(|char| char.is_ascii())
                            .filter(|char| !(char.is_control() || char.is_whitespace()))
                            .count() as f64
                            / data.len() as f64)
                            >= 0.8
                })
                .for_each(|data| println!("Potential message: {}", data))
        }
        _ => {}
    }

    Ok(())
}

fn read_png(path: &Path) -> Result<Png> {
    let bytes = std::fs::read(path)?;
    Png::try_from(bytes.as_slice())
}

fn write(path: &Path, png: &Png) -> Result<()> {
    Ok(std::fs::write(path, png.as_bytes())?)
}
