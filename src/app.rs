use eframe::NativeOptions;
use egui::{Color32, RichText};

#[derive(Default)]
pub struct SinonMonitorApp {}

impl SinonMonitorApp {
    pub fn run(options: NativeOptions) -> Result<(), eframe::Error> {
        eframe::run_native(
            "sinon-monitor",
            options,
            Box::new(|_cc| Ok(Box::<SinonMonitorApp>::default())),
        )
    }
}

impl eframe::App for SinonMonitorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.style_mut(|s| s.visuals.panel_fill = Color32::BLACK);

        let now = chrono::Local::now();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new(now.format("%H:%M").to_string()).size(400.));

            ui.label(crate::android::battery_level().to_string());

            if let Some(time) = frame.info().cpu_usage {
                ui.label(format!("{:.2}", time * 1000.));
            }
        });

        if let Some(pos) = ctx.pointer_hover_pos() {
            ctx.debug_painter()
                .circle_stroke(pos, 50., (1., Color32::RED));
        }
    }
}
