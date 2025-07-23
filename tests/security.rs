mod worlds;
use cucumber::World;
use worlds::SecurityWorld;
use worlds::BasicsWorld;

#[tokio::main]
async fn main() {
    SecurityWorld::cucumber()
        .max_concurrent_scenarios(1)
        .run("tests/features/security")
        .await;
}
