use eframe::{App, Frame};

use egui::{self, Align, Color32, FontDefinitions, Layout, RichText, TextStyle, Ui, Visuals};
use egui::{vec2, CentralPanel, Context};
use egui_material_icons::icons::*;

use widgets::*;

#[derive(Clone, PartialEq)]
enum TabName {
    Dashboard,
    Logs,
    Settings,
    ExitSelect,
}

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
    tab: TabName,
    switch_on: bool,
    settings_on: bool,
    selected: String,
    exit_selector_show_all: bool,
    selected_exit: Option<ExitDescriptor>,
}

impl Demo {
    fn new(ctx: &Context) -> Self {
        let mut fonts = FontDefinitions::default();
        insert_icon_font(&mut fonts);
        ctx.set_fonts(fonts);

        ctx.style_mut(|style| {
            style.spacing.item_spacing = vec2(8.0, 8.0);
            style.visuals = Visuals::light();
        });
        ctx.set_zoom_factor(1.1);

        Self {
            tab: TabName::Dashboard,
            switch_on: false,
            settings_on: false,
            selected: String::new(),
            exit_selector_show_all: false,
            selected_exit: None,
        }
    }
}

impl eframe::App for Demo {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let frame = egui::Frame {
            inner_margin: egui::Margin::same(16.0),
            fill: ctx.style().visuals.panel_fill,
            ..Default::default()
        };
        CentralPanel::default().frame(frame).show(ctx, |ui| {
            TabBar::new(
                "tab_bar",
                vec![
                    TabBarItem::new(
                        TabName::Dashboard,
                        String::from("Dashboard"),
                        ICON_DASHBOARD.to_string(),
                    ),
                    TabBarItem::new(
                        TabName::Logs,
                        String::from("Logs"),
                        ICON_DESCRIPTION.to_string(),
                    ),
                    TabBarItem::new(
                        TabName::Settings,
                        String::from("Settings"),
                        ICON_SETTINGS.to_string(),
                    ),
                    TabBarItem::new(
                        TabName::ExitSelect,
                        String::from("Exit Select"),
                        ICON_LOCATION_PIN.to_string(),
                    ),
                ],
                64.0,
            )
            .show(ctx, &mut self.tab);

            match self.tab {
                TabName::Dashboard => {
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
                TabName::Logs => {
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
                TabName::Settings => {
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
                TabName::ExitSelect => {
                    let exits = vec![
                        ExitDescriptor::new(
                            "sg-sgp-04.exits.geph.io".to_owned(),
                            "sg".to_owned(),
                            "sgp".to_owned(),
                            0.1,
                            vec!["plus".to_owned()],
                        ),
                        ExitDescriptor::new(
                            "us-hio-03.exits.geph.io".to_owned(),
                            "us".to_owned(),
                            "pdx".to_owned(),
                            0.78,
                            vec!["free".to_owned(), "plus".to_owned()],
                        ),
                        ExitDescriptor::new(
                            "us-hio-04.exits.geph.io".to_owned(),
                            "us".to_owned(),
                            "pdx".to_owned(),
                            0.78,
                            vec!["free".to_owned(), "plus".to_owned()],
                        ),
                        ExitDescriptor::new(
                            "us-hio-04.exits.geph.io".to_owned(),
                            "us".to_owned(),
                            "pdx".to_owned(),
                            0.78,
                            vec!["free".to_owned(), "plus".to_owned()],
                        ),
                        ExitDescriptor::new(
                            "us-hio-05.exits.geph.io".to_owned(),
                            "us".to_owned(),
                            "pdx".to_owned(),
                            0.78,
                            vec!["free".to_owned(), "plus".to_owned()],
                        ),
                        ExitDescriptor::new(
                            "us-hio-06.exits.geph.io".to_owned(),
                            "us".to_owned(),
                            "pdx".to_owned(),
                            0.78,
                            vec!["free".to_owned(), "plus".to_owned()],
                        ),
                        ExitDescriptor::new(
                            "us-hio-03.exits.geph.io".to_owned(),
                            "us".to_owned(),
                            "pdx".to_owned(),
                            0.99,
                            vec!["free".to_owned(), "plus".to_owned()],
                        ),
                    ];
                    let block_plus = true;
                    let mut exit_server_list = ExitSelector::new(
                        exits,
                        block_plus,
                        &mut self.exit_selector_show_all,
                        &mut self.selected_exit,
                    );
                    exit_server_list.show(ui);
                }
            }
        });
    }
}
