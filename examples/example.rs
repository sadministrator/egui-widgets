use eframe::App;
use egui::{self, Layout, Ui, Visuals};
use egui_material_icons::icons::*;

use widgets::*;

fn main() {
    let mut switch_on = false;
    let mut settings_on = false;
    let options = eframe::NativeOptions::default();
    let mut cell: Option<ExampleApp> = None;

    eframe::run_simple_native("Widgets", options, move |ctx, frame| {
        let app =
            cell.get_or_insert_with(|| ExampleApp::new(ctx, &mut switch_on, &mut settings_on));
        app.update(ctx, frame)
    })
    .unwrap();
}

struct ExampleApp {
    tab_idx: usize,
    switch_on: bool,
    settings_on: bool,
}

impl ExampleApp {
    fn new(ctx: &egui::Context, switch_on: &mut bool, settings_on: &mut bool) -> Self {
        egui_material_icons::initialize(ctx);

        ctx.style_mut(|style| {
            style.spacing.item_spacing = egui::vec2(8.0, 8.0);
            style.visuals = Visuals::light();
        });
        ctx.set_zoom_factor(1.1);

        Self {
            tab_idx: 0,
            switch_on: *switch_on,
            settings_on: *settings_on,
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
                    ui.add(GephButton::primary(
                        "Log in with existing account".to_string(),
                        ButtonSize::Large,
                    ));
                    ui.add(GephButton::secondary(
                        "Hello world".to_string(),
                        ButtonSize::Small,
                    ));
                    ui.add(GephButton::warning("Delete".to_string(), ButtonSize::Large));
                    ui.add(GephButton::black(
                        "Hello world".to_string(),
                        ButtonSize::Small,
                    ));
                    ui.add(GephButton::green("Connect".to_string(), ButtonSize::Large));

                    ui.add(
                        GephButton::primary("Buy Plus!".to_string(), ButtonSize::Large)
                            .invert(true),
                    );
                    ui.add(
                        GephButton::secondary("Hello world".to_string(), ButtonSize::Small)
                            .invert(true),
                    );
                    ui.add(
                        GephButton::warning("Log out".to_string(), ButtonSize::Large).invert(true),
                    );
                    ui.add(
                        GephButton::black("Change location".to_string(), ButtonSize::Large)
                            .invert(true),
                    );
                    ui.add(
                        GephButton::green("Hello world".to_string(), ButtonSize::Small)
                            .invert(true),
                    );

                    ui.add(Switch::new(&mut self.switch_on));
                }
                1 => {
                    ui.add(Card::new(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(0.0, 8.0);

                        ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                        ui.style_mut().spacing.icon_width = 24.0;
                        ui.style_mut().spacing.icon_spacing = 8.0;

                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(ICON_DOWNLOAD).size(20.0));
                            ui.label(" Download");
                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new("- MB").color(egui::Color32::GRAY));
                            });
                        });

                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(ICON_UPLOAD).size(20.0));
                            ui.label(" Upload");
                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new("- MB").color(egui::Color32::GRAY));
                            });
                        });

                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(ICON_SWAP_VERT).size(20.0));
                            ui.label(" Latency");
                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new("- ms").color(egui::Color32::GRAY));
                            });
                        });

                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(ICON_COMPUTER).size(20.0));
                            ui.label(" Via");
                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new("-").color(egui::Color32::GRAY));
                            });
                        });

                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(ICON_RADIO).size(20.0));
                            ui.label(" Protocol");
                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(egui::RichText::new("-").color(egui::Color32::GRAY));
                            });
                        });
                    }));
                }
                2 => {
                    ui.add(Card::new(|ui| {
                        ui.add(SettingsLine::new(
                            ICON_LAN.to_string(),
                            String::from("Listen on all interfaces"),
                            Box::new(|ui: &mut Ui| {
                                ui.add(Switch::new(&mut self.settings_on));
                            }),
                        ));
                    }));
                }
                _ => (),
            }
        });
    }
}
