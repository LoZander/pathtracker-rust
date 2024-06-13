use pathtracker_rust::conditions::{self, CondTrigger, ConditionManager};


#[test]
fn add_condition() {
    let mut cm = ConditionManager::new();
    let condition = conditions::Condition {
        name: String::from("Bleed"),
        level: 10,
        trigger: CondTrigger::EndOfTurn,
        reduction: None
    };

    cm.add("test character", condition.clone());

    println!("{cm:?}");

    assert!(cm.get("test character").unwrap().contains(&condition))
}
