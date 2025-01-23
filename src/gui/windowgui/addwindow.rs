use egui::{Context, Id, Modal, Ui};

use crate::{character::{Chr, Health}, saver::Saver, tracker::Tracker};

use super::error_window;

#[derive(Default)]
pub struct AddWindow {
    show: bool,
    focus: bool,
    name: String,
    init: i32,
    player: bool,
    enable_health: bool,
    health: u32
}



impl AddWindow {
    fn reset(&mut self) {
        self.name = String::new();
        self.init = 0;
        self.player = false;
        self.enable_health = false;
    }

    pub fn open(&mut self) {
        self.show = true;
        self.focus = true;
    }

    pub fn close(&mut self) {
        self.show = false;
        self.focus = false;
        self.reset();
    }

    pub fn init<S: Saver>(&mut self, tracker: &mut Tracker<S>, ctx: &Context) {
        if !self.show { return }
        Modal::new(Id::new("add_character"))
            .show(ctx, |ui| {
                ui.heading("Add character");
                let name_edit = ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.text_edit_singleline(&mut self.name)
                }).inner;

                ui.horizontal(|ui| {
                    ui.label("Initiative: ");
                    let drag = egui::DragValue::new(&mut self.init).range(0..=50);
                    ui.add(drag);
                });

                ui.checkbox(&mut self.player, "Player");

                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.enable_health, "Track HP");

                    if self.enable_health {
                        ui.add_space(12.0);
                        ui.label("Max HP:");
                        let drag = egui::DragValue::new(&mut self.health).range(0..=999);
                        ui.add(drag);
                    }   
                });

                ui.separator();

                egui::Sides::new().show(ui, 
                    |_| {},
                    |ui| {
                    if ui.button("confirm").clicked() {
                        let c1 = Chr::builder(self.name.clone(), self.init, self.player);
                        let c2 = if self.enable_health { c1.with_health(Health::new(self.health)) } else { c1 };
                        let character = c2.build();
                        if let Err(err) = tracker.add_chr(character) {
                            error_window(ctx, "Save error", err.to_string());
                        };
                        self.close();
                    }
                    if ui.button("cancel").clicked() {
                        self.close();
                    }
                });

                if self.focus {
                    name_edit.request_focus();
                    self.focus = false;
                }
                
            });
    }

}
