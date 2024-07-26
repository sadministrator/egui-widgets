use egui::{Color32, Id, Response, Stroke, Ui, Widget};
use egui_material_icons::icons::{ICON_ARROW_DROP_DOWN, ICON_ARROW_DROP_UP};

use super::ButtonStyle;

pub struct Dropdown<'a> {
    id_source: Id,
    options: Vec<String>,
    selected: &'a mut String,
}

impl<'a> Dropdown<'a> {
    pub fn new(
        id_source: impl std::hash::Hash,
        options: Vec<String>,
        selected: &'a mut String,
    ) -> Self {
        if selected.is_empty() && !options.is_empty() {
            *selected = options[0].clone();
        }

        Self {
            id_source: Id::new(id_source),
            options,
            selected,
        }
    }
}

impl<'a> Widget for Dropdown<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let Dropdown {
            id_source,
            options,
            selected,
        } = self;

        let button_id = ui.make_persistent_id(id_source);
        let popup_id = button_id.with("popup");

        let is_open = ui.memory(|m| m.is_popup_open(popup_id));
        let icon = if is_open {
            ICON_ARROW_DROP_UP
        } else {
            ICON_ARROW_DROP_DOWN
        };

        let button_text = format!("{}     {}", selected, icon);
        let mut button_response = ui.add(
            crate::Button::new(
                button_text,
                super::ButtonVariant::Custom(Color32::from_rgb(117, 117, 117)),
                super::ButtonSize::Large,
                true,
                false,
            )
            .text_color(Color32::BLACK)
            .on_hover_style(ButtonStyle::new(
                Color32::TRANSPARENT,
                Stroke::new(1.2, Color32::from_rgb(0, 123, 187)),
                Color32::BLACK,
            )),
        );

        let mut selected_changed = false;

        if button_response.clicked() {
            ui.memory_mut(|m| m.toggle_popup(popup_id));
        }

        if ui.memory(|m| m.is_popup_open(popup_id)) {
            let popup_width = button_response.rect.width();
            let popup_position = button_response.rect.left_bottom() + egui::vec2(-6.0, 2.0);

            let area_response = egui::Area::new(popup_id)
                .order(egui::Order::Foreground)
                .fixed_pos(popup_position)
                .show(ui.ctx(), |ui| {
                    ui.set_min_width(popup_width);
                    egui::Frame::popup(ui.style())
                        .stroke(Stroke::NONE)
                        .show(ui, |ui| {
                            ui.set_min_width(popup_width);
                            for option in &options {
                                let is_selected = *option == *selected;
                                let response = ui.add_sized(
                                    [popup_width, 0.0],
                                    egui::SelectableLabel::new(is_selected, option),
                                );
                                if response.clicked() {
                                    *selected = option.clone();
                                    selected_changed = true;
                                    ui.memory_mut(|m| m.close_popup());
                                }
                            }
                        });
                });

            if !button_response.clicked()
                && (ui.input(|i| i.key_pressed(egui::Key::Escape))
                    || area_response.response.clicked_elsewhere())
            {
                ui.memory_mut(|m| m.close_popup());
            }
        }

        if button_response.has_focus() {
            ui.painter()
                .rect_stroke(button_response.rect, 0.0, Stroke::new(2.0, Color32::BLUE));
        }

        if selected_changed {
            button_response.mark_changed();
            button_response
        } else {
            button_response
        }
    }
}
