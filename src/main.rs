use core::f32;
use eframe::egui;
use std::process::Command;

fn main() -> eframe::Result {
    eframe::run_native(
        "MRN Generator",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MrnGeneratorApp::new(cc)))),
    )
}

#[derive(Default)]
struct MrnGeneratorApp {
    country_code: String,
    number_of_mrns: String,
    procedure_category: String,
    combined: String,
    declaration_office: String,
    output: String,
}

impl MrnGeneratorApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MrnGeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MRN Generator");

            ui.horizontal(|ui| {
                ui.label("Country Code:");
                ui.add(egui::TextEdit::singleline(&mut self.country_code).char_limit(2)
                    .hint_text("Country code, this is mandatory"));
            });

            ui.horizontal(|ui| {
                ui.label("Number of MRNs:");
                ui.add(egui::TextEdit::singleline(&mut self.number_of_mrns).char_limit(3)
                .hint_text("Number of MRNs to generate, default is 1"));
            });

            ui.horizontal(|ui| {
                ui.label("Procedure Category:");
                ui.add(egui::TextEdit::singleline(&mut self.procedure_category)
                .desired_width(f32::INFINITY).char_limit(2)
                    .hint_text("Change penultimate digit based on procedure category, optional"));
            });

            ui.horizontal(|ui| {
                ui.label("Combined Procedure Category:");
                ui.add(egui::TextEdit::singleline(&mut self.combined)
                .desired_width(f32::INFINITY).char_limit(1)
                    .hint_text("Change penultimate digit based on combined procedure category, optional"));
            });

            ui.horizontal(|ui| {
                ui.label("Declaration Office:");
                ui.add(egui::TextEdit::singleline(&mut self.declaration_office).char_limit(6)
                .hint_text("Customs office of declaration, optional"));
            });

            if ui.button("Generate MRNs").clicked() {
                self.output = generate_mrns(
                    &self.country_code,
                    &self.number_of_mrns,
                    &self.procedure_category,
                    &self.combined,
                    &self.declaration_office,
                );
            }

            ui.separator();
            ui.label("Output:");
            ui.text_edit_multiline(&mut self.output);
        });
    }
}

fn generate_mrns(
    country_code: &str,
    number_of_mrns: &str,
    procedure_category: &str,
    combined: &str,
    declaration_office: &str,
) -> String {
    let mut command = Command::new("mrn-generator.exe");

    command.arg("--country-code").arg(country_code);

    if !number_of_mrns.is_empty() {
        command.arg("--number-of-mrns").arg(number_of_mrns);
    }

    if !procedure_category.is_empty() {
        command.arg("--procedure-category").arg(procedure_category);
    }

    if !combined.is_empty() {
        command.arg("--combined").arg(combined);
    }

    if !declaration_office.is_empty() {
        command.arg("--declaration-office").arg(declaration_office);
    }

    match command.output() {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).to_string()
            } else {
                format!("Error: {}", String::from_utf8_lossy(&output.stderr))
            }
        }
        Err(e) => format!("Failed to execute process: {}", e),
    }
}
