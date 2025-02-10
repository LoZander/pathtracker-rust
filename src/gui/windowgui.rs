use addwindow::AddWindow;
use condwindow::CondWindow;
use dragvaluewindow::DragValueWindow;
use egui::{Context, Ui};
use errorwindow::ErrorWindow;
use healthwindow::HealthWindow;
use renamewindow::RenameWindow;

use crate::{character::Chr, saver::Saver, tracker::{self, Tracker}};

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
    damage_window: DragValueWindow<u32, Chr>,
    heal_window: DragValueWindow<u32, Chr>,
    add_temp_hp_window: DragValueWindow<u32, Chr>,
}

impl<S: Saver> eframe::App for WindowApp<S> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.error_window.show(ctx);
        let res = self.show_main_window(ctx)
            .and(self.add_window.show(&mut self.tracker, ctx))
            .and(self.add_cond_window.show(&mut self.tracker, ctx))
            .and(self.rename_window.show(&mut self.tracker, ctx))
            .and(self.health_window.show(&mut self.tracker, ctx))
            .and(self.show_damage_window(ctx))
            .and(self.show_heal_window(ctx))
            .and(self.show_add_temp_hp_window(ctx));

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
            |c,_| format!("Damage {}", c.name), 
            |_,_| "Amount: ".into(),
            |c,d|{
                self.tracker.damage(&c.name, d)?;
                Ok(())
            }
        )
    }

    fn show_heal_window(&mut self, ctx: &Context) -> Result<()> {
        self.heal_window.show("heal_window".into(), ctx, 
            |c,_| format!("Heal {}", c.name), 
            |_,_| "Amount: ".into(),
            |c,d|{
                self.tracker.heal(&c.name, d)?;
                Ok(())
            }
        )
    }

    fn show_add_temp_hp_window(&mut self, ctx: &Context) -> Result<()> {
        self.add_temp_hp_window.show("add_temp_hp_window".into(), ctx, 
            |c,_| format!("Add temp HP to {}", c.name), 
            |_,_| "Amount: ".into(),
            |c,d| {
                self.tracker.add_temp_health(&c.name, d)?;
                Ok(())
            }
        )
    }

    fn show_character_panel(&mut self, ctx: &Context) -> Result<()> {
        let frame = egui::Frame::default().inner_margin(egui::Margin::symmetric(40.0, 20.0));
        egui::CentralPanel::default()
            .frame(frame)
            .show(ctx, |ui| {
                let responses = characters::show(&self.tracker, ui);

                for resp in responses {
                    match resp {
                        characters::Response::RemoveCharacter(chr) => { 
                            self.tracker.rm_chr(&chr.name)?;
                        },
                        characters::Response::OpenCondWindow(chr) => {
                            self.add_cond_window.open(chr);
                        },
                        characters::Response::RenameCharacter(chr) => {
                            self.rename_window.open(chr);
                        },
                        characters::Response::OpenHealthWindow(chr) => {
                            self.health_window.open(chr);
                        },
                        characters::Response::OpenDamageWindow(chr) => {
                            self.damage_window.open(chr);
                        },
                        characters::Response::OpenHealWindow(chr) => {
                            self.heal_window.open(chr);
                        },
                        characters::Response::OpenAddTempHpWindow(chr) => {
                            self.add_temp_hp_window.open(chr);
                        },
                    }
                }

                Ok(())
            }).inner
    }

    fn show_button_panel(&mut self, ctx: &Context) -> Result<()> {
        egui::TopBottomPanel::bottom("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Next").clicked() {
                    self.tracker.end_turn()?;
                }
                if ui.button("add").clicked() {
                    self.add_window.open();
                }

                Ok::<(), Error>(())
            }).inner
        }).inner
    }
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
