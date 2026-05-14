use std::fmt;

use crate::poly::Poly;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lens {
    pub src: Poly,
    pub tgt: Poly,
    pub on_pos: Vec<usize>,
    pub on_dir: Vec<Vec<usize>>,
}

impl Lens {
    pub fn new(src: Poly, tgt: Poly, on_pos: Vec<usize>, on_dir: Vec<Vec<usize>>) -> Self {
        let l = Self {
            src,
            tgt,
            on_pos,
            on_dir,
        };
        l.validate().expect("invalid lens");
        l
    }

    pub fn id(p: &Poly) -> Self {
        let on_pos = (0..p.npositions()).collect();
        let on_dir = p
            .positions
            .iter()
            .map(|pos| (0..pos.arity()).collect())
            .collect();
        Self {
            src: p.clone(),
            tgt: p.clone(),
            on_pos,
            on_dir,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.on_pos.len() != self.src.npositions() {
            return Err(format!(
                "on_pos has {} entries, expected {}",
                self.on_pos.len(),
                self.src.npositions()
            ));
        }
        if self.on_dir.len() != self.src.npositions() {
            return Err(format!(
                "on_dir has {} entries, expected {}",
                self.on_dir.len(),
                self.src.npositions()
            ));
        }
        for (i, &j) in self.on_pos.iter().enumerate() {
            if j >= self.tgt.npositions() {
                return Err(format!("on_pos[{i}] = {j} out of range"));
            }
            let expected = self.tgt.arity(j);
            let got = self.on_dir[i].len();
            if got != expected {
                return Err(format!(
                    "on_dir[{i}] has {got} entries, expected {expected} (arity of tgt[{j}])"
                ));
            }
            let src_arity = self.src.arity(i);
            for (k, &d) in self.on_dir[i].iter().enumerate() {
                if d >= src_arity {
                    return Err(format!(
                        "on_dir[{i}][{k}] = {d} out of range (src arity {src_arity})"
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn compose(f: &Self, g: &Self) -> Result<Self, String> {
        if f.tgt != g.src {
            return Err("cannot compose: f.tgt != g.src".into());
        }
        let on_pos: Vec<usize> = f.on_pos.iter().map(|&p| g.on_pos[p]).collect();
        let on_dir: Vec<Vec<usize>> = (0..f.src.npositions())
            .map(|i| {
                let mid = f.on_pos[i];
                g.on_dir[mid]
                    .iter()
                    .map(|&r_dir| f.on_dir[i][r_dir])
                    .collect()
            })
            .collect();
        Ok(Self {
            src: f.src.clone(),
            tgt: g.tgt.clone(),
            on_pos,
            on_dir,
        })
    }
}

impl fmt::Display for Lens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "lens {} -> {}",
            self.src.show_named(),
            self.tgt.show_named()
        )?;
        for (i, p) in self.src.positions.iter().enumerate() {
            let tgt_idx = self.on_pos[i];
            let tgt_pos = &self.tgt.positions[tgt_idx];
            write!(f, "  {} |-> {}", p.name, tgt_pos.name)?;
            if tgt_pos.arity() > 0 {
                let pullback: Vec<String> = tgt_pos
                    .dirs
                    .iter()
                    .zip(self.on_dir[i].iter())
                    .map(|(td, &si)| format!("{td}<-{}", p.dirs[si]))
                    .collect();
                write!(f, " [{}]", pullback.join(", "))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
