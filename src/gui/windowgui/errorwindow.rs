use egui::Context;

use super::Error;

#[derive(Debug)]
#[derive(Default)]
pub struct ErrorWindow {
    open: bool,
    err: Option<Error>
}

impl ErrorWindow {
    pub fn open(&mut self, err: Error) {
        self.err = Some(err);
        self.open = true;
    }

    const fn close(&mut self) {
        self.open = false;
    }

    pub fn show(&mut self, ctx: &Context) {
        if !self.open { return }
        egui::Modal::new("error".into()).show(ctx, |ui| {
            ui.heading("Error");

            ui.separator();
            
            ui.label(self.err.as_ref().map_or("no error? This window opening is an error in and of itself.".into(), ToString::to_string));

            ui.separator();

            egui::Sides::new().show(ui, 
                |_|{},
                |ui|{
                   if ui.button("ok").clicked() {
                       self.close();
                   }
                });
        });
    }
}
