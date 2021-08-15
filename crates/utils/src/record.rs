use nannou::prelude::*;
use once_cell::sync::Lazy;
static RECORDING: Lazy<bool> = Lazy::new(|| {
    let args: Vec<String> = std::env::args().collect();
    args.len() > 1 && args[1] == "-record"
});

pub fn record(app: &App, frame: &Frame) {
    if !*RECORDING {
        return;
    }

    // save frame
    let path = app
        .project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join("recordings")
        .join(app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png");

    println!("frame: {} {:.3}", frame.nth(), frame.nth() as f32 / 60.0);
    app.main_window().capture_frame(path);
}
