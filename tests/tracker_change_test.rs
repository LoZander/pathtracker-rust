#![allow(clippy::unwrap_used)]

use pathtracker_rust::{
	character::Chr, saver::NoSaver, tracker::{self, MovedStatus, Tracker}
};

#[test]
fn rename_renames() -> tracker::Result<()> {
    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        Chr::builder("Link", 24, true).build(),
    ]).build();

    t.rename("Link", "Ganon")?;

    assert_eq!(Some(&Chr::builder("Ganon", 24, true).build()), t.get_chr("Ganon"));

    Ok(())
}

#[test]
fn rename_into_already_existing_fails() {
    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        Chr::builder("Link", 24, true).build(),
        Chr::builder("Ganon", 30, false).build(),
    ]).build();

    let res = t.rename("Link", "Ganon");
    assert_eq!(Err(tracker::Error::RenameDupError{ old: "Link".into(), new: "Ganon".into() }), res);
}

#[test]
fn rename_preserves_order() -> tracker::Result<()> {
    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        Chr::builder("Lucifer", 24, true).build(),
        Chr::builder("Link", 24, true).build(),
        Chr::builder("Lament", 24, true).build(),
    ]).build();

    t.rename("Link", "Ganon")?;
    assert_eq!(Ok(Some(&Chr::builder("Lucifer", 24, true).build())), t.end_turn());
    assert_eq!(Ok(Some(&Chr::builder("Ganon", 24, true).build())), t.end_turn());
    assert_eq!(Ok(Some(&Chr::builder("Lament", 24, true).build())), t.end_turn());

    Ok(())
}

#[test]
fn change_init_changes_init() -> tracker::Result<()> {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        lucifer,
        link,
        hugo,
    ]).build();

    t.change_init("Hugo", 14)?;

    assert_eq!(14, t.get_chr("Hugo").unwrap().init);

    Ok(())
}

#[test]
fn change_init_preserves_sorting() -> tracker::Result<()> {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        lucifer,
        link,
        hugo,
    ]).build();

    t.change_init("Lucifer", 19)?;

    assert_eq!("Link", t.end_turn()?.unwrap().name);
    assert_eq!("Lucifer", t.end_turn()?.unwrap().name);
    assert_eq!("Hugo", t.end_turn()?.unwrap().name);

    Ok(())
}

#[test]
fn change_init_not_in_turn_preserves_in_turn() -> tracker::Result<()> {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        lucifer.clone(),
        link,
        hugo,
    ]).build();

    t.end_turn()?;
    t.change_init("Link", 25)?;

    assert_eq!(Some(&lucifer), t.get_in_turn());

    Ok(())
}

#[test]
fn change_init_not_in_turn_so_skipped_preserves_in_turn() -> tracker::Result<()> {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        lucifer,
        link,
        hugo,
    ]).build();

    t.end_turn()?;
    t.end_turn()?;

    assert_eq!("Link", t.get_in_turn().unwrap().name);
    t.change_init("Hugo", 22)?;
    assert_eq!("Link", t.get_in_turn().unwrap().name);

    Ok(())
}

#[test]
fn change_init_not_in_turn_so_two_turns_preserves_in_turn() -> tracker::Result<()> {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        lucifer,
        link,
        hugo,
    ]).build();

    t.end_turn()?;
    t.end_turn()?;

    assert_eq!("Link", t.get_in_turn().unwrap().name);
    t.change_init("Lucifer", 5)?;
    assert_eq!("Link", t.get_in_turn().unwrap().name);

    Ok(())
}

#[test]
fn change_init_not_in_turn_so_skipped_returns_skipped() -> tracker::Result<()> {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        lucifer,
        link,
        hugo,
    ]).build();

    t.end_turn()?;
    t.end_turn()?;

    let skipped = t.change_init("Hugo", 22)?;

    assert_eq!(Some(MovedStatus::Skipped(Chr::builder("Hugo", 22, true).build())), skipped);

    Ok(())
}

#[test]
fn change_init_not_in_turn_so_two_turns_returns_two_turns() -> tracker::Result<()> {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        lucifer,
        link,
        hugo,
    ]).build();

    t.end_turn()?;
    t.end_turn()?;

    let skipped = t.change_init("Lucifer", 7)?;

    assert_eq!(Some(MovedStatus::TwoTurns(Chr::builder("Lucifer", 7, true).build())), skipped);

    Ok(())
}

#[test]
fn change_init_in_turn_wheearlier_order_changes_in_turn() -> tracker::Result<()> {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        lucifer,
        link,
        hugo,
    ]).build();

    t.end_turn()?;
    t.end_turn()?;
    
    assert_eq!("Link", t.get_in_turn().unwrap().name);
    t.change_init("Link", 30)?;
    assert_eq!("Lucifer", t.get_in_turn().unwrap().name);

    Ok(())
}

#[test]
fn change_init_in_turn_when_earlier_order_changes_in_turn() -> tracker::Result<()> {
    let lucifer = Chr::builder("Lucifer", 24, true).build();
    let link = Chr::builder("Link", 20, true).build();
    let hugo = Chr::builder("Hugo", 10, true).build();

    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        lucifer,
        link,
        hugo,
    ]).build();

    t.end_turn()?;
    t.end_turn()?;

    assert_eq!("Link", t.get_in_turn().unwrap().name);
    t.change_init("Link", 8)?;
    assert_eq!("Hugo", t.get_in_turn().unwrap().name);    

    Ok(())
}

#[test]
fn set_player_can_make_player() -> tracker::Result<()> {
    let barbosa = Chr::builder("Barbosa", 23, true).build();

    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![barbosa]).build();

    assert!(t.get_chr("Barbosa").is_some_and(|c| c.player));
    t.set_player("Barbosa", false)?;
    assert!(!t.get_chr("Barbosa").is_some_and(|c| c.player));

    Ok(())
}

#[test]
fn set_player_can_make_enemy() -> tracker::Result<()> {
    let barbosa = Chr::builder("Barbosa", 23, false).build();

    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![barbosa]).build();

    assert!(!t.get_chr("Barbosa").is_some_and(|c| c.player));
    t.set_player("Barbosa", true)?;
    assert!(t.get_chr("Barbosa").is_some_and(|c| c.player));

    Ok(())
}
