use crate::lens::Lens;
use crate::poly::{Poly, Position};

#[derive(Clone, Debug)]
pub struct Moore {
    pub states: Vec<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub readout: Vec<usize>,
    pub update: Vec<Vec<usize>>,
}

impl Moore {
    pub fn new(
        states: Vec<String>,
        inputs: Vec<String>,
        outputs: Vec<String>,
        readout: Vec<usize>,
        update: Vec<Vec<usize>>,
    ) -> Self {
        assert_eq!(
            readout.len(),
            states.len(),
            "readout must have one entry per state"
        );
        assert_eq!(
            update.len(),
            states.len(),
            "update must have one row per state"
        );
        for row in &update {
            assert_eq!(
                row.len(),
                inputs.len(),
                "update row width must equal #inputs"
            );
        }
        Self {
            states,
            inputs,
            outputs,
            readout,
            update,
        }
    }

    pub fn state_poly(&self) -> Poly {
        let positions = self
            .states
            .iter()
            .map(|s| Position::new(s.clone(), self.states.clone()))
            .collect();
        Poly::new(positions)
    }

    pub fn interface_poly(&self) -> Poly {
        let positions = self
            .outputs
            .iter()
            .map(|o| Position::new(o.clone(), self.inputs.clone()))
            .collect();
        Poly::new(positions)
    }

    pub fn as_lens(&self) -> Lens {
        Lens::new(
            self.state_poly(),
            self.interface_poly(),
            self.readout.clone(),
            self.update.clone(),
        )
    }

    pub fn step(&self, state: usize, input: usize) -> usize {
        self.update[state][input]
    }

    pub fn output(&self, state: usize) -> usize {
        self.readout[state]
    }

    pub fn run(&self, mut state: usize, inputs: &[usize]) -> Vec<(usize, usize)> {
        let mut trace = Vec::with_capacity(inputs.len() + 1);
        trace.push((state, self.readout[state]));
        for &a in inputs {
            state = self.update[state][a];
            trace.push((state, self.readout[state]));
        }
        trace
    }

    pub fn show_trace(&self, trace: &[(usize, usize)]) -> String {
        trace
            .iter()
            .map(|&(s, o)| format!("{}/{}", self.states[s], self.outputs[o]))
            .collect::<Vec<_>>()
            .join(" -> ")
    }

    pub fn parity() -> Self {
        Self::new(
            vec!["even".into(), "odd".into()],
            vec!["0".into(), "1".into()],
            vec!["0".into(), "1".into()],
            vec![0, 1],
            vec![vec![0, 1], vec![1, 0]],
        )
    }

    pub fn traffic_light() -> Self {
        Self::new(
            vec!["red".into(), "green".into(), "yellow".into()],
            vec!["tick".into(), "emergency".into()],
            vec!["STOP".into(), "GO".into(), "SLOW".into()],
            vec![0, 1, 2],
            vec![vec![1, 0], vec![2, 0], vec![0, 0]],
        )
    }
}
