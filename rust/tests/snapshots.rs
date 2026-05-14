#![allow(clippy::many_single_char_names)]

use std::fmt::Write;

use tiny_poly::{
    compose, parallel, series, sum, tensor, Category, Lens, Moore, Morphism, Poly, Position,
};

fn small_p() -> Poly {
    Poly::new(vec![
        Position::of_arity("a", 2),
        Position::of_arity("b", 1),
        Position::of_arity("c", 0),
    ])
}

fn small_q() -> Poly {
    Poly::new(vec![Position::of_arity("x", 1), Position::of_arity("y", 0)])
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

fn traffic_light() -> Moore {
    Moore::new(
        vec!["red".into(), "green".into(), "yellow".into()],
        vec!["tick".into(), "emergency".into()],
        vec!["STOP".into(), "GO".into(), "SLOW".into()],
        vec![0, 1, 2],
        vec![vec![1, 0], vec![2, 0], vec![0, 0]],
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

fn monoid_z2() -> Category {
    Category {
        objects: vec!["*".into()],
        morphisms: vec![vec![
            Morphism {
                name: "e".into(),
                target: 0,
            },
            Morphism {
                name: "t".into(),
                target: 0,
            },
        ]],
        table: vec![vec![vec![0, 1], vec![1, 0]]],
    }
}

fn discrete(objs: &[&str]) -> Category {
    let objects = objs.iter().map(|s| (*s).to_string()).collect();
    let morphisms = (0..objs.len())
        .map(|i| {
            vec![Morphism {
                name: format!("id_{}", objs[i]),
                target: i,
            }]
        })
        .collect();
    let table = (0..objs.len()).map(|_| vec![vec![0]]).collect();
    Category {
        objects,
        morphisms,
        table,
    }
}

#[test]
fn poly_display() {
    let p = small_p();
    insta::assert_snapshot!(format!("aggregate: {}\nnamed:     {}", p, p.show_named()));
}

#[test]
fn poly_constants() {
    let mut out = String::new();
    writeln!(out, "zero       = {}", Poly::zero()).unwrap();
    writeln!(out, "one        = {}", Poly::one()).unwrap();
    writeln!(out, "y          = {}", Poly::y()).unwrap();
    writeln!(out, "y^3        = {}", Poly::monomial("p", 3)).unwrap();
    writeln!(
        out,
        "2y^2 + y   = {}",
        sum(&Poly::term(2, 2), &Poly::term(1, 1))
    )
    .unwrap();
    insta::assert_snapshot!(out);
}

#[test]
fn poly_sum() {
    let p = small_p();
    let q = small_q();
    let s = sum(&p, &q);
    insta::assert_snapshot!(format!("{}\n{}", s, s.show_named()));
}

#[test]
fn poly_tensor() {
    let p = small_p();
    let q = small_q();
    let t = tensor(&p, &q);
    insta::assert_snapshot!(format!("{}\n{}", t, t.show_named()));
}

#[test]
fn poly_composition() {
    let p = small_p();
    let q = small_q();
    let c = compose(&p, &q);
    insta::assert_snapshot!(format!("{}\n{}", c, c.show_named()));
}

#[test]
fn poly_compose_with_identity() {
    let p = small_p();
    let lhs = compose(&p, &Poly::y());
    let rhs = compose(&Poly::y(), &p);
    insta::assert_snapshot!(format!(
        "P <| y = {}\ny <| P = {}",
        lhs.show_named(),
        rhs.show_named()
    ));
}

#[test]
fn lens_identity() {
    let p = small_p();
    insta::assert_snapshot!(format!("{}", Lens::id(&p)));
}

#[test]
fn lens_compose() {
    let p = Poly::new(vec![Position::of_arity("a", 2), Position::of_arity("b", 1)]);
    let q = Poly::new(vec![Position::of_arity("x", 1), Position::of_arity("y", 2)]);
    let r = Poly::new(vec![Position::of_arity("u", 1)]);

    let f = Lens::new(p, q.clone(), vec![1, 0], vec![vec![0, 1], vec![0]]);
    let g = Lens::new(q, r, vec![0, 0], vec![vec![0], vec![1]]);
    let h = Lens::compose(&f, &g).unwrap();
    insta::assert_snapshot!(format!("f:\n{}g:\n{}h = g . f:\n{}", f, g, h));
}

#[test]
fn moore_parity_trace() {
    let m = parity();
    let trace = m.run(0, &[1, 1, 0, 1, 0, 1]);
    insta::assert_snapshot!(format!(
        "inputs: 1 1 0 1 0 1\ntrace : {}\nlens:\n{}",
        m.show_trace(&trace),
        m.as_lens()
    ));
}

#[test]
fn moore_traffic_light_trace() {
    let m = traffic_light();
    let trace = m.run(0, &[0, 0, 0, 0, 1, 0, 0]);
    insta::assert_snapshot!(format!(
        "inputs: tick tick tick tick emergency tick tick\ntrace : {}",
        m.show_trace(&trace)
    ));
}

#[test]
fn wiring_parallel() {
    let m = parallel(&parity(), &parity());
    let trace = m.run(0, &[0, 1, 2, 3]);
    insta::assert_snapshot!(format!(
        "states  : {}\ninputs  : {}\noutputs : {}\ntrace   : {}",
        m.states.join(","),
        m.inputs.join(","),
        m.outputs.join(","),
        m.show_trace(&trace)
    ));
}

#[test]
fn wiring_series_two_parities() {
    let m = series(&parity(), &parity());
    let trace = m.run(0, &[1, 1, 1, 1, 1]);
    insta::assert_snapshot!(format!(
        "series(parity, parity)\ntrace: {}",
        m.show_trace(&trace)
    ));
}

#[test]
fn comonoid_walking_arrow() {
    let cat = walking_arrow();
    cat.check_axioms().expect("valid category");
    let c = cat.comonoid();
    let mut out = String::new();
    writeln!(out, "P(C) = {}", c.poly.show_named()).unwrap();
    writeln!(out, "epsilon:").unwrap();
    write!(out, "{}", c.epsilon).unwrap();
    writeln!(out, "delta:").unwrap();
    write!(out, "{}", c.delta).unwrap();
    insta::assert_snapshot!(out);
}

#[test]
fn comonoid_z2() {
    let cat = monoid_z2();
    cat.check_axioms().expect("Z/2 is a valid monoid");
    let c = cat.comonoid();
    let mut out = String::new();
    writeln!(out, "P(C) = {}", c.poly.show_named()).unwrap();
    writeln!(out, "epsilon:").unwrap();
    write!(out, "{}", c.epsilon).unwrap();
    writeln!(out, "delta:").unwrap();
    write!(out, "{}", c.delta).unwrap();
    insta::assert_snapshot!(out);
}

#[test]
fn comonoid_discrete() {
    let cat = discrete(&["red", "green", "blue"]);
    cat.check_axioms().expect("discrete category is valid");
    let c = cat.comonoid();
    insta::assert_snapshot!(format!(
        "P(C) = {}\nepsilon:\n{}delta:\n{}",
        c.poly.show_named(),
        c.epsilon,
        c.delta
    ));
}
