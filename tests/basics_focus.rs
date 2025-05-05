mod worlds;
use worlds::BasicsWorld;
use cucumber::World;

#[tokio::main]
async fn main() {

    //
    // @focus
    //

    BasicsWorld::filter_run("tests/features/basics",
        |_feature, _rule, scenario| scenario.tags.contains(&"focus".to_string())
    ).await;

    // BasicsWorld::filter_run("tests/features/basics/boolean.feature", 
    //     |feature, rule, scenario| scenario.name == "Manage WO & RO boolean attributes"
    // ).await;
}
