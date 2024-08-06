#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use eframe::egui;
use mrn_generator::{generate_random_mrn, match_procedure};

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

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label("Country Code:");
                ui.add(egui::TextEdit::singleline(&mut self.country_code).char_limit(2)
                    .hint_text("Country code, this is mandatory"));
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label("Number of MRNs:");
                ui.add(egui::TextEdit::singleline(&mut self.number_of_mrns).char_limit(3)
                .hint_text("Number of MRNs to generate, default is 1"));
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label("Procedure Category:");
                ui.add(egui::TextEdit::singleline(&mut self.procedure_category)
                .desired_width(400.0).char_limit(2)
                    .hint_text("Change penultimate digit based on procedure category, optional"));
            });

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label("Combined Procedure Category:");
                egui::ComboBox::from_label("Select Combined Procedure Category")
                    .selected_text(&self.combined)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.combined, String::new(), "None");
                        ui.selectable_value(&mut self.combined, "A".to_string(), "A");
                        ui.selectable_value(&mut self.combined, "F".to_string(), "F");
                    });
            });

            ui.add_space(5.0);
            ui.label("Combines the given procedure category with a different kind of procedure category, specifically for A* and F* categories.");

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label("Declaration Office:");
                ui.add(egui::TextEdit::singleline(&mut self.declaration_office).char_limit(6)
                .hint_text("Customs office of declaration, optional"));
            });

            ui.add_space(10.0);
            if ui.button("Generate MRNs").clicked() {
                self.output = generate_mrns(
                    &self.country_code,
                    self.number_of_mrns.parse::<u32>().unwrap_or(1),
                    (!self.procedure_category.is_empty()).then_some(&self.procedure_category),
                    (!self.combined.is_empty()).then_some(&self.combined),
                    (!self.declaration_office.is_empty()).then_some(&self.declaration_office),
                );
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            ui.label("Output:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.text_edit_multiline(&mut self.output);
            });
        });
    }
}

fn generate_mrns(
    country_code: &str,
    number_of_mrns: u32,
    procedure_category_opt: Option<&str>,
    combined_opt: Option<&str>,
    declaration_office_opt: Option<&str>,
) -> String {
    let mut output = String::new();

    if country_code.is_empty() {
        return "Please input a country code with two characters (e.g. 'IT')".to_string();
    }

    for _ in 0..number_of_mrns {
        let mut is_err = false;

        let mut procedure_opt = None;
        if let Some(procedure_category) = procedure_category_opt {
            let procedure_res = match_procedure(procedure_category, combined_opt);

            match procedure_res {
                Ok(procedure) => procedure_opt = Some(procedure),
                Err(e) => {
                    output = e.to_string();
                    is_err = true;
                }
            }
        } else if let Some(combined) = combined_opt {
            output = format!("You can't combine {combined} without providing a procedure category")
                .to_string();
            break;
        };

        if is_err {
            break;
        }

        let mrn_res = generate_random_mrn(country_code, procedure_opt, declaration_office_opt);

        match mrn_res {
            Ok(mrn) => {
                if output.is_empty() {
                    output = mrn
                } else {
                    output = [output, mrn].join("\n")
                };
            }
            Err(e) => {
                output = e.to_string();
                break;
            }
        }
    }

    output
}
