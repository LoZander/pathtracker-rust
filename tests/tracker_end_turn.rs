use pathtracker_rust::{
    tracker::Tracker,
    character::Chr,
};

#[test]
fn end_turn_loops_around() {
    let mut t = Tracker::new(vec![
        Chr::builder("Bucky", 30, true).build(),
        Chr::builder("Hellen", 27, true).build(),
        Chr::builder("Skelly Boy", 3, false).build(),
    ]);

    t.end_turn();
    t.end_turn();

    assert_eq!(Some(&Chr::builder("Skelly Boy", 3, false).build()), t.end_turn());
    assert_eq!(Some(&Chr::builder("Bucky", 30, true).build()), t.end_turn())
}

#[test]
fn end_turn_when_no_chrs_makes_it_no_ones_turn() {
    let mut t = Tracker::new(vec![]);
    assert!(t.end_turn().is_none())
}
