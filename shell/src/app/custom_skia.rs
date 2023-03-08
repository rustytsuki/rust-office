use std::sync::Arc;
use eframe::egui;
use eframe::{EguiSkiaPaintCallback, skia_safe::{Point, Paint, self, font_style::{Weight, Width, Slant}, Color4f}};

pub struct CustomSkia {

}

impl CustomSkia {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        egui::Window::new("Draw to skia").show(ctx, |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                let (rect, _) = ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());
                ui.painter().add(egui::PaintCallback {
                    rect: rect.clone(),
                    callback: Arc::new(EguiSkiaPaintCallback::new(move |canvas, physical_rect, pixels_per_point| {
                        let mut paint = Paint::default();
                        
                        paint.set_color4f(Color4f::new(0.1, 0.1, 0.1, 1.), None);
                        canvas.draw_rect(physical_rect, &paint);
    
                        canvas.translate((physical_rect.left, physical_rect.top));
    
                        paint.set_color4f(Color4f::new(1., 0., 0., 1.), None);
    
                        let typeface = skia_safe::Typeface::default();
                        let font = skia_safe::Font::new(typeface, 25.0 * pixels_per_point);
                        let blob = skia_safe::TextBlob::from_str("hello, world!", &font).unwrap();
                        canvas.draw_text_blob(&blob, (0. * pixels_per_point, 50. * pixels_per_point), &paint);
                    })),
                })
            });
        });
    }

    pub fn on_exit(&mut self) {

    }
}
