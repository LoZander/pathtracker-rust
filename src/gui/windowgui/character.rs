use std::ops::Index;

use egui::{Align, Context, ProgressBar, Ui};
use egui_extras::{Column, TableBuilder};

use crate::{character::{Chr, Health}, saver::Saver, tracker::Tracker};

#[derive(Debug, Clone)]
pub enum Response {
    RemoveCharacter(Chr),
    OpenCondWindow(Chr)
}

const IN_TURN_OFFSET: f32 = 20.0;
const SEP: f32 = 10.0;
const ENTRY_HEIGHT: f32 = 25.0;

pub fn init_characters<S: Saver>(tracker: &Tracker<S>, ui: &mut Ui) -> Vec<Response> {
    let mut table = TableBuilder::new(ui)
        .cell_layout(egui::Layout::left_to_right(Align::Center))
        .auto_shrink(false)
        .striped(true)
        .column(Column::exact(20.0))
        .column(Column::auto()) // Initiative and name
        .column(Column::auto()) // Optional health
        .column(Column::remainder())
        .column(Column::auto()) // Conditions
        .column(Column::auto())
        .column(Column::auto());

    let in_turn_index = tracker.get_chrs().iter().enumerate().find(|(_, c)| Some(*c) == tracker.get_in_turn()).map(|(i, _)| i);
    if let Some(index) = in_turn_index {
        table = table.scroll_to_row(index, Some(Align::Center));
    }

    let mut responses = Vec::new();

    table.body(|body| {
        let row_height = 18.0;
        let number_of_rows = tracker.get_chrs().len();
        body.rows(row_height, number_of_rows, |mut row| {
            let index = row.index();
            let character = &tracker.get_chrs()[index];
            let is_in_turn = Some(character) == tracker.get_in_turn();

            row.col(|ui| {
                if is_in_turn {
                    ui.add(egui::Label::new(egui::RichText::new(">").heading().strong()));
                }
            });

            row.col(|ui| {
                if is_in_turn {
                    ui.add(egui::Label::new(egui::RichText::new(format!("{:>2}", character.init.to_string())).size(18.0).monospace().strong()));
                    ui.add(egui::Label::new(egui::RichText::new(character.name.clone()).size(16.0).strong()));
                } else {
                    ui.add(egui::Label::new(egui::RichText::new(format!("{:>2}", character.init.to_string())).size(18.0).monospace()));
                    ui.add(egui::Label::new(egui::RichText::new(character.name.clone()).size(16.0)));
                }
            });

            row.col(|ui| {
                if let Some(health) = &character.health {
                    ui.add(health_bar(health));
                };
            });

            row.col(|_| {});

            row.col(|ui| {
                let mut conditions: Vec<_> = tracker.get_conditions(&character.name).into_iter().map(ToOwned::to_owned).collect();
                conditions.sort();
                let condition_str = conditions.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");

                if ui.add(egui::Label::new(condition_str).truncate().halign(Align::Max)).clicked() {
                    responses.push(Response::OpenCondWindow(character.clone()));
                }
            });

            row.col(|ui| {
                ui.menu_button("...", |ui| {
                    if ui.button("Conditions").clicked() {
                        responses.push(Response::OpenCondWindow(character.clone()));
                    }
                });
            });

            row.col(|ui| {
                if ui.small_button("x").clicked() {
                    responses.push(Response::RemoveCharacter(character.clone()));
                }
            });
        });
    });

    responses
}

pub fn init_left<S: Saver>(tracker: &Tracker<S>, ui: &mut Ui, character: &Chr) {
    let is_in_turn = tracker.get_in_turn() == Some(character);
    ui.horizontal(|ui| {
        ui.set_min_height(ENTRY_HEIGHT);
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
            ui.add_space(HP_WIDTH + SEP);
        }
    });
}

pub fn init_right<S: Saver>(tracker: &Tracker<S>, ui: &mut Ui, character: &Chr) -> Option<Response> {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {        
        ui.set_min_height(ENTRY_HEIGHT);
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

        let mut conditions: Vec<_> = tracker.get_conditions(&character.name).into_iter().map(ToOwned::to_owned).collect();
        conditions.sort();
        let condition_str = conditions.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");

        let open_cond_window = if ui.add(egui::Label::new(condition_str).truncate()).clicked() {
            Some(Response::OpenCondWindow(character.clone()))
        } else {
            None
        };

        open_cond_window_menu
            .or(open_cond_window)
            .or(remove)
    }).inner
}

const HP_WIDTH: f32 = 100.0;

fn health_bar(hp: &Health) -> ProgressBar {
    let rel_hp: f32 = (hp.current as f32) / (hp.max as f32);
    egui::ProgressBar::new(rel_hp)
        .text(format!("{}/{}", hp.current, hp.max))
        .rounding(2.0)
        .desired_width(HP_WIDTH)
}
