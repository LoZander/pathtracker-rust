use egui::Context;
use crate::{character::ChrName, saver::Saver, tracker::Tracker};

use super::Confirmation;

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct HealthWindow {
    show: bool,
    focus: bool,
    character: Option<ChrName>,
    health: HealthData
}

#[derive(Debug, Clone, Copy)]
#[derive(Default)]
struct HealthData {
    current: u32,
    max: u32,
    temp: u32
}

impl HealthWindow {
    pub fn open<S: Saver>(&mut self, tracker: &Tracker<S>, character: ChrName) {
        if let Some(health) = tracker.get_chr(&character).and_then(|c| c.health.as_ref()) {
            self.health = HealthData {
                current: health.current,
                max: health.max,
                temp: health.temp
            }
        }

        self.character = Some(character);

        self.show = true;
        self.focus = true;
    }

    pub fn close(&mut self) {
        self.show = false;
        self.focus = false;
        self.character = None;
        self.health = HealthData::default();
    }

    pub fn show(&mut self, tracker: &mut Tracker<impl Saver>, ctx: &Context) -> super::Result<()> {
        self.character.as_ref().map(|c| c.clone()).map_or(Ok(()), 
            |name| egui::Modal::new("health window".into()).show(ctx, |ui| {
                ui.heading(format!("Health of {name}"));

                ui.separator();

                self.show_health_input(ui);

                ui.separator();

                self.show_confirmation_bar(tracker, ui, &name)?;

                Ok(())
            }).inner)
    }

    fn show_confirmation_bar(&mut self, tracker: &mut Tracker<impl Saver>, ui: &mut egui::Ui, name: &ChrName) -> super::Result<()> {
        let confirmation = super::show_confirmation_bar(ui);

        match confirmation {
            Some(Confirmation::Confirm) => {
                tracker.change_max_health(name, self.health.max)?;
                tracker.set_current_health(name, self.health.current)?;
                tracker.set_temp_health(name, self.health.temp)?;

                self.close();
            },
            Some(Confirmation::Cancel) => self.close(),
            None => ()
        }

        Ok(())
    }

    fn show_health_input(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut self.health.current).range(0..=self.health.max));
            ui.label(" / ");
            ui.add(egui::DragValue::new(&mut self.health.max).range(0..=999));
            ui.label(" + ");
            ui.add(egui::DragValue::new(&mut self.health.temp).range(0..=999));
        });
    }
}
