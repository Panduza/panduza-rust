use tokio::time::sleep;
use tokio::time::Duration;

use futures::future::BoxFuture;

fn test(val: u8) -> BoxFuture<'static, Result<bool, ()>> {
    Box::pin(async move {
        println!("cbbbbb!!! {}", val);
        Ok(true)
    })
}

#[tokio::main]
async fn main() {
    println!("test cb");

    let mut cbb: Option<fn(u8) -> BoxFuture<'static, Result<bool, ()>>> = None;

    cbb = Some(test);

    tokio::spawn(cbb.unwrap()(1));

    tokio::spawn(test(1));

    tokio::spawn(test(2));

    tokio::spawn(test(3));

    tokio::spawn(test(4));

    tokio::spawn(test(5));

    sleep(Duration::from_secs(2)).await
}
