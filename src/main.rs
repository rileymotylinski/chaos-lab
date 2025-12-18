#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::logistic_map::iterative_logistic_map;
use std::process;

mod math;
mod integrators;
mod logistic_map;
mod rng;
mod tests;
mod lorenz;
mod double_pendulum;


use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

#[derive(Default)]
struct MyEguiApp {
    
    // lorenz
    pub sigma: f64,
    pub ro: f64,
    pub beta: f64

}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default();
        
        // state is stored int the struct; egui is stateless
        Self { sigma: 0.0, ro: 0.0, beta: 0.0 }
    }
}

impl eframe::App for MyEguiApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
           
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.add(egui::Slider::new(&mut self.sigma, 0.0..=100.0));
                ui.add(egui::Slider::new(&mut self.ro, 0.0..=100.0));
                ui.add(egui::Slider::new(&mut self.beta, 0.0..=100.0));
            });

            if ui.button("Run Simulation").clicked() {
                // lorenz attractor stuff
                // writing data
                if let Err(err) = crate::lorenz::lorenz(self.sigma, self.ro,self.beta) {
                    println!("{}", err);
                    process::exit(1);
                }

            }   
        });
    }
}

    

// bandaid catchall solution while I migrate to egui
fn update_points() {
    //let simulation = std::env::args().nth(1).expect("No Simulation was given");

    //println!("Simulation selected: {:?}", simulation);

    // logistic map stuff
    let _path = "./src/csv/test.csv";
    // generating data
    let _data = iterative_logistic_map(0.3,10,3.6, 0.0001,2000);

    if let Err(err) = crate::double_pendulum::double_pendulum() {
        println!("{}", err);
        process::exit(1);
    }

    
}

