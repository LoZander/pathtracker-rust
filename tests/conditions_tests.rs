use pathtracker_rust::conditions::{self, Condition, ConditionManager, ConditionType, ReductionTrigger};

#[test]
fn add_condition_adds() {
    let mut cm = ConditionManager::new();
    let condition = conditions::Condition {
        name: String::from("Frightened"),
        level: 3,
        cond_type: ConditionType::ReductionBased { reduction: Some(1), trigger: ReductionTrigger::EndOfTurn(String::from("test_character")) },
    };

    cm.add_conditions("bob", condition.clone());

    assert!(cm.get_conditions("bob").contains(&condition))
}

#[test]
fn remove_condition_removes() {
    let mut cm = ConditionManager::new();
    let condition = conditions::Condition {
        name: String::from("Frightened"),
        level: 3,
        cond_type: ConditionType::ReductionBased { reduction: Some(1), trigger: ReductionTrigger::EndOfTurn(String::from("bob")) },
    };

    cm.add_conditions("bob", condition.clone());
    cm.remove_condition("bob", "Frightened");

    assert!(!cm.get_conditions("bob").contains(&condition))
}

#[test]
fn remove_bleed_doesnt_remove_frighten() {
    let mut cm = ConditionManager::new();
    let bleed = Condition {
        name: String::from("Bleed"),
        level: 5,
        cond_type: ConditionType::Manual
    };

    cm.add_conditions("bob", bleed.clone());
    
    let frightened = Condition {
        name: String::from("Frightened"),
        level: 2,
        cond_type: ConditionType::ReductionBased { 
            reduction: Some(1), 
            trigger: ReductionTrigger::EndOfTurn(String::from("bob"))}
    };

    cm.add_conditions("bob", frightened.clone());
    cm.remove_condition("bob", "Frightened");

    assert!(cm.get_conditions("bob").contains(&bleed))
}

#[test]
fn end_of_bob_turn_removes_end_of_bob_turn_condition() {
    let mut cm = ConditionManager::new();
    let condition = conditions::Condition {
        name: String::from("some affliction"),
        level: 3,
        cond_type: ConditionType::ReductionBased { reduction: None, trigger: ReductionTrigger::EndOfTurn(String::from("bob")) },
    };

    cm.add_conditions("bob", condition.clone());
    assert!(cm.get_conditions("bob").contains(&condition));

    cm.end_of_turn("bob");
    assert!(!cm.get_conditions("bob").contains(&condition))
}

#[test]
fn start_of_bob_turn_removes_start_of_turn_condition() {
    let mut cm = ConditionManager::new();
    let condition = conditions::Condition {
        name: String::from("some afflication"),
        level: 4,
        cond_type: ConditionType::ReductionBased { reduction: None, trigger: ReductionTrigger::StartOfTurn(String::from("bob")) },
    };

    cm.add_conditions("bob", condition.clone());
    assert!(cm.get_conditions("bob").contains(&condition));

    cm.start_of_turn("bob");
    assert!(!cm.get_conditions("bob").contains(&condition))
}
