//! This module provides the `RataguiBackend` implementation for the [`Backend`] trait.
//! It is used in the integration tests to verify the correctness of the library.

use egui::{ColorImage, Response, TextureHandle, TextureOptions, Ui, Vec2};

use ratatui::layout::Position;
use soft_ratatui::SoftBackend;

use std::io;

use ratatui::{
    backend::{Backend, WindowSize},
    buffer::Cell,
    layout::Size,
};

//use egui::Label as TerminalLine;

///The RataguiBackend is the widget+backend itself , from which you can make a ratatui terminal ,
/// then you can do ui.add(terminal.backend_mut()) inside an egui context
/// Spawn with RataguiBackend::new() or RataguiBackend::new_with_system_fonts()
///
/// For more documentation see the soft_ratatui crate
/// https://github.com/gold-silver-copper/soft_ratatui
/// https://docs.rs/soft_ratatui/latest/soft_ratatui/
///

pub struct RataguiBackend {
    soft_backend: SoftBackend,
    cur_size: Vec2,
    name: String,
    text_handle: Option<TextureHandle>,
}
impl egui::Widget for &mut RataguiBackend {
    fn ui(self, ui: &mut Ui) -> Response {
        let av_size = ui.available_size();

        if self.cur_size != av_size {
            self.cur_size = av_size;

            let av_width = (av_size.x).clamp(1.0, 10000.0);
            let av_height = (av_size.y).clamp(1.0, 10000.0);
            let available_chars_width = (av_width / (self.soft_backend.char_width as f32)) as u16;

            let available_chars_height =
                (av_height / (self.soft_backend.char_height as f32)) as u16;
            let cur_size = self.size().expect("COULD NOT GET CURRENT BACKEND SIZE");
            if (cur_size.width != available_chars_width)
                || (cur_size.height != available_chars_height)
            {
                self.soft_backend
                    .resize(available_chars_width, available_chars_height);
            }
        }

        let texture =
            ui.ctx()
                .load_texture(&self.name, self.to_egui_image(), TextureOptions::NEAREST);
        self.text_handle = Some(texture.clone());

        ui.image((texture.id(), texture.size_vec2()))
    }
}

impl RataguiBackend {
    /// Creates a new `RataguiBackend` with the specified name,font size and font data. WASM compatible.
    pub fn new(name: &str, font_size: u16, font_data: &[u8]) -> Self {
        let backend = SoftBackend::new_with_font(10, 10, font_size as i32, font_data);
        let name = name.to_string();

        Self {
            soft_backend: backend,
            cur_size: Vec2::new(1.0, 1.0),
            name,

            text_handle: None,
        }
    }
    /// Creates a new `RataguiBackend` with the specified name, font size and using system fonts, this is not Web WASM compatible.
    pub fn new_with_system_fonts(name: &str, font_size: u16) -> Self {
        let backend = SoftBackend::new_with_system_fonts(10, 10, font_size as i32);
        let name = name.to_string();
        Self {
            soft_backend: backend,
            cur_size: Vec2::new(1.0, 1.0),
            name,

            text_handle: None,
        }
    }

    ///Creates an Egui ColorImage from the terminal buffer.
    pub fn to_egui_image(&self) -> ColorImage {
        egui::ColorImage::from_rgb(
            [
                self.soft_backend.get_pixmap_width(),
                self.soft_backend.get_pixmap_height(),
            ],
            self.soft_backend.get_pixmap_data(),
        )
    }
}

impl Backend for RataguiBackend {
    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        self.soft_backend.draw(content)
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        self.soft_backend.hide_cursor()
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        self.soft_backend.show_cursor()
    }

    fn get_cursor_position(&mut self) -> io::Result<Position> {
        self.soft_backend.get_cursor_position()
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, position: P) -> io::Result<()> {
        self.soft_backend.set_cursor_position(position)
    }

    fn clear(&mut self) -> io::Result<()> {
        self.soft_backend.clear()
    }

    fn size(&self) -> io::Result<Size> {
        self.soft_backend.size()
    }

    fn window_size(&mut self) -> io::Result<WindowSize> {
        self.soft_backend.window_size()
    }

    fn flush(&mut self) -> io::Result<()> {
        self.soft_backend.flush()
    }
}
