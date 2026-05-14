// A deterministic finite automaton, recognising "ends in 01".
//
// You feed it a string of 0s and 1s, one symbol at a time. After each
// symbol it shines a green light (accept) if everything seen so far ends
// in 01, otherwise a red light (reject). Three states are enough to keep
// track: q0 = haven't seen anything useful yet, q1 = just saw a 0, q2 =
// just saw a 0 then a 1.
//
// What the polynomial framing buys us: the automaton is a lens
// `S y^S -> 2 y^A`. The 2 in the codomain is literally the set
// {reject, accept}: the same shape we'd use for any yes/no output, and
// the same shape that lets us later compose this DFA with anything else
// expecting a 2-output.
//
// Spivak & Niu, Section 4.1.1 (Deterministic state automata), p.88.

use tiny_poly::Moore;

fn main() {
    let dfa = Moore::new(
        vec!["q0".into(), "q1".into(), "q2".into()],
        vec!["0".into(), "1".into()],
        vec!["reject".into(), "accept".into()],
        vec![0, 0, 1],
        vec![
            vec![1, 0], // q0: on 0 -> q1, on 1 -> q0
            vec![1, 2], // q1: on 0 -> q1, on 1 -> q2
            vec![1, 0], // q2: on 0 -> q1, on 1 -> q0
        ],
    );

    println!("state polynomial    S y^S = {}", dfa.state_poly());
    println!("interface polynomial B y^A = {}", dfa.interface_poly());

    let words: &[&[usize]] = &[
        &[0, 1],          // 01 -> accept
        &[1, 0, 1],       // 101 -> accept
        &[0, 0, 1],       // 001 -> accept
        &[1, 1, 0],       // 110 -> reject
        &[0, 1, 0, 1, 1], // 01011 -> reject
    ];
    for w in words {
        let trace = dfa.run(0, w);
        let final_state = trace.last().unwrap().0;
        let label = &dfa.outputs[dfa.readout[final_state]];
        let s: Vec<String> = w.iter().map(usize::to_string).collect();
        println!();
        println!("input  : {}", s.join(""));
        println!("trace  : {}", dfa.show_trace(&trace));
        println!("verdict: {label}");
    }
}
