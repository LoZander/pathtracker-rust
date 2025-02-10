use addwindow::AddWindow;
use condwindow::CondWindow;
use egui::{Context, Ui};
use healthwindow::HealthWindow;
use renamewindow::RenameWindow;

use crate::{saver::Saver, tracker::{self, Tracker}};

mod condwindow;
mod addwindow;
mod characters;
mod renamewindow;
mod healthwindow;

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
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    TrackerError(#[from] tracker::Error)
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[derive(Default)]
struct ErrorWindow {
    open: bool,
    err: Option<Error>
}

impl ErrorWindow {
    fn open(&mut self, err: Error) {
        self.err = Some(err);
        self.open = true;
    }

    const fn close(&mut self) {
        self.open = false;
    }

    fn show(&mut self, ctx: &Context) {
        if !self.open { return }
        egui::Modal::new("error".into()).show(ctx, |ui| {
            ui.heading("Error");

            ui.separator();
            
            ui.label(self.err.as_ref().map_or("no error? This window opening is an error in and of itself.".into(), ToString::to_string));

            ui.separator();

            egui::Sides::new().show(ui, 
                |_|{},
                |ui|{
                   if ui.button("ok").clicked() {
                       self.close();
                   }
                });
        });
    }
}

impl<S: Saver> WindowApp<S> {
    fn show_main_window(&mut self, ctx: &Context) -> Result<()> {
        self.show_button_panel(ctx)?;
        self.show_character_panel(ctx)
    }

    fn show_character_panel(&mut self, ctx: &Context) -> std::result::Result<(), Error> {
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

impl<S: Saver> eframe::App for WindowApp<S> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.error_window.show(ctx);
        let res = self.show_main_window(ctx)
            .and(self.add_window.show(&mut self.tracker, ctx))
            .and(self.add_cond_window.show(&mut self.tracker, ctx))
            .and(self.rename_window.show(&mut self.tracker, ctx))
            .and(self.health_window.show(&mut self.tracker, ctx));

        if let Err(err) = res {
            self.error_window.open(err);
        }
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
