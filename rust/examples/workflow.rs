// A PR review workflow, with all the rules made explicit.
//
//   Draft  --submit-->  InReview  --merge-->  Merged
//   `---------- submit_then_merge ---------->'
//
// We list the states (Draft, InReview, Merged) and the allowed
// transitions, including the composite "submit and then merge". The
// library checks that the bookkeeping is consistent: doing two steps in
// a row and then a third should give the same result as doing the first
// step and then a combined "second-and-third" step. That sanity check is
// the same thing as checking the workflow really is a workflow.
//
// Why bother? Because once a workflow lines up like this, the library can
// hand back a `comonoid` for it: a compact algebraic description that
// other tools can plug into without re-deriving the state diagram. It is
// the same trick a parser generator pulls when it takes your grammar and
// hands you a parse table.

use tiny_poly::{Category, Morphism};

fn main() {
    let workflow = Category {
        objects: vec!["Draft".into(), "InReview".into(), "Merged".into()],
        morphisms: vec![
            vec![
                Morphism {
                    name: "id_Draft".into(),
                    target: 0,
                },
                Morphism {
                    name: "submit".into(),
                    target: 1,
                },
                Morphism {
                    name: "submit_then_merge".into(),
                    target: 2,
                },
            ],
            vec![
                Morphism {
                    name: "id_InReview".into(),
                    target: 1,
                },
                Morphism {
                    name: "merge".into(),
                    target: 2,
                },
            ],
            vec![Morphism {
                name: "id_Merged".into(),
                target: 2,
            }],
        ],
        table: vec![
            vec![vec![0, 1, 2], vec![1, 2], vec![2]],
            vec![vec![0, 1], vec![1]],
            vec![vec![0]],
        ],
    };

    workflow
        .check_axioms()
        .expect("workflow must be a category");

    let c = workflow.comonoid();
    println!("polynomial P_C = {}", c.poly.show_named());
    println!();
    println!("epsilon : P_C -> y");
    print!("{}", c.epsilon);
    println!("delta   : P_C -> P_C <| P_C");
    print!("{}", c.delta);

    println!("derived facts:");
    let n_morph: usize = workflow.morphisms.iter().map(Vec::len).sum();
    println!(
        "  {} objects, {} morphisms total, {} of them composites",
        workflow.objects.len(),
        n_morph,
        n_morph - workflow.objects.len()
    );
}
