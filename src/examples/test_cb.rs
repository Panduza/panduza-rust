use std::sync::Arc;

use tokio::time::sleep;
use tokio::time::Duration;

use futures::future::BoxFuture;
use futures::Future;

type POK = fn(u8) -> BoxFuture<'static, Result<bool, ()>>;

// fn wrap<F>(function: F) -> POK
// where
//     F: Future<Output = Result<bool, ()>> + Send + 'static,
// {
//     return |val: u8| -> BoxFuture<'static, Result<bool, ()>> { Box::pin(function) };
// }

fn test(val: u8) -> BoxFuture<'static, Result<bool, ()>> {
    Box::pin(async move {
        println!("cbbbbb!!! {}", val);
        Ok(true)
    })
}

struct Test {
    b: Option<BoxFuture<'static, Result<bool, ()>>>,
}

impl Test {
    fn set_b<F>(&mut self, function: F)
    where
        F: Future<Output = Result<bool, ()>> + Send + 'static,
    {
        self.b = Some(Box::pin(function));
    }

    // async fn fire(&self) {
    //     self.b.unwrap().await
    // }
}

#[tokio::main]
async fn main() {
    println!("test cb");

    let mut cbb: Option<POK> = None;

    cbb = Some(test);

    // let a = wrap(async move {
    //     println!("neww!! !!!!");
    //     Ok(true)
    // });
    // cbb = Some(a);

    let mut abc = Test { b: None };
    att.on_change(async move {
        println!("neww!! !!!!");
        Ok(true)
    });

    tokio::spawn(cbb.unwrap()(1));

    tokio::spawn(test(1));

    tokio::spawn(test(2));

    tokio::spawn(test(3));

    tokio::spawn(test(4));

    tokio::spawn(test(5));

    sleep(Duration::from_secs(2)).await
}
