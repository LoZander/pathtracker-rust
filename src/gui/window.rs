use egui::{vec2, Context, RichText, Ui, Window};

use crate::{character::Chr, saver::Saver, tracker::Tracker};

pub fn run<S: Saver>(mut t: Tracker<S>) -> eframe::Result {
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
}

struct AddWindow {
    show: bool,
    focus: bool,
    name: String,
    init: i32,
    player: bool
}

impl Default for AddWindow {
    fn default() -> Self {
        Self {
            show: false,
            focus: false,
            name: String::new(),
            init: 0,
            player: false
        }
    }
}

impl AddWindow {
    fn reset(&mut self) {
        self.name = String::new();
        self.init = 0;
        self.player = false;
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

impl<S: Saver> WindowApp<S> {
    pub fn new(tracker: Tracker<S>) -> Self {
        Self {
            tracker,
            add_window: AddWindow::default()
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
            egui::Window::new("Add character")
                .default_size(vec2(200.0, 100.0))
                .show(ctx, |ui| {
                    let name_edit = ui.text_edit_singleline(&mut add_window.name);
                    let drag = egui::DragValue::new(&mut add_window.init).range(0..=50);
                    ui.add(drag);
                    ui.toggle_value(&mut add_window.player, "Player");
                    if ui.button("add").clicked() {
                        let character = Chr::builder(add_window.name.clone(), add_window.init, add_window.player).build();
                        if let Err(err) = self.tracker.add_chr(character) {
                            error_window(ctx, "Save error", err.to_string());
                        };
                        add_window.close();
                    };
                    if ui.button("cancel").clicked() {
                        add_window.close();
                    }
                    if add_window.focus {
                        name_edit.request_focus();
                        add_window.focus = false;
                    }
                });
        }
    }

    fn init_character(&mut self, ctx: &Context, ui: &mut Ui, character: &Chr) {
        ui.style_mut().spacing.indent = 20.0;
        if self.tracker.get_in_turn() == Some(character) {
            ui.horizontal(|ui| {
                ui.heading(character.init.to_string());
                ui.label(character.name.clone());
                if ui.small_button("x").clicked() {
                    if let Err(err) = self.tracker.rm_chr(&character.name) {
                        error_window(ctx, "Save error", err.to_string());
                    };
                }
            });
        } else {
            ui.indent(0, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(character.init.to_string());
                    ui.label(character.name.clone());
                    if ui.small_button("x").clicked() {
                        if let Err(err) = self.tracker.rm_chr(&character.name) {
                            error_window(ctx, "Save error", err.to_string());
                        }
                    }
                });
            });
        }
    }
}


impl<S: Saver> eframe::App for WindowApp<S> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.init_main(ctx);
        self.init_add(ctx);
    }
}
