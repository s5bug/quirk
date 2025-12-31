use qiskit_rs::{QiskitError, QuantumCircuit};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct VisualCircuit {
    num_rows: u32,
    columns: Vec<VisualColumn>,
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
    gates: Vec<Option<Box<dyn VisualGate>>>
}

#[typetag::serde(tag = "type")]
pub trait VisualGate {
    fn size(&self) -> u32;

    // TODO functions for rendering

    // FIXME qiskit-cext broken on mac
    fn apply_to_circuit(&self, top: u32, qc: &mut QuantumCircuit) -> QiskitError;
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Rx { theta: f64 }

#[typetag::serde]
impl VisualGate for Rx {
    fn size(&self) -> u32 {
        1
    }

    fn apply_to_circuit(&self, top: u32, qc: &mut QuantumCircuit) -> QiskitError {
        qc.rx(self.theta, top)
    }
}
