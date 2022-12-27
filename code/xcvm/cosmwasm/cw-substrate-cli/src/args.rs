use crate::{new_cmd::NewCommand, substrate_client::SubstrateCommand};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	#[command(subcommand)]
	pub main_command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
	New(NewCommand),
	Substrate(SubstrateCommand),
}