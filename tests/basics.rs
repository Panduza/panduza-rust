mod worlds;
use worlds::BasicsWorld;
use cucumber::World;

#[tokio::main]
async fn main() {
    let features = [
        "tests/features/basics/reactor.feature",
        "tests/features/basics/boolean.feature",
    ];

    for feature in features.iter() {
        BasicsWorld::run(feature).await;
    }
}
