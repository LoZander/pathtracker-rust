use pathtracker_rust::{character::{Chr, ChrName, Health}, conditions::{Condition, DamageType, ValuedCondition, ValuedTerm}, saver::NoSaver, tracker::{self, Tracker}};

#[test]
fn undo_next_turn_nonempty() -> tracker::Result<()> {
    let jevil = Chr::builder("Jevil", 20, false).build();
    let chris = Chr::builder("Chris", 17, true).build();
    let mut t = Tracker::builder()
        .with_saver(NoSaver)
        .with_chrs(vec![jevil, chris])
        .build();

    t.end_turn()?;

    let before = t.clone();

    t.end_turn()?;

    t.undo()?;

    assert_eq!(t.get_chrs(), before.get_chrs());
    assert_eq!(t.get_in_turn(), before.get_in_turn());
    t.get_chrs().iter().for_each(|c| assert_eq!(t.get_conditions(&c.name), before.get_conditions(&c.name)));

    Ok(())
}

#[test]
fn undo_next_turn_twice() -> tracker::Result<()> {
    let jevil = Chr::builder("Jevil", 20, false).build();
    let chris = Chr::builder("Chris", 17, true).build();
    let mut t = Tracker::builder()
        .with_saver(NoSaver)
        .with_chrs(vec![jevil, chris])
        .build();

    t.end_turn()?;

    let before = t.clone();

    t.end_turn()?;
    t.end_turn()?;

    t.undo()?;
    t.undo()?;

    assert_eq!(t.get_chrs(), before.get_chrs());
    assert_eq!(t.get_in_turn(), before.get_in_turn());
    t.get_chrs().iter().for_each(|c| assert_eq!(t.get_conditions(&c.name), before.get_conditions(&c.name)));

    Ok(())
}

#[test]
fn undo_add_character() -> tracker::Result<()> {
    let mut t = Tracker::builder().with_saver(NoSaver).build();
    let chris = Chr::builder("Chris", 17, true).build();

    let before = t.clone();
    t.add_chr(chris)?;
    t.undo()?;

    assert_eq!(t.get_chrs(), before.get_chrs());

    Ok(())
}


#[test]
fn undo_rm_character() -> tracker::Result<()> {
    let jevil = Chr::builder("Jevil", 20, false).build();
    let chris = Chr::builder("Chris", 17, true).build();
    let mut t = Tracker::builder()
        .with_saver(NoSaver)
        .with_chrs(vec![jevil, chris])
        .build();

    t.end_turn()?;
    let before = t.clone();
    t.rm_chr(&ChrName::new("Jevil"))?;
    t.undo()?;

    assert_eq!(t.get_chrs(), before.get_chrs());
    assert_eq!(t.get_in_turn(), before.get_in_turn());

    Ok(())
}

#[test]
fn redo_undo_inv_rm_character() -> tracker::Result<()> {
    let jevil = Chr::builder("Jevil", 20, false).build();
    let chris = Chr::builder("Chris", 17, true).build();
    let mut t = Tracker::builder()
        .with_saver(NoSaver)
        .with_chrs(vec![jevil, chris])
        .build();

    t.end_turn()?;
    t.rm_chr(&ChrName::new("Jevil"))?;

    let before = t.clone();
    t.undo()?;
    t.redo()?;


    assert_eq!(t.get_chrs(), before.get_chrs());
    assert_eq!(t.get_in_turn(), before.get_in_turn());

    Ok(())
}

#[test]
fn undo_redo_inv_add_cond() -> tracker::Result<()> {
    let jevil = Chr::builder("Jevil", 20, false).build();
    let chris = Chr::builder("Chris", 17, true).build();
    let ralsei = Chr::builder("Ralsei", 12, false).build();
    let mut t = Tracker::builder()
        .with_saver(NoSaver)
        .with_chrs(vec![jevil, chris, ralsei])
        .build();

    t.end_turn()?;

    let bleed = Condition::builder()
        .condition(ValuedCondition::PersistentDamage(DamageType::Bleed))
        .term(ValuedTerm::Manual)
        .value(5)
        .build();
    t.add_condition(ChrName::new("Chris"), bleed)?;
    t.undo()?;
    let before = t.clone();
    t.redo()?;
    t.undo()?;

    t.get_chrs().iter().for_each(|c| assert_eq!(t.get_conditions(&c.name), before.get_conditions(&c.name)));

    Ok(())
}

#[test]
fn undo_undo_redo() -> tracker::Result<()> {
    let jevil = Chr::builder("Jevil", 20, false).build();
    let chris = Chr::builder("Chris", 17, true).with_health(Health::new(30)).build();
    let ralsei = Chr::builder("Ralsei", 12, false).build();
    let mut t = Tracker::builder()
        .with_saver(NoSaver)
        .with_chrs(vec![jevil, chris, ralsei])
        .build();

    t.end_turn()?; // Jevil turn
    let bleed = Condition::builder()
        .condition(ValuedCondition::PersistentDamage(DamageType::Bleed))
        .value(5)
        .term(ValuedTerm::Manual)
        .build();
    t.add_condition(ChrName::new("Chris"), bleed)?;
    t.end_turn()?; // Chris turn
    t.end_turn()?; // Ralsei turn
    let goal = t.clone();
    t.damage(&ChrName::new("Chris"), 6)?;

    t.undo()?;
    t.undo()?;
    t.redo()?;

    assert_eq!(Some(25), t.get_chr(&ChrName::new("Chris")).and_then(|c| c.health.as_ref()).map(|h| h.current));
    assert_eq!(t.get_chrs(), goal.get_chrs());
    assert_eq!(t.get_in_turn(), goal.get_in_turn());

    Ok(())
}
