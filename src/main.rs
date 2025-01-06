use pathtracker_rust::character::Chr;
use pathtracker_rust::saver::{self, FileSaver};
use pathtracker_rust::{gui, tracker};
use pathtracker_rust::tracker::Tracker;
use pathtracker_rust::gui::terminalgui;


fn main() {
    let mut t: Tracker<FileSaver> = match Tracker::load(&FileSaver, "auto.save") {
        Ok(t) => t,
        Err(tracker::Error::LoadError(saver::Error::LoadIOError(_, _))) => Tracker::default(),
        Err(e) => panic!("{e:?}")
    };

    //terminalgui::run(t).expect("Tracker Error");

    gui::window::run(t);
}
