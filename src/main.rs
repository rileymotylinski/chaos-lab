#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui_plot::{Line, Plot, PlotPoints, Points};
use egui::*;



use crate::{double_pendulum::DoublePendulum, logistic_map::LogisticMap, lorenz::Lorenz};

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

#[derive(Debug,Clone,Copy)]
enum Simulation {
    Lorenz,
    Dp,
    Lmap
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
            (Simulation::Lmap, Simulation::Lmap) => true,
            _ => false
        }
    }
}

impl ToString for Simulation {
    fn to_string(&self) -> String {
        match self {
            Simulation::Lorenz => String::from("lorenz"),
            Simulation::Dp => String::from("double pendulum"),
            Simulation::Lmap => String::from("logistic map")
        }
    }
}




struct MyEguiApp {
    
    pub simulation: Simulation,
    pub is_playing: bool,
    pub speed: u64,

    pub points: Vec<Vec<[f64; 2]>>,
    // TODO: refactor, definitely a way to compress the points into a single vector

    // lorenz
    pub lorenz_system: Lorenz,
    pub lorenz_state: [f64; 3],

    // double pendulum
    pub dp_system: DoublePendulum,
    pub dp_state: [f64; 4],

    pub lmap_system: LogisticMap,
    pub lmap_state: [f64; 1]

    
}

