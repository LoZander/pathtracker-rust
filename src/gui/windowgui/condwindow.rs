use std::fmt::Display;

use egui::{vec2, Context, Ui, WidgetText};

use crate::{
    character::Chr,
    conditions::{
        Condition, DamageType, NonValuedCondition, NonValuedTerm, TurnEvent, ValuedCondition,
        ValuedTerm,
    },
    duration::Duration,
    saver::Saver,
    tracker::Tracker,
};

#[derive(Debug, Clone)]
pub enum Response {
    AddCondition{character: String, cond: Condition},
    RemoveCondition{character: String, cond: Condition}
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Default)]
pub struct CondWindow {
    open: bool,
    data: Data
}

#[derive(Debug, Clone, Default)]
struct Data {
    character: Option<Chr>,
    selected: ConditionEntry,
    cond_value: u8,
    auto_tracking: bool,
    selected_nonvalued_term: NonValuedTermEntry,
    selected_valued_term: ValuedTermEntry,
    term_rounds: u32,
    selected_turn_event: TurnEventEntry,
    selected_turn_event_character: String,
    reduction: u8
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum TurnEventEntry {
    StartOfNextTurn,
    #[default]
    EndOfNextTurn,
}

impl From<TurnEvent> for TurnEventEntry {
    fn from(value: TurnEvent) -> Self {
        match value {
            TurnEvent::StartOfNextTurn(_) => TurnEventEntry::StartOfNextTurn,
            TurnEvent::EndOfNextTurn(_) => TurnEventEntry::EndOfNextTurn,
        }
    }
}

impl Display for TurnEventEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TurnEventEntry::StartOfNextTurn => write!(f, "start of next turn"),
            TurnEventEntry::EndOfNextTurn => write!(f, "end of next turn"),
        }
    }
}

impl CondWindow {
    pub fn open(&mut self, character: Chr) {
        self.reset();
        self.data.selected_turn_event_character = character.name.clone();
        self.data.character = Some(character);
        self.open = true;
    }

