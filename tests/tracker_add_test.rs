use pathtracker_rust::{
    character::Chr, saver::NoSaver, tracker::{Tracker, TrackerResult}
};

#[test]
fn add_player_chr_alison_adds_chr() -> TrackerResult {
    let mut t: Tracker<NoSaver> = Tracker::default();
    t.add_chr(Chr::builder("Alison", 21, true).build())?;
    assert_eq!(Some(&Chr::builder("Alison", 21, true).build()), t.get_chr("Alison"));
    Ok(())
}

#[test]
fn add_preserves_descending_inititative_order() -> TrackerResult {
    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        Chr::builder("Bucky", 30, true).build(),
        Chr::builder("Hellen", 27, true).build(),
        Chr::builder("Skelly Boy", 3, false).build(),
    ]).build();

    t.add_chr(Chr::builder("Kristy", 24, true).build())?;

    assert_eq!(Some(&Chr::builder("Bucky", 30, true).build()), t.end_turn());
    assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.end_turn());
    assert_eq!(Some(&Chr::builder("Kristy", 24, true).build()), t.end_turn());
    assert_eq!(Some(&Chr::builder("Skelly Boy", 3, false).build()), t.end_turn());

    Ok(())
}

#[test]
fn add_chr_before_in_turn_preserves_in_turn() -> TrackerResult {
    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        Chr::builder("Bucky", 30, true).build(),
        Chr::builder("Hellen", 27, true).build(),
        Chr::builder("Skelly Boy", 3, false).build(),
    ]).build();

    t.end_turn();
    t.end_turn();

    assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());

    t.add_chr(Chr::builder("Lucky", 28, false).build())?;
    assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());

    Ok(())
}

#[test]
fn add_chr_after_in_turn_preserves_in_turn() -> TrackerResult {
    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        Chr::builder("Bucky", 30, true).build(),
        Chr::builder("Hellen", 27, true).build(),
        Chr::builder("Skelly Boy", 3, false).build(),
    ]).build();

    t.end_turn();
    t.end_turn();

    assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());
    t.add_chr(Chr::builder("Unlucky", 24, false).build())?;
    assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());

    Ok(())
}
