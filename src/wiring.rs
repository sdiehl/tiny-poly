use crate::moore::Moore;

pub fn parallel(m1: &Moore, m2: &Moore) -> Moore {
    let n1 = m1.states.len();
    let n2 = m2.states.len();
    let na2 = m2.inputs.len();
    let nb2 = m2.outputs.len();

    let mut states = Vec::with_capacity(n1 * n2);
    let mut readout = Vec::with_capacity(n1 * n2);
    for i in 0..n1 {
        for j in 0..n2 {
            states.push(format!("({},{})", m1.states[i], m2.states[j]));
            readout.push(m1.readout[i] * nb2 + m2.readout[j]);
        }
    }

    let mut inputs = Vec::with_capacity(m1.inputs.len() * na2);
    for a in &m1.inputs {
        for b in &m2.inputs {
            inputs.push(format!("({a},{b})"));
        }
    }
    let mut outputs = Vec::with_capacity(m1.outputs.len() * nb2);
    for a in &m1.outputs {
        for b in &m2.outputs {
            outputs.push(format!("({a},{b})"));
        }
    }

    let mut update = Vec::with_capacity(n1 * n2);
    for s1 in 0..n1 {
        for s2 in 0..n2 {
            let mut row = Vec::with_capacity(inputs.len());
            for a1 in 0..m1.inputs.len() {
                for a2 in 0..na2 {
                    let ns1 = m1.update[s1][a1];
                    let ns2 = m2.update[s2][a2];
                    row.push(ns1 * n2 + ns2);
                }
            }
            update.push(row);
        }
    }
    Moore::new(states, inputs, outputs, readout, update)
}

pub fn series(m1: &Moore, m2: &Moore) -> Moore {
    assert_eq!(
        m1.outputs, m2.inputs,
        "series wiring needs M1 outputs to match M2 inputs"
    );
    let n1 = m1.states.len();
    let n2 = m2.states.len();

    let mut states = Vec::with_capacity(n1 * n2);
    let mut readout = Vec::with_capacity(n1 * n2);
    for i in 0..n1 {
        for j in 0..n2 {
            states.push(format!("({},{})", m1.states[i], m2.states[j]));
            readout.push(m2.readout[j]);
        }
    }
    let inputs = m1.inputs.clone();
    let outputs = m2.outputs.clone();

    // Each tick: external input drives M1; M1's current output is fed into M2.
    let mut update = Vec::with_capacity(n1 * n2);
    for s1 in 0..n1 {
        for s2 in 0..n2 {
            let bridge = m1.readout[s1];
            let mut row = Vec::with_capacity(inputs.len());
            for a in 0..m1.inputs.len() {
                let ns1 = m1.update[s1][a];
                let ns2 = m2.update[s2][bridge];
                row.push(ns1 * n2 + ns2);
            }
            update.push(row);
        }
    }
    Moore::new(states, inputs, outputs, readout, update)
}
