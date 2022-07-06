//! A template CLI with clap and input validation with validator
//! https://github.com/clap-rs/clap/blob/v3.2.7/examples/README.md
//! https://docs.rs/clap/latest/clap/index.html
//! https://github.com/Keats/validator

use clap::Parser;
use validator::{Validate, ValidationError};

/// A template CLI
#[derive(Clone, Default, Debug, Parser, Validate)]
#[clap(author, version, about, long_about = None, bin_name = "gadget")]
pub(crate) struct Args {
	#[clap(short, long, default_value = " 🪁")]
	#[validate(length(max = 20))]
	pub name:  String,
	/// Number of times to greet
	#[clap(short, long, default_value_t = 1)]
	#[validate(range(min = 1, max = 64))]
	pub count: u8,
}

// https://lib.rs/crates/clap-cargo
#[cfg(feature = "clap_cargo")]
#[derive(Clone, Debug, Parser)]
pub(crate) struct CliInfo {
	#[clap(flatten)]
	pub manifest:  clap_cargo::Manifest,
	// #[cfg(feature = "clap_cargo")]
	#[clap(flatten)]
	pub workspace: clap_cargo::Workspace,
	// #[cfg(feature = "clap_cargo")]
	#[clap(flatten)]
	pub features:  clap_cargo::Features,
}
