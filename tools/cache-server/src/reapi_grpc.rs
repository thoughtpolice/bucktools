// SPDX-FileCopyrightText: © 2023 Austin Seipp
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{net::SocketAddr, time::Duration};

use crate::protos::google::bytestream::byte_stream_server::ByteStreamServer;

use crate::protos::build::bazel::remote::execution::v2::{
    action_cache_server::ActionCacheServer, capabilities_server::CapabilitiesServer,
    content_addressable_storage_server::ContentAddressableStorageServer,
    execution_server::ExecutionServer,
};
use crate::protos::google::longrunning::operations_server::OperationsServer;

// ---------------------------------------------------------------------------------------------------------------------

const FILE_DESCRIPTOR_SET: &[u8] =
    include_bytes!(concat!(env!("PROTOBUFS"), "/reapi_descriptor.bin"));

pub struct ReapiGrpcSettings {
    pub address: SocketAddr,
    pub tls_key: Option<String>,
    pub tls_cert: Option<String>,
}

pub async fn start_reapi_grpc(
    settings: ReapiGrpcSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::service;

    let (health_reporter, health_service) = tonic_health::server::health_reporter();
    tokio::spawn(report_service_status(health_reporter.clone())); // XXX FIXME (aseipp)

    let cas_service = service::ContentAddressableStorageService::default();
    let action_cache_service = service::ActionCacheService::default();
    let bytestream_service = service::ByteStreamService::default();
    let execution_service = service::ExecutionService::default();
    let capabilities_service = service::CapabilitiesService::default();
    let operations_service = service::OperationsService::default();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(tonic_health::pb::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    tonic::transport::Server::builder()
        .add_service(CapabilitiesServer::new(capabilities_service))
        .add_service(ContentAddressableStorageServer::new(cas_service))
        .add_service(ActionCacheServer::new(action_cache_service))
        .add_service(ExecutionServer::new(execution_service))
        .add_service(ByteStreamServer::new(bytestream_service))
        .add_service(OperationsServer::new(operations_service))
        .add_service(health_service)
        .add_service(reflection_service)
        .serve(settings.address)
        .await?;
    Ok(())
}

async fn report_service_status(mut reporter: tonic_health::server::HealthReporter) {
    use crate::service;
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        reporter
            .set_serving::<CapabilitiesServer<service::CapabilitiesService>>()
            .await;
        reporter
            .set_serving::<ContentAddressableStorageServer<service::ContentAddressableStorageService>>()
            .await;
        reporter
            .set_serving::<ActionCacheServer<service::ActionCacheService>>()
            .await;
        reporter
            .set_serving::<ExecutionServer<service::ExecutionService>>()
            .await;
        reporter
            .set_serving::<ByteStreamServer<service::ByteStreamService>>()
            .await;
    }
}

// XXX FIXME (aseipp): just a placeholder for now to make sure the openssl build
// works
fn _make_tls_acceptor(tls_cert: &str, tls_key: &str) {
    use openssl::ssl::{select_next_proto, SslAcceptor, SslFiletype, SslMethod};
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor
        .set_private_key_file(tls_key, SslFiletype::PEM)
        .unwrap();
    acceptor.set_certificate_chain_file(tls_cert).unwrap();
    acceptor.check_private_key().unwrap();
    let acceptor = acceptor.build();
}
