use addwindow::AddWindow;
use condwindow::CondWindow;
use dragvaluewindow::DragValueWindow;
use egui::{Context, IntoAtoms, Ui};
use errorwindow::ErrorWindow;
use healthwindow::HealthWindow;
use renamewindow::RenameWindow;

use crate::{character::ChrName, saver::Saver, tracker::{self, Tracker}};

mod condwindow;
mod errorwindow;
mod addwindow;
mod characters;
mod renamewindow;
mod healthwindow;
mod dragvaluewindow;

#[derive(Debug)]
#[derive(thiserror::Error)]
enum Error {
    #[error(transparent)]
    TrackerError(#[from] tracker::Error)
}

type Result<T> = std::result::Result<T, Error>;

pub fn run<S: Saver>(t: Tracker<S>) -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 300.0])
            .with_min_inner_size([350.0, 200.0]),
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
    add_cond_window: CondWindow,
    rename_window: RenameWindow,
    health_window: HealthWindow,
    error_window: ErrorWindow,
    damage_window: DragValueWindow<u32, ChrName>,
    heal_window: DragValueWindow<u32, ChrName>,
    add_temp_hp_window: DragValueWindow<u32, ChrName>,
}

impl<S: Saver> eframe::App for WindowApp<S> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.error_window.show(ctx);
        let res = self.show_main_window(ctx)
            .and_then(|()| self.add_window.show(&mut self.tracker, ctx))
            .and_then(|()| self.add_cond_window.show(&mut self.tracker, ctx))
            .and_then(|()| self.rename_window.show(&mut self.tracker, ctx))
            .and_then(|()| self.health_window.show(&mut self.tracker, ctx))
            .and_then(|()| self.show_damage_window(ctx))
            .and_then(|()| self.show_heal_window(ctx))
            .and_then(|()| self.show_add_temp_hp_window(ctx));

        if let Err(err) = res {
            self.error_window.open(err);
        }
    }
}

impl<S: Saver> WindowApp<S> {
    pub fn new(tracker: Tracker<S>) -> Self {
        Self {
            tracker,
            add_window: AddWindow::default(),
            add_cond_window: CondWindow::default(),
            rename_window: RenameWindow::default(),
            health_window: HealthWindow::default(),
            error_window: ErrorWindow::default(),
            damage_window: DragValueWindow::default(),
            heal_window: DragValueWindow::default(),
            add_temp_hp_window: DragValueWindow::default(),
        }
    }
    
    fn show_main_window(&mut self, ctx: &Context) -> Result<()> {
        self.show_button_panel(ctx)?;
        self.show_character_panel(ctx)
    }

    fn show_damage_window(&mut self, ctx: &Context) -> Result<()> {
        self.damage_window.show("damage_window".into(), ctx, 
            |c,_| format!("Damage {}", c), 
            |_,_| "Amount: ".into(),
            |c,d|{
                self.tracker.damage(&c, d)?;
                Ok(())
            }
        )
    }

    fn show_heal_window(&mut self, ctx: &Context) -> Result<()> {
        self.heal_window.show("heal_window".into(), ctx, 
            |c,_| format!("Heal {}", c), 
            |_,_| "Amount: ".into(),
            |c,d|{
                self.tracker.heal(&c, d)?;
                Ok(())
            }
        )
    }

    fn show_add_temp_hp_window(&mut self, ctx: &Context) -> Result<()> {
        self.add_temp_hp_window.show("add_temp_hp_window".into(), ctx, 
            |c,_| format!("Add temp HP to {}", c), 
            |_,_| "Amount: ".into(),
            |c,d| {
                self.tracker.add_temp_health(&c, d)?;
                Ok(())
            }
        )
    }

    fn show_character_panel(&mut self, ctx: &Context) -> Result<()> {
        let frame = egui::Frame::default().inner_margin(egui::Margin::symmetric(40, 20));
        egui::CentralPanel::default()
            .frame(frame)
            .show(ctx, |ui| {
                let responses = characters::show(&self.tracker, ui);

                for resp in responses {
                    match resp {
                        characters::Response::RemoveCharacter(name) => { 
                            self.tracker.rm_chr(&name)?;
                        },
                        characters::Response::OpenCondWindow(name) => {
                            self.add_cond_window.open(name);
                        },
                        characters::Response::RenameCharacter(name) => {
                            self.rename_window.open(name);
                        },
                        characters::Response::OpenHealthWindow(name) => {
                            self.health_window.open(&self.tracker, name);
                        },
                        characters::Response::OpenDamageWindow(name) => {
                            self.damage_window.open(name);
                        },
                        characters::Response::OpenHealWindow(name) => {
                            self.heal_window.open(name);
                        },
                        characters::Response::OpenAddTempHpWindow(name) => {
                            self.add_temp_hp_window.open(name);
                        },
                    }
                }

                Ok(())
            }).inner
    }

    fn show_button_panel(&mut self, ctx: &Context) -> Result<()> {
        egui::TopBottomPanel::bottom("controls").show(ctx, |ui| {
            let (lret, rret) = egui::containers::Sides::new().show(ui, 
                |ui|{
                    if button_panel_button(ui,"\u{25B6}").on_hover_text("Makes it the next characters turn.").clicked() { return Some(ButtonPanelResponse::EndTurn) }
                    if button_panel_button(ui, egui::RichText::new("+")).on_hover_text("Adds a character.").clicked() { return Some(ButtonPanelResponse::Add) }
                    if button_panel_button(ui, "\u{27F2}").on_hover_text("Undoes the last change.").clicked() { return Some(ButtonPanelResponse::Undo) }
                    if button_panel_button(ui, "\u{27F3}").on_hover_text("Redoes the last undone change.").clicked(){ return Some(ButtonPanelResponse::Redo) }
                    None
                },
                |ui|{
                    if button_panel_button(ui, "\u{1F5D1}").on_hover_text("Removes every character.").clicked() { return Some(ButtonPanelResponse::Clear) }
                    None
                }
            );

            if let Some(res) = lret.or(rret) {
                match res {
                    ButtonPanelResponse::EndTurn => {self.tracker.end_turn()?;},
                    ButtonPanelResponse::Add => {self.add_window.open();},
                    ButtonPanelResponse::Undo => {self.tracker.undo()?;},
                    ButtonPanelResponse::Redo => {self.tracker.redo()?;},
                    ButtonPanelResponse::Clear => {self.tracker.clear();},
                }
            }

            Ok::<(), Error>(())
        }).inner
    }
}

const BUTTON_SIZE: f32 = 20.0;
fn button_panel_button<'a>(ui: &mut egui::Ui, atoms: impl IntoAtoms<'a>) -> egui::Response {
    let button = egui::Button::new(atoms).min_size((BUTTON_SIZE, BUTTON_SIZE).into());
    ui.add(button)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ButtonPanelResponse {
    EndTurn,
    Add,
    Undo,
    Redo,
    Clear,
}

#[derive(Debug, Clone, Copy)]
#[derive(Default)]
enum Confirmation {
    Confirm,
    #[default] Cancel
}

fn show_confirmation_bar(ui: &mut Ui) -> Option<Confirmation> {
    egui::Sides::new().show(ui, 
        |_| {},
        |ui| {
        if ui.button("confirm").clicked() {
            return Some(Confirmation::Confirm)
        }
        if ui.button("cancel").clicked() {
            return Some(Confirmation::Cancel)
        }

        None
    }).1
}
