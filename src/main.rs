use crate::logistic_map::{iterative_logistic_map, write_logistic_data};
use std::process;

mod math;
mod integrators;
mod logistic_map;
mod rng;
mod tests;
mod lorenz;
mod double_pendulum;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       });
   }
}
    

// bandaid catchall solution while I migrate to egui
fn update_points() {
    //let simulation = std::env::args().nth(1).expect("No Simulation was given");

    //println!("Simulation selected: {:?}", simulation);

    // logistic map stuff
    let path = "./src/csv/test.csv";
    // generating data
    let data = iterative_logistic_map(0.3,10,3.6, 0.0001,2000);

    if let Err(err) = crate::double_pendulum::double_pendulum() {
        println!("{}", err);
        process::exit(1);
    }

    // lorenz attractor stuff
    // writing data
    if let Err(err) = crate::lorenz::lorenz() {
        println!("{}", err);
        process::exit(1);
    }

}

