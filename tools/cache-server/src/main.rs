// SPDX-FileCopyrightText: © 2023 Austin Seipp
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Happy Fun Ball. Do not taunt.

use std::str::FromStr;

use clap::Parser;
use tracing_subscriber::{filter, prelude::*};

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(name = "buck2-cache-server", author = "Austin Seipp", version = "0.1.0")]
struct Cli {
    /// The address to listen on
    #[clap(short, long, default_value = "[::1]:8080")]
    address: String,

    /// `tracing` filter for the console logs.
    #[clap(long, default_value = "info")]
    console_log: String,

    /// TLS key
    #[clap(long)]
    tls_key: Option<String>,

    /// TLS certificate
    #[clap(long)]
    tls_cert: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let tokio_console_layer = console_subscriber::spawn();
    let cli_console_layer = tracing_subscriber::fmt::layer()
        .with_filter(filter::LevelFilter::from_str(cli.console_log.as_str()).unwrap());

    tracing_subscriber::registry()
        .with(tokio_console_layer)
        .with(cli_console_layer)
        //  .with(..potential additional layer..)
        .init();

    let address = cli.address.parse().unwrap();

    tracing::info!(
        message = "Starting buck2-cache-server",
        address = format!("{}", address)
    );

    reapi_grpc::start_reapi_grpc(reapi_grpc::ReapiGrpcSettings {
        address,
        tls_key: cli.tls_key,
        tls_cert: cli.tls_cert,
    })
    .await?;
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------------

pub mod protos;
pub mod reapi_grpc;
pub mod service;
pub mod store;

// ---------------------------------------------------------------------------------------------------------------------
