mod worlds;
use cucumber::World;
use worlds::BasicsWorld;

#[tokio::main]
async fn main() {
    BasicsWorld::run("tests/features/basics").await;
}
