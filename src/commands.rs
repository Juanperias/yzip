mod compress;
mod decompress;

use anyhow::Result;
use clap::Subcommand;
use compress::compress_command;
use decompress::decompress_command;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Compress {
        file: String,
    },
    Descompress {
        file: String,
        #[clap(long, short)]
        dist: String,
    },
}

impl Commands {
    pub fn exec(&self) -> Result<()> {
        match self {
            Commands::Compress { file } => {
                compress_command(file.to_owned())?;
            }
            Commands::Descompress { file, dist } => {
                decompress_command(file.to_owned(), dist.to_owned())?;
            }
        }

        Ok(())
    }
}
