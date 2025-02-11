use std::sync::Arc;

use egui::{text::LayoutJob, Color32, Context, FontFamily, FontId, TextEdit, TextStyle, Theme, Ui};

const FONT_PATH:&'static str = "../../tiny_font/hei.TTF";
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TourApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    font_size: f32,
}

impl Default for TourApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            font_size: 32.0,
        }
    }
}

fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert("my_font".to_owned(),
    Arc::new(egui::FontData::from_static(include_bytes!("../../tiny_font/hei.TTF"))));

    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, "my_font".to_owned());

    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
        .push("my_font".to_owned());

    ctx.set_fonts(fonts);
}


// Demonstrates how to replace all fonts.
// fn replace_fonts(ctx: &egui::Context) {
//     // Start with the default fonts (we will be adding to them rather than replacing them).
//     let mut fonts = egui::FontDefinitions::default();

//     // Install my own font (maybe supporting non-latin characters).
//     // .ttf and .otf files supported.
//     fonts.font_data.insert(
//         "my_font".to_owned(),
//         std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
//             "E:/github/yan4rust/fonts/msyhl.ttc"
//         ))),
//     );

//     // Put my font first (highest priority) for proportional text:
//     fonts
//         .families
//         .entry(egui::FontFamily::Proportional)
//         .or_default()
//         .insert(0, "my_font".to_owned());

//     // Put my font as last fallback for monospace:
//     fonts
//         .families
//         .entry(egui::FontFamily::Monospace)
//         .or_default()
//         .push("my_font".to_owned());

//     // Tell egui to use these fonts:
//     ctx.set_fonts(fonts);
// }

// fn set_font_size(ctx: &Context,size: f32) {
//     let fid = FontId::proportional(size);
//     ctx.style_mut(|style|{
//         style.override_font_id.replace(fid);
//     })
// }
fn set_style(ctx: & Context) {
    let mut style = (*ctx.style()).clone();
    // for (k,v) in style.text_styles.iter() {
    //     println!("{} , size: {}",k,v.size);
    // }
    style.text_styles = [
        (TextStyle::Heading, FontId::new(24.0, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(16.0, FontFamily::Proportional)),
        (TextStyle::Monospace, FontId::new(16.0, FontFamily::Monospace)),
        (TextStyle::Button, FontId::new(16.0, FontFamily::Proportional)),
        (TextStyle::Small, FontId::new(12.0, FontFamily::Proportional)),
    ]
    .into();
    ctx.set_style(style);

    // ctx.vi(|style|{
    //     TextStyle::Body
    //     style.override_text_style
    // });
}
fn set_font_color(ui:&mut Ui) {
    let theme = ui.ctx().theme();
    let color = match theme {
        Theme::Dark =>{
            Color32::from_rgb(255, 255, 255)
        }
        Theme::Light=>{
            Color32::from_rgb(0, 0, 0)
        }
    };
    ui.visuals_mut().override_text_color.replace(color);
}
impl TourApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        load_fonts(&cc.egui_ctx);
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for TourApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // set_style(ctx);
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            // set_font_color(ui);
            egui::menu::bar(ui, |ui| {
                // set_font_color(ui);
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(14.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // set_font_color(ui);
            // The central panel the region left after adding TopPanel's and SidePanel's
            // ui.heading("eframe template");
            ui.heading("EGUI Tour");

            ui.horizontal(|ui| {
                ui.label("Write something: ");

                // @note TextEdit layout, set letter spacing for chinese
                let mut layouter = |ui: &egui::Ui, txt: &str, wrap_width: f32| {
                    let mut job = LayoutJob::single_section(
                        txt.to_owned(),
                        egui::TextFormat {
                            extra_letter_spacing:2.0,
                            ..Default::default()
                        },
                    );
                    job.wrap.max_width = wrap_width;
                    ui.fonts(|f| f.layout_job(job))
                };
                let edit = TextEdit::singleline(&mut self.label).layouter(&mut layouter);
                //ui.text_edit_singleline(&mut self.label);
                ui.add(edit);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
