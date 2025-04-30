mod worlds;
use worlds::BooleanWorld;



use cucumber::{World};



#[tokio::main]
async fn main() {
    BooleanWorld::run(
        "tests/features/basics/boolean.feature",
    ).await;
}
