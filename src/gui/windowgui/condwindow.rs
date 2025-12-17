use std::fmt::Display;

use egui::{vec2, Context, Ui};

use crate::{
    character::{Chr, ChrName}, conditions::{
        CondFormat, Condition, DamageType, NonValuedCondition, NonValuedTerm, TurnEvent, ValuedCondition, ValuedTerm
    }, duration::Duration, saver::Saver, settings, tracker::Tracker
};

#[derive(Debug, Clone)]
pub enum Response {
    AddCondition{character: ChrName, cond: Condition},
    RemoveCondition{character: ChrName, cond: Condition}
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ConditionEntry {
    Valued(ValuedCondition),
    NonValued(NonValuedCondition),
}

impl ConditionEntry {
    #[must_use]
    pub fn to_string(&self, format: CondFormat) -> String {
        match self {
            Self::Valued(valued_condition) => valued_condition.to_string(format),
            Self::NonValued(non_valued_condition) => non_valued_condition.to_string(format),
        }
    }
}

impl Default for ConditionEntry {
    fn default() -> Self {
        Self::NonValued(NonValuedCondition::FlatFooted)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CondWindow {
    open: bool,
    data: Data
}

#[derive(Debug, Clone, Default)]
struct Data {
    character: Option<ChrName>,
    selected: ConditionEntry,
    cond_value: u8,
    auto_tracking: bool,
    selected_nonvalued_term: NonValuedTermEntry,
    selected_valued_term: ValuedTermEntry,
    term_rounds: u32,
    selected_turn_event: TurnEventEntry,
    selected_turn_event_character: ChrName,
    reduction: u8
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[allow(clippy::enum_variant_names)]
enum TurnEventEntry {
    StartOfNextTurn,
    #[default]
    EndOfNextTurn,
    EndOfCurrentTurn,
}

impl From<TurnEvent> for TurnEventEntry {
    fn from(value: TurnEvent) -> Self {
        match value {
            TurnEvent::StartOfNextTurn(_) => Self::StartOfNextTurn,
            TurnEvent::EndOfNextTurn(_) => Self::EndOfNextTurn,
            TurnEvent::EndOfCurrentTurn(_) => Self::EndOfCurrentTurn,
        }
    }
}

impl Display for TurnEventEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StartOfNextTurn => write!(f, "start of next turn"),
            Self::EndOfNextTurn => write!(f, "end of next turn"),
            Self::EndOfCurrentTurn => write!(f, "end of current turn"),
        }
    }
}

impl CondWindow {
    pub fn open(&mut self, character: ChrName) {
        self.reset();
        self.data.selected_turn_event_character = character.clone();
        self.data.character = Some(character);
        self.open = true;
    }

