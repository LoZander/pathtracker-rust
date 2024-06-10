use pathtracker_rust::saver::{self, FileSaver};
use pathtracker_rust::tracker;
use pathtracker_rust::{character::Chr, saver::NoSaver, tracker::Tracker};
use pathtracker_rust::gui::terminalgui;


fn main() {
    println!("Hello, world!");  

    let t: Tracker<FileSaver> = match Tracker::load(FileSaver, "auto.save") {
        Ok(t) => t,
        Err(tracker::Error::LoadError(saver::Error::LoadIOError(_, _))) => Tracker::default(),
        Err(e) => panic!("{:?}", e)
    };
    terminalgui::run(t).unwrap()
}
