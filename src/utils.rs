use egui::{Color32, FontData, FontDefinitions, FontFamily};
use egui_material_icons::FONT_DATA;

pub fn insert_icon_font(fonts: &mut FontDefinitions) {
    let material_icon_data = FontData::from_static(FONT_DATA);

    fonts
        .font_data
        .insert("material-icons".to_owned(), material_icon_data);
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .push("material-icons".to_owned());
}

pub fn disable_color(color: Color32, multiply: f32, opacity: f32) -> Color32 {
    increase_opacity(color.linear_multiply(multiply), opacity)
}

fn increase_opacity(color: Color32, increase_percentage: f32) -> Color32 {
    let new_alpha = (color.a() as f32 * (1.0 + increase_percentage)).min(255.0) as u8;
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), new_alpha)
}