    fn reset(&mut self) {
        self.data.character = None;
        self.data.selected = ConditionEntry::default();
        self.data.cond_value = 0;
        self.data.auto_tracking = false;
        self.data.selected_nonvalued_term = Default::default();
        self.data.selected_valued_term = Default::default();
        self.data.term_rounds = 0;
        self.data.selected_turn_event_character = String::new();
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    pub fn show<S: Saver>(&mut self, tracker: &mut Tracker<S>, ctx: &Context) -> super::Result<()> {
        //let frame = egui::Frame::default().inner_margin(egui::Margin::symmetric(20.0, 20.0));
        let open = &mut self.open;
        let data = &mut self.data;
        if let Some(character) = data.character.clone() {
            let res = egui::Window::new(format!("{} Conditions", character.name))
                .open(open)
                .show(ctx, |ui| {
                    // TODO: consider changing returning tuple of (Option<AddResponse>,
                    // Vec<RemoveResponse>) instead of a vec of both
                    let responses = ui
                        .horizontal(|ui| {
                            let add = show_cond_section(ui, data, &character, tracker.get_chrs().to_vec());

                            let mut remove = show_cond_list_section(tracker, ui, &character);

                            if let Some(inner) = add {
                                remove.push(inner);
                            }

                            remove
                        })
                        .inner;

                    for resp in responses {
                        match resp {
                            Response::AddCondition { character, cond } => {
                                tracker.add_condition(&character, cond)?;
                            }
                            Response::RemoveCondition { character, cond } => {
                                tracker.rm_condition(&character, &cond)
                            }
                        }
                    }

                    Ok::<(), super::Error>(())
                });
            match res {
                Some(inner) => inner.inner.unwrap_or(Ok::<(), super::Error>(())),
                None => Ok(()),
            }
        } else {
            Ok(())
        }
    }
}

fn show_cond_section(ui: &mut Ui, data: &mut Data, character: &Chr, characters: Vec<Chr>) -> Option<Response> {
    ui.vertical(|ui| {
        ui.set_max_width(200.);
        ui.label("Add Condition:");

        show_cond_selector(ui, data, character, characters);

        ui.separator();

        show_add_button(ui, data, character)
    })
    .inner
}

fn show_add_button(ui: &mut Ui, data: &Data, character: &Chr) -> Option<Response> {
    if ui.button("Add").clicked() {
        let condition = match data.selected {
            ConditionEntry::Valued(valued_condition) => {
                let builder = Condition::builder()
                    .condition(valued_condition)
                    .value(data.cond_value);

                if data.auto_tracking {
                    match data.selected_valued_term {
                        ValuedTermEntry::For => builder.term(ValuedTerm::For(Duration::from_turns(data.term_rounds))).build(),
                        ValuedTermEntry::Until => builder.term(ValuedTerm::Until(create_turn_event(data))).build(),
                        ValuedTermEntry::Reduced => builder.term(ValuedTerm::Reduced(create_turn_event(data), data.reduction)).build()
                    }
                } else {
                    builder.build()
                }
            }
            ConditionEntry::NonValued(non_valued_condition) => {
                let builder = Condition::builder().condition(non_valued_condition);

                if data.auto_tracking {
                    match data.selected_nonvalued_term {
                        NonValuedTermEntry::For => builder.term(NonValuedTerm::For(Duration::from_turns(data.term_rounds))).build(),
                        NonValuedTermEntry::Until => builder.term(NonValuedTerm::Until(create_turn_event(data))).build()
                    }
                } else {
                    builder.build()
                }
            }
        };

        Some(Response::AddCondition {
            character: character.name.to_string(),
            cond: condition,
        })
    } else {
        None
    }
}

fn create_turn_event(data: &Data) -> TurnEvent {
    let turn_event = match data.selected_turn_event {
        TurnEventEntry::StartOfNextTurn => TurnEvent::StartOfNextTurn(data.selected_turn_event_character.clone()),
        TurnEventEntry::EndOfNextTurn => TurnEvent::EndOfNextTurn(data.selected_turn_event_character.clone()),
    };
    turn_event
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum NonValuedTermEntry {
    #[default]
    For,
    Until,
}

impl Display for NonValuedTermEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NonValuedTermEntry::For => write!(f, "For"),
            NonValuedTermEntry::Until => write!(f, "Until"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ValuedTermEntry {
    #[default]
    For,
    Until,
    Reduced,
}

impl Display for ValuedTermEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValuedTermEntry::For => write!(f, "For"),
            ValuedTermEntry::Until => write!(f, "Until"),
            ValuedTermEntry::Reduced => write!(f, "Reduced"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TermEntry {
    NonValuedTermEntry(NonValuedTermEntry),
    ValuedTermEntry(ValuedTermEntry),
}

impl Display for TermEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TermEntry::NonValuedTermEntry(non_valued_term_entry) => {
                write!(f, "{non_valued_term_entry}")
            }
            TermEntry::ValuedTermEntry(valued_term_entry) => write!(f, "{valued_term_entry}"),
        }
    }
}

impl Default for TermEntry {
    fn default() -> Self {
        Self::NonValuedTermEntry(Default::default())
    }
}

fn show_cond_selector(ui: &mut Ui, data: &mut Data, character: &Chr, characters: Vec<Chr>) {
    ui.set_max_width(200.);
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

    ui.checkbox(&mut data.auto_tracking, "auto tracking");

    if data.auto_tracking {
        show_auto_tracking_options(ui, data, character, characters);
    }
}

fn show_auto_tracking_options(ui: &mut Ui, data: &mut Data, character: &Chr, characters: Vec<Chr>) {
    ui.separator();
    ui.horizontal(|ui| {
        let selected = data.selected_term_string();

        egui::ComboBox::from_id_salt("termination")
            .width(30.)
            .selected_text(selected)
            .show_ui(ui, |ui| selectable_terms(ui, data));

        match data.selected {
            ConditionEntry::Valued(_) => match data.selected_valued_term {
                ValuedTermEntry::For => show_for_options(ui, data),
                ValuedTermEntry::Until => show_until_options(ui, data, characters),
                ValuedTermEntry::Reduced => show_reduced_options(ui, data, characters),
            },
            ConditionEntry::NonValued(_) => match data.selected_nonvalued_term {
                NonValuedTermEntry::For => show_for_options(ui, data),
                NonValuedTermEntry::Until => show_until_options(ui, data, characters)
            },
        };
    });
}

fn show_reduced_options(ui: &mut Ui, data: &mut Data, characters: Vec<Chr>) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label("by");
            let drag = egui::DragValue::new(&mut data.reduction).range(0..=999);
            ui.add(drag);
        });
        ui.horizontal(|ui| {
            ui.label("at");
            egui::ComboBox::from_id_salt("turn event")
                .width(30.)
                .selected_text(data.selected_turn_event.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut data.selected_turn_event,
                        TurnEventEntry::EndOfNextTurn,
                        TurnEventEntry::EndOfNextTurn.to_string(),
                    );

                    ui.selectable_value(
                        &mut data.selected_turn_event,
                        TurnEventEntry::StartOfNextTurn,
                        TurnEventEntry::StartOfNextTurn.to_string(),
                    );
                });
        });

        egui::ComboBox::from_id_salt("turn event character")
            .width(30.)
            .selected_text(data.selected_turn_event_character.clone())
            .show_ui(ui, |ui| {
                characters.into_iter().for_each(|c| {
                    ui.selectable_value(
                        &mut data.selected_turn_event_character,
                        c.name.clone(),
                        c.name,
                    );
                })
            });
    });
}

