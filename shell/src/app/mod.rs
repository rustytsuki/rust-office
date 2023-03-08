#[cfg(feature = "skia")]
mod custom_skia;

#[cfg(feature = "glow")]
mod custom_glow;

use eframe::egui;

#[cfg(feature = "egui_demo")]
use egui_demo_lib;

pub struct RustOfficeApp {
    // Example stuff:
    label: String,
    value: f32,

    #[cfg(feature = "egui_demo")]
    demo_windows: egui_demo_lib::DemoWindows,

    #[cfg(all(feature = "glow"))]
    custom_glow: Option<custom_glow::CustomGlow>,

    #[cfg(all(feature = "skia"))]
    custom_skia: Option<custom_skia::CustomSkia>,
}

impl RustOfficeApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        setup_custom_styles(&cc.egui_ctx);
        
        #[cfg(not(feature = "default_fonts"))]
        setup_custom_fonts(&cc.egui_ctx);

        let mut app = Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            
            #[cfg(feature = "egui_demo")]
            demo_windows: egui_demo_lib::DemoWindows::default(),

            #[cfg(all(feature = "glow"))]
            custom_glow: None,

            #[cfg(all(feature = "skia"))]
            custom_skia: None,
        };

        #[cfg(all(feature = "glow"))]
        if cc.gl.is_some() {
            app.custom_glow = Some(custom_glow::CustomGlow::new(cc.gl.as_ref().unwrap()));
        } else {
            #[cfg(all(feature = "skia"))] {
                app.custom_skia = Some(custom_skia::CustomSkia::new());
            }
        }
        #[cfg(all(not(feature = "glow"), feature = "skia"))] {
            app.custom_skia = Some(custom_skia::CustomSkia::new());
        }

        app
    }

    #[cfg(not(feature = "egui_demo"))]
    fn ui(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, value, .. } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}

impl eframe::App for RustOfficeApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(not(feature = "egui_demo"))]
        // self.ui(ctx, frame);

        #[cfg(feature = "egui_demo")]
        self.demo_windows.ui(ctx);

        #[cfg(feature = "skia")]
        if let Some(custom_skia) = self.custom_skia.as_mut() {
            custom_skia.update(ctx);
        }

        #[cfg(all(feature = "glow"))]
        if let Some(custom_glow) = self.custom_glow.as_mut() {
            custom_glow.update(ctx);
        }
    }

    #[cfg(feature = "glow")]
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Some(custom_glow) = self.custom_glow.as_mut() {
            custom_glow.on_exit(_gl);
        }
    }

    #[cfg(not(feature = "glow"))]
    fn on_exit(&mut self) {
        if let Some(custom_skia) = self.custom_skia.as_mut() {
            custom_skia.on_exit();
        }
    }
}

fn setup_custom_styles(ctx: &egui::Context) {
    let style = egui::Style {
        visuals: egui::Visuals::light(),
        // animation_time: 0.,
        ..egui::Style::default()
    };
    ctx.set_style(style);
}

#[cfg(not(feature = "default_fonts"))]
fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../../fonts/Allerta-Regular.ttf"
        )),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
