use egui::{Color32, Stroke, Widget};

pub struct Switch<'a> {
    on: &'a mut bool,
}

impl<'a> Switch<'a> {
    pub fn new(on: &'a mut bool) -> Self {
        Self { on }
    }
}

impl<'a> Widget for Switch<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(1.9, 1.1);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if response.clicked() {
            *self.on = !*self.on;
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let how_on = ui.ctx().animate_bool_responsive(response.id, *self.on);
            let visuals = ui.style().interact_selectable(&response, *self.on);
            let rect = rect.expand(visuals.expansion);

            let radius = 0.5 * rect.height();
            let knob_fill = Color32::WHITE;
            let knob_stroke = Stroke::new(1.0, knob_fill);
            let track_fill = if *self.on {
                Color32::from_rgb(91, 194, 54)
            } else {
                Color32::from_rgb(229, 229, 229)
            };
            ui.painter()
                .rect(rect, radius, track_fill, visuals.bg_stroke);
            let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
            let center = egui::pos2(circle_x, rect.center().y);
            ui.painter()
                .circle(center, 0.75 * radius, knob_fill, knob_stroke);
        }

        response
    }
}
