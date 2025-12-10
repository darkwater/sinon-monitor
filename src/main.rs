use eframe::NativeOptions;
use sinon_monitor::SinonMonitorApp;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();
    SinonMonitorApp::run(options)
}
