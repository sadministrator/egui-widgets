use egui::{pos2, Color32, NumExt, Rounding, Stroke, TextStyle, Ui, Vec2, Widget, WidgetText};

pub enum ButtonSize {
    Small,
    Large,
}

pub enum ButtonVariant {
    Primary,
    Secondary,
    Warning,
    Black,
    Green,
}

impl ButtonVariant {
    fn color(&self) -> Color32 {
        match self {
            ButtonVariant::Primary => Color32::from_rgb(0, 123, 187),
            ButtonVariant::Secondary => Color32::from_rgb(103, 103, 120),
            ButtonVariant::Warning => Color32::from_rgb(183, 28, 28),
            ButtonVariant::Black => Color32::BLACK,
            ButtonVariant::Green => Color32::from_rgb(30, 112, 30),
        }
    }
}

pub struct GephButton {
    text: String,
    variant: ButtonVariant,
    size: ButtonSize,
    inverted: bool,
    disabled: bool,
}

impl GephButton {
    pub fn primary(text: String, size: ButtonSize) -> Self {
        Self {
            text,
            variant: ButtonVariant::Primary,
            size,
            inverted: false,
            disabled: false,
        }
    }

    pub fn secondary(text: String, size: ButtonSize) -> Self {
        Self {
            text,
            variant: ButtonVariant::Secondary,
            size,
            inverted: false,
            disabled: false,
        }
    }

    pub fn warning(text: String, size: ButtonSize) -> Self {
        Self {
            text,
            variant: ButtonVariant::Warning,
            size,
            inverted: false,
            disabled: false,
        }
    }

    pub fn black(text: String, size: ButtonSize) -> Self {
        Self {
            text,
            variant: ButtonVariant::Black,
            size,
            inverted: false,
            disabled: false,
        }
    }

    pub fn green(text: String, size: ButtonSize) -> Self {
        Self {
            text,
            variant: ButtonVariant::Green,
            size,
            inverted: false,
            disabled: false,
        }
    }

    pub fn new(
        text: String,
        variant: ButtonVariant,
        size: ButtonSize,
        inverted: bool,
        disabled: bool,
    ) -> Self {
        Self {
            text,
            variant,
            size,
            inverted,
            disabled,
        }
    }

    pub fn invert(mut self, invert: bool) -> Self {
        self.inverted = invert;
        self
    }

    pub fn disable(mut self, disable: bool) -> Self {
        self.disabled = disable;
        self
    }

    fn fill_color(&self) -> Color32 {
        let fill_color = if self.inverted {
            Color32::TRANSPARENT
        } else {
            self.variant.color()
        };

        if self.disabled {
            fill_color.linear_multiply(0.5)
        } else {
            fill_color
        }
    }

    fn text_color(&self) -> Color32 {
        if self.inverted {
            self.variant.color()
        } else {
            Color32::WHITE
        }
    }

    fn stroke_color(&self) -> Color32 {
        self.variant.color()
    }
}

impl Widget for GephButton {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let widget_text = WidgetText::from(self.text.clone());
        let text_color = self.text_color();
        let stroke_color = self.stroke_color();

        let button_padding = match self.size {
            ButtonSize::Small => Vec2::new(
                ui.spacing().button_padding.x * 1.5,
                ui.spacing().button_padding.y * 0.75,
            ),
            ButtonSize::Large => ui.spacing().button_padding * 4.0,
        };
        let text_wrap_width = ui.available_width() - 2.0 * button_padding.x;
        let text_style = match self.size {
            ButtonSize::Small => TextStyle::Button,
            ButtonSize::Large => TextStyle::Heading,
        };
        let galley = widget_text.into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            text_wrap_width,
            text_style,
        );

        let min_size = ui.spacing().interact_size;
        let mut desired_size = Vec2::ZERO;
        desired_size.x += galley.size().x;
        desired_size.y = desired_size.y.max(galley.size().y);
        desired_size += 2.0 * button_padding;

        match self.size {
            ButtonSize::Small => desired_size = desired_size.at_least(min_size),
            ButtonSize::Large => {
                desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y)
            }
        };

        let (rect, mut response) = ui.allocate_at_least(desired_size, egui::Sense::click());

        let (rect, fill_color) = if response.clicked() {
            response.mark_changed();
            (rect.shrink(2.0), self.fill_color().linear_multiply(0.85))
        } else {
            (rect, self.fill_color())
        };

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);
            let rect = rect.expand(visuals.expansion);

            if response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }

            let rounding = Rounding::same(4.0);
            let stroke = Stroke::new(1.5, stroke_color);

            ui.painter().rect(rect, rounding, fill_color, stroke);

            let cursor_x = rect.min.x + button_padding.x;
            let text_pos = pos2(cursor_x, rect.center().y - 0.5 * galley.size().y);

            ui.painter().galley(text_pos, galley, text_color);
        }

        response
    }
}
