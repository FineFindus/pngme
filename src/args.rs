use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::chunk_type::ChunkType;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encode a secret message into the chunk_type chunk
    Encode {
        file: PathBuf,
        chunk_type: ChunkType,
        message: String,
        output_file: Option<PathBuf>,
    },
    /// Decode the message stored in the specified chunk
    Decode {
        file: PathBuf,
        chunk_type: ChunkType,
    },
    /// Remove the message stored in the specified chunk
    Remove {
        file: PathBuf,
        chunk_type: ChunkType,
    },
    /// Print all chunks
    Print { file: PathBuf },
}
