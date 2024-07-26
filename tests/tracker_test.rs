use pathtracker_rust::{
    character::Chr, saver::NoSaver, tracker::Tracker
};

#[test]
fn initial_chrs_have_descending_initiative_order() {
    let mut t: Tracker<NoSaver> = Tracker::builder().with_chrs(vec![
        Chr::builder("Skelly Boy", 3, false).build(),
        Chr::builder("Bucky", 30, true).build(),
        Chr::builder("Hellen", 27, true).build(),
    ]).build();

    assert_eq!(Ok(Some(&Chr::builder("Bucky", 30, true).build())), t.end_turn());
    assert_eq!(Ok(Some(&Chr::builder("Hellen", 27, true).build())), t.end_turn());
    assert_eq!(Ok(Some(&Chr::builder("Skelly Boy", 3, false).build())), t.end_turn())
}
