use egui::Context;

use crate::{gui::windowgui::Confirmation, saver::Saver, settings::Pf2eVersion, tracker::{self, Tracker}};

#[derive(Debug, Clone, Default)]
pub struct SettingsWindow {
    open: bool,
    remastered: bool,
    undo_size: usize,
}

impl SettingsWindow {
    pub fn open<S: Saver>(&mut self, tracker: &Tracker<S>) {
        self.open = true;
        self.remastered = match tracker.get_pf2e_version_setting() {
            Pf2eVersion::Old => false,
            Pf2eVersion::Remastered => true,
        };
        self.undo_size = tracker.get_undo_size_setting();
    }

    pub fn show<S: Saver>(&mut self, tracker: &mut Tracker<S>, ctx: &Context) -> tracker::Result<()> {
        if !self.open { return Ok(()) }

        let open = &mut self.open;
        egui::Window::new("Settings")
            .open(open)
            .show(ctx, |ui| {

                ui.checkbox(&mut self.remastered, "Remastered");

                let undo_size_slider = egui::Slider::new(&mut self.undo_size, 0..=124).text("Undo history size");
                ui.add(undo_size_slider);

                match super::show_confirmation_bar(ui) {
                    None => Ok(()),
                    Some(Confirmation::Cancel) => {
                        ui.close_kind(egui::UiKind::Window);
                        Ok(())
                    }
                    Some(Confirmation::Confirm) => {
                        if self.remastered { 
                            tracker.set_pf2e_version_setting(Pf2eVersion::Remastered);
                        } else {
                            tracker.set_pf2e_version_setting(Pf2eVersion::Old);
                        }

                        tracker.set_undo_size_setting(self.undo_size);

                        tracker.auto_save()?;
                        ui.close_kind(egui::UiKind::Window);

                        Ok(())
                    }
                }
            }).and_then(|res| res.inner).unwrap_or(Ok(()))
    }
}

