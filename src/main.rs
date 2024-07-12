// SPDX-FileCopyrightText: © 2023 Austin Seipp
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Happy Fun Ball. Do not taunt.

use clap::Parser;

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(name = "reapi-server", author = "Austin Seipp", version = "0.1.0")]
struct Cli {
    /// The address to listen on
    #[clap(short, long, default_value = "[::1]:8080")]
    address: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    console_subscriber::init();
    let address = cli.address.parse().unwrap();
    println!("Starting reapi-server on {}", address);
    reapi_grpc::start_reapi_grpc(address).await?;
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------------

pub mod protos;
pub mod reapi_grpc;
pub mod service;
pub mod store;

// ---------------------------------------------------------------------------------------------------------------------
