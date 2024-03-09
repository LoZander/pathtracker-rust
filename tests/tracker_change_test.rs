use pathtracker_rust::{
	character::Chr, tracker::{self, MovedStatus, Tracker, TrackerResult}
};

#[test]
fn rename_renames() -> TrackerResult {
    let mut t = Tracker::new(vec![
        Chr::builder("Link", 24, true).build(),
    ]);

    t.rename("Link", "Ganon")?;

    assert_eq!(Some(&Chr::builder("Ganon", 24, true).build()), t.get_chr("Ganon"));

    Ok(())
}

#[test]
fn rename_into_already_existing_fails() -> TrackerResult {
    let mut t = Tracker::new(vec![
        Chr::builder("Link", 24, true).build(),
        Chr::builder("Ganon", 30, false).build(),
    ]);

    let res = t.rename("Link", "Ganon");
    assert_eq!(Err(tracker::Error::RenameDuplicateError{ old: "Link".into(), new: "Ganon".into() }), res);

    Ok(())
}

#[test]
fn rename_preserves_order() -> TrackerResult {
    let mut t = Tracker::new(vec![
        Chr::builder("Lucifer", 24, true).build(),
        Chr::builder("Link", 24, true).build(),
        Chr::builder("Lament", 24, false).build(),
    ]);

    t.rename("Link", "Ganon")?;
    assert_eq!(Some(&Chr::builder("Lucifer", 24, true).build()), t.end_turn());
    assert_eq!(Some(&Chr::builder("Ganon", 24, true).build()), t.end_turn());
    assert_eq!(Some(&Chr::builder("Lament", 24, false).build()), t.end_turn());

    Ok(())
}

#[test]
fn change_init_changes_init() -> TrackerResult {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t = Tracker::new(vec![
        lucifer.clone(),
        link.clone(),
        hugo.clone(),
    ]);

    t.change_init("Hugo", 14)?;

    assert_eq!(14, t.get_chr("Hugo").unwrap().init);

    Ok(())
}

#[test]
fn change_init_preserves_sorting() -> TrackerResult {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t = Tracker::new(vec![
        lucifer.clone(),
        link.clone(),
        hugo.clone(),
    ]);

    println!("{:?}", t);

    t.change_init("Lucifer", 19)?;

    assert_eq!("Link", t.end_turn().unwrap().name);
    assert_eq!("Lucifer", t.end_turn().unwrap().name);
    assert_eq!("Hugo", t.end_turn().unwrap().name);

    Ok(())
}

#[test]
fn change_init_not_in_turn_preserves_in_turn() -> TrackerResult {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t = Tracker::new(vec![
        lucifer.clone(),
        link.clone(),
        hugo.clone(),
    ]);

    t.end_turn();
    t.change_init("Link", 25)?;

    assert_eq!(Some(&lucifer), t.get_in_turn());

    Ok(())
}

#[test]
fn change_init_not_in_turn_so_skipped_preserves_in_turn() -> TrackerResult {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t = Tracker::new(vec![
        lucifer.clone(),
        link.clone(),
        hugo.clone(),
    ]);

    t.end_turn();
    t.end_turn();

    assert_eq!("Link", t.get_in_turn().unwrap().name);
    t.change_init("Hugo", 22)?;
    assert_eq!("Link", t.get_in_turn().unwrap().name);

    Ok(())
}

#[test]
fn change_init_not_in_turn_so_two_turns_preserves_in_turn() -> TrackerResult {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t = Tracker::new(vec![
        lucifer.clone(),
        link.clone(),
        hugo.clone(),
    ]);

    t.end_turn();
    t.end_turn();

    assert_eq!("Link", t.get_in_turn().unwrap().name);
    t.change_init("Lucifer", 5)?;
    assert_eq!("Link", t.get_in_turn().unwrap().name);

    Ok(())
}

#[test]
fn change_init_not_in_turn_so_skipped_returns_skipped() -> TrackerResult {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t = Tracker::new(vec![
        lucifer.clone(),
        link.clone(),
        hugo.clone(),
    ]);

    t.end_turn();
    t.end_turn();

    let skipped = t.change_init("Hugo", 22)?;

    assert_eq!(Some(MovedStatus::Skipped(Chr::builder("Hugo", 22, true).build())), skipped);

    Ok(())
}

#[test]
fn change_init_not_in_turn_so_two_turns_returns_two_turns() -> TrackerResult {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t = Tracker::new(vec![
        lucifer.clone(),
        link.clone(),
        hugo.clone(),
    ]);

    t.end_turn();
    t.end_turn();

    let skipped = t.change_init("Lucifer", 7)?;

    assert_eq!(Some(MovedStatus::TwoTurns(Chr::builder("Lucifer", 7, true).build())), skipped);

    Ok(())
}

#[test]
fn change_init_in_turn_wheearlier_order_changes_in_turn() -> TrackerResult {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t = Tracker::new(vec![
        lucifer.clone(),
        link.clone(),
        hugo.clone(),
    ]);

    t.end_turn();
    t.end_turn();
    
    assert_eq!("Link", t.get_in_turn().unwrap().name);
    t.change_init("Link", 30)?;
    assert_eq!("Lucifer", t.get_in_turn().unwrap().name);

    Ok(())
}

#[test]
fn change_init_in_turn_when_earlier_order_changes_in_turn() -> TrackerResult {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t = Tracker::new(vec![
        lucifer.clone(),
        link.clone(),
        hugo.clone(),
    ]);

    t.end_turn();
    t.end_turn();

    assert_eq!("Link", t.get_in_turn().unwrap().name);
    t.change_init("Link", 8)?;
    assert_eq!("Hugo", t.get_in_turn().unwrap().name);    

    Ok(())
}

#[test]
fn set_player_can_make_player() -> TrackerResult {
    let barbosa = Chr::builder("Barbosa", 23, true).build();

    let mut t = Tracker::new(vec![barbosa.clone()]);

    assert!(t.get_chr("Barbosa").unwrap().player);
    t.set_player("Barbosa", false)?;
    assert!(!t.get_chr("Barbosa").unwrap().player);

    Ok(())
}

#[test]
fn set_player_can_make_enemy() -> TrackerResult {
    let barbosa = Chr::builder("Barbosa", 23, false).build();

    let mut t = Tracker::new(vec![barbosa.clone()]);

    assert!(!t.get_chr("Barbosa").unwrap().player);
    t.set_player("Barbosa", true)?;
    assert!(t.get_chr("Barbosa").unwrap().player);

    Ok(())
}
