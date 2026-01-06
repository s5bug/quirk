// use qiskit_rs::{QiskitError, QuantumCircuit};

use std::str::FromStr;
use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct VisualCircuit {
    pub num_rows: u32,
    pub columns: Vec<VisualColumn>,
}

impl Default for VisualCircuit {
    fn default() -> Self {
        VisualCircuit {
            num_rows: 1,
            columns: vec![]
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct VisualColumn {
    pub gates: Vec<Option<Box<dyn VisualGate>>>
}

#[typetag::serde(tag = "type")]
pub trait VisualGate {
    fn size(&self) -> u32;

    // TODO introduce time variable to affect rendering
    // TODO change return type to indicate the circuit should be recompiled
    fn show(&mut self, ui: &mut egui::Ui) -> ();

    // FIXME qiskit-cext broken on mac
    // fn apply_to_circuit(&self, top: u32, qc: &mut QuantumCircuit) -> QiskitError;
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Rx { pub theta: f64 }

#[typetag::serde]
impl VisualGate for Rx {
    fn size(&self) -> u32 {
        1
    }

    fn show(&mut self, ui: &mut Ui) -> () {
        let mut modified = false;

        // TODO create a helper for gate frames
        let mut frame = egui::Frame::new();
        frame.stroke = egui::Stroke::new(1f32, egui::Color32::from_black_alpha(255));
        frame.fill = egui::Color32::from_white_alpha(255);
        frame.show(ui, |ui| {
            ui.centered_and_justified(|ui| {
                ui.horizontal_centered(|ui| {
                    ui.label("Rx(");

                    let mut text = self.theta.to_string();
                    let response =
                        egui::TextEdit::singleline(&mut text)
                            .desired_width(0f32)
                            .clip_text(false)
                            .show(ui).response;
                    if response.changed() {
                        if let Ok(new_theta) = f64::from_str(&text) {
                            self.theta = new_theta;
                            modified = true;
                        }
                    }

                    ui.label(")");
                });
            })
        });
    }

    // fn apply_to_circuit(&self, top: u32, qc: &mut QuantumCircuit) -> QiskitError {
    //     qc.rx(self.theta, top)
    // }
}

pub trait VisualGateFactory {
    type Parameters; // TODO trait bound for rendering parameters
    type Gate : VisualGate;

    fn create(params: Self::Parameters) -> Self::Gate;
    fn show(ui: &mut egui::Ui) -> ();
}

impl VisualGateFactory for Rx {
    type Parameters = Rx;
    type Gate = Rx;

    fn create(params: Self::Parameters) -> Self::Gate {
        params
    }

    fn show(ui: &mut Ui) -> () {
        let mut frame = egui::Frame::new();
        frame.stroke = egui::Stroke::new(1f32, egui::Color32::from_black_alpha(255));
        frame.fill = egui::Color32::from_white_alpha(255);
        frame.show(ui, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label("Rx(θ)");
            })
        });
    }
}
