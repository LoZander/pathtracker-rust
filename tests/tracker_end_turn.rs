use pathtracker_rust::{
    character::Chr, saver::NoSaver, tracker::{self, Tracker}
};

#[test]
fn end_turn_loops_around() -> tracker::Result<()> {
    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        Chr::builder("Bucky", 30, true).build(),
        Chr::builder("Hellen", 27, true).build(),
        Chr::builder("Skelly Boy", 3, false).build(),
    ]).build();

    t.end_turn()?;
    t.end_turn()?;

    assert_eq!(Ok(Some(&Chr::builder("Skelly Boy", 3, false).build())), t.end_turn());
    assert_eq!(Ok(Some(&Chr::builder("Bucky", 30, true).build())), t.end_turn());

    Ok(())
}

#[test]
fn end_turn_when_no_chrs_makes_it_no_ones_turn() -> tracker::Result<()> {
    let mut t: Tracker<NoSaver> = Tracker::default();
    assert!(t.end_turn()?.is_none());

    Ok(())
}