    fn reset(&mut self) {
        self.data.character = None;
        self.data.selected = ConditionEntry::default();
        self.data.cond_value = 0;
        self.data.auto_tracking = false;
        self.data.selected_nonvalued_term = NonValuedTermEntry::default();
        self.data.selected_valued_term = ValuedTermEntry::default();
        self.data.term_rounds = 0;
        self.data.selected_turn_event_character = ChrName::default();
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    pub fn show<S: Saver>(&mut self, tracker: &mut Tracker<S>, ctx: &Context) -> super::Result<()> {
        //let frame = egui::Frame::default().inner_margin(egui::Margin::symmetric(20.0, 20.0));
        let open = &mut self.open;
        let data = &mut self.data;
        let format = CondFormat::default().set_version(tracker.get_pf2e_version_setting());
        if let Some(character) = data.character.clone() {
            let res = egui::Window::new(format!("{} Conditions", character))
                .open(open)
                .show(ctx, |ui| {
                    // TODO: consider changing returning tuple of (Option<AddResponse>,
                    // Vec<RemoveResponse>) instead of a vec of both
                    let responses = ui
                        .horizontal(|ui| {
                            let add = show_cond_section(ui, data, &character, tracker.get_chrs().to_vec(), format);

                            let mut remove = show_cond_list_section(tracker, ui, character, format);

                            if let Some(inner) = add {
                                remove.push(inner);
                            }

                            remove
                        })
                        .inner;

                    for resp in responses {
                        match resp {
                            Response::AddCondition { character, cond } => {
                                tracker.add_condition(character, cond)?;
                            }
                            Response::RemoveCondition { character, cond } => {
                                tracker.rm_condition(&character, &cond);
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

fn show_cond_section(ui: &mut Ui, data: &mut Data, character: &ChrName, characters: Vec<Chr>, format: CondFormat) -> Option<Response> {
    ui.vertical(|ui| {
        ui.set_max_width(200.);
        ui.label("Add Condition:");

        show_cond_selector(ui, data, character, characters, format);

        ui.separator();

        show_add_button(ui, data, character)
    })
    .inner
}

fn show_add_button(ui: &mut Ui, data: &Data, character: &ChrName) -> Option<Response> {
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
            character: character.clone(),
            cond: condition,
        })
    } else {
        None
    }
}

fn create_turn_event(data: &Data) -> TurnEvent {
    match data.selected_turn_event {
        TurnEventEntry::StartOfNextTurn => TurnEvent::StartOfNextTurn(data.selected_turn_event_character.clone()),
        TurnEventEntry::EndOfNextTurn => TurnEvent::EndOfNextTurn(data.selected_turn_event_character.clone()),
        TurnEventEntry::EndOfCurrentTurn => TurnEvent::EndOfCurrentTurn(data.selected_turn_event_character.clone())
    }
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
            Self::For => write!(f, "For"),
            Self::Until => write!(f, "Until"),
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
            Self::For => write!(f, "For"),
            Self::Until => write!(f, "Until"),
            Self::Reduced => write!(f, "Reduced"),
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
            Self::NonValuedTermEntry(non_valued_term_entry) => {
                write!(f, "{non_valued_term_entry}")
            }
            Self::ValuedTermEntry(valued_term_entry) => write!(f, "{valued_term_entry}"),
        }
    }
}

impl Default for TermEntry {
    fn default() -> Self {
        Self::NonValuedTermEntry(NonValuedTermEntry::default())
    }
}

fn show_cond_selector(ui: &mut Ui, data: &mut Data, character: &ChrName, characters: Vec<Chr>, format: CondFormat) {
    ui.set_max_width(200.);
    egui::ComboBox::from_label("Condition")
        .selected_text(format!("{}", data.selected.to_string(format)))
        .show_ui(ui, |ui| selectable_conds(ui, data, format));

    if let ConditionEntry::Valued(cond) = data.selected {
        ui.horizontal(|ui| {
            ui.label(cond.to_string(format));
            let drag = egui::DragValue::new(&mut data.cond_value).range(0..=9);
            ui.add(drag);
        });
    }

    ui.checkbox(&mut data.auto_tracking, "auto tracking");

    if data.auto_tracking {
        show_auto_tracking_options(ui, data, character, characters);
    }
}

fn show_auto_tracking_options(ui: &mut Ui, data: &mut Data, character: &ChrName, characters: Vec<Chr>) {
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
        }
    });
}

fn show_turn_event_options(ui: &mut Ui, data: &mut Data) {
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
                TurnEventEntry::EndOfCurrentTurn,
                TurnEventEntry::EndOfCurrentTurn.to_string(),
            );

            ui.selectable_value(
                &mut data.selected_turn_event,
                TurnEventEntry::StartOfNextTurn,
                TurnEventEntry::StartOfNextTurn.to_string(),
            );

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
            show_turn_event_options(ui, data);
        });

        egui::ComboBox::from_id_salt("turn event character")
            .width(30.)
            .selected_text(String::from(data.selected_turn_event_character.clone()))
            .show_ui(ui, |ui| {
                for c in characters {
                    ui.selectable_value(
                        &mut data.selected_turn_event_character,
                        c.name.clone(),
                        String::from(c.name),
                    );
                }
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
            show_turn_event_options(ui, data);
            ui.label("of");
        });

        egui::ComboBox::from_id_salt("turn event character")
            .width(30.)
            .selected_text(data.selected_turn_event_character.clone())
            .show_ui(ui, |ui| {
                for c in characters {
                    ui.selectable_value(
                        &mut data.selected_turn_event_character,
                        c.name.clone(),
                        c.name,
                    );
                }
            });
    });
}

