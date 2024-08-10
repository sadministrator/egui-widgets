use egui::{Color32, RichText, Rounding, Stroke, Ui, Vec2};
use egui_material_icons::icons::{
    ICON_ATTACH_MONEY, ICON_INSERT_CHART_OUTLINED, ICON_MONEY_OFF_CSRED,
};
use std::collections::HashSet;

use crate::utils;

const DISABLE_MULTIPLY: f32 = 1.0;
const DISABLE_OPACITY: f32 = 0.5;

#[derive(Clone, Debug)]
pub struct ExitDescriptor {
    hostname: String,
    country_code: String,
    city_code: String,
    load: f32,
    allowed_levels: Vec<String>,
}

impl ExitDescriptor {
    pub fn new(
        hostname: String,
        country_code: String,
        city_code: String,
        load: f32,
        allowed_levels: Vec<String>,
    ) -> Self {
        Self {
            hostname,
            country_code,
            city_code,
            load,
            allowed_levels,
        }
    }
}

pub struct ExitSelector<'a> {
    exit_list: Vec<ExitDescriptor>,
    block_plus: bool,
    show_all: &'a mut bool,
    selected: &'a mut Option<ExitDescriptor>,
}

impl<'a> ExitSelector<'a> {
    pub fn new(
        exit_list: Vec<ExitDescriptor>,
        block_plus: bool,
        show_all: &'a mut bool,
        selected: &'a mut Option<ExitDescriptor>,
    ) -> Self {
        Self {
            exit_list,
            block_plus,
            show_all,
            selected,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading("Select exit server");
            ui.add(crate::Card::new(|ui| {
                ui.label("Free accounts are limited to 125 KB/s.\nUpgrade to Plus to enjoy unlimited speed!");
            }));
            ui.add_space(12.0);

            let list = if *self.show_all {
                &self.exit_list
            } else {
                &self.short_exit_list()
            };

            for exit in list {
                let is_disabled =
                    self.block_plus && !exit.allowed_levels.contains(&"free".to_string());

                let (rect, response) = ui.allocate_exact_size(
                    Vec2::new(ui.available_width(), ui.spacing().interact_size.y * 1.5),
                    egui::Sense::click(),
                );

                let rounding = 4.0;
                let default_bg_color = Color32::from_rgb(238, 238, 238);
                let bg_color = if is_disabled {
                    utils::disable_color(default_bg_color, DISABLE_MULTIPLY, DISABLE_OPACITY)
                } else {
                    default_bg_color
                };
                let stroke = Stroke::new(1.0, bg_color);

                ui.painter()
                    .rect(rect, Rounding::same(rounding), bg_color, stroke);

                    ui.allocate_ui_at_rect(rect, |ui| {
                        ui.add_enabled_ui(!is_disabled, |ui| {
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                let left_text = if *self.show_all {
                                    RichText::new(format!(
                                        "{} {} / {}-{}",
                                        to_flag(&exit.country_code),
                                        exit.country_code.to_uppercase(),
                                        exit.city_code,
                                        Self::numerify(&exit.hostname)
                                    ))
                                } else {
                                    RichText::new(format!(
                                        "{} {} / {}",
                                        to_flag(&exit.country_code),
                                        exit.country_code.to_uppercase(),
                                        exit.city_code
                                    ))
                                };

                                ui.label(left_text);
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        ui.add_space(4.0);
                                        ui.add(self.draw_tier_badge(exit));
                                        ui.add_space(4.0);
                                        ui.add(self.draw_load_badge(exit.load));
                                    },
                                );
                            });
                        });
                    });

                if response.clicked() && !is_disabled {
                    *self.selected = Some(exit.clone());
                }
            }

            ui.checkbox(&mut self.show_all, "Show all servers");
        });
    }

    fn short_exit_list(&self) -> Vec<ExitDescriptor> {
        let mut new_exits = Vec::new();
        let mut seen = HashSet::new();

        for exit in &self.exit_list {
            let key = format!("{}.{}", exit.country_code, exit.city_code);
            if !seen.contains(&key) {
                seen.insert(key);
                new_exits.push(exit.clone());
            }
        }

        new_exits
    }

    fn draw_tier_badge<'b>(&'b self, exit: &'b ExitDescriptor) -> impl egui::Widget + 'b {
        move |ui: &mut Ui| {
            const PLUS_BG_COLOR: Color32 = Color32::from_rgb(220, 220, 255);
            const FREE_BG_COLOR: Color32 = Color32::from_rgb(226, 255, 226);
            const TEXT_COLOR: Color32 = Color32::BLACK;

            let (text, text_color, bg_color) = if exit.allowed_levels.contains(&"free".to_string())
            {
                (
                    format!("{} Free", ICON_MONEY_OFF_CSRED),
                    Color32::BLACK,
                    FREE_BG_COLOR,
                )
            } else {
                if self.block_plus {
                    (
                        format!("{} Plus", ICON_ATTACH_MONEY),
                        utils::disable_color(TEXT_COLOR, DISABLE_MULTIPLY, DISABLE_OPACITY),
                        utils::disable_color(PLUS_BG_COLOR, DISABLE_MULTIPLY, DISABLE_OPACITY),
                    )
                } else {
                    (
                        format!("{} Plus", ICON_ATTACH_MONEY),
                        TEXT_COLOR,
                        PLUS_BG_COLOR,
                    )
                }
            };
            let badge = egui::Button::new(RichText::new(text).color(text_color).size(14.0))
                .fill(bg_color)
                .rounding(Rounding::same(14.0))
                .min_size(Vec2::new(60.0, 21.0))
                .frame(false);

            ui.add(badge)
        }
    }

    fn draw_load_badge(&self, load: f32) -> impl egui::Widget + '_ {
        move |ui: &mut Ui| {
            let bg_color = load_to_color(load);
            let text_color = Color32::WHITE;

            let text = format!(
                "{}{}%",
                ICON_INSERT_CHART_OUTLINED,
                (load * 100.0).round() as i32
            );

            let badge = egui::Button::new(RichText::new(text).color(text_color).size(14.0))
                .fill(bg_color)
                .rounding(Rounding::same(10.0))
                .min_size(Vec2::new(52.0, 21.0))
                .frame(false);

            ui.add(badge)
        }
    }

    fn numerify(s: &str) -> String {
        s.chars().filter(|c| c.is_numeric()).collect()
    }

    pub fn selected(&self) -> Option<&ExitDescriptor> {
        self.selected.as_ref()
    }
}

fn load_to_color(load: f32) -> Color32 {
    let capped_load = load.min(1.3);
    let hue = 150.0 - capped_load * 160.0;
    let rgb = hsl_to_rgb(hue, 0.8, 0.3);

    Color32::from_rgb(rgb.0, rgb.1, rgb.2)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn to_flag(country_code: &str) -> String {
    let country_code = country_code.to_uppercase();
    if country_code.len() != 2 {
        return String::from("Invalid country code");
    }

    let flag = country_code
        .chars()
        .map(|c| char::from_u32(c as u32 + 127397).unwrap_or(c))
        .collect::<String>();

    flag
}
