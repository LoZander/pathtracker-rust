use std::fmt::Display;

use egui::{vec2, Context, Id, Modal, ProgressBar, RichText, Ui};

use crate::{character::{Chr, Health}, conditions::{Condition, DamageType, NonValuedCondition, ValuedCondition}, saver::Saver, tracker::Tracker};

pub fn run<S: Saver>(t: Tracker<S>) -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Pathtracker",
        native_options,
        Box::new(|_| Ok(Box::new(WindowApp::new(t))))
    )
}

struct WindowApp<S: Saver> {
    tracker: Tracker<S>,
    add_window: AddWindow,
    add_cond_window: AddCondWindow,
    add_cond_window_open: bool
}

#[derive(Default)]
struct AddWindow {
    show: bool,
    focus: bool,
    name: String,
    init: i32,
    player: bool,
    enable_health: bool,
    health: u32
}


impl AddWindow {
    fn reset(&mut self) {
        self.name = String::new();
        self.init = 0;
        self.player = false;
        self.enable_health = false;
    }

    fn open(&mut self) {
        self.show = true;
        self.focus = true;
    }

    fn close(&mut self) {
        self.show = false;
        self.focus = false;
        self.reset();
    }
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
struct AddCondWindow {
    character: Option<Chr>,
    selected: ConditionEntry,
    cond_value: u8
}

impl AddCondWindow {
    fn prepare(&mut self, character: Chr) {
        self.reset();
        self.character = Some(character);
    }
    fn reset(&mut self) {
        self.character = None;
        self.selected = ConditionEntry::default();
        self.cond_value = 0;
    }
    
