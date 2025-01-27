use egui::Context;

use crate::{character::Chr, saver::Saver, tracker::Tracker};

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct RenameWindow {
    character: Option<Chr>,
    new_name: String,
    show: bool,
    focus: bool
}

impl RenameWindow {
    pub fn open(&mut self, character: Chr) {
        self.new_name = character.name.to_string();
        self.character = Some(character);
        self.show = true;
        self.focus = true;
    }

    pub fn close(&mut self) {
        self.show = false;
        self.focus = false;
    }

    pub fn init<S: Saver>(&mut self, tracker: &mut Tracker<S>, ctx: &Context) {
        if !self.show { return }
        egui::Modal::new("rename modal".into()).show(ctx, |ui| {
            ui.heading("Rename");
            let name_edit = ui.horizontal(|ui| {
                ui.label("new name: ");
                ui.text_edit_singleline(&mut self.new_name)
            }).inner;

            ui.separator();

            egui::Sides::new().show(ui,
                |_| {},
                |ui| {
                    if ui.button("confirm").clicked() {
                        if let Some(character) = &self.character {
                            if let Err(err) = tracker.rename(&character.name, self.new_name.clone()) {
                                super::error_window(ctx, "Save error", err.to_string());
                            }
                        }
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
