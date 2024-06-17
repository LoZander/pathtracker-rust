use pathtracker_rust::conditions::{condition_manager::ConditionManager, Condition, DamageType, TurnEvent, ValuedCondition, ValuedTerm};

#[test]
fn add_condition_adds() {
    let mut cm = ConditionManager::new();
    let condition = Condition::builder()
        .condition(ValuedCondition::Frightened)
        .value(3)
        .term(ValuedTerm::Reduced(TurnEvent::EndOfTurn(String::from("bob")), 1))
        .build();

    cm.add_condition("bob", condition.clone());

    assert!(cm.get_conditions("bob").contains(&condition))
}

#[test]
fn remove_condition_removes() {
    let mut cm = ConditionManager::new();
    let condition = Condition::Valued {
        cond: ValuedCondition::Frightened,
        level: 5,
        term: ValuedTerm::Reduced(TurnEvent::EndOfTurn(String::from("bob")), 1)
    };

    cm.add_condition("bob", condition.clone());
    cm.remove_condition("bob", &condition);

    assert!(!cm.get_conditions("bob").contains(&condition))
}

#[test]
fn remove_bleed_doesnt_remove_frighten() {
    let mut cm = ConditionManager::new();
    let bleed = Condition::Valued {
        cond: ValuedCondition::PersistentDamage(DamageType::Bleed),
        level: 7,
        term: ValuedTerm::Manual
    };

    cm.add_condition("bob", bleed.clone());
    
    let frightened = Condition::Valued {
        cond: ValuedCondition::Frightened,
        level: 3,
        term: ValuedTerm::Reduced(TurnEvent::EndOfTurn(String::from("bob")), 1)
    };

    cm.add_condition("bob", frightened.clone());
    cm.remove_condition("bob", &frightened);

    assert!(cm.get_conditions("bob").contains(&bleed))
}
