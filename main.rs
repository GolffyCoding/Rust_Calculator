use eframe::egui::{self, Button, Color32, FontId, TextEdit, Vec2, Stroke, Rounding};

struct CalculatorApp {
    input: String,
    result: String,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            result: String::new(),
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = Vec2::new(0.0, 15.0);
        style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
        style.visuals.widgets.inactive.rounding = Rounding::same(8.0);
        style.visuals.widgets.active.rounding = Rounding::same(8.0);
        style.visuals.widgets.hovered.rounding = Rounding::same(8.0);
        ctx.set_style(style);

        egui::CentralPanel::default()
            .frame(egui::Frame::default()
                .fill(Color32::from_rgb(28, 28, 35))
                .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
                .inner_margin(20.0))
            .show(ctx, |ui: &mut egui::Ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    // Title with glow effect
                    let title = "🧮 Rust Calculator";
     

                    ui.colored_label(
                        Color32::from_rgb(150, 150, 255),
                        egui::RichText::new(title).font(FontId::proportional(32.0))
                    );

                    ui.add_space(20.0);

                    // Styled input field
                    let input_frame = egui::Frame::default()
                        .fill(Color32::from_rgb(40, 40, 50))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(100, 100, 255)))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(Vec2::new(10.0, 8.0));

                    input_frame.show(ui, |ui| {
                        ui.add_sized(
                            Vec2::new(300.0, 40.0),
                            TextEdit::singleline(&mut self.input)
                                .hint_text("Enter expression (e.g., sqrt(25) + 3)")
                                .font(FontId::proportional(18.0))
                                .text_color(Color32::BLACK)
                        );
                    });

                    ui.add_space(10.0);

                    // Neon-style calculate button
                    let button_response = ui.add_sized(
                        Vec2::new(150.0, 45.0),
                        Button::new(
                            egui::RichText::new("✨ Calculate")
                                .font(FontId::proportional(20.0))
                                .color(Color32::WHITE)
                        )
                        .fill(Color32::from_rgb(90, 90, 255))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(130, 130, 255)))
                    );

                    if button_response.clicked() {
                        self.result = eval_expr(&self.input);
                    }

                    ui.add_space(20.0);

                    // Result display with neon card effect
                    let result_frame = egui::Frame::default()
                        .fill(Color32::from_rgb(35, 35, 45))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(100, 100, 255)))
                        .rounding(Rounding::same(8.0));

                    result_frame.show(ui, |ui| {
                        ui.add_space(10.0);
                        ui.colored_label(
                            Color32::from_rgb(200, 200, 255),
                            egui::RichText::new(format!("✨ Result: {}", self.result))
                                .font(FontId::proportional(20.0))
                        );
                        ui.add_space(10.0);
                    });
                });
            });
    }
}

fn eval_expr(expr: &str) -> String {
    let parsed = meval::eval_str(expr);
    match parsed {
        Ok(value) => format!("{:.6}", value),
        Err(_) => "❌ Error: Invalid expression".to_string(),
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 400.0])
            .with_min_inner_size([300.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Calculator",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(CalculatorApp::default()))
        }),
    )
}