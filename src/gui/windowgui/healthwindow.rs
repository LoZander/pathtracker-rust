use egui::Context;
use crate::{character::Chr, saver::Saver, tracker::Tracker};

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct HealthWindow {
    show: bool,
    focus: bool,
    character: Option<Chr>,
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
    pub fn open(&mut self, character: Chr) {
        if let Some(health) = &character.health { 
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

    pub fn init(&mut self, tracker: &mut Tracker<impl Saver>, ctx: &Context) {
        if let Some(name) = self.character.as_ref().map(|c| c.name.to_string()) { 
            egui::Modal::new("health window".into()).show(ctx, |ui| {
                ui.heading(format!("Health of {name}"));

                ui.separator();

                self.init_health_input(ui);

                ui.separator();

                self.init_confirmation_bar(tracker, ctx, ui, &name);
            }); 
        }
    }

    fn init_confirmation_bar(&mut self, tracker: &mut Tracker<impl Saver>, ctx: &Context, ui: &mut egui::Ui, name: &str) {
        egui::Sides::new().show(ui, 
            |_| {},
            |ui| {
                if ui.button("confirm").clicked() {
                    let res = tracker.change_max_health(name, self.health.max)
                        .and(tracker.set_current_health(name, self.health.current))
                        .and(tracker.set_temp_health(name, self.health.temp));

                    if let Err(err) = res {
                        super::error_window(ctx, "Error", err.to_string());
                    }

                    self.close();
                }

                if ui.button("cancel").clicked() {
                    self.close();
                }
            });
    }

    fn init_health_input(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut self.health.current).range(0..=self.health.max));
            ui.label(" / ");
            ui.add(egui::DragValue::new(&mut self.health.max).range(0..=999));
            ui.label(" + ");
            ui.add(egui::DragValue::new(&mut self.health.temp).range(0..=999));
        });
    }
}
