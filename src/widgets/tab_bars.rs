use egui::{pos2, Align2, Color32, FontFamily, FontId, Rect, Response, Sense, Stroke, Ui, Vec2};

pub struct TabBar<T: PartialEq + Clone> {
    tabs: Vec<TabBarItem<T>>,
    id: egui::Id,
    height: f32,
}

pub struct TabBarItem<T: PartialEq + Clone> {
    value: T,
    label: String,
    icon: String,
}

impl<T: PartialEq + Clone> TabBar<T> {
    pub fn new(id: impl Into<egui::Id>, tabs: Vec<TabBarItem<T>>) -> Self {
        TabBar {
            tabs,
            id: id.into(),
            height: 48.0,
        }
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn show(self, ctx: &egui::Context, selected: &mut T) -> Response {
        let TabBar { tabs, id, height } = self;
        let padding = 16.0;

        egui::TopBottomPanel::bottom(id)
            .exact_height(height)
            .show(ctx, |ui| {
                let tab_bar_rect = ui.max_rect();
                Self::ui_content(ui, tab_bar_rect, tabs, selected, height + padding)
            })
            .inner
    }

    fn ui_content(
        ui: &mut Ui,
        tab_bar_rect: Rect,
        tabs: Vec<TabBarItem<T>>,
        selected: &mut T,
        height: f32,
    ) -> Response {
        let response = ui.allocate_rect(tab_bar_rect, Sense::click());

        let fill_color = Color32::from_rgba_premultiplied(229, 229, 234, 230);
        let stroke_color = Color32::from_rgb(142, 142, 147);
        let selected_color = Color32::from_rgb(0, 122, 255);
        let stroke = Stroke::new(1.0, stroke_color);

        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        ui.painter()
            .rect_filled(tab_bar_rect.expand(9.0), 2.0, fill_color);
        ui.painter()
            .rect_stroke(tab_bar_rect.expand(9.0), 2.0, stroke);

        let item_width = tab_bar_rect.width() / tabs.len() as f32;

        for (index, item) in tabs.into_iter().enumerate() {
            let item_rect = Rect::from_min_size(
                pos2(
                    tab_bar_rect.min.x + item_width * index as f32,
                    tab_bar_rect.min.y,
                ),
                Vec2::new(item_width, height),
            );

            let item_response = ui.allocate_rect(item_rect, Sense::click());

            if item_response.clicked() {
                *selected = item.value.clone();
                ui.ctx().request_repaint();
            }

            let label_color = if *selected == item.value {
                selected_color
            } else {
                stroke_color
            };

            ui.painter().text(
                item_rect.center() - Vec2::new(0.0, 6.0),
                Align2::CENTER_BOTTOM,
                &item.icon,
                FontId::new(20.0, FontFamily::Proportional),
                label_color,
            );

            ui.painter().text(
                item_rect.center() - Vec2::new(0.0, 6.0),
                Align2::CENTER_TOP,
                item.label,
                FontId::new(12.0, FontFamily::Proportional),
                label_color,
            );
        }

        response
    }
}

impl<T: Clone + PartialEq> TabBarItem<T> {
    pub fn new(value: T, label: String, icon: String) -> Self {
        TabBarItem { label, icon, value }
    }
}