impl Default for MyEguiApp {
    fn default() -> Self {
        MyEguiApp {
            simulation: Default::default(),
            is_playing: false,
            speed:50,

            points: vec![vec![]],

            lorenz_system: Default::default(),
            lorenz_state: [1.0,1.0,1.0],

            dp_system: Default::default(),
            dp_state: [1.0,1.0,1.0,1.0],

            lmap_system: Default::default(),
            lmap_state: [0.7]
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
        MyEguiApp::default()
    }


}

impl eframe::App for MyEguiApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.speed, 0..=100));
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
                    
                    // TODO : make this more generic/applicable to all simulations
                    if response.clicked() {
                        self.points = MyEguiApp::default().points;
                        self.lorenz_state = MyEguiApp::default().lorenz_state;
                        
                        self.dp_state = MyEguiApp::default().dp_state;

                        // pause on reset, annoying to have it continue to play?
                        if self.is_playing {
                            self.is_playing = false;
                        }
                       
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
                
                });

                // default state
                let mut before = self.simulation;
                // dropdown menu for simulation selection
                egui::ComboBox::from_label("Select one!")
                .selected_text(self.simulation.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.simulation, Simulation::Lorenz, Simulation::Lorenz.to_string());
                    ui.selectable_value(&mut self.simulation, Simulation::Dp, Simulation::Dp.to_string());
                    ui.selectable_value(&mut self.simulation, Simulation::Lmap, Simulation::Lmap.to_string());
                });
                // on change
                if self.simulation != before {
                    // reset graph(s)
                    self.points = MyEguiApp::default().points;
                    self.lorenz_state = MyEguiApp::default().lorenz_state;
                    self.dp_state = MyEguiApp::default().dp_state;
                    // stop simulation when switching
                    self.is_playing = false;
                }
                
            });
            
            // should always have at least one line being plotted
            // points[0] is *always* user modfied trajectory
            if self.points.len() == 0 {
                self.points.push(Vec::new());
            }

            if self.simulation == Simulation::Lorenz {
                // sliders for ro, sigma, beta
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.label("ro");
                    ui.add(egui::Slider::new(&mut self.lorenz_system.ro, 0.0..=100.0));
                    ui.label("sigma");
                    ui.add(egui::Slider::new(&mut self.lorenz_system.sigma, 0.0..=100.0));
                    ui.label("beta");
                    ui.add(egui::Slider::new(&mut self.lorenz_system.beta, 0.0..=100.0));
                });
                
                // pushing points
                if self.is_playing {
                    // maybe change t, dt to state variables? not really sure.
                    crate::integrators::rk4_step(&self.lorenz_system, &mut self.lorenz_state, 0.0, 0.01);
                    // discard z value or self.lorenz_state[2]
                    // points[0] is *always* user controlled trajectory
                    
                    self.points[0].push([self.lorenz_state[0], self.lorenz_state[1]]);
                    
                // only allow modifcation of the inital state
                } else if self.points[0].len() == 0 { 
                    // sliders for inital state
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.label("x");
                        ui.add(egui::Slider::new(&mut self.lorenz_state[0], 0.0..=100.0));
                        ui.label("y");
                        ui.add(egui::Slider::new(&mut self.lorenz_state[1], 0.0..=100.0));
                        ui.label("z");
                        ui.add(egui::Slider::new(&mut self.lorenz_state[2], 0.0..=100.0));
                        ui.heading("initial state");
                    });
                }

                // plotting
                let cur_points: PlotPoints = self.points[0].iter().map(|i| {
                        [i[0],i[1]]
                }).collect();

                let line = Line::new("Lorenz Attractor", cur_points);
                Plot::new("Lorenz Attractor")
                .view_aspect(2.0)
                .x_axis_label("x")
                .y_axis_label("y")
                .show(ui, |plot_ui| plot_ui.line(line));
            
                
                
            } else if self.simulation == Simulation::Dp { 
                // sliders for length1, length2, mass1, mass2
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.label("l1");
                    ui.add(egui::Slider::new(&mut self.dp_system.l1, 0.0..=100.0));
                    ui.label("l2");
                    ui.add(egui::Slider::new(&mut self.dp_system.l2, 0.0..=100.0));
                    ui.label("m1");
                    ui.add(egui::Slider::new(&mut self.dp_system.m1, 0.0..=100.0));
                    ui.label("m2");
                    ui.add(egui::Slider::new(&mut self.dp_system.m2, 0.0..=100.0));
                });
                
                // pushing points
                if self.is_playing {
                    // maybe change t, dt to state variables? not really sure.
                    crate::integrators::rk4_step(&self.dp_system, &mut self.dp_state, 0.0, 0.01);
                    // discard z value or self.lorenz_state[2]
                    self.points[0].push([self.dp_state[0], self.dp_state[1]]);
                // manage inital state, only when on pause   
                } else if self.points[0].len() == 0 { 
                    // sliders for inital state
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        
                        ui.label("theta1");
                        ui.add(egui::Slider::new(&mut self.dp_state[0], 0.0..=100.0));
                        ui.label("theta2");
                        ui.add(egui::Slider::new(&mut self.dp_state[1], 0.0..=100.0));
                        ui.label("omega1");
                        ui.add(egui::Slider::new(&mut self.dp_state[2], 0.0..=100.0));
                        ui.label("omega2");
                        ui.add(egui::Slider::new(&mut self.dp_state[3], 0.0..=100.0));
                        ui.heading("initial state");
                    });
                }

                // plotting points
                let cur_points: PlotPoints = self.points[0].iter().map(|i| {
                        [i[0],i[1]]
                }).collect();

                let line = Line::new("Double Pendulum", cur_points);
                Plot::new("Double Pendulum")
                .view_aspect(2.0)
                .x_axis_label("Theta 1")
                .y_axis_label("Theta 2")
                .show(ui, |plot_ui| plot_ui.line(line));
            
                
                
            } else {
                //if ui.add(egui::Slider::new(&mut self.lmap_system.r,0.3..=0.4)).changed() {
                    //self.lmap_state = MyEguiApp::default().lmap_state;
                //};
                
                // running simulation
                if self.is_playing {

                    for s in 2500..4000 {
                        let r = s as f64 / 100.0;

                        for _ in 0..50 {
                            // maybe change t, dt to state variables? not really sure.
                            crate::integrators::rk4_step(&LogisticMap {r: r}, &mut self.lmap_state, 0.0, 0.01);
                            // discard z value or self.lorenz_state[2]
                            self.points[0].push([r, self.lmap_state[0]]);
                        }

                        self.lmap_state = MyEguiApp::default().lmap_state;
                    }
                    
                   
                    
                    self.is_playing = !self.is_playing;
                }

                // plotting points
                let cur_points: PlotPoints = self.points[0].iter().map(|i| {
                        [i[0],i[1]]
                }).collect();

                let pts = Points::new("pts", cur_points).radius(0.9).color(egui::Color32::LIGHT_BLUE);
                
                Plot::new("my_plot")
                .view_aspect(2.0)
                .x_axis_label("r")
                .y_axis_label("x_n")
                .show(ui, |plot_ui| {
                    plot_ui.points(pts);
                });
            }
        });
        // update every 32ms, regardless of user input
        // subtract 100 to intuitively increase speed.
        // realistically we are chaning the length between updates, wo increasing the "speed" will actually increase the
        ctx.request_repaint_after(std::time::Duration::from_millis((100 as u64).abs_diff(self.speed)));
    }
}


