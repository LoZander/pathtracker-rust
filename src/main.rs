use pathtracker_rust::saver::FileSaver;
use pathtracker_rust::{character::Chr, saver::NoSaver, tracker::Tracker};
use pathtracker_rust::gui::terminalgui;


fn main() {
    println!("Hello, world!");  
    let mut t: Tracker<FileSaver> = Tracker::builder().with_chrs(vec![
        Chr::builder("Kristy", 20, true).build(),
        Chr::builder("Frog", 27, false).with_dex(3).build(),
        Chr::builder("Link", 22, true).build(),
    ]).build();

    t.end_turn();
    t.end_turn();

    terminalgui::run(t).unwrap()
}
