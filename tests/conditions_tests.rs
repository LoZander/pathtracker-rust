use pathtracker_rust::{character::{Chr, Health}, conditions::{condition_manager::ConditionManager, Condition, DamageType, NonValuedCondition, NonValuedTerm, TurnEvent, ValuedCondition, ValuedTerm}, duration::Duration, saver::NoSaver, tracker::{self, Tracker}};

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
    t.add_condition("Alice", blinded.clone()).unwrap();

    assert!(t.get_conditions("Alice").contains(&blinded));
    t.end_turn().unwrap();
    assert!(t.get_conditions("Alice").contains(&blinded));
    t.end_turn().unwrap();
    assert!(t.get_conditions("Alice").contains(&blinded));
    t.end_turn().unwrap();
    assert!(t.get_conditions("Alice").contains(&blinded));
}

#[test]
fn three_turn_nonvalued_condition_duration_reduced_to_2_turns_after_character_end_of_turn() {
    let mut cm = ConditionManager::new();
    let blinded = Condition::builder()
        .condition(NonValuedCondition::Blinded)
        .term(NonValuedTerm::For(Duration::from_turns(3)))
        .build();
    cm.add_condition("Alice", blinded.clone());
    
    cm.end_of_turn("Alice");
    match cm.get_conditions("Alice").get(&blinded) {
        Some(Condition::NonValued { term: NonValuedTerm::For(dur), .. }) => 
            assert_eq!(&Duration::from_turns(2), dur),
        _ => panic!(),
    }
}

#[test]
fn three_turn_nonvalued_condition_duration_unchanged_after_other_character_end_of_turn() {
    let mut cm = ConditionManager::new();
    let blinded = Condition::builder()
        .condition(NonValuedCondition::Blinded)
        .term(NonValuedTerm::For(Duration::from_turns(3)))
        .build();
    cm.add_condition("Alice", blinded.clone());
    
    cm.end_of_turn("Bob");
    match cm.get_conditions("Alice").get(&blinded) {
        Some(Condition::NonValued { term: NonValuedTerm::For(dur), .. }) => 
            assert_eq!(&Duration::from_turns(3), dur),
        _ => panic!(),
    }
}

#[test]
fn three_turn_valued_condition_duration_reduced_to_2_turns_after_character_end_of_turn() {
    let mut cm = ConditionManager::new();
    let bleed = Condition::builder()
        .condition(ValuedCondition::PersistentDamage(DamageType::Bleed))
        .value(4)
        .term(ValuedTerm::For(Duration::from_turns(3)))
        .build();
    cm.add_condition("Alice", bleed.clone());

    cm.end_of_turn("Alice");
    match cm.get_conditions("Alice").get(&bleed) {
        Some(Condition::Valued { term: ValuedTerm::For(dur), .. }) =>
            assert_eq!(&Duration::from_turns(2), dur),
        _ => panic!(),
    }
}

#[test]
fn one_turn_nonvalued_condition_duration_removed_after_character_end_of_turn() {
    let mut cm = ConditionManager::new();
    let blinded = Condition::builder()
        .condition(NonValuedCondition::Blinded)
        .term(NonValuedTerm::For(Duration::from_turns(1)))
        .build();
    cm.add_condition("Alice", blinded.clone());

    cm.end_of_turn("Alice");
    assert_eq!(None, cm.get_conditions("Alice").get(&blinded))
}

#[test]
fn persistent_damage_reduces_health_at_end_of_turn() -> tracker::Result<()> {
    let mut t: Tracker<NoSaver> = Tracker::default();
    let c = Chr::builder("Alice", 32, true).with_health(Health::new(30)).build();
    t.add_chr(c)?;
    let bleed = Condition::builder()
        .condition(ValuedCondition::PersistentDamage(DamageType::Bleed))
        .value(5)
        .build();
    t.add_condition("Alice", bleed)?;

    t.end_turn()?;
    t.end_turn()?;

    assert_eq!(25, t.get_chr("Alice").unwrap().health.as_ref().unwrap().current);

    Ok(())
}
