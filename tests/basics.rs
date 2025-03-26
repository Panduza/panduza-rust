mod worlds;
use worlds::BooleanWorld;



use cucumber::{given, World};



#[tokio::main]
async fn main() {

    
    

    BooleanWorld::run(
        "tests/features/basics/boolean.feature",
    ).await;
}
