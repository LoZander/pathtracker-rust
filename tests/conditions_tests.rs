#![allow(clippy::unwrap_used)]

use pathtracker_rust::{character::{Chr, ChrName, Health}, conditions::{Condition, DamageType, NonValuedCondition, NonValuedTerm, TurnEvent, ValuedCondition, ValuedTerm, condition_manager::ConditionManager}, duration::Duration, saver::NoSaver, tracker::{self, Tracker}};

#[test]
fn add_condition_adds() {
    let mut cm = ConditionManager::new();
    let condition = Condition::builder()
        .condition(ValuedCondition::Frightened)
        .value(3)
        .term(ValuedTerm::Reduced(TurnEvent::EndOfNextTurn(ChrName::new("bob")), 1))
        .build();

    let bob = ChrName::new("bob");

    cm.add_condition(bob.clone(), condition.clone());

    assert!(cm.get_conditions(&bob).contains(&condition));
}

#[test]
fn remove_condition_removes() {
    let mut cm = ConditionManager::new();
    let condition = Condition::Valued {
        cond: ValuedCondition::Frightened,
        level: 5,
        term: ValuedTerm::Reduced(TurnEvent::EndOfNextTurn(ChrName::new("bob")), 1)
    };

    let bob = ChrName::new("bob");

    cm.add_condition(bob.clone(), condition.clone());
    cm.remove_condition(&bob, &condition);

    assert!(!cm.get_conditions(&bob).contains(&condition));
}

#[test]
fn remove_bleed_doesnt_remove_frighten() {
    let mut cm = ConditionManager::new();
    let bleed = Condition::Valued {
        cond: ValuedCondition::PersistentDamage(DamageType::Bleed),
        level: 7,
        term: ValuedTerm::Manual
    };

    cm.add_condition(ChrName::new("bob"), bleed.clone());
    
    let frightened = Condition::Valued {
        cond: ValuedCondition::Frightened,
        level: 3,
        term: ValuedTerm::Reduced(TurnEvent::EndOfNextTurn(ChrName::new("bob")), 1)
    };

    let bob = ChrName::new("bob");

    cm.add_condition(bob.clone(), frightened.clone());
    cm.remove_condition(&bob, &frightened);

    assert!(cm.get_conditions(&bob).contains(&bleed));
}

#[test]
fn alice_manual_condition_remains_after_alice_turn_ends() {
    let mut cm = ConditionManager::new();
    let bleed = Condition::builder()
        .condition(ValuedCondition::PersistentDamage(DamageType::Bleed))
        .value(4)
        .build();

    let alice = ChrName::new("Alice");

    cm.add_condition(alice.clone(), bleed.clone());
    cm.end_of_turn(alice.clone());

    assert!(cm.get_conditions(&alice).contains(&bleed));
}

#[test]
fn alice_manual_condition_remains_after_alice_turn_starts() {
    let mut cm = ConditionManager::new();
    let bleed = Condition::builder()
        .condition(ValuedCondition::PersistentDamage(DamageType::Bleed))
        .value(4)
        .build();

    let alice = ChrName::new("Alice");

    cm.add_condition(alice.clone(), bleed.clone());
    cm.start_of_turn(alice.clone());

    assert!(cm.get_conditions(&alice).contains(&bleed));
}

#[test]
fn alice_manual_condition_remains_after_bob_turn_ends() {
    let mut cm = ConditionManager::new();
    let blinded = Condition::builder().condition(NonValuedCondition::Blinded).build();

    let alice = ChrName::new("Alice");
    let bob = ChrName::new("Bob");

    cm.add_condition(alice.clone(), blinded.clone());
    cm.end_of_turn(bob);
    assert!(cm.get_conditions(&alice).contains(&blinded));
}

