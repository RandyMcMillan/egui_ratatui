use eframe::{egui, NativeOptions};

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Window Size Example",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

struct MyApp {
    window_size: egui::Vec2,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            window_size: egui::Vec2::ZERO,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.window_size = ctx.available_rect().size(); // Get window size here.
            ui.label(format!(
                "({:?},{:?})",
                self.window_size.x, self.window_size.y
            ));
        });
    }
}
