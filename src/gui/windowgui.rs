use addwindow::AddWindow;
use character::init_characters;
use condwindow::CondWindow;
use egui::{vec2, Context, RichText};
use healthwindow::HealthWindow;
use renamewindow::RenameWindow;

use crate::{saver::Saver, tracker::Tracker};

mod condwindow;
mod addwindow;
mod character;
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
    add_cond_window_open: bool,
    rename_window: RenameWindow,
    health_window: HealthWindow
}

impl<S: Saver> WindowApp<S> {
    pub fn new(tracker: Tracker<S>) -> Self {
        Self {
            tracker,
            add_window: AddWindow::default(),
            add_cond_window: CondWindow::default(),
            add_cond_window_open: false,
            rename_window: RenameWindow::default(),
            health_window: HealthWindow::default()
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
        let frame = egui::Frame::default().inner_margin(egui::Margin::symmetric(40.0, 20.0));
        egui::TopBottomPanel::bottom("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Next").clicked() {
                    if let Err(err) = self.tracker.end_turn() {
                        error_window(ctx, "Save error:", err.to_string());
                    }
                }
                if ui.button("add").clicked() {
                    self.add_window.open();
                }
            });
        });

        egui::CentralPanel::default()
            .frame(frame)
            .show(ctx, |ui| {
                let responses = init_characters(&self.tracker, ui);
                for resp in responses {
                    match resp {
                        character::Response::RemoveCharacter(chr) => { 
                            if let Err(err) = self.tracker.rm_chr(&chr.name) {
                                error_window(ctx, "Save error", err.to_string());
                            }
                        },
                        character::Response::OpenCondWindow(chr) => {
                            self.add_cond_window.prepare(chr);
                            self.add_cond_window_open = true;
                        },
                        character::Response::RenameCharacter(chr) => {
                            self.rename_window.open(chr);
                        },
                        character::Response::OpenHealthWindow(chr) => {
                            self.health_window.open(chr);
                        },
                    }
                }
            });
    }

}

impl<S: Saver> eframe::App for WindowApp<S> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.init_main(ctx);
        self.add_window.init(&mut self.tracker, ctx);
        self.add_cond_window.init(&mut self.tracker, ctx, &mut self.add_cond_window_open);
        self.rename_window.init(&mut self.tracker, ctx);
        self.health_window.init(&mut self.tracker, ctx);
    }
}
