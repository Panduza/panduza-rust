mod worlds;
use worlds::BasicsWorld;
use cucumber::World;

#[tokio::main]
async fn main() {
    BasicsWorld::run(
        "tests/features/basics/boolean.feature",
    ).await;
}
