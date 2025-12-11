use egui::Context;

use crate::{character::{Chr, ChrName}, saver::Saver, tracker::Tracker};

use super::Confirmation;

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct RenameWindow {
    character: Option<ChrName>,
    new_name: String,
    show: bool,
    focus: bool
}

impl RenameWindow {
    fn reset(&mut self) {
        self.character = None;
        self.new_name = String::new();
    }

    pub fn open(&mut self, character: ChrName) {
        self.new_name = character.to_string();
        self.character = Some(character);
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
        egui::Modal::new("rename modal".into()).show(ctx, |ui| {
            ui.heading("Rename");
            let name_edit = ui.horizontal(|ui| {
                ui.label("new name: ");
                ui.text_edit_singleline(&mut self.new_name)
            }).inner;

            ui.separator();

            let confirmation = super::show_confirmation_bar(ui);

            match confirmation {
                Some(Confirmation::Confirm) => {
                    if let Some(character) = &self.character {
                        tracker.rename(character, self.new_name.clone())?;
                        // if let Err(err) = tracker.rename(&character.name, self.new_name.clone()) {
                            
                        //     super::error_window(ctx, "Error", err.to_string());
                        // }
                    }
                    self.close();
                    
                },
                Some(Confirmation::Cancel) => self.close(),
                None => ()
            }

            if self.focus {
                name_edit.request_focus();
                self.focus = false;
            }

            Ok(())
        }).inner
    }
}
