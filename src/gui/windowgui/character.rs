use egui::{Context, ProgressBar, Ui};

use crate::{character::{Chr, Health}, saver::Saver, tracker::Tracker};

#[derive(Debug, Clone)]
pub enum Response {
    RemoveCharacter(Chr),
    OpenCondWindow(Chr)
}

const IN_TURN_OFFSET: f32 = 20.0;
const SEP: f32 = 10.0;

pub fn init<S: Saver>(tracker: &Tracker<S>, ui: &mut Ui, character: &Chr) -> Option<Response> {
    ui.style_mut().spacing.indent = 20.0;
    let is_in_turn = tracker.get_in_turn() == Some(character);
    let mut conditions: Vec<_> = tracker.get_conditions(&character.name).into_iter().map(ToOwned::to_owned).collect();
    conditions.sort();
    let (left, right) = egui::containers::Sides::new().show(ui,
        |ui| {
            if is_in_turn {
                ui.add(egui::Label::new(egui::RichText::new(character.init.to_string()).heading().strong()));
                ui.strong(character.name.clone());

                ui.add_space(IN_TURN_OFFSET);
            } else {
                ui.add_space(IN_TURN_OFFSET);
                
                ui.heading(character.init.to_string());
                ui.label(character.name.clone());
            }
            
            ui.add_space(SEP);

            if let Some(hp) = &character.health {
                ui.add(health_bar(hp));
                ui.add_space(SEP);
            } else {
                ui.add_space(HP_WIDTH + SEP + SEP);
            }
        },
        |ui| {
            let remove = if ui.small_button("x").clicked() {
                Some(Response::RemoveCharacter(character.clone()))
            } else {
                None
            };

            let mut open_cond_window_menu = None;
            ui.menu_button("...", |ui| {
                if ui.button("Conditions").clicked() {
                    open_cond_window_menu = Some(Response::OpenCondWindow(character.clone()));
                }
            });

            let condition_str = conditions.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");

            let open_cond_window = if ui.add(egui::Label::new(condition_str).truncate()).clicked() {
                Some(Response::OpenCondWindow(character.clone()))
            } else {
                None
            };

            open_cond_window_menu
                .or(open_cond_window)
                .or(remove)
        }
    );
    right
}

const HP_WIDTH: f32 = 100.0;

fn health_bar(hp: &Health) -> ProgressBar {
    let rel_hp: f32 = (hp.current as f32) / (hp.max as f32);
    egui::ProgressBar::new(rel_hp)
        .text(format!("{}/{}", hp.current, hp.max))
        .rounding(2.0)
        .desired_width(HP_WIDTH)
}
