use std::sync::Arc;

#[cfg(all(feature = "skia", not(target_arch = "wasm32")))]
use eframe::skia_safe::{self, Surface, ImageInfo, ColorType, AlphaType, Paint, Color4f, font_style::{Weight, Width, Slant}};

use eframe::{glow, epaint::mutex::Mutex, egui, egui_glow};
struct RotatingTriangle {
    program: glow::Program,
    vertex_array: glow::VertexArray,
}

impl RotatingTriangle {
    fn new(gl: &glow::Context) -> Self {
        use glow::HasContext as _;

        let shader_version = if cfg!(target_arch = "wasm32") {
            "#version 300 es"
        } else {
            "#version 330"
        };

        unsafe {
            let program = gl.create_program().expect("Cannot create program");
            let shader_sources = [
                (glow::VERTEX_SHADER, include_str!("./custom_glow_vs.glsl")),
                (glow::FRAGMENT_SHADER, include_str!("./custom_glow_ps.glsl")),
            ];

            let shaders: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, shader_source)| {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("Cannot create shader");
                    gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
                    gl.compile_shader(shader);
                    if !gl.get_shader_compile_status(shader) {
                        panic!("{}", gl.get_shader_info_log(shader));
                    }
                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            Self {
                program,
                vertex_array,
            }
        }
    }

    fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }

    fn paint(&self, gl: &glow::Context, angle: f32) {
        use glow::HasContext as _;
        unsafe {
            gl.use_program(Some(self.program));

            let texture = gl.create_texture().ok();
            gl.pixel_store_i32(glow::PACK_ALIGNMENT, 1);
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, texture);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);

            #[cfg(all(feature = "skia", not(target_arch = "wasm32")))]
            Self::set_texture_native(gl);

            #[cfg(target_arch = "wasm32")]
            Self::set_texture_web(gl);

            let u_sampler = gl.get_uniform_location(self.program, "u_sampler");
            gl.uniform_1_i32(u_sampler.as_ref(), 0);
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
    
    #[cfg(all(feature = "skia", not(target_arch = "wasm32")))]
    fn set_texture_native(gl: &glow::Context) {
        let image_info = ImageInfo::new((600, 600), ColorType::RGBA8888, AlphaType::Premul, None);
        let mut surface = Surface::new_raster(&image_info, None, None).unwrap();
        let canvas = surface.canvas();

        let mut paint = Paint::default();
        paint.set_color4f(Color4f::new(1., 0., 0., 1.), None);
        let typeface = skia_safe::Typeface::default();
        let font = skia_safe::Font::new(typeface, 50.);
        let blob = skia_safe::TextBlob::from_str("Hello, world!", &font).unwrap();
        canvas.draw_text_blob(&blob, (0., 50.), &paint);

        let pixels = surface.peek_pixels().unwrap();
        use glow::HasContext as _;
        unsafe {
            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, 600, 600, 0, glow::RGBA, glow::UNSIGNED_BYTE, pixels.bytes());
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn set_texture_web(gl: &glow::Context) {
        use wasm_bindgen::{JsCast, JsValue};
        use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};
        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas: HtmlCanvasElement = document.create_element("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
        canvas.set_width(600);
        canvas.set_height(600);
        let ctx: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
        ctx.set_font("bold italic 50px Arial");
        ctx.set_fill_style(&JsValue::from("#FF0000"));
        ctx.fill_text("Hello, world!", 0., 50.).unwrap();
        
        use glow::HasContext as _;
        unsafe {
            gl.tex_image_2d_with_html_canvas(glow::TEXTURE_2D, 0, glow::RGBA as i32, glow::RGBA, glow::UNSIGNED_BYTE, &canvas);
        }

        canvas.remove();
    }
}


pub struct CustomGlow {
    rotating_triangle: Arc<Mutex<RotatingTriangle>>,
    angle: f32,
}

impl CustomGlow {
    pub fn new(gl: &glow::Context) -> Self {
        Self {
            rotating_triangle: Arc::new(Mutex::new(RotatingTriangle::new(gl))),
            angle: 0.0,
        }
    }

    pub fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.rotating_triangle.lock().destroy(gl);
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        egui::Window::new("Draw with OpenGL").show(ctx, |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                let (rect, response) =
                ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());
    
                self.angle += response.drag_delta().x * 0.01;
        
                // Clone locals so we can move them into the paint callback:
                let angle = self.angle;
                let rotating_triangle = self.rotating_triangle.clone();
        
                let callback = egui::PaintCallback {
                    rect,
                    callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                        rotating_triangle.lock().paint(painter.gl(), angle);
                    })),
                };
                ui.painter().add(callback);
            });
        });
    }
}