    fn init<S: Saver>(&mut self, tracker: &mut Tracker<S>, ctx: &Context, open: &mut bool) {
        //let frame = egui::Frame::default().inner_margin(egui::Margin::symmetric(20.0, 20.0));
        let c = self.character.as_ref().map_or("missingno", |c| &c.name);
        egui::Window::new(format!("{c} Conditions"))
            .open(open)
            .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Add Condition:");
                    egui::ComboBox::from_label("Condition")
                        .selected_text(format!("{}", self.selected))
                        .show_ui(ui, |ui| self.init_selectable_conds(ui));

                    if let ConditionEntry::Valued(cond) = self.selected {
                        ui.horizontal(|ui| {
                            ui.label(cond.to_string());
                            let drag = egui::DragValue::new(&mut self.cond_value).range(0..=9);
                            ui.add(drag); 
                        });
                    }

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
                        tracker.add_condition(self.character.as_ref().unwrap().name.as_str(), condition).unwrap();
                    }
                });
                
                ui.group(|ui| {
                    ui.set_min_size(vec2(100.0, 100.0));
                    ui.vertical_centered(|ui| {
                        ui.label("test 1");
                        ui.label("test 2");
                    });
                });
            });
        });
    }

    fn init_selectable_conds(&mut self, ui: &mut Ui) {
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


impl<S: Saver> WindowApp<S> {
    pub fn new(tracker: Tracker<S>) -> Self {
        Self {
            tracker,
            add_window: AddWindow::default(),
            add_cond_window: AddCondWindow::default(),
            add_cond_window_open: false
        }
    }
}


fn error_window(ctx: &Context, title: impl Into<RichText>, err: String) {
    egui::Window::new("Error")
        .fixed_size(vec2(200.0, 100.0))
        .show(ctx, |ui| {
            ui.heading(title);
            ui.label(err)
        });
}

impl<S: Saver> WindowApp<S> {
    fn init_main(&mut self, ctx: &Context) {
        let frame = egui::Frame::default().inner_margin(egui::Margin::symmetric(50.0, 20.0));
        egui::TopBottomPanel::bottom("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Next").clicked() {
                    if let Err(err) = self.tracker.end_turn() {
                        error_window(ctx, "Save error:", err.to_string());
                    };
                };
                if ui.button("add").clicked() {
                    self.add_window.open();
                };
            });
        });

        egui::CentralPanel::default()
            .frame(frame)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.tracker
                        .get_chrs()
                        .to_owned()
                        .iter()
                        .for_each(|c| self.init_character(ctx, ui, c));
                });
            });
    }

    fn init_add(&mut self, ctx: &Context) {
        let add_window = &mut self.add_window;
        if add_window.show {
            Modal::new(Id::new("add_character"))
                .show(ctx, |ui| {
                    ui.heading("Add character");
                    let name_edit = ui.horizontal(|ui| {
                        ui.label("Name: ");
                        ui.text_edit_singleline(&mut add_window.name)
                    }).inner;

                    ui.horizontal(|ui| {
                        ui.label("Initiative: ");
                        let drag = egui::DragValue::new(&mut add_window.init).range(0..=50);
                        ui.add(drag);
                    });

                    ui.checkbox(&mut add_window.player, "Player");

                    ui.horizontal(|ui| {
                        ui.checkbox(&mut add_window.enable_health, "Track HP");

                        if add_window.enable_health {
                            ui.add_space(12.0);
                            ui.label("Max HP:");
                            let drag = egui::DragValue::new(&mut add_window.health).range(0..=999);
                            ui.add(drag);
                        }   
                    });

                    ui.separator();

                    egui::Sides::new().show(ui, 
                        |_| {},
                        |ui| {
                        if ui.button("confirm").clicked() {
                            let c1 = Chr::builder(add_window.name.clone(), add_window.init, add_window.player);
                            let c2 = if add_window.enable_health { c1.with_health(Health::new(add_window.health)) } else { c1 };
                            let character = c2.build();
                            if let Err(err) = self.tracker.add_chr(character) {
                                error_window(ctx, "Save error", err.to_string());
                            };
                            add_window.close();
                        }
                        if ui.button("cancel").clicked() {
                            add_window.close();
                        }
                    });

                    if add_window.focus {
                        name_edit.request_focus();
                        add_window.focus = false;
                    }
                    
                });
        }
    }

    fn init_character(&mut self, ctx: &Context, ui: &mut Ui, character: &Chr) {
        ui.style_mut().spacing.indent = 20.0;
        let space = if self.tracker.get_in_turn() == Some(character) {
            0.0
        } else {
            20.0
        };
        let mut conditions: Vec<_> = self.tracker.get_conditions(&character.name).into_iter().map(ToOwned::to_owned).collect();
        conditions.sort();
        let (res, _) = egui::containers::Sides::new().show(ui,
            |ui| {
                ui.add_space(space);
                ui.heading(character.init.to_string());
                ui.label(character.name.clone());

                if let Some(hp) = &character.health {
                    ui.add_space(10.0);
                    ui.add(health_bar(hp));
                }

                let condition_str = conditions.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ");

                if conditions.len() <= 2 {
                    ui.label(condition_str);
                } else {
                    let cond_label = ui.label(format!("{condition_str} (+)"));
                }

            },
            |ui| {
                ui.menu_button("...", |ui| {
                    if ui.button("Conditions").clicked() {
                        self.add_cond_window.prepare(character.clone());
                        self.add_cond_window_open = true;
                    }
                });
                if ui.small_button("x").clicked() {
                    if let Err(err) = self.tracker.rm_chr(&character.name) {
                        error_window(ctx, "Save error", err.to_string());
                    };
                }

            }
        );
    }
}

fn health_bar(hp: &Health) -> ProgressBar {
    let rel_hp: f32 = (hp.current as f32) / (hp.max as f32);
    egui::ProgressBar::new(rel_hp)
        .text(format!("{}/{}", hp.current, hp.max))
        .rounding(2.0)
        .desired_width(100.0)
}


impl<S: Saver> eframe::App for WindowApp<S> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.init_main(ctx);
        self.init_add(ctx);
        self.add_cond_window.init(&mut self.tracker, ctx, &mut self.add_cond_window_open);
    }
}
