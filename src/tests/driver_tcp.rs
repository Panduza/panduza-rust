mod worlds;
use cucumber::World;
use worlds::ClientWorld;

#[tokio::main]
async fn main() {
    ClientWorld::run("src/tests/features/driver_tcp/client.feature").await;
}
