use anyhow::Result;
use sshx_core::proto::{sshx_service_client::SshxServiceClient, HelloRequest};

use crate::common::*;

pub mod common;

#[tokio::test]
async fn test_rpc() -> Result<()> {
    let server = TestServer::new()?;
    let mut client = SshxServiceClient::connect(server.endpoint()).await.unwrap();

    let req = HelloRequest {
        name: "adam".into(),
    };
    let resp = client.hello(req).await.unwrap();
    assert_eq!(&resp.into_inner().message, "Hello adam!");

    Ok(())
}