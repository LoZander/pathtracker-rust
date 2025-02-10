use egui::{Align, ProgressBar, Ui}; use egui_extras::{Column, TableBuilder, TableRow};
use crate::{character::{Chr, Health}, saver::Saver, tracker::Tracker};

#[derive(Debug, Clone)]
pub enum Response {
    RemoveCharacter(Chr),
    OpenCondWindow(Chr),
    RenameCharacter(Chr),
    OpenHealthWindow(Chr)
}

pub fn show<S: Saver>(tracker: &Tracker<S>, ui: &mut Ui) -> Vec<Response> {
    let mut table = TableBuilder::new(ui)
        .cell_layout(egui::Layout::left_to_right(Align::Center))
        .auto_shrink(false)
        .striped(true)
        .column(Column::exact(20.0))
        .column(Column::auto()) // Initiative and name
        .column(Column::auto()) // Optional health
        .column(Column::remainder())
        .column(Column::auto()) // Conditions
        .column(Column::auto()) // Options
        .column(Column::auto()); // Remove

    let in_turn_index = tracker.get_chrs().iter().enumerate().find(|(_, c)| Some(*c) == tracker.get_in_turn()).map(|(i, _)| i);
    if let Some(index) = in_turn_index {
        table = table.scroll_to_row(index, Some(Align::Center));
    }

    let mut responses = Vec::new();

    table.body(|body| {
        let row_height = 30.0;
        let number_of_rows = tracker.get_chrs().len();
        body.rows(row_height, number_of_rows, |mut row| {
            let index = row.index();
            let character = &tracker.get_chrs()[index];
            let is_in_turn = Some(character) == tracker.get_in_turn();

            show_in_turn_marker_col(&mut row, is_in_turn);

            show_name_col(&mut responses, &mut row, character, is_in_turn);

            show_health_col(&mut row, character);

            row.col(|_| {});

            show_conds_col(tracker, &mut responses, &mut row, character);

            show_options_col(&mut responses, &mut row, character);

            show_remove_col(&mut responses, row, character);
        });
    });

    responses
}

fn show_remove_col(responses: &mut Vec<Response>, mut row: TableRow<'_, '_>, character: &Chr) {
    row.col(|ui| {
        if ui.small_button("x").clicked() {
            responses.push(Response::RemoveCharacter(character.clone()));
        }
    });
}

#[allow(clippy::collapsible_if)]
fn show_options_col(responses: &mut Vec<Response>, row: &mut TableRow<'_, '_>, character: &Chr) {
    row.col(|ui| {
        ui.menu_button("...", |ui| {
            // NB: these ifs are nested instead of collapsed using && as the
            // condition of the inner is effectful and adds a button. 
            // As such, collapsing the two using &&, while having the exact 
            // same behaviour due to the shortcircuting of &&, might accidentally
            // imply that the order is insignificant.
            if character.health.is_some() {
                if ui.button("Health").clicked() {
                    responses.push(Response::OpenHealthWindow(character.clone()));
                }
            }
            if ui.button("Conditions").clicked() {
                responses.push(Response::OpenCondWindow(character.clone()));
            }
        });
    });
}

fn show_conds_col(tracker: &Tracker<impl Saver>, responses: &mut Vec<Response>, row: &mut TableRow<'_, '_>, character: &Chr) {
    row.col(|ui| {
        let mut conditions: Vec<_> = tracker.get_conditions(&character.name).into_iter().map(ToOwned::to_owned).collect();
        conditions.sort();
        let condition_str = conditions.iter().take(2).map(ToString::to_string).collect::<Vec<_>>().join("\n");

        let conds = if conditions.len() <= 2 {
            ui.add(egui::Label::new(condition_str).halign(Align::Max))
        } else {
            ui.add(egui::Label::new(format!("{condition_str} (+)")))
        };

        if conds.clicked() {
            responses.push(Response::OpenCondWindow(character.clone()));
        }

        conds.on_hover_text(conditions.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n"));
    });
}

fn show_in_turn_marker_col(row: &mut TableRow<'_, '_>, is_in_turn: bool) {
    row.col(|ui| {
        if is_in_turn {
            ui.add(egui::Label::new(egui::RichText::new(">").heading().strong()));
        }
    });
}

fn show_health_col(row: &mut TableRow<'_, '_>, character: &Chr) {
    row.col(|ui| {
        if let Some(health) = &character.health {
            ui.add(health_bar(health));
        }
    });
}

fn show_name_col(responses: &mut Vec<Response>, row: &mut TableRow<'_, '_>, character: &Chr, is_in_turn: bool) {
    row.col(|ui| {
        let name = if is_in_turn {
            ui.add(egui::Label::new(egui::RichText::new(format!("{:>2}", character.init.to_string())).size(18.0).monospace().strong()));
            ui.add(egui::Label::new(egui::RichText::new(character.name.clone()).size(16.0).strong()))
        } else {
            ui.add(egui::Label::new(egui::RichText::new(format!("{:>2}", character.init.to_string())).size(18.0).monospace()));
            ui.add(egui::Label::new(egui::RichText::new(character.name.clone()).size(16.0)))
        };

        if name.clicked() {
            responses.push(Response::RenameCharacter(character.clone()));
        }
    });
}

const HP_WIDTH: f32 = 100.0;

fn health_bar(hp: &Health) -> ProgressBar {
    let rel_hp: f32 = (hp.current as f32) / (hp.max as f32);

    let hp_str = if hp.temp > 0 {
        format!("{}/{} + {}", hp.current, hp.max, hp.temp)
    } else {
        format!("{}/{}", hp.current, hp.max)
    };
    
    egui::ProgressBar::new(rel_hp)
        .text(hp_str)
        .rounding(2.0)
        .desired_width(HP_WIDTH)
}
