use pathtracker_rust::{
    character::Chr,
    tracker::Tracker,
};

#[test]
fn initial_chrs_have_descending_initiative_order() {
    let mut t = Tracker::new(vec![
        Chr::builder("Skelly Boy", 3, false).build(),
        Chr::builder("Bucky", 30, true).build(),
        Chr::builder("Hellen", 27, true).build(),
    ]);

    assert_eq!(Some(&Chr::builder("Bucky", 30, true).build()), t.end_turn());
    assert_eq!(Some(&Chr::builder("Hellen", 27, true).build()), t.end_turn());
    assert_eq!(Some(&Chr::builder("Skelly Boy", 3, false).build()), t.end_turn())
}
