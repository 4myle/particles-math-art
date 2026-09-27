/*
	Original idea from https://x.com/yuruyurau/status/2100230050063024467.
	Translated to Rust and egui with help from Mistral AI (https://chat.mistral.ai/chat/9b8ff3dc-da7e-4c26-ba52-66fc2f27ca3c).
*/

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::f64::consts::PI;
use eframe::egui::{
    self, 
    Color32, 
    Pos2, 
    Vec2,
    Shape
};

mod widgets;
use widgets::themer::Themer;
use widgets::factor::Factor;

#[derive(serde::Deserialize, serde::Serialize)]
struct Application 
{
    themer: Themer,
    iterations: u32,
    complexity: f32,
    figuration: f32,
    disperaion: f32,
    modulation: f32,
    t: f32
}

impl Default for Application 
{
    fn default() -> Self {
        Self {
            themer: Themer::new(egui::Color32::LIGHT_BLUE),
            iterations: 20000,
            complexity: 600.0,
            figuration: 1.95,
            disperaion: 3.5,
            modulation: 11.0,
            t: 0.0
        }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
impl Application
{
    fn new (context: &eframe::CreationContext<'_>) -> Self {
        let mut view: Application = if let Some(ps) = context.storage { eframe::get_value(ps, eframe::APP_KEY).unwrap_or_default() } else { Application::default() };
        view.t = 0.0; // Always grows otherwise.
        view.themer.set_fonts(&context.egui_ctx, &include_bytes!("../assets/SairaSemiCondensed-Regular.ttf")[..]);
        view.themer.set_style(&context.egui_ctx);
        view
    }
    
    fn ui_parameters (&mut self, ui: &mut egui::Ui) {
        ui.add(Factor::new(&mut self.iterations, 10000..=30000, "Iterations"));
        ui.add(Factor::new(&mut self.complexity, 400.0..=700.0, "Complexity"));
        ui.add(Factor::new(&mut self.figuration, 1.0..=4.0, "Figuration"));
        ui.add(Factor::new(&mut self.disperaion, 1.0..=7.0, "Dispersion"));
        ui.add(Factor::new(&mut self.modulation, 7.0..=13.0, "Modulation"));
        if ui.link("Reset values").clicked() {
            *self = Self::default();
        }
    }
    
    fn ui_visualizer (&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());
        let extent = response.rect.size();
        let center = Pos2::new(extent.x / 2.0 + 220.0 + 24.0, extent.y / 2.0 + 24.0);
        let color  = if ui.visuals().dark_mode {Color32::LIGHT_BLUE} else {Color32::LIGHT_BLUE.lerp_to_gamma(Color32::BLACK, 0.2)};
        let mut points: Vec<Shape> = Vec::new();
        for i in 0..self.iterations {
            points.push(Shape::circle_filled(
                center + 2.5*calculate(
                    i as f32, 
                    self.t, 
                    self.complexity, 
                    self.figuration, 
                    self.disperaion,
                    self.modulation
                ), 
                0.5, 
                color
            ));
        }
        painter.extend(points);
        self.t += PI as f32 / 75.0;
        ui.request_repaint();
    }

}

#[allow(clippy::cast_lossless, clippy::cast_possible_truncation, clippy::many_single_char_names)]
fn calculate(i: f32, t: f32, complexity: f32, figuration: f32, dispersion: f32, modulation: f32) -> Vec2 {
    let y = i / complexity;
    let k = (dispersion + y.sin()) * (i / figuration).cos();
    let e = y / 5.0 - modulation;
    let d = (k * k + e * e).sqrt(); //.powf(0.6) - 6.0;
    let q = 100.0 + d * (t - d).sin() + y / 23.0 * k * (3.0 * e.sin() + e * (e * 2.0).sin() + (d * 4.0).sin());
    let c = d / 4.0 - t / 8.0 + (t + e).cos() / 9.0;
    Vec2::new(q * c.sin(), q * c.cos())
}

impl eframe::App for Application 
{
    fn save (&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn ui (&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("Factors" ).frame(self.themer.theming_frame()).min_size(220.0).resizable(false).show(ui, |ui| {
            self.ui_parameters(ui);
            self.themer.show(ui);
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);
            ui.hyperlink_to("Original code by @yuruyurau", "https://x.com/yuruyurau/status/2100230050063024467");
        });
        egui::CentralPanel::default().frame(self.themer.central_frame()).show(ui, |ui| {
            self.ui_visualizer(ui);
        });
    }
    
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    eframe::run_native(
        env!("CARGO_PKG_NAME"), 
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_inner_size([960.0 + 220.0, 960.0]),
            ..Default::default()
        },
        Box::new(|context| {
            Ok(Box::new(Application::new(context)))
        })
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen(start)]
    pub async fn start() -> Result<(), JsValue> {
        let window   = web_sys::window().ok_or_else(|| wasm_bindgen::JsValue::from_str("No global `window` exists"))?;
        let document = window.document().ok_or_else(|| wasm_bindgen::JsValue::from_str("Should have a document on window"))?;
        let element  = document.get_element_by_id("the_canvas_id").ok_or_else(|| wasm_bindgen::JsValue::from_str("Canvas element 'the_canvas_id' not found"))?;
        let canvas   = element
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| wasm_bindgen::JsValue::from_str("Element is not a canvas"))?;
        let web_options = eframe::WebOptions::default();
        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(crate::Application::new(cc)))),
            )
            .await
    }
}