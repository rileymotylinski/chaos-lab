#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui_plot::{Line, Plot, PlotPoints};
use egui::*;



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


struct MyEguiApp {
    
    pub simulation: Simulation,
    pub is_playing: bool,

    // lorenz
    pub lorenz_system: Lorenz,
    pub lorenz_points: Vec<[f64; 2]>,
    pub lorenz_state: [f64; 3]
}

impl Default for MyEguiApp {
    fn default() -> Self {
        MyEguiApp {
            simulation: Default::default(),
            is_playing: false,

            lorenz_system: Default::default(),
            lorenz_points: vec![],
            lorenz_state: [1.0,1.0,1.0]
        }
    }
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
            is_playing: false,
            lorenz_system: Default::default(),
            lorenz_points: Vec::new(),
            lorenz_state: [1.0,1.0,1.0]
        }
    }


}

impl eframe::App for MyEguiApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                // play / pause shape button
                // where to put the widget
                
                ui.horizontal(|ui| {
                    // setting size of widget
                    let size = egui::vec2(36.0, 36.0);
                    // defining shape and behavior of button
                    // rect is shape, response is current state of the widget
                    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
                    if response.clicked() {
                        self.is_playing = !self.is_playing;
                    }
                    // object for drawing stuff on gui
                    let painter = ui.painter();
                    // defining some behavior for the background
                    let bg = if response.hovered() {
                        ui.visuals().widgets.hovered.bg_fill
                    } else {
                        ui.visuals().widgets.inactive.bg_fill
                    };
                    // draw the rectangle
                    painter.rect_filled(rect, 4.0, bg);
                    
                    // defining inner rectanble where we draw the play/pause
                    let inner = rect.shrink2(egui::vec2(8.0,4.0));
                    let color = ui.visuals().text_color();
                    let stroke = egui::Stroke::new(1.0, color);

                    if self.is_playing {
                        // draw pause (two bars)
                        let w = inner.width();
                        let h = inner.height();
                        let bar_w = w * 0.4;
                        let gap = w * 0.25;
                        let left_rect = egui::Rect::from_min_size(inner.min, egui::vec2(bar_w, h));
                        let right_rect = egui::Rect::from_min_size(egui::pos2(inner.min.x + bar_w + gap, inner.min.y), egui::vec2(bar_w, h));
                        painter.rect_filled(left_rect, 1.0, color);
                        painter.rect_filled(right_rect, 1.0, color);
                    } else {
                        // draw play (triangle)
                        let p1 = egui::pos2(inner.left(), inner.top());
                        let p2 = egui::pos2(inner.left(), inner.bottom());
                        let p3 = egui::pos2(inner.right(), inner.center().y);
                        painter.add(egui::Shape::convex_polygon(vec![p1, p2, p3], color, stroke));
                    }
                });

                ui.horizontal(|ui| {
                    // setting size of widget
                    let size = egui::vec2(36.0, 36.0);
                    // defining shape and behavior of button
                    // rect is shape, response is current state of the widget
                    let (rect, response) = ui.allocate_at_least(size, egui::Sense::click());
                    
                    if response.clicked() {
                        self.lorenz_state = MyEguiApp::default().lorenz_state;
                        self.lorenz_points = vec![];
                    }

                    // object for drawing stuff on gui
                    let painter = ui.painter();
                    // defining some behavior for the background
                    let bg = if response.hovered() {
                        ui.visuals().widgets.hovered.bg_fill
                    } else {
                        ui.visuals().widgets.inactive.bg_fill
                    };

                    let color = ui.visuals().text_color();
                    
                   
                    painter.rect_filled(rect, 4.0, bg);
                    painter.circle_filled(rect.center(), 13.0, color);
                    painter.circle_filled(rect.center(), 8.0, bg);
                    painter.arrow(rect.center() + egui::vec2(8.0, -4.5), egui::vec2(9.0,9.0), egui::Stroke::new(2.0,color));
                    
                    
                    

                
                });
                
                egui::ComboBox::from_label("Select one!")
                .selected_text(self.simulation.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.simulation, Simulation::Lorenz, Simulation::Lorenz.to_string());
                    ui.selectable_value(&mut self.simulation, Simulation::Dp, Simulation::Dp.to_string());
                });
                
            });
            
            
        
            
            

            if self.simulation == Simulation::Lorenz {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.add(egui::Slider::new(&mut self.lorenz_system.ro, 0.0..=100.0));
                    ui.add(egui::Slider::new(&mut self.lorenz_system.sigma, 0.0..=100.0));
                    ui.add(egui::Slider::new(&mut self.lorenz_system.beta, 0.0..=100.0));
                });
                
                
                if self.is_playing {
                    // maybe change t, dt to state variables? not really sure.
                    crate::integrators::rk4_step(&self.lorenz_system, &mut self.lorenz_state, 0.0, 0.01);
                    // discard z value or self.lorenz_state[2]
                    self.lorenz_points.push([self.lorenz_state[0], self.lorenz_state[1]]);
                    
                } 

                let points: PlotPoints = self.lorenz_points.iter().map(|i| {
                        [i[0],i[1]]
                }).collect();

                let line = Line::new("sin", points);
                Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));
            
                
                
            } else { 
                ui.heading("This is a double pendulum");

                // TODO: add double pendulum interaction
            }
        });
        // update every 32ms, regardless of user input
        ctx.request_repaint_after(std::time::Duration::from_millis(32));
    }
}


