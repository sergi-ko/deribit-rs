#![feature(async_await)]

use deribit::models::{Either, HeartbeatType, SetHeartbeatRequest, TestRequest};
use deribit::DeribitBuilder;
use dotenv::dotenv;
use failure::Fallible;
use futures::StreamExt;
use runtime_tokio::Tokio;

#[runtime::main(Tokio)]
async fn main() -> Fallible<()> {
    dotenv().unwrap();

    let drb = DeribitBuilder::default().testnet(true).build().unwrap();

    let (mut client, mut subscription) = drb.connect().await?;

    let resp = client.call(SetHeartbeatRequest::with_interval(10)).await?;
    println!("Hearbet response {:?}", resp.await?);

    while let Some(sub) = subscription.next().await {
        match sub {
            Either::Right(l) => match l.params.r#type {
                HeartbeatType::TestRequest => {
                    println!("Test Requested");
                    client.call(TestRequest::default()).await?;
                }
                _ => println!("Heartbeat"),
            },
            _ => {}
        }
    }

    Ok(())
}
