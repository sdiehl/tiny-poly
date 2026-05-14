// Two small machines piped together, exactly like `cmd1 | cmd2` in a shell.
//
// Stage 1 is a parity counter: it reads bits and outputs 0 or 1 depending
// on whether it has seen an even or odd number of 1s so far.
// Stage 2 watches that stream and trips an ALARM if it ever sees three
// 1s in a row. So the combined pipeline is an anomaly detector: each
// stage is dumb on its own, the whole thing computes something useful.
//
// The interesting bit: the "wire two machines together" operation is
// itself a first-class thing in this library. You did not have to write
// any glue. Same reason Unix pipes feel powerful: a tiny composition
// operator on tiny components covers an enormous amount of ground.

use tiny_poly::{series, Moore};

fn main() {
    let parity = Moore::parity();

    let detector = Moore::new(
        vec!["q0".into(), "q1".into(), "q2".into(), "ALARM".into()],
        vec!["0".into(), "1".into()],
        vec!["idle".into(), "ALARM".into()],
        vec![0, 0, 0, 1],
        vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![3, 3]],
    );

    let wired = series(&parity, &detector);

    // Bridge value at tick t is readout(parity) at tick t-1, so the detector
    // sees three '1's whenever parity stays odd for three consecutive ticks.
    let bits: Vec<usize> = vec![1, 0, 0, 0, 1, 1, 0, 0, 0];
    let trace = wired.run(0, &bits);

    println!(
        "stage 1 interface: {}",
        parity.interface_poly().show_aggregate()
    );
    println!(
        "stage 2 interface: {}",
        detector.interface_poly().show_aggregate()
    );
    println!(
        "wired   interface: {}",
        wired.interface_poly().show_aggregate()
    );

    let bit_str: Vec<String> = bits.iter().map(usize::to_string).collect();
    println!();
    println!("inputs : {}", bit_str.join(" "));
    println!("trace  :");
    for (state, out) in &trace {
        println!(
            "  state={:<14} output={}",
            wired.states[*state], wired.outputs[*out]
        );
    }
}
