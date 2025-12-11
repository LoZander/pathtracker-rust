use egui::{Context, Id, Modal, Ui};

use crate::{character::{Chr, Health}, saver::Saver, tracker::Tracker};

use super::Confirmation;

#[derive(Default)]
#[allow(clippy::struct_excessive_bools)]
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

    pub const fn open(&mut self) {
        self.show = true;
        self.focus = true;
    }

    pub fn close(&mut self) {
        self.show = false;
        self.focus = false;
        self.reset();
    }

    pub fn show<S: Saver>(&mut self, tracker: &mut Tracker<S>, ctx: &Context) -> super::Result<()> {
        if !self.show { return Ok(()) }
        Modal::new(Id::new("add_character"))
            .show(ctx, |ui| {
                let name_edit = self.show_name_label(ui);

                self.show_initiative(ui);

                self.show_is_player(ui);

                self.show_health_tracking_option(ui);

                ui.separator();

                self.show_confirmation_bar(tracker, ui)?;

                if self.focus {
                    name_edit.request_focus();
                    self.focus = false;
                }

                Ok(())
            }).inner
    }

    fn show_confirmation_bar(&mut self, tracker: &mut Tracker<impl Saver>, ui: &mut Ui) -> super::Result<()> {
        let confirmation = super::show_confirmation_bar(ui);

        match confirmation {
            Some(Confirmation::Confirm) => {
                let c1 = Chr::builder(self.name.clone(), self.init, self.player);
                let c2 = if self.enable_health { c1.with_health(Health::new(self.health)) } else { c1 };
                let character = c2.build();
                tracker.add_chr(character)?;
                self.close();
            },
            Some(Confirmation::Cancel) => self.close(),
            None => (),
        }

        Ok(())
    }


    fn show_health_tracking_option(&mut self, ui: &mut Ui) {
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

    fn show_is_player(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.player, "Player");
    }

    fn show_initiative(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Initiative: ");
            let drag = egui::DragValue::new(&mut self.init).range(0..=50);
            ui.add(drag);
        });
    }

    fn show_name_label(&mut self, ui: &mut Ui) -> egui::Response {
        ui.heading("Add character");

        ui.horizontal(|ui| {
            ui.label("Name: ");
            ui.text_edit_singleline(&mut self.name)
        }).inner
    }
}
