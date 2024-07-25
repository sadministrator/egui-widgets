use egui::{Layout, Response, Ui, Widget};

pub struct SettingsLine<'a> {
    icon: String,
    label: String,
    switch: Box<dyn FnMut(&mut Ui) + 'a>,
}

impl<'a> SettingsLine<'a> {
    pub fn new(icon: String, label: String, switch: Box<dyn FnMut(&mut Ui) + 'a>) -> Self {
        Self {
            icon,
            label,
            switch,
        }
    }
}

impl<'a> Widget for SettingsLine<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.heading(self.icon);
            ui.heading(self.label);

            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                (self.switch)(ui);
            });
        })
        .response
    }
}
