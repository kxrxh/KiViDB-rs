use tokio::runtime::Runtime;

mod database;
mod interface;
fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        interface::rest::start().await;
    });
    println!("Hello, world!");
}
