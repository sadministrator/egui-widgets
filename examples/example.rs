use eframe::App;
use egui::{self, Visuals};

use widgets::*;

fn main() {
    // let ctx: egui::Context = egui::Context::default();
    let options = eframe::NativeOptions::default();

    eframe::run_simple_native("Widgets", options, move |ctx, frame| {
        ExampleApp::new(ctx).update(ctx, frame)
    })
    .unwrap();
}

struct ExampleApp {}

impl ExampleApp {
    fn new(ctx: &egui::Context) -> Self {
        // egui_extras::install_image_loaders(ctx);
        ctx.style_mut(|style| {
            // style.spacing.item_spacing = egui::vec2(8.0, 8.0);
            style.visuals = Visuals::light();
        });
        ctx.set_zoom_factor(1.1);

        Self {}
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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

            ui.add(GephButton::primary("Buy Plus!".to_string(), ButtonSize::Large).invert(true));
            ui.add(
                GephButton::secondary("Hello world".to_string(), ButtonSize::Small).invert(true),
            );
            ui.add(GephButton::warning("Log out".to_string(), ButtonSize::Large).invert(true));
            ui.add(
                GephButton::black("Change location".to_string(), ButtonSize::Large).invert(true),
            );
            ui.add(GephButton::green("Hello world".to_string(), ButtonSize::Small).invert(true));
        });
    }
}
