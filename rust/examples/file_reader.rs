// A file reader, and the same reader wrapped into a file searcher.
//
// The reader walks a fixed file one character at a time. Its only input
// is "advance", and at each step it shows the character at the cursor;
// when it falls off the end it shows EOF. As a Moore machine, that's
// states 0..length with a one-step update and a readout that maps each
// cursor to its character.
//
// The searcher is a one-character pattern matcher: it watches a stream
// of characters and flips a "found" flag whenever it sees the target
// (here 'L'). On its own it has nothing to do with files.
//
// The interesting move is the wiring. We pipe the reader's output into
// the searcher's input via `series`. The result is a new Moore machine
// whose state is the pair (reader cursor, searcher memory) and whose
// only external input is still "advance": one button, one verdict per
// step. You did not have to write any plumbing; the polynomial framing
// hands you the wired machine for free, and it would have hooked up any
// two machines with compatible interfaces the same way.
//
// Spivak & Niu, Exercise 4.14 (File reader) + Exercise 4.40 (wrapper
// turning a reader into a searcher), pp.87, 101.

use tiny_poly::{series, Moore};

fn main() {
    let file: Vec<char> = "HELLO".chars().collect();
    let alphabet: Vec<String> = "HELO".chars().map(|c| c.to_string()).collect();
    let mut outputs = alphabet.clone();
    outputs.push("EOF".into());

    let states: Vec<String> = (0..=file.len()).map(|i| format!("c{i}")).collect();
    let readout: Vec<usize> = (0..=file.len())
        .map(|i| {
            if i < file.len() {
                outputs.iter().position(|o| o.starts_with(file[i])).unwrap()
            } else {
                outputs.len() - 1
            }
        })
        .collect();
    let update: Vec<Vec<usize>> = (0..=file.len())
        .map(|i| vec![(i + 1).min(file.len())])
        .collect();

    let reader = Moore::new(
        states,
        vec!["advance".into()],
        outputs.clone(),
        readout,
        update,
    );

    let target = 'L';
    let target_idx = outputs.iter().position(|o| o == "L").unwrap();
    let searcher_states: Vec<String> = outputs.iter().map(|o| format!("saw_{o}")).collect();
    let searcher_readout: Vec<usize> = (0..outputs.len())
        .map(|i| usize::from(i == target_idx))
        .collect();
    let searcher_update: Vec<Vec<usize>> = (0..outputs.len())
        .map(|_| (0..outputs.len()).collect())
        .collect();

    let searcher = Moore::new(
        searcher_states,
        outputs.clone(),
        vec!["skip".into(), "found".into()],
        searcher_readout,
        searcher_update,
    );

    let wired = series(&reader, &searcher);

    println!(
        "reader   interface: {}",
        reader.interface_poly().show_aggregate()
    );
    println!(
        "searcher interface: {}",
        searcher.interface_poly().show_aggregate()
    );
    println!(
        "wired    interface: {}",
        wired.interface_poly().show_aggregate()
    );

    let trace = wired.run(0, &vec![0; file.len() + 1]);
    println!();
    println!("file      : {}", file.iter().collect::<String>());
    println!("looking for: {target}");
    println!("trace     : {}", wired.show_trace(&trace));
}
