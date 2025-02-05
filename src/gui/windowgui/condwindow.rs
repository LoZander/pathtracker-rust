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
    character: Option<Chr>,
    selected: ConditionEntry,
    cond_value: u8
}

impl CondWindow {
    pub fn prepare(&mut self, character: Chr) {
        self.reset();
        self.character = Some(character);
    }
    pub fn reset(&mut self) {
        self.character = None;
        self.selected = ConditionEntry::default();
        self.cond_value = 0;
    }
    
    pub fn init<S: Saver>(&mut self, tracker: &mut Tracker<S>, ctx: &Context, open: &mut bool) {
        //let frame = egui::Frame::default().inner_margin(egui::Margin::symmetric(20.0, 20.0));
        if let Some(character) = self.character.clone() {
            egui::Window::new(format!("{} Conditions", character.name))
                .open(open)
                .show(ctx, |ui| {
                    let responses = ui.horizontal(|ui| {
                        let add = self.add_cond_section(ui, &character);
                
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

    fn add_cond_section(&mut self, ui: &mut Ui, character: &Chr) -> Option<Response> {
        ui.vertical(|ui| {
            ui.label("Add Condition:");

            self.cond_selector(ui);

            self.add_button(character, ui)
        }).inner
    }

    fn add_button(&self, character: &Chr, ui: &mut Ui) -> Option<Response> {
        if ui.button("Add").clicked() {
            let condition = match self.selected {
                ConditionEntry::Valued(valued_condition) => {
                    Condition::builder().condition(valued_condition)
                        .value(self.cond_value)
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

    fn cond_selector(&mut self, ui: &mut Ui) {
        egui::ComboBox::from_label("Condition")
            .selected_text(format!("{}", self.selected))
            .show_ui(ui, |ui| self.selectable_conds(ui));

        if let ConditionEntry::Valued(cond) = self.selected {
            ui.horizontal(|ui| {
                ui.label(cond.to_string());
                let drag = egui::DragValue::new(&mut self.cond_value).range(0..=9);
                ui.add(drag); 
            });
        }
    }

    fn selectable_conds(&mut self, ui: &mut Ui) {
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Bleed));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Poison));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Piercing));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Bludgeoning));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Slashing));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Acid));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Cold));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Electricity));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Sonic));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Positive));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Negative));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Force));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Chaotic));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Evil));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Good));
        self.selectable_valued_cond(ui, ValuedCondition::PersistentDamage(DamageType::Lawful));
        self.selectable_valued_cond(ui, ValuedCondition::Clumsy);
        self.selectable_valued_cond(ui, ValuedCondition::Doomed);
        self.selectable_valued_cond(ui, ValuedCondition::Drained);
        self.selectable_valued_cond(ui, ValuedCondition::Dying);
        self.selectable_valued_cond(ui, ValuedCondition::Enfeebled);
        self.selectable_valued_cond(ui, ValuedCondition::Frightened);
        self.selectable_valued_cond(ui, ValuedCondition::Sickened);
        self.selectable_valued_cond(ui, ValuedCondition::Slowed);
        self.selectable_valued_cond(ui, ValuedCondition::Stunned);
        self.selectable_valued_cond(ui, ValuedCondition::Stupified);
        self.selectable_valued_cond(ui, ValuedCondition::Wounded);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Blinded);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Broken);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Concealed);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Confused);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Controlled);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Dazzled);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Deafened);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Encumbered);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Fascinated);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Fatigued);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::FlatFooted);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Fleeing);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Friendly);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Grabbed);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Helpful);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Hidden);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Hostile);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Immobilized);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Indifferent);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Invisible);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Observed);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Paralyzed);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Petrified);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Prone);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Quickened);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Restrained);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Unconscious);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Undetected);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Unfriendly);
        self.selectable_nonvalued_cond(ui, NonValuedCondition::Unnoticed);
    }

    fn selectable_nonvalued_cond(&mut self, ui: &mut Ui, cond: NonValuedCondition) {
        ui.selectable_value(&mut self.selected, ConditionEntry::NonValued(cond), cond.to_string());
    }

    fn selectable_valued_cond(&mut self, ui: &mut Ui, cond: ValuedCondition) {
        ui.selectable_value(&mut self.selected, ConditionEntry::Valued(cond), cond.to_string());
    }
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

