use egui::{
    pos2, Color32, NumExt, Response, Rounding, Stroke, TextStyle, Ui, Vec2, Widget, WidgetText,
};

pub enum ButtonSize {
    Small,
    Large,
}

#[derive(Clone, Copy)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Warning,
    Black,
    Green,
    Custom(Color32),
}

impl ButtonVariant {
    fn color(&self) -> Color32 {
        match self {
            ButtonVariant::Primary => Color32::from_rgb(0, 123, 187),
            ButtonVariant::Secondary => Color32::from_rgb(103, 103, 120),
            ButtonVariant::Warning => Color32::from_rgb(183, 28, 28),
            ButtonVariant::Black => Color32::BLACK,
            ButtonVariant::Green => Color32::from_rgb(30, 112, 30),
            ButtonVariant::Custom(color) => *color,
        }
    }
}

#[derive(Clone)]
pub struct ButtonStyle {
    fill_color: Color32,
    stroke: Stroke,
    text_color: Color32,
}

impl ButtonStyle {
    pub fn new(fill_color: Color32, stroke: Stroke, text_color: Color32) -> ButtonStyle {
        ButtonStyle {
            fill_color,
            stroke,
            text_color,
        }
    }
}

pub struct Button {
    text: String,
    variant: ButtonVariant,
    size: ButtonSize,
    inverted: bool,
    disabled: bool,
    style: ButtonStyle,
    hover_style: Option<ButtonStyle>,
    disabled_style: Option<ButtonStyle>,
}

impl Button {
    pub fn primary(text: String, size: ButtonSize) -> Self {
        let variant = ButtonVariant::Primary;
        let inverted = false;
        let disabled = false;
        let style = default_style(variant, inverted, disabled);

        Self {
            text,
            variant,
            size,
            inverted,
            disabled,
            style,
            disabled_style: None,
            hover_style: None,
        }
    }

    pub fn secondary(text: String, size: ButtonSize) -> Self {
        let variant = ButtonVariant::Secondary;
        let inverted = false;
        let disabled = false;
        let style = default_style(variant, inverted, disabled);

        Self {
            text,
            variant,
            size,
            inverted,
            disabled,
            style,
            disabled_style: None,
            hover_style: None,
        }
    }

    pub fn warning(text: String, size: ButtonSize) -> Self {
        let variant = ButtonVariant::Warning;
        let inverted = false;
        let disabled = false;
        let style = default_style(variant, inverted, disabled);

        Self {
            text,
            variant,
            size,
            inverted,
            disabled,
            style,
            disabled_style: None,
            hover_style: None,
        }
    }

    pub fn black(text: String, size: ButtonSize) -> Self {
        let variant = ButtonVariant::Black;
        let inverted = false;
        let disabled = false;
        let style = default_style(variant, inverted, disabled);

        Self {
            text,
            variant,
            size,
            inverted,
            disabled,
            style,
            disabled_style: None,
            hover_style: None,
        }
    }

    pub fn green(text: String, size: ButtonSize) -> Self {
        let variant = ButtonVariant::Green;
        let inverted = false;
        let disabled = false;
        let style = default_style(variant, inverted, disabled);

        Self {
            text,
            variant,
            size,
            inverted,
            disabled,
            style,
            disabled_style: None,
            hover_style: None,
        }
    }

    pub fn new(
        text: String,
        variant: ButtonVariant,
        size: ButtonSize,
        inverted: bool,
        disabled: bool,
    ) -> Self {
        let style = default_style(variant, inverted, disabled);

        Self {
            text,
            variant,
            size,
            inverted,
            disabled,
            style,
            disabled_style: None,
            hover_style: None,
        }
    }

    pub fn invert(mut self, invert: bool) -> Self {
        self.inverted = invert;
        self.style = default_style(self.variant, self.inverted, self.disabled);
        self
    }

    pub fn disable(mut self, disable: bool) -> Self {
        self.disabled = disable;
        self.style = default_style(self.variant, self.inverted, self.disabled);
        self
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn on_hover_style(mut self, style: ButtonStyle) -> Self {
        self.hover_style = Some(style);
        self
    }

    pub fn disabled_style(mut self, style: ButtonStyle) -> Self {
        self.disabled_style = Some(style);
        self
    }

    pub fn text_color(mut self, text_color: Color32) -> Self {
        self.style.text_color = text_color;
        self
    }

    fn current_style(&self, hovered: bool) -> ButtonStyle {
        if self.disabled {
            self.disabled_style.clone().unwrap_or(self.style.clone())
        } else if hovered {
            self.hover_style.clone().unwrap_or(self.style.clone())
        } else {
            self.style.clone()
        }
    }
}

impl Widget for Button {
    fn ui(self, ui: &mut Ui) -> Response {
        let widget_text = WidgetText::from(self.text.clone());
        let button_padding = match self.size {
            ButtonSize::Small => Vec2::new(
                ui.spacing().button_padding.x * 1.5,
                ui.spacing().button_padding.y * 0.8,
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

        match &self.size {
            ButtonSize::Small => desired_size = desired_size.at_least(min_size),
            ButtonSize::Large => {
                desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y)
            }
        };

        let (rect, mut response) = ui.allocate_at_least(desired_size, egui::Sense::click());

        let (rect, fill_color) = if response.clicked() && !self.disabled {
            response.mark_changed();
            (
                rect.shrink(2.0),
                self.style.fill_color.linear_multiply(0.85),
            )
        } else {
            (rect, self.style.fill_color)
        };

        if ui.is_rect_visible(rect) {
            let style = self.current_style(response.hovered());
            let rect = if response.hovered() && !self.disabled {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                rect.expand(1.0)
            } else {
                rect
            };
            let rounding = Rounding::same(4.0);

            ui.painter().rect(rect, rounding, fill_color, style.stroke);

            let cursor_x = rect.min.x + button_padding.x;
            let text_pos = pos2(cursor_x, rect.center().y - 0.5 * galley.size().y);

            ui.painter().galley(text_pos, galley, self.style.text_color);
        }

        response
    }
}

fn default_fill_color(variant: ButtonVariant, inverted: bool, disabled: bool) -> Color32 {
    if inverted {
        Color32::TRANSPARENT
    } else {
        if disabled {
            disable_color(variant.color())
        } else {
            variant.color()
        }
    }
}

fn default_stroke(inverted: bool, variant: ButtonVariant) -> Stroke {
    let width = 1.0;
    if inverted {
        Stroke::new(width, variant.color())
    } else {
        Stroke::new(width, Color32::WHITE)
    }
}

fn default_text_color(variant: ButtonVariant, inverted: bool, disabled: bool) -> Color32 {
    let mut color = if inverted {
        variant.color()
    } else {
        Color32::WHITE
    };

    if disabled {
        color = disable_color(color);
    }

    color
}

fn disable_color(color: Color32) -> Color32 {
    color.linear_multiply(0.5)
}

fn default_style(variant: ButtonVariant, inverted: bool, disabled: bool) -> ButtonStyle {
    ButtonStyle {
        fill_color: default_fill_color(variant.clone(), inverted, disabled),
        stroke: default_stroke(inverted, variant),
        text_color: default_text_color(variant, inverted, disabled),
    }
}