fn show_for_options(ui: &mut Ui, data: &mut Data) {
    let drag = egui::DragValue::new(&mut data.term_rounds).suffix(" rounds");
    ui.add(drag);
}

fn show_until_options(ui: &mut Ui, data: &mut Data, characters: Vec<Chr>) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt("turn event")
                .width(30.)
                .selected_text(data.selected_turn_event.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut data.selected_turn_event,
                        TurnEventEntry::EndOfNextTurn,
                        TurnEventEntry::EndOfNextTurn.to_string(),
                    );

                    ui.selectable_value(
                        &mut data.selected_turn_event,
                        TurnEventEntry::StartOfNextTurn,
                        TurnEventEntry::StartOfNextTurn.to_string(),
                    );
                });
            ui.label("of");
        });

        egui::ComboBox::from_id_salt("turn event character")
            .width(30.)
            .selected_text(data.selected_turn_event_character.clone())
            .show_ui(ui, |ui| {
                characters.into_iter().for_each(|c| {
                    ui.selectable_value(
                        &mut data.selected_turn_event_character,
                        c.name.clone(),
                        c.name,
                    );
                })
            });
    });
}

impl Data {
    fn selected_term_string(&self) -> String {
        let selected = match self.selected {
            ConditionEntry::Valued(_) => self.selected_valued_term.to_string(),
            ConditionEntry::NonValued(_) => self.selected_nonvalued_term.to_string(),
        };

        selected
    }
}

fn selectable_terms(ui: &mut Ui, data: &mut Data) {
    match data.selected {
        ConditionEntry::Valued(_) => selectable_valued_terms(ui, data),
        ConditionEntry::NonValued(_) => selectable_nonvalued_terms(ui, data),
    }
}

fn selectable_nonvalued_terms(ui: &mut Ui, data: &mut Data) {
    selectable_nonvalued_term(ui, data, NonValuedTermEntry::For);
    selectable_nonvalued_term(ui, data, NonValuedTermEntry::Until);
}

fn selectable_valued_terms(ui: &mut Ui, data: &mut Data) {
    selectable_valued_term(ui, data, ValuedTermEntry::For);
    selectable_valued_term(ui, data, ValuedTermEntry::Until);
    selectable_valued_term(ui, data, ValuedTermEntry::Reduced);
}

fn selectable_nonvalued_term(ui: &mut Ui, data: &mut Data, term_entry: NonValuedTermEntry) {
    ui.selectable_value(
        &mut data.selected_nonvalued_term,
        term_entry,
        term_entry.to_string(),
    );
}

fn selectable_valued_term(ui: &mut Ui, data: &mut Data, term_entry: ValuedTermEntry) {
    ui.selectable_value(
        &mut data.selected_valued_term,
        term_entry,
        term_entry.to_string(),
    );
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
    ui.selectable_value(
        &mut data.selected,
        ConditionEntry::NonValued(cond),
        cond.to_string(),
    );
}

fn selectable_valued_cond(ui: &mut Ui, data: &mut Data, cond: ValuedCondition) {
    ui.selectable_value(
        &mut data.selected,
        ConditionEntry::Valued(cond),
        cond.to_string(),
    );
}

fn show_cond_list_section(
    tracker: &Tracker<impl Saver>,
    ui: &mut Ui,
    character: &Chr,
) -> Vec<Response> {
    ui.group(|ui| {
        ui.set_min_size(vec2(100.0, 100.0));
        ui.vertical_centered(|ui| {
            let mut list: Vec<_> = tracker
                .get_conditions(&character.name)
                .into_iter()
                .collect();

            list.sort();

            list.into_iter()
                .filter(|&cond| {
                    let (_, remove) = egui::Sides::new().show(
                        ui,
                        |ui| ui.label(cond.to_long_string()),
                        |ui| ui.button("x").clicked(),
                    );
                    remove
                })
                .map(|removed| Response::RemoveCondition {
                    cond: removed.clone(),
                    character: character.name.to_string(),
                })
                .collect()
        })
        .inner
    })
    .inner
}
