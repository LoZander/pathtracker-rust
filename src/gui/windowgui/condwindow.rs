use std::fmt::Display;

use egui::{vec2, Context, Ui};

use crate::{character::Chr, conditions::{Condition, DamageType, NonValuedCondition, ValuedCondition}, saver::Saver, tracker::Tracker};

use super::error_window;

#[derive(Debug, Clone)]
pub enum Response {
    AddCondition{character: String, cond: Condition},
    RemoveCondition{character: String, cond: Condition}
}

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
enum ConditionEntry {
    Valued(ValuedCondition),
    NonValued(NonValuedCondition),
}

impl Default for ConditionEntry {
    fn default() -> Self {
        Self::NonValued(NonValuedCondition::FlatFooted)
    }
}

impl Display for ConditionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Valued(valued_condition) => write!(f, "{valued_condition}"),
            Self::NonValued(non_valued_condition) => write!(f, "{non_valued_condition}"),
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Default)]
pub struct CondWindow {
    open: bool,
    data: Data
}

#[derive(Debug, Clone)]
#[derive(Default)]
struct Data {
    character: Option<Chr>,
    selected: ConditionEntry,
    cond_value: u8
}

impl CondWindow {
    pub fn open(&mut self, character: Chr) {
        self.reset();
        self.data.character = Some(character);
        self.open = true;
    }

    fn reset(&mut self) {
        self.data.character = None;
        self.data.selected = ConditionEntry::default();
        self.data.cond_value = 0;
    }

    pub fn close(&mut self) {
        self.open = false;
    }
    
    pub fn show<S: Saver>(&mut self, tracker: &mut Tracker<S>, ctx: &Context) {
        //let frame = egui::Frame::default().inner_margin(egui::Margin::symmetric(20.0, 20.0));
        let open = &mut self.open;
        let data = &mut self.data;
        if let Some(character) = data.character.clone() {
            egui::Window::new(format!("{} Conditions", character.name))
                .open(open)
                .show(ctx, |ui| {
                    let responses = ui.horizontal(|ui| {
                        let add = add_cond_section(ui, data, &character);
                
                        let mut remove = cond_list_section(tracker, ui, &character);

                        if let Some(inner) = add {
                            remove.push(inner);
                        }

                        remove
                    }).inner;

                    for resp in responses {
                        match resp {
                            Response::AddCondition { character, cond } => {
                                if let Err(err) = tracker.add_condition(&character, cond) {
                                    error_window(ctx, "Save error", err.to_string());
                                }
                            },
                            Response::RemoveCondition { character, cond } => tracker.rm_condition(&character, &cond),
                        }
                    }
                });
        }
    }

}

fn add_cond_section(ui: &mut Ui, data: &mut Data, character: &Chr) -> Option<Response> {
    ui.vertical(|ui| {
        ui.label("Add Condition:");

        cond_selector(ui, data);

        add_button(ui, data, character)
    }).inner
}

fn add_button(ui: &mut Ui, data: &Data, character: &Chr) -> Option<Response> {
    if ui.button("Add").clicked() {
        let condition = match data.selected {
            ConditionEntry::Valued(valued_condition) => {
                Condition::builder().condition(valued_condition)
                    .value(data.cond_value)
                    .build()
            },
            ConditionEntry::NonValued(non_valued_condition) => {
                Condition::builder().condition(non_valued_condition)
                    .build()
            },
        };

        Some(Response::AddCondition { character: character.name.to_string(), cond: condition })
    } else {
        None
    }
}

fn cond_selector(ui: &mut Ui, data: &mut Data) {
    egui::ComboBox::from_label("Condition")
        .selected_text(format!("{}", data.selected))
        .show_ui(ui, |ui| selectable_conds(ui, data));

    if let ConditionEntry::Valued(cond) = data.selected {
        ui.horizontal(|ui| {
            ui.label(cond.to_string());
            let drag = egui::DragValue::new(&mut data.cond_value).range(0..=9);
            ui.add(drag); 
        });
    }
}

fn selectable_conds(ui: &mut Ui, data: &mut Data) {
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Bleed));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Poison));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Piercing));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Bludgeoning));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Slashing));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Acid));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Cold));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Electricity));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Sonic));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Positive));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Negative));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Force));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Chaotic));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Evil));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Good));
    selectable_valued_cond(ui, data, ValuedCondition::PersistentDamage(DamageType::Lawful));
    selectable_valued_cond(ui, data, ValuedCondition::Clumsy);
    selectable_valued_cond(ui, data, ValuedCondition::Doomed);
    selectable_valued_cond(ui, data, ValuedCondition::Drained);
    selectable_valued_cond(ui, data, ValuedCondition::Dying);
    selectable_valued_cond(ui, data, ValuedCondition::Enfeebled);
    selectable_valued_cond(ui, data, ValuedCondition::Frightened);
    selectable_valued_cond(ui, data, ValuedCondition::Sickened);
    selectable_valued_cond(ui, data, ValuedCondition::Slowed);
    selectable_valued_cond(ui, data, ValuedCondition::Stunned);
    selectable_valued_cond(ui, data, ValuedCondition::Stupified);
    selectable_valued_cond(ui, data, ValuedCondition::Wounded);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Blinded);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Broken);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Concealed);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Confused);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Controlled);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Dazzled);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Deafened);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Encumbered);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Fascinated);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Fatigued);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::FlatFooted);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Fleeing);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Friendly);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Grabbed);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Helpful);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Hidden);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Hostile);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Immobilized);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Indifferent);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Invisible);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Observed);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Paralyzed);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Petrified);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Prone);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Quickened);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Restrained);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Unconscious);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Undetected);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Unfriendly);
    selectable_nonvalued_cond(ui, data, NonValuedCondition::Unnoticed);
}

fn selectable_nonvalued_cond(ui: &mut Ui, data: &mut Data, cond: NonValuedCondition) {
    ui.selectable_value(&mut data.selected, ConditionEntry::NonValued(cond), cond.to_string());
}

fn selectable_valued_cond(ui: &mut Ui, data: &mut Data, cond: ValuedCondition) {
    ui.selectable_value(&mut data.selected, ConditionEntry::Valued(cond), cond.to_string());
}

fn cond_list_section(tracker: &Tracker<impl Saver>, ui: &mut Ui, character: &Chr) -> Vec<Response> {
    ui.group(|ui| {
        ui.set_min_size(vec2(100.0, 100.0));
        ui.vertical_centered(|ui| {
            let mut list: Vec<_> = tracker.get_conditions(&character.name).into_iter().collect();

            list.sort();

            list.into_iter().filter(|&cond| {
                let (_, remove) = egui::Sides::new().show(ui,
                    |ui| ui.label(cond.to_string()),
                    |ui| ui.button("x").clicked()
                );
                remove
            }).map(|removed| Response::RemoveCondition{cond: removed.clone(), character: character.name.to_string()})
            .collect()
        }).inner
    }).inner
}

