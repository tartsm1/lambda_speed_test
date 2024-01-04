/*
 lambda speed testing
 */

use tokio::runtime::Runtime;
use std::time::Instant;
use std::env;
use aws_smithy_types::Blob;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_lambda::Client;
use aws_sdk_lambda::operation::invoke::InvokeOutput;

fn main() {
    let s: String = env::var("times").unwrap();
    let lambda_name: String = env::var("name").unwrap();
    println!("will call lambda {} : {} times ", &lambda_name, &s);
    let times: i32 = s.parse::<i32>().unwrap();
    let now = Instant::now();
    let rt = Runtime::new().unwrap();
    println!("work in progress ");
    for _ in 0..times {
        // Block the synchronous function to run the asynchronous code
        rt.block_on(async {
            let res = my_async(&lambda_name).await;
            let s = String::from_utf8(res.payload().unwrap().clone()
                .into_inner()).expect("Our bytes should be valid utf8");
            println!("result: {}", if s.contains("152415787.501905") { "Ok" } else { "no match" });
        });
    }
    println!("time total {} msec", now.elapsed().as_millis());
}

async fn get_client() -> Client {
    let region_provider = RegionProviderChain::default_provider()
        .or_else("eu-north-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&config)
}

async fn my_async(lambda_name: &str) -> InvokeOutput {
    let client = get_client();
    let res = client.await.invoke()
        .function_name(lambda_name)
        .payload(Blob::new("{\"command\": \"12345.6789\"}"))
        .send();
    let xx = res.await.map_err(anyhow::Error::from);
    xx.unwrap()
}
