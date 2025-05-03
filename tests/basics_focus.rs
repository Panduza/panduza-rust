mod worlds;
use worlds::BasicsWorld;
use cucumber::World;

#[tokio::main]
async fn main() {

    //
    // @focus
    //

    let features = [
        "tests/features/basics/reactor.feature",
        "tests/features/basics/boolean.feature",
    ];

    for feature in features.iter() {
        BasicsWorld::filter_run(feature, 
        |_feature, _rule, scenario| scenario.tags.contains(&"focus".to_string())
    ).await;

    }

    // BasicsWorld::filter_run("tests/features/basics/boolean.feature", 
    //     |feature, rule, scenario| scenario.name == "Manage WO & RO boolean attributes"
    // ).await;
}
