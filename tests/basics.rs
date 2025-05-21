mod worlds;
use cucumber::World;
use worlds::BasicsWorld;

#[tokio::main]
async fn main() {
    // BasicsWorld::run("tests/features/basics").await;

    
    // BasicsWorld::run("tests/features/basics/boolean.feature").await;

    BasicsWorld::cucumber().max_concurrent_scenarios(1)
        .run("tests/features/basics")
        .await;

}
