use async_pipeline::begin;
use async_pipeline::link::{Linkable, Pipeline};
use async_pipeline::Error;
use std::time::Duration;
use tokio::time::sleep;

fn square(n: i32) -> String {
    (n * n).to_string()
}

fn from_str(n: String) -> Result<i32, Error> {
    Ok(n.parse::<i32>()?)
}

#[tokio::main]
async fn main() {
    let pipeline = begin()
        .then(square)
        .then_async(|mut s| async {
            s.push('9');
            println!("wait 1 second");
            sleep(Duration::from_secs(1)).await;
            s
        })
        .then_result(from_str)
        .then_async_result(|i| async move {
            match i % 7 {
                0 => Err("inject error".into()),
                _ => Ok(i),
            }
        })
        .then(|i| {
            println!("last step");
            i
        });
    let r1 = pipeline.process(2).await;
    println!("{}", r1.err().unwrap());
    //assert!(r1.err().unwrap());
    let r2 = pipeline.process(3).await;
    println!("{}", r2.unwrap());
}
