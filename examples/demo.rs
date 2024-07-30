use eframe::{App, Frame};

use egui::{self, Align, Color32, Layout, RichText, TextStyle, Ui, Visuals};
use egui::{vec2, CentralPanel, Context};
use egui_material_icons::icons::*;

use widgets::*;

fn main() {
    let options = eframe::NativeOptions::default();
    let mut cell: Option<Demo> = None;

    eframe::run_simple_native("Widgets", options, move |ctx, frame| {
        let app = cell.get_or_insert_with(|| Demo::new(ctx));
        app.update(ctx, frame)
    })
    .unwrap();
}

struct Demo {
    tab_idx: usize,
    switch_on: bool,
    settings_on: bool,
    selected: String,
}

impl Demo {
    fn new(ctx: &Context) -> Self {
        egui_material_icons::initialize(ctx);

        ctx.style_mut(|style| {
            style.spacing.item_spacing = vec2(8.0, 8.0);
            style.visuals = Visuals::light();
        });
        ctx.set_zoom_factor(1.1);

        Self {
            tab_idx: 0,
            switch_on: false,
            settings_on: false,
            selected: String::new(),
        }
    }
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add(TabBar::new(
                vec![
                    TabBarItem::new(String::from("Dashboard"), ICON_DASHBOARD.to_string()),
                    TabBarItem::new(String::from("Logs"), ICON_DESCRIPTION.to_string()),
                    TabBarItem::new(String::from("Settings"), ICON_SETTINGS.to_string()),
                ],
                &mut self.tab_idx,
            ));

            match self.tab_idx {
                0 => {
                    ui.add(Button::primary(
                        "Log in with existing account".to_string(),
                        ButtonSize::Large,
                    ));
                    ui.add(
                        Button::secondary("Disabled".to_string(), ButtonSize::Large).disable(true),
                    );
                    ui.add(Button::warning("Delete".to_string(), ButtonSize::Large));
                    ui.add(Button::black("Hello world".to_string(), ButtonSize::Small));
                    ui.add(Button::green("Connect".to_string(), ButtonSize::Large));

                    ui.add(
                        Button::primary("Buy Plus!".to_string(), ButtonSize::Large).invert(true),
                    );
                    ui.add(
                        Button::secondary("Hello world".to_string(), ButtonSize::Small)
                            .invert(true),
                    );
                    ui.add(Button::warning("Log out".to_string(), ButtonSize::Large).invert(true));
                    ui.add(
                        Button::black("Change location".to_string(), ButtonSize::Large)
                            .invert(true),
                    );
                    ui.add(
                        Button::green("Hello world".to_string(), ButtonSize::Small).invert(true),
                    );

                    ui.add(Switch::new(&mut self.switch_on));
                }
                1 => {
                    ui.add(Card::new(|ui| {
                        let text_size = 16.0;
                        ui.spacing_mut().item_spacing = vec2(0.0, 8.0);

                        ui.style_mut().override_text_style = Some(TextStyle::Body);
                        ui.style_mut().spacing.icon_width = 24.0;
                        ui.style_mut().spacing.icon_spacing = 8.0;

                        ui.horizontal(|ui| {
                            ui.label(RichText::new(ICON_DOWNLOAD).size(text_size));
                            ui.label(RichText::new(" Download").size(text_size));
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(
                                    RichText::new("- MB").size(text_size).color(Color32::BLUE),
                                );
                            });
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new(ICON_UPLOAD).size(text_size));
                            ui.label(RichText::new(" Upload").size(text_size));
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(
                                    RichText::new("- MB")
                                        .size(text_size)
                                        .color(Color32::DARK_RED),
                                );
                            });
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new(ICON_SWAP_VERT).size(text_size));
                            ui.label(RichText::new(" Latency").size(text_size));
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(RichText::new("- ms").size(text_size));
                            });
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new(ICON_COMPUTER).size(text_size));
                            ui.label(RichText::new(" Via").size(text_size));
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(RichText::new("-").size(text_size));
                            });
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new(ICON_RADIO).size(text_size));
                            ui.label(RichText::new(" Protocol").size(text_size));
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.label(RichText::new("-").size(text_size));
                            });
                        });
                    }));
                }
                2 => {
                    ui.add(Card::new(|ui| {
                        ui.add(SettingsLine::new(
                            ICON_LAN.to_string(),
                            String::from("Listen on all interfaces"),
                            Box::new(|ui: &mut Ui| ui.add(Switch::new(&mut self.settings_on))),
                        ));

                        let options = vec![
                            "English".to_string(),
                            "Traditional Chinese".to_string(),
                            "Simplified Chinese".to_string(),
                            "Persian".to_string(),
                            "Svitannski".to_string(),
                        ];

                        ui.add(SettingsLine::new(
                            ICON_LANGUAGE.to_string(),
                            "Language".to_string(),
                            Box::new(|ui: &mut Ui| {
                                ui.add(Dropdown::new(
                                    "my_dropdown",
                                    options.clone(),
                                    &mut self.selected,
                                ))
                            }),
                        ));
                    }));
                }
                _ => (),
            }
        });
    }
}
