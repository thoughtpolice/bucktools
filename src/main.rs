//! Happy Fun Ball. Do not taunt.

// ---------------------------------------------------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:8080".parse().unwrap();

    reapi_grpc::start_reapi_grpc(address).await?;
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------------

mod protos;
mod reapi_grpc;
mod remote_asset;
mod remote_execution;
mod remote_logstream;

// ---------------------------------------------------------------------------------------------------------------------
