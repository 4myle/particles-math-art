
use eframe::egui;
use crate::widgets::switch::Switch;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Copy, Clone)]
enum InterfaceMode
{
    Dark,
    Light
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Themer 
{
    ui_mode: InterfaceMode,
    ui_size: f32,
    accent: egui::Color32
}

impl Themer
{
    pub fn new (accent: egui::Color32) -> Self {
        Self {
            ui_mode: InterfaceMode::Dark,
            ui_size: 1.2,
            accent 
        }
    }

    pub fn central_frame (&self) -> egui::Frame {
        let cb = match self.ui_mode {
            InterfaceMode::Dark  => self.accent.gamma_multiply(0.05),
            InterfaceMode::Light => self.accent.lerp_to_gamma(egui::Color32::WHITE, 0.9)
        };
        egui::Frame {
            inner_margin: egui::Margin::same(24),
            fill: cb,
            ..Default::default()
        }
    }

    pub fn theming_frame (&self) -> egui::Frame {
        let cb = match self.ui_mode {
            InterfaceMode::Dark  => egui::Color32::from_gray(27),
            InterfaceMode::Light => egui::Color32::from_gray(248)
        };
        egui::Frame {
            inner_margin: egui::Margin::symmetric(24, 8),
            fill: cb,
            ..Default::default()
        }
    }
    
    #[allow(clippy::unused_self)]
    pub fn set_fonts (&self, context: &egui::Context, bytes: &[u8]) {
        let class = "Sans Font";
        let mut font = egui::FontDefinitions::default();
        font.font_data.insert(class.to_string(), std::sync::Arc::new(egui::FontData::from_owned(bytes.to_vec())));
        if let Some(properties) = font.families.get_mut(&egui::FontFamily::Proportional) {
            properties.insert(0, class.to_string());
            context.set_fonts(font);
        }
    }

    pub fn set_style (&self, context: &egui::Context) {
        let mut visuals: egui::Visuals;
        match self.ui_mode {
            InterfaceMode::Dark  => {
                context.set_theme(egui::Theme::Dark);
                visuals = egui::Visuals::dark();
                visuals.override_text_color = Option::Some(egui::Color32::from_gray(255));
            },
            InterfaceMode::Light => {
                context.set_theme(egui::Theme::Light);
                visuals = egui::Visuals::light();
                visuals.override_text_color = Option::Some(egui::Color32::from_gray(0));
            }
        }
        visuals.widgets.active.bg_fill = self.accent;
        visuals.widgets.noninteractive.bg_fill = self.accent;
        visuals.widgets.hovered.bg_fill = self.accent;
        visuals.hyperlink_color = self.accent;
        visuals.widgets.hovered.weak_bg_fill = self.accent.gamma_multiply(0.1);
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, self.accent.gamma_multiply(0.2));
        visuals.selection.bg_fill = self.accent.gamma_multiply(0.6);
        visuals.selection.stroke = egui::Stroke::new(1.0, self.accent.lerp_to_gamma(egui::Color32::WHITE, 0.5));
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, self.accent.gamma_multiply(0.2));
        visuals.slider_trailing_fill = true;
        visuals.window_shadow.offset = [4,4];
        context.style_mut_of(context.theme(), |style| { 
            style.spacing.item_spacing = egui::Vec2::new(16.0, 8.0);
            style.spacing.button_padding = egui::Vec2::new(12.0, 4.0); 
        });
        context.set_visuals(visuals);
        context.set_zoom_factor(self.ui_size);

    }

    pub fn show (&mut self, ui: &mut egui::Ui) {
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);
        ui.vertical(|ui| {
            ui.style_mut().spacing.button_padding = egui::Vec2::new(4.0, 2.0);
            ui.label(egui::RichText::new("TEXT SIZE").small().weak());
            if ui.add(egui::Slider::new(&mut self.ui_size, 0.7..=1.7)).changed() {
                ui.ctx().set_zoom_factor(self.ui_size);
            }
        });
        ui.vertical(|ui| {
            ui.label(egui::RichText::new("DARK MODE").small().weak());
            if ui.add(Switch::new(self.ui_mode == InterfaceMode::Dark)).clicked() {
                match self.ui_mode {
                    InterfaceMode::Dark  => { 
                        self.ui_mode = InterfaceMode::Light;
                        self.set_style(ui.ctx());
                    },
                    InterfaceMode::Light => { 
                        self.ui_mode = InterfaceMode::Dark;
                        self.set_style(ui.ctx());
                    }
                }
            }
        });
    }

}