#[test]
fn alice_manual_condition_tracker_integration() -> tracker::Result<()> {
    let mut t: Tracker<NoSaver> = Tracker::builder().build();
    let alice = Chr::builder("Alice", 32, true).build();
    let bob = Chr::builder("Bob", 23, true).build();
    t.add_chr(alice)?;
    t.add_chr(bob)?;
    let blinded = Condition::builder().condition(NonValuedCondition::Blinded).build();

    let alice = ChrName::new("Alice");

    t.add_condition(alice.clone(), blinded.clone())?;

    assert!(t.get_conditions(&alice).contains(&blinded));
    t.end_turn()?;
    assert!(t.get_conditions(&alice).contains(&blinded));
    t.end_turn()?;
    assert!(t.get_conditions(&alice).contains(&blinded));
    t.end_turn()?;
    assert!(t.get_conditions(&alice).contains(&blinded));

    Ok(())
}

#[test]
fn three_turn_nonvalued_condition_duration_reduced_to_2_turns_after_character_end_of_turn() {
    let mut cm = ConditionManager::new();
    let blinded = Condition::builder()
        .condition(NonValuedCondition::Blinded)
        .term(NonValuedTerm::For(Duration::from_turns(3)))
        .build();

    let alice = ChrName::new("Alice");

    cm.add_condition(alice.clone(), blinded.clone());
    
    cm.end_of_turn(alice.clone());
    match cm.get_conditions(&alice).get(&blinded) {
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

    let alice = ChrName::new("Alice");
    let bob = ChrName::new("Bob");

    cm.add_condition(alice.clone(), blinded.clone());
    
    cm.end_of_turn(bob);
    match cm.get_conditions(&alice).get(&blinded) {
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

    let alice = ChrName::new("Alice");

    cm.add_condition(alice.clone(), bleed.clone());

    cm.end_of_turn(alice.clone());
    match cm.get_conditions(&alice).get(&bleed) {
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

    let alice = ChrName::new("Alice");

    cm.add_condition(alice.clone(), blinded.clone());

    cm.end_of_turn(alice.clone());
    assert_eq!(None, cm.get_conditions(&alice).get(&blinded));
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

    let alice = ChrName::new("Alice");

    t.add_condition(alice.clone(), bleed)?;

    t.end_turn()?;
    t.end_turn()?;

    assert_eq!(25, t.get_chr(&alice).unwrap().health.as_ref().unwrap().current);

    Ok(())
}

#[test]
fn persistent_bleed_10_reduced_start_alice_on_bob() -> tracker::Result<()> {
    let mut t: Tracker<NoSaver> = Tracker::default();
    let alice = Chr::builder("Alice", 10, false).build();
    let bob = Chr::builder("Bob", 13, false).with_health(Health::new(40)).build();
    t.add_chr(alice)?;
    t.add_chr(bob)?;
    let bleed = Condition::builder()
        .condition(ValuedCondition::PersistentDamage(DamageType::Bleed))
        .value(10)
        .term(ValuedTerm::Reduced(TurnEvent::StartOfNextTurn(ChrName::new("Alice")), 3))
        .build();

    let bob = ChrName::new("Bob");

    t.add_condition(bob.clone(), bleed)?;

    t.end_turn()?;
    t.end_turn()?;

    assert_eq!(30, t.get_chr(&bob).unwrap().health.as_ref().unwrap().current);

    t.end_turn()?;
    t.end_turn()?;

    assert_eq!(23, t.get_chr(&bob).unwrap().health.as_ref().unwrap().current);

    Ok(())
}

#[test]
fn flat_footed_until_end_of_current_turn_ends_with_current_turn() {
    let mut conds = ConditionManager::new();

    let alice = ChrName::new("Alice");

    let cond = Condition::builder().condition(NonValuedCondition::FlatFooted)
        .term(NonValuedTerm::Until(TurnEvent::EndOfCurrentTurn(alice.clone())))
        .build();


    conds.add_condition(alice.clone(), cond.clone());

    assert!(conds.get_conditions(&alice).contains(&cond));
    conds.end_of_turn(alice.clone());

    assert!(!conds.get_conditions(&alice).contains(&cond));
}

#[test]
fn clumsy_reduced_one_end_of_current_turn_is_reduced() {
    let mut conds = ConditionManager::new();

    let alice = ChrName::new("Alice");
    
    let cond = Condition::builder().condition(ValuedCondition::Clumsy)
        .value(3)
        .term(ValuedTerm::Reduced(TurnEvent::EndOfCurrentTurn(alice.clone()), 1))
        .build();
    conds.add_condition(alice.clone(), cond.clone());

    match conds.get_conditions(&alice).get(&cond) {
        Some(&c @ Condition::Valued{level, ..}) if c == &cond =>
            assert_eq!(3, *level),
        _ => panic!()
    }

    conds.end_of_turn(alice.clone());

    match conds.get_conditions(&alice).get(&cond) {
        Some(&c @ Condition::Valued{level, ..}) if c == &cond =>
            assert_eq!(2, *level),
        _ => panic!()
    }
}

#[test]
fn flat_footed_until_end_of_next_turn_doesnt_end_with_current() {
    let mut conds = ConditionManager::new();

    let alice = ChrName::new("Alice");

    let cond = Condition::builder().condition(NonValuedCondition::FlatFooted)
        .term(NonValuedTerm::Until(TurnEvent::EndOfNextTurn(alice.clone())))
        .build();
    conds.add_condition(alice.clone(), cond.clone());

    assert!(conds.get_conditions(&alice).contains(&cond));

    conds.end_of_turn(alice.clone());

    assert!(conds.get_conditions(&alice).contains(&cond));
}

#[test]
fn flat_footed_until_end_of_next_turn_ends_after_it_end_of_other_turn_and_then_affecteds() {
    let mut conds = ConditionManager::new();

    let alice = ChrName::new("Alice");
    let bob = ChrName::new("Bob");

    let cond = Condition::builder().condition(NonValuedCondition::FlatFooted)
        .term(NonValuedTerm::Until(TurnEvent::EndOfNextTurn(alice.clone())))
        .build();
    conds.add_condition(alice.clone(), cond.clone());

    assert!(conds.get_conditions(&alice).contains(&cond));

    conds.end_of_turn(bob);
    conds.end_of_turn(alice.clone());

    assert!(!conds.get_conditions(&alice).contains(&cond));
}

#[test]
fn clumsy_reduced_end_of_next_turn_doesnt_reduce_after_end_of_current() {
    let mut conds = ConditionManager::new();

    let alice = ChrName::new("Alice");

    let cond = Condition::builder().condition(ValuedCondition::Clumsy)
        .value(3)
        .term(ValuedTerm::Reduced(TurnEvent::EndOfNextTurn(alice.clone()), 1))
        .build();
    conds.add_condition(alice.clone(), cond.clone());

    match conds.get_conditions(&alice).get(&cond) {
        Some(&c @ Condition::Valued{level, ..}) if c == &cond =>
            assert_eq!(3, *level),
        _ => panic!()
    }

    conds.end_of_turn(alice.clone());

    match conds.get_conditions(&alice).get(&cond) {
        Some(&c @ Condition::Valued{level, ..}) if c == &cond =>
            assert_eq!(3, *level),
        _ => panic!()
    }
}

#[test]
fn clumsy_reduced_end_of_next_turn_reduces_after_end_of_other_turn_and_then_affecteds() {
    let mut conds = ConditionManager::new();

    let alice = ChrName::new("Alice");
    let bob = ChrName::new("Bob");

    let cond = Condition::builder().condition(ValuedCondition::Clumsy)
        .value(3)
        .term(ValuedTerm::Reduced(TurnEvent::EndOfNextTurn(alice.clone()), 1))
        .build();
    conds.add_condition(alice.clone(), cond.clone());

    conds.end_of_turn(bob);
    conds.end_of_turn(alice.clone());

    match conds.get_conditions(&alice).get(&cond) {
        Some(&c @ Condition::Valued{level, ..}) if c == &cond =>
            assert_eq!(2, *level),
        _ => panic!()
    }
}
