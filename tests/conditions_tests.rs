use pathtracker_rust::{character::Chr, conditions::{condition_manager::ConditionManager, Condition, DamageType, NonValuedCondition, TurnEvent, ValuedCondition, ValuedTerm}, saver::NoSaver, tracker::Tracker};

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

#[test]
fn alice_manual_condition_remains_after_alice_turn_ends() {
    let mut cm = ConditionManager::new();
    let bleed = Condition::builder()
        .condition(ValuedCondition::PersistentDamage(DamageType::Bleed))
        .value(4)
        .build();

    cm.add_condition("Alice", bleed.clone());
    cm.end_of_turn("Alice");

    assert!(cm.get_conditions("Alice").contains(&bleed))
}

#[test]
fn alice_manual_condition_remains_after_alice_turn_starts() {
    let mut cm = ConditionManager::new();
    let bleed = Condition::builder()
        .condition(ValuedCondition::PersistentDamage(DamageType::Bleed))
        .value(4)
        .build();
    cm.add_condition("Alice", bleed.clone());
    cm.start_of_turn("Alice");

    assert!(cm.get_conditions("Alice").contains(&bleed))
}

#[test]
fn alice_manual_condition_remains_after_bob_turn_ends() {
    let mut cm = ConditionManager::new();
    let blinded = Condition::builder().condition(NonValuedCondition::Blinded).build();
    cm.add_condition("Alice", blinded.clone());
    cm.end_of_turn("Bob");
    assert!(cm.get_conditions("Alice").contains(&blinded))
}

#[test]
fn alice_manual_condition_tracker_integration() {
    let mut t: Tracker<NoSaver> = Tracker::builder().build();
    let alice = Chr::builder("Alice", 32, true).build();
    let bob = Chr::builder("Bob", 23, true).build();
    t.add_chr(alice).unwrap();
    t.add_chr(bob).unwrap();
    let blinded = Condition::builder().condition(NonValuedCondition::Blinded).build();
    t.add_cond("Alice", blinded.clone()).unwrap();

    assert!(t.get_conditions("Alice").contains(&blinded));
    t.end_turn();
    assert!(t.get_conditions("Alice").contains(&blinded));
    t.end_turn();
    assert!(t.get_conditions("Alice").contains(&blinded));
    t.end_turn();
    assert!(t.get_conditions("Alice").contains(&blinded));
}
