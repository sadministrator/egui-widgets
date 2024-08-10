use egui::{Color32, Response, Rounding, Stroke, TextStyle, Ui, Vec2, Widget, WidgetText};

use crate::utils::disable_color;

const DISABLE_MULTIPLY: f32 = 0.9;
const DISABLE_OPACITY: f32 = 0.5;

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
        let style = derive_style(&variant, inverted, disabled);

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
        let style = derive_style(&variant, inverted, disabled);

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
        let style = derive_style(&variant, inverted, disabled);

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
        let style = derive_style(&variant, inverted, disabled);

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
        let style = derive_style(&variant, inverted, disabled);

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
        let style = derive_style(&variant, inverted, disabled);

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
        self.style = derive_style(&self.variant, self.inverted, self.disabled);
        self
    }

    pub fn disable(mut self, disable: bool) -> Self {
        self.disabled = disable;
        self.style = derive_style(&self.variant, self.inverted, self.disabled);
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
        let button_padding = match self.size {
            ButtonSize::Small => ui.spacing().button_padding * 1.5,
            ButtonSize::Large => ui.spacing().button_padding * 4.0,
        };
        let text_wrap_width = ui.available_width() - 2.0 * button_padding.x;
        let text_style = match self.size {
            ButtonSize::Small => TextStyle::Button,
            ButtonSize::Large => TextStyle::Heading,
        };
        let widget_text = WidgetText::from(self.text.clone());
        let galley = widget_text.into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            text_wrap_width,
            text_style,
        );
        let mut desired_size = Vec2::ZERO;
        desired_size.x += galley.size().x;
        desired_size.y = desired_size.y.max(galley.size().y);
        desired_size += 2.0 * button_padding;

        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let opacity_factor = 0.75;
            let rounding = Rounding::same(4.0);
            let current_style = self.current_style(response.hovered());
            let rect = if response.hovered() && !self.disabled {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                rect.expand(1.0)
            } else {
                rect
            };

            let ButtonStyle {
                fill_color,
                stroke,
                text_color,
            } = if response.is_pointer_button_down_on() && !self.disabled {
                response.mark_changed();

                ButtonStyle::new(
                    current_style.fill_color.linear_multiply(opacity_factor),
                    Stroke::new(
                        current_style.stroke.width,
                        current_style.stroke.color.linear_multiply(opacity_factor),
                    ),
                    current_style.text_color.linear_multiply(opacity_factor),
                )
            } else {
                ButtonStyle::new(
                    current_style.fill_color,
                    current_style.stroke,
                    current_style.text_color,
                )
            };

            ui.painter().rect(rect, rounding, fill_color, stroke);

            let text_pos = rect.center() - galley.size() / 2.0;

            ui.painter().galley(text_pos, galley, text_color);
        }

        response
    }
}

fn derive_fill_color(variant: &ButtonVariant, inverted: bool, disabled: bool) -> Color32 {
    if inverted {
        Color32::TRANSPARENT
    } else {
        if disabled {
            disable_color(variant.color(), DISABLE_MULTIPLY, DISABLE_OPACITY)
        } else {
            variant.color()
        }
    }
}

fn derive_stroke(variant: &ButtonVariant, inverted: bool, disabled: bool) -> Stroke {
    let width = 1.0;
    let mut stroke;

    if inverted {
        stroke = Stroke::new(width, variant.color())
    } else {
        stroke = Stroke::new(width, Color32::WHITE)
    }

    if disabled {
        stroke.color = disable_color(stroke.color, DISABLE_MULTIPLY, 0.5);
    }

    stroke
}

fn derive_text_color(variant: &ButtonVariant, inverted: bool, disabled: bool) -> Color32 {
    let mut color = if inverted {
        variant.color()
    } else {
        Color32::WHITE
    };

    if disabled {
        color = disable_color(color, DISABLE_OPACITY, 0.5);
    }

    color
}

fn derive_style(variant: &ButtonVariant, inverted: bool, disabled: bool) -> ButtonStyle {
    ButtonStyle {
        fill_color: derive_fill_color(&variant, inverted, disabled),
        stroke: derive_stroke(&variant, inverted, disabled),
        text_color: derive_text_color(&variant, inverted, disabled),
    }
}
