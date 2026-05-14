// Vending machine as a state machine.
//
// This is the toy example everyone draws on a whiteboard, but the point is
// what the polynomial framing reveals: there are two completely separate
// "shapes" hiding inside any state machine. The internal shape says
// "from any state I can move to any state"; the external shape says
// "at any output I might receive any input". Treating these as two
// polynomials and the machine itself as a structure-preserving map
// between them is what lets us later wire machines together like Lego
// blocks (see examples/pipeline.rs) without writing glue code.

use tiny_poly::Moore;

fn main() {
    let m = Moore::new(
        vec![
            "0c".into(),
            "5c".into(),
            "10c".into(),
            "15c".into(),
            "20c".into(),
            "READY".into(),
        ],
        vec![
            "nickel".into(),
            "dime".into(),
            "select".into(),
            "refund".into(),
        ],
        vec![
            "0c".into(),
            "5c".into(),
            "10c".into(),
            "15c".into(),
            "20c".into(),
            "READY".into(),
        ],
        vec![0, 1, 2, 3, 4, 5],
        vec![
            vec![1, 2, 0, 0],
            vec![2, 3, 1, 0],
            vec![3, 4, 2, 0],
            vec![4, 5, 3, 0],
            vec![5, 5, 4, 0],
            vec![5, 5, 0, 0],
        ],
    );

    println!(
        "state polynomial    S y^S = {}",
        m.state_poly().show_aggregate()
    );
    println!(
        "interface polynomial B y^A = {}",
        m.interface_poly().show_aggregate()
    );

    let session = ["nickel", "dime", "dime", "select", "nickel", "refund"];
    let inputs: Vec<usize> = session
        .iter()
        .map(|name| m.inputs.iter().position(|i| i == name).unwrap())
        .collect();
    let trace = m.run(0, &inputs);

    println!();
    println!("inputs : {}", session.join(" "));
    println!("trace  : {}", m.show_trace(&trace));

    let lens = m.as_lens();
    println!();
    println!(
        "lens shape: {} positions on the left, {} positions on the right",
        lens.src.npositions(),
        lens.tgt.npositions()
    );
}
