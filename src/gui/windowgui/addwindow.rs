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
                let name_edit = self.init_name_label(ui);

                self.init_initiative(ui);

                self.init_is_player(ui);

                self.init_health_tracking(ui);

                ui.separator();

                self.init_confirmation_bar(tracker, ctx, ui);

                if self.focus {
                    name_edit.request_focus();
                    self.focus = false;
                }
                
            });
    }

    fn init_confirmation_bar(&mut self, tracker: &mut Tracker<impl Saver>, ctx: &Context, ui: &mut Ui) {
        egui::Sides::new().show(ui, 
            |_| {},
            |ui| {
            if ui.button("confirm").clicked() {
                let c1 = Chr::builder(self.name.clone(), self.init, self.player);
                let c2 = if self.enable_health { c1.with_health(Health::new(self.health)) } else { c1 };
                let character = c2.build();
                if let Err(err) = tracker.add_chr(character) {
                    error_window(ctx, "Save error", err.to_string());
                }
                self.close();
            }
            if ui.button("cancel").clicked() {
                self.close();
            }
        });
    }

    fn init_health_tracking(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.enable_health, "Track HP");

            if self.enable_health {
                ui.add_space(12.0);
                ui.label("Max HP:");
                let drag = egui::DragValue::new(&mut self.health).range(0..=999);
                ui.add(drag);
            }   
        });
    }

    fn init_is_player(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.player, "Player");
    }

    fn init_initiative(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Initiative: ");
            let drag = egui::DragValue::new(&mut self.init).range(0..=50);
            ui.add(drag);
        });
    }

    fn init_name_label(&mut self, ui: &mut Ui) -> egui::Response {
        ui.heading("Add character");
        
        ui.horizontal(|ui| {
            ui.label("Name: ");
            ui.text_edit_singleline(&mut self.name)
        }).inner
    }

}
