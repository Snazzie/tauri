// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::path::{Path, PathBuf};

use crate::{
  helpers::updater_signature::{read_key_from_file, sign_file},
  Result,
};
use anyhow::Context;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about = "Sign a file")]
pub struct Options {
  /// Load the private key from a file
  #[clap(short = 'k', long, conflicts_with("private_key_path"))]
  private_key: Option<String>,
  /// Load the private key from a string
  #[clap(short = 'f', long, conflicts_with("private_key"))]
  private_key_path: Option<PathBuf>,
  /// Set private key password when signing
  #[clap(short, long)]
  password: Option<String>,
  /// Sign the specified file
  #[clap(short, long)]
  file: Option<PathBuf>,
}

pub fn command(mut options: Options) -> Result<()> {
  options.private_key = if let Some(private_key) = options.private_key_path {
    Some(read_key_from_file(Path::new(&private_key)).expect("Unable to extract private key"))
  } else {
    options.private_key
  };
  if options.private_key.is_none() {
    return Err(anyhow::anyhow!(
      "Key generation aborted: Unable to find the private key".to_string(),
    ));
  }

  if options.password.is_none() {
    println!("Signing without password.");
  }

  let (manifest_dir, signature) = sign_file(
    options.private_key.unwrap(),
    options.password.unwrap(),
    options.file.unwrap(),
  )
  .with_context(|| "failed to sign file")?;

  println!(
           "\nYour file was signed successfully, You can find the signature here:\n{}\n\nPublic signature:\n{}\n\nMake sure to include this into the signature field of your update server.",
           manifest_dir.display(),
           signature
         );

  Ok(())
}
