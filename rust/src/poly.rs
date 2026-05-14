use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub name: String,
    pub dirs: Vec<String>,
}

impl Position {
    pub fn new<S: Into<String>>(name: S, dirs: Vec<String>) -> Self {
        Self {
            name: name.into(),
            dirs,
        }
    }

    pub fn of_arity<S: Into<String>>(name: S, n: usize) -> Self {
        let dirs = (0..n).map(|i| format!("d{i}")).collect();
        Self {
            name: name.into(),
            dirs,
        }
    }

    pub const fn arity(&self) -> usize {
        self.dirs.len()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly {
    pub positions: Vec<Position>,
}

impl Poly {
    pub const fn new(positions: Vec<Position>) -> Self {
        Self { positions }
    }

    pub const fn zero() -> Self {
        Self {
            positions: Vec::new(),
        }
    }

    pub fn one() -> Self {
        Self {
            positions: vec![Position::new("*", vec![])],
        }
    }

    pub fn y() -> Self {
        Self {
            positions: vec![Position::new("y", vec!["d0".into()])],
        }
    }

    pub fn monomial<S: Into<String>>(name: S, n: usize) -> Self {
        Self {
            positions: vec![Position::of_arity(name, n)],
        }
    }

    pub fn term(count: usize, arity: usize) -> Self {
        let positions = (0..count)
            .map(|i| Position::of_arity(format!("p{i}"), arity))
            .collect();
        Self { positions }
    }

    pub const fn npositions(&self) -> usize {
        self.positions.len()
    }

    pub fn arity(&self, i: usize) -> usize {
        self.positions[i].arity()
    }

    pub fn signature(&self) -> Vec<(usize, usize)> {
        let mut counts: BTreeMap<usize, usize> = BTreeMap::new();
        for p in &self.positions {
            *counts.entry(p.arity()).or_default() += 1;
        }
        counts.into_iter().rev().collect()
    }

    pub fn show_aggregate(&self) -> String {
        if self.positions.is_empty() {
            return "0".into();
        }
        let parts: Vec<String> = self
            .signature()
            .into_iter()
            .map(|(arity, count)| match (count, arity) {
                (c, 0) => c.to_string(),
                (1, 1) => "y".into(),
                (c, 1) => format!("{c}y"),
                (1, a) => format!("y^{a}"),
                (c, a) => format!("{c}y^{a}"),
            })
            .collect();
        parts.join(" + ")
    }

    pub fn show_named(&self) -> String {
        if self.positions.is_empty() {
            return "0".into();
        }
        let parts: Vec<String> = self
            .positions
            .iter()
            .map(|p| match p.dirs.len() {
                0 => format!("[{}]", p.name),
                _ => format!("[{}]y^{{{}}}", p.name, p.dirs.join(",")),
            })
            .collect();
        parts.join(" + ")
    }
}

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.show_aggregate())
    }
}
