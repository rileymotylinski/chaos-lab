#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui_plot::{Line, Plot, PlotPoints};



use crate::{lorenz::Lorenz};

mod math;
mod integrators;
mod logistic_map;
mod rng;
mod tests;
mod lorenz;
mod double_pendulum;
mod lyapunov;
mod dynamical_system;



use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

#[derive(Debug)]
enum Simulation {
    Lorenz,
    Dp,
}

impl std::default::Default for Simulation {
    fn default() -> Self {
        Simulation::Lorenz
    }
}

impl PartialEq for Simulation {
    fn eq(&self, other: &Simulation) -> bool {
        // matching a tuple of enums
        // will need to update later
        match (self,other) {
            (Simulation::Lorenz, Simulation::Lorenz) => true,
            (Simulation::Dp, Simulation::Dp) => true,
            _ => false
        }
    }
}

impl ToString for Simulation {
    fn to_string(&self) -> String {
        match self {
            Simulation::Lorenz => String::from("lorenz"),
            Simulation::Dp => String::from("double pendulum")
        }
    }
}

#[derive(Default)]
struct MyEguiApp {
    
    pub simulation: Simulation,

    // lorenz
    pub lorenz_system: Lorenz,
    pub lorenz_points: Vec<[f64; 3]>,
    pub lorenz_state: [f64; 3]
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default();
        
        // state is stored int the struct; egui is stateless
        Self {  
            simulation: Simulation::Lorenz,
            lorenz_system: Default::default(),
            lorenz_points: Vec::new(),
            lorenz_state: [1.0,1.0,1.0]
        }
    }


}

impl eframe::App for MyEguiApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            egui::ComboBox::from_label("Select one!")
                .selected_text(self.simulation.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.simulation, Simulation::Lorenz, Simulation::Lorenz.to_string());
                    ui.selectable_value(&mut self.simulation, Simulation::Dp, Simulation::Dp.to_string());
                }
            );

            if self.simulation == Simulation::Lorenz {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.add(egui::Slider::new(&mut self.lorenz_system.ro, 0.0..=100.0));
                    ui.add(egui::Slider::new(&mut self.lorenz_system.sigma, 0.0..=100.0));
                    ui.add(egui::Slider::new(&mut self.lorenz_system.beta, 0.0..=100.0));
                });

                let points: PlotPoints = self.lorenz_points.iter().map(| i | {
                    println!("{}", i[0]);
                    [i[0],i[1]]
                }).collect();
                let line = Line::new("sin", points);
                Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));

                // maybe change t, dt to state variables? not really sure.
                crate::integrators::rk4_step(&self.lorenz_system, &mut self.lorenz_state, 0.0, 0.1);
                self.lorenz_points.push([self.lorenz_state[0], self.lorenz_state[1], self.lorenz_state[2]]);
                
            } else { 
                ui.heading("This is a double pendulum");

                // TODO: add double pendulum interaction
            }
            

             
        });
    }
}


