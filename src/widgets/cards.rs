use egui::{Color32, Layout, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2, Widget};

pub struct Card<'a> {
    content: Box<dyn Fn(&mut Ui) + 'a>,
}

impl<'a> Card<'a> {
    pub fn new(content: impl Fn(&mut Ui) + 'a) -> Self {
        Card {
            content: Box::new(content),
        }
    }
}

impl<'a> Widget for Card<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let padding: f32 = 8.0;
        let available_width = ui.available_width();

        let mut content_ui = ui.child_ui(ui.available_rect_before_wrap(), *ui.layout(), None);
        (self.content)(&mut content_ui);
        let mut content_size = content_ui.min_size();
        content_size.x = available_width - padding * 2.0;

        let card_size = content_size + Vec2::splat(padding * 2.0);
        let outer_rect = Rect::from_min_size(ui.available_rect_before_wrap().min, card_size);
        let inner_rect = outer_rect.shrink(padding);

        let corner_radius = Rounding::same(8.0);
        let stroke = Stroke::new(1.0, Color32::from_rgb(204, 204, 204));

        ui.painter()
            .rect(outer_rect, corner_radius, Color32::WHITE, stroke);

        let mut inner_ui = ui.child_ui(inner_rect, *ui.layout(), None);
        inner_ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
            (self.content)(ui);
        });

        ui.allocate_rect(outer_rect, Sense::hover())
    }
}
