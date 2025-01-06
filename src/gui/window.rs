use egui::{vec2, Context, InnerResponse, Window};

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

impl AddWindow {
    pub fn reset(&mut self) {
        self.name = String::new();
        self.init = 0;
        self.player = false;
    }

    pub fn open(&mut self) {
        self.show = true;
        self.focus = true;
    }

    pub fn close(&mut self) {
        self.show = false;
        self.focus = false;
        self.reset();
    }

    pub fn init<S: Saver>(&mut self, ctx: &Context, tracker: &mut Tracker<S>) {
    }
}

impl<S: Saver> WindowApp<S> {
    pub const fn new(tracker: Tracker<S>) -> Self {
        Self {
            tracker,
            add_window: AddWindow {
                show: false,
                focus: false,
                name: String::new(),
                init: 0,
                player: false
            }
        }
    }
}

impl<S: Saver> WindowApp<S> {
    pub fn init_main(&mut self, ctx: &Context) {
        let frame = egui::Frame::default().inner_margin(egui::Margin::symmetric(50.0, 20.0));
        egui::TopBottomPanel::bottom("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Next").clicked() {
                    if let Err(err) = self.tracker.end_turn() {
                        egui::Window::new("Error")
                            .fixed_size(vec2(200.0, 100.0))
                            .show(ctx, |ui| {
                                ui.heading("Save error:");
                                ui.label(err.to_string())
                            });
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
                    self.tracker.get_chrs().iter().for_each(|c| {
                        ui.style_mut().spacing.indent = 20.0;
                        if self.tracker.get_in_turn() == Some(c) {
                            ui.horizontal(|ui| {
                                ui.heading(c.init.to_string());
                                ui.label(c.name.clone());
                            });
                        } else {
                            ui.indent(0, |ui| {
                                ui.horizontal(|ui| {
                                    ui.heading(c.init.to_string());
                                    ui.label(c.name.clone());
                                });
                            });
                        }
                    });
                });
            });
    }

    pub fn init_add(&mut self, ctx: &Context) {
        let add_window = &mut self.add_window;
        if add_window.show {
            egui::Window::new("Add character")
                .default_size(vec2(200.0, 100.0))
                .show(ctx, |ui| {
                    let name_edit = ui.text_edit_singleline(&mut add_window.name);
                    if add_window.focus {
                        name_edit.request_focus();
                        add_window.focus = false;
                    }
                    let drag = egui::DragValue::new(&mut add_window.init).range(0..=50);
                    ui.add(drag);
                    ui.toggle_value(&mut add_window.player, "Player");
                    if ui.button("add").clicked() {
                        self.tracker.add_chr(Chr::builder(add_window.name.clone(), add_window.init, add_window.player).build());
                        add_window.close();
                    };
                    if ui.button("cancel").clicked() {
                        add_window.close();
                    }
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
