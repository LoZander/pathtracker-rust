use egui::{Context, Id};
use super::Confirmation;

#[derive(Debug)]
pub struct DragValueWindow<T, E> where
    T: egui::emath::Numeric,
{
    open: bool,
    focus: bool,
    elem: Option<E>,
    val: T,
}

impl<T,E> Default for DragValueWindow<T, E> where
    T: egui::emath::Numeric + Default
{
    fn default() -> Self {
        Self {
            open: false,
            focus: false,
            elem: None,
            val: T::default()
        }
    }
}

impl<T, E> DragValueWindow<T, E> where
    T: egui::emath::Numeric + Default,
    E: Clone,
{
    fn reset(&mut self) {
        self.elem = None;
        self.val = T::default();
    }

    pub fn open(&mut self, elem: E) {
        self.open = true;
        self.focus = true;
        self.elem = Some(elem);
    }

    fn close(&mut self) {
        self.open = false;
        self.reset();
    }

    pub fn show<F, G, H>(&mut self, id: Id, ctx: &Context, heading: F, label: G, action: H) -> super::Result<()> where 
        F: FnOnce(E, T) -> String,
        G: FnOnce(E, T) -> String,
        H: FnOnce(E, T) -> super::Result<()>,
    {
        self.elem.clone().map_or(Ok(()), |elem| egui::Modal::new(id).show(ctx, |ui| {
            ui.heading(heading(elem.clone(), self.val));
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(label(elem.clone(), self.val));
                ui.add(egui::DragValue::new(&mut self.val).range(0..=999));
            });
            ui.separator();

            let response = super::show_confirmation_bar(ui);

            match response {
                Some(Confirmation::Confirm) => {
                    action(elem, self.val)?;
                    self.close();
                },
                Some(Confirmation::Cancel) => {
                    self.close();
                }
                None => (),
            }

            Ok(())
        }).inner)
    }
}
