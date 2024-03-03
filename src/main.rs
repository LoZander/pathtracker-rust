use pathtracker_rust::{character::Chr, gui::{terminalgui::TerminalGui, Gui}, tracker::Tracker};


fn main() {
    println!("Hello, world!");  
    let mut t = Tracker::new(vec![
        Chr::builder("Kristy", 20, true).build(),
        Chr::builder("Frog", 27, false).with_dex(3).build(),
        Chr::builder("Link", 22, true).build(),
    ]);

    t.end_turn();
    t.end_turn();

    TerminalGui::run(t).unwrap()
}
