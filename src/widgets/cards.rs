use egui::{Color32, Layout, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

pub struct Card<'a> {
    content: Box<dyn FnMut(&mut Ui) + 'a>,
}

impl<'a> Card<'a> {
    pub fn new(content: impl FnMut(&mut Ui) + 'a) -> Self {
        Card {
            content: Box::new(content),
        }
    }
}

impl<'a> Widget for Card<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let padding: f32 = 8.0;
        let available_width = ui.available_width();

        let (_id, outer_rect) = ui.allocate_space(Vec2::new(available_width, 0.0));
        let inner_rect = outer_rect.shrink(padding);

        let mut content_ui = ui.child_ui(inner_rect, Layout::top_down(egui::Align::Center), None);
        (self.content)(&mut content_ui);
        let content_size = content_ui.min_size();

        let card_size = content_size + Vec2::new(padding * 2.0, padding * 2.0);
        let outer_rect = Rect::from_min_size(outer_rect.min, card_size);
        let inner_rect = outer_rect.shrink(padding);

        let corner_radius = Rounding::same(8.0);
        let stroke = Stroke::new(1.0, Color32::from_rgb(204, 204, 204));

        ui.painter()
            .rect(outer_rect, corner_radius, Color32::WHITE, stroke);

        let content_response = ui.allocate_ui_at_rect(inner_rect, |ui| {
            (self.content)(ui);
        });

        ui.interact(outer_rect, ui.id(), Sense::hover());

        content_response.response
    }
}
