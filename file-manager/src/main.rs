use clap::{Parser, Subcommand};
use std::io;
use std::path::PathBuf;

use file_manager::{copy, delete, list, move_file};

#[derive(Parser)]
#[command(name = "file-manager")]
#[command(about = "A simple file management system")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List files in a directory
    List { path: PathBuf },
    /// Copy file from source to destination
    Copy { src: PathBuf, dest: PathBuf },
    /// Move file from source to destination
    Move { src: PathBuf, dest: PathBuf },
    /// Delete a file or directory
    Delete { path: PathBuf },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::List { path } => {
            for entry in list(&path)? {
                println!("{}", entry.display());
            }
        }
        Commands::Copy { src, dest } => {
            copy(&src, &dest)?;
        }
        Commands::Move { src, dest } => {
            move_file(&src, &dest)?;
        }
        Commands::Delete { path } => {
            delete(&path)?;
        }
    }
    Ok(())
}
