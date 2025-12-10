mod android;

mod app;
pub use app::SinonMonitorApp;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: egui_winit::winit::platform::android::activity::AndroidApp) {
    use eframe::NativeOptions;
    use eframe::Renderer;
    use egui_winit::winit::platform::android::activity::WindowManagerFlags;

    unsafe {
        std::env::set_var("RUST_BACKTRACE", "full");
    }

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    app.set_window_flags(WindowManagerFlags::FULLSCREEN, WindowManagerFlags::empty());

    let options = NativeOptions {
        android_app: Some(app),
        renderer: Renderer::Wgpu,
        ..Default::default()
    };
    SinonMonitorApp::run(options).unwrap();
}
