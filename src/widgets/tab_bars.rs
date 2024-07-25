use egui::{pos2, Align2, Color32, FontId, Rect, Response, Sense, Stroke, Ui, Vec2, Widget};

pub struct TabBar<'a> {
    tabs: Vec<TabBarItem>,
    selected_idx: &'a mut usize,
}

pub struct TabBarItem {
    label: String,
    icon: String,
}

impl<'a> TabBar<'a> {
    pub fn new(tabs: Vec<TabBarItem>, selected_idx: &'a mut usize) -> Self {
        TabBar { tabs, selected_idx }
    }
}

impl<'a> Widget for TabBar<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tab_bar_height = 64.0;
        let tab_bar_rect = Rect::from_min_size(
            pos2(ui.min_rect().min.x, ui.max_rect().max.y - tab_bar_height),
            Vec2::new(ui.min_rect().width(), tab_bar_height),
        );

        let mut tab_bar_ui = ui.child_ui(
            tab_bar_rect,
            egui::Layout::left_to_right(egui::Align::Center),
            None,
        );

        let response = tab_bar_ui.allocate_rect(tab_bar_rect, Sense::click());

        let fill_color = Color32::from_rgba_premultiplied(229, 229, 234, 230);
        let stroke_color = Color32::from_rgb(142, 142, 147);
        let selected_color = Color32::from_rgb(0, 122, 255);
        let stroke = Stroke::new(1.0, stroke_color);

        if response.hovered() {
            tab_bar_ui
                .ctx()
                .set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        tab_bar_ui
            .painter()
            .rect_filled(tab_bar_rect.expand(9.0), 2.0, fill_color);
        tab_bar_ui
            .painter()
            .rect_stroke(tab_bar_rect.expand(9.0), 2.0, stroke);

        let item_width = tab_bar_rect.width() / self.tabs.len() as f32;

        for (index, item) in self.tabs.into_iter().enumerate() {
            let item_rect = Rect::from_min_size(
                pos2(
                    tab_bar_rect.min.x + item_width * index as f32,
                    tab_bar_rect.min.y,
                ),
                Vec2::new(item_width, tab_bar_height),
            );

            let mut item_response = tab_bar_ui.allocate_rect(item_rect, Sense::click());

            if item_response.clicked() {
                *self.selected_idx = index;
                item_response.mark_changed();
            }

            let label_color = if *self.selected_idx == index {
                selected_color
            } else {
                stroke_color
            };

            tab_bar_ui.painter().text(
                item_rect.center() - Vec2::new(0.0, 6.0),
                Align2::CENTER_BOTTOM,
                &item.icon,
                FontId::new(24.0, Default::default()),
                label_color,
            );

            tab_bar_ui.painter().text(
                item_rect.center() - Vec2::new(0.0, 6.0),
                Align2::CENTER_TOP,
                item.label,
                FontId::new(12.0, Default::default()),
                label_color,
            );
        }

        let mut cursor = ui.cursor();
        cursor.max.y = tab_bar_rect.min.y;

        response
    }
}

impl TabBarItem {
    pub fn new(label: String, icon: String) -> Self {
        TabBarItem { label, icon }
    }
}