impl Data {
    fn selected_term_string(&self) -> String {
        match self.selected {
            ConditionEntry::Valued(_) => self.selected_valued_term.to_string(),
            ConditionEntry::NonValued(_) => self.selected_nonvalued_term.to_string(),
        }
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

fn selectable_conds(ui: &mut Ui, data: &mut Data, format: CondFormat) {
    selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Bleed));
    selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Poison));
    selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Piercing));
    selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Bludgeoning));
    selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Slashing));
    selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Acid));
    selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Cold));
    selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Electricity));
    selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Sonic));
    selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Positive));
    selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Negative));
    selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Force));
    if let settings::Pf2eVersion::Old = format.get_version() {
        selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Chaotic));
        selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Evil));
        selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Good));
        selectable_valued_cond(ui, data, format, ValuedCondition::PersistentDamage(DamageType::Lawful));
    }
    selectable_valued_cond(ui, data, format, ValuedCondition::Clumsy);
    selectable_valued_cond(ui, data, format, ValuedCondition::Doomed);
    selectable_valued_cond(ui, data, format, ValuedCondition::Drained);
    selectable_valued_cond(ui, data, format, ValuedCondition::Dying);
    selectable_valued_cond(ui, data, format, ValuedCondition::Enfeebled);
    selectable_valued_cond(ui, data, format, ValuedCondition::Frightened);
    selectable_valued_cond(ui, data, format, ValuedCondition::Sickened);
    selectable_valued_cond(ui, data, format, ValuedCondition::Slowed);
    selectable_valued_cond(ui, data, format, ValuedCondition::Stunned);
    selectable_valued_cond(ui, data, format, ValuedCondition::Stupified);
    selectable_valued_cond(ui, data, format, ValuedCondition::Wounded);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Blinded);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Broken);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Concealed);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Confused);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Controlled);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Dazzled);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Deafened);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Encumbered);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Fascinated);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Fatigued);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::FlatFooted);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Fleeing);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Friendly);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Grabbed);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Helpful);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Hidden);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Hostile);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Immobilized);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Indifferent);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Invisible);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Observed);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Paralyzed);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Petrified);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Prone);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Quickened);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Restrained);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Unconscious);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Undetected);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Unfriendly);
    selectable_nonvalued_cond(ui, data, format, NonValuedCondition::Unnoticed);
}

fn selectable_nonvalued_cond(ui: &mut Ui, data: &mut Data, format: CondFormat, cond: NonValuedCondition) {
    ui.selectable_value(
        &mut data.selected,
        ConditionEntry::NonValued(cond),
        cond.to_string(format),
    );
}

fn selectable_valued_cond(ui: &mut Ui, data: &mut Data, format: CondFormat, cond: ValuedCondition) {
    ui.selectable_value(
        &mut data.selected,
        ConditionEntry::Valued(cond),
        cond.to_string(format),
    );
}

fn show_cond_list_section(
    tracker: &Tracker<impl Saver>,
    ui: &mut Ui,
    character: ChrName,
    format: CondFormat,
) -> Vec<Response> {
    ui.group(|ui| {
        ui.set_min_size(vec2(100.0, 100.0));
        ui.vertical_centered(|ui| {
            let mut list: Vec<_> = tracker
                .get_conditions(&character)
                .into_iter()
                .collect();

            list.sort();

            list.into_iter()
                .filter(|&cond| {
                    let (_, remove) = egui::Sides::new().show(
                        ui,
                        |ui| ui.label(cond.to_string(format)),
                        |ui| ui.button("x").clicked(),
                    );
                    remove
                })
                .map(|removed| Response::RemoveCondition {
                    cond: removed.clone(),
                    character: character.clone(),
                })
                .collect()
        })
        .inner
    })
    .inner
}
