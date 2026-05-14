// Conway's Game of Life on an 8x8 torus.
//
// Each cell is a tiny state machine with two states (dead, alive) and
// 256 possible inputs (one per configuration of its eight neighbours).
// We build that machine through the library as a Moore machine with
// interface polynomial 2 y^256: each readout reports "dead" or "alive",
// each input is "the next neighbourhood I'm about to see".
//
// The whole grid is then "all those cells wired together so that each
// one's input is read off from its neighbours' outputs". Spivak and Niu
// call this a closed wiring diagram (a section of the tensor of all
// the per-cell polynomials onto y); from the cell's point of view there
// is no external input, the grid is closed.
//
// We don't expand the joint state space (2^64 states would be enormous),
// so the evolution is done by hand on a fixed initial pattern. The cell
// machine printed up front is the genuine library object; the runner
// below is exactly what the wiring section in Example 4.66 unfolds to.
//
// Spivak & Niu, Example 4.66 + Exercise 4.67 (Game of Life), p.118.

use tiny_poly::Moore;

const R: usize = 8;
const C: usize = 8;

fn cell_machine() -> Moore {
    let inputs: Vec<String> = (0..256).map(|n| format!("n{n:08b}")).collect();
    let update: Vec<Vec<usize>> = (0..2)
        .map(|s| {
            (0..256)
                .map(|n| {
                    let count = (n as u8).count_ones();
                    let alive = if s == 1 {
                        count == 2 || count == 3
                    } else {
                        count == 3
                    };
                    usize::from(alive)
                })
                .collect()
        })
        .collect();
    Moore::new(
        vec!["dead".into(), "alive".into()],
        inputs,
        vec!["dead".into(), "alive".into()],
        vec![0, 1],
        update,
    )
}

fn neighbour_mask(grid: &[Vec<u8>], r: usize, c: usize) -> usize {
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut mask = 0usize;
    for (k, (dr, dc)) in offsets.iter().enumerate() {
        let nr = (r as isize + dr).rem_euclid(R as isize) as usize;
        let nc = (c as isize + dc).rem_euclid(C as isize) as usize;
        if grid[nr][nc] == 1 {
            mask |= 1 << k;
        }
    }
    mask
}

fn step(cell: &Moore, grid: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut next = vec![vec![0u8; C]; R];
    for r in 0..R {
        for c in 0..C {
            let s = grid[r][c] as usize;
            let mask = neighbour_mask(grid, r, c);
            next[r][c] = cell.update[s][mask] as u8;
        }
    }
    next
}

fn show(grid: &[Vec<u8>]) -> String {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|&c| if c == 1 { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let cell = cell_machine();
    println!(
        "per-cell interface polynomial = {}",
        cell.interface_poly().show_aggregate()
    );
    println!("grid                          = {R}x{C} torus");

    let mut grid = vec![vec![0u8; C]; R];
    grid[3][2] = 1;
    grid[3][3] = 1;
    grid[3][4] = 1;

    for gen in 0..4 {
        println!();
        println!("generation {gen}:");
        println!("{}", show(&grid));
        grid = step(&cell, &grid);
    }
}
