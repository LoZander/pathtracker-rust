use pathtracker_rust::{
    character::Chr,
    tracker::Tracker,
    tracker::TrackerResult,
};

#[test]
fn rm_chr_before_in_turn_preserves_in_turn() -> TrackerResult {
    let mut t = Tracker::new(vec![
        Chr::builder("Bucky", 30, true).build(),
        Chr::builder("Hellen", 27, true).build(),
        Chr::builder("Skelly Boy", 3, false).build(),
    ]);

    t.end_turn();
    t.end_turn();

    assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());
    t.rm_chr("Bucky")?;
    assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());

    Ok(())
}

#[test]
fn rm_chr_after_in_turn_preserves_in_turn() -> TrackerResult {
    let mut t = Tracker::new(vec![
        Chr::builder("Bucky", 30, true).build(),
        Chr::builder("Hellen", 27, true).build(),
        Chr::builder("Skelly Boy", 3, false).build(),
    ]);

    t.end_turn();
    t.end_turn();

    assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());
    t.rm_chr("Skelly Boy")?;
    assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());

    Ok(())
}

#[test]
fn rm_only_chr_in_turn_makes_no_one_in_turn() -> TrackerResult {
    let mut t = Tracker::new(vec![
        Chr::builder("Bucky", 30, true).build(),
    ]);

    t.end_turn();

    assert_eq!(Some(&Chr::builder("Bucky", 30, true).build()), t.get_in_turn());
    t.rm_chr("Bucky")?;
    assert_eq!(None, t.get_in_turn());

    Ok(())
}

#[test]
fn rm_only_chr_makes_no_one_in_turn() -> TrackerResult {
    let mut t = Tracker::new(vec![
        Chr::builder("Bucky", 30, true).build(),
    ]);

    assert_eq!(None, t.get_in_turn());
    t.rm_chr("Bucky")?;
    assert_eq!(None, t.get_in_turn());

    Ok(())
}

#[test]
fn rm_in_turn_makes_it_next_ups_turn() -> TrackerResult {
    let mut t = Tracker::new(vec![
        Chr::builder("Bucky", 30, true).build(),
        Chr::builder("Hellen", 27, true).build(),
        Chr::builder("Skelly Boy", 3, false).build(),
    ]);

    t.end_turn();
    t.end_turn();

    assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.get_in_turn());
    t.rm_chr("Hellen")?;
    assert_eq!(Some(&Chr::builder("Skelly Boy", 3, false).build()), t.get_in_turn());

    Ok(())
}

#[test]
fn rm_last_when_its_turn_makes_it_top_of_round() -> TrackerResult {
    let mut t = Tracker::new(vec![
        Chr::builder("Bucky", 30, true).build(),
        Chr::builder("Hellen", 27, true).build(),
        Chr::builder("Skelly Boy", 3, false).build(),
    ]);

    t.end_turn();
    t.end_turn();
    t.end_turn();

    assert_eq!(Some(&Chr::builder("Skelly Boy", 3, false).build()), t.get_in_turn());
    t.rm_chr("Skelly Boy")?;
    assert_eq!(Some(&Chr::builder("Bucky", 30, true).build()), t.get_in_turn());

    Ok(())
}
