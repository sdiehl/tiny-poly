use tiny_poly::{compose, series, sum, tensor, Category, Moore, Morphism, Poly, Position};

fn banner(title: &str) {
    println!();
    println!("== {title} ==");
}

fn parity() -> Moore {
    Moore::new(
        vec!["even".into(), "odd".into()],
        vec!["0".into(), "1".into()],
        vec!["0".into(), "1".into()],
        vec![0, 1],
        vec![vec![0, 1], vec![1, 0]],
    )
}

fn walking_arrow() -> Category {
    Category {
        objects: vec!["a".into(), "b".into()],
        morphisms: vec![
            vec![
                Morphism {
                    name: "id_a".into(),
                    target: 0,
                },
                Morphism {
                    name: "f".into(),
                    target: 1,
                },
            ],
            vec![Morphism {
                name: "id_b".into(),
                target: 1,
            }],
        ],
        table: vec![vec![vec![0, 1], vec![1]], vec![vec![0]]],
    }
}

fn main() {
    banner("polynomials");
    let p = Poly::new(vec![
        Position::of_arity("a", 2),
        Position::of_arity("b", 1),
        Position::of_arity("c", 0),
    ]);
    let q = Poly::new(vec![Position::of_arity("x", 1), Position::of_arity("y", 0)]);
    println!("P = {p}");
    println!("Q = {q}");

    banner("operations");
    println!("P + Q   = {}", sum(&p, &q));
    println!("P (x) Q = {}", tensor(&p, &q));
    println!("P <| Q  = {}", compose(&p, &q));

    banner("Moore parity machine");
    let parity = parity();
    let trace = parity.run(0, &[1, 1, 0, 1, 0, 1]);
    println!("inputs : 1 1 0 1 0 1");
    println!("trace  : {}", parity.show_trace(&trace));

    banner("wiring (series)");
    let wired = series(&parity, &parity);
    let trace = wired.run(0, &[1, 1, 1]);
    println!("trace  : {}", wired.show_trace(&trace));

    banner("comonoids are categories");
    let cat = walking_arrow();
    cat.check_axioms().expect("walking arrow is a category");
    let c = cat.comonoid();
    println!("P(C)        = {}", c.poly.show_named());
    println!("epsilon:");
    print!("{}", c.epsilon);
    println!("delta:");
    print!("{}", c.delta);
}
