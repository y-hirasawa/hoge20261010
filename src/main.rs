use std::env;
use std::io::{self, BufWriter, Write};
use std::process;

struct PowerSet {
    bits: Vec<bool>,
    first: bool,
    done: bool,
}

impl PowerSet {
    fn new(element_count: usize) -> Self {
        Self {
            bits: vec![false; element_count],
            first: true,
            done: false,
        }
    }
}

impl Iterator for PowerSet {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if self.first {
            self.first = false;
        } else {
            let mut carry = true;
            for bit in &mut self.bits {
                if carry {
                    if *bit {
                        *bit = false;
                    } else {
                        *bit = true;
                        carry = false;
                    }
                }
            }
            if carry {
                self.done = true;
                return None;
            }
        }

        Some(
            self.bits
                .iter()
                .enumerate()
                .filter_map(|(element, included)| included.then_some(element))
                .collect(),
        )
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let count = args
        .next()
        .ok_or_else(|| "Usage: powerset <element-count>".to_string())?
        .parse::<usize>()
        .map_err(|_| "element-count must be a non-negative integer".to_string())?;

    if args.next().is_some() {
        return Err("Usage: powerset <element-count>".to_string());
    }

    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());
    for subset in PowerSet::new(count) {
        writeln!(output, "{subset:?}")
            .map_err(|error| format!("failed to write output: {error}"))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::PowerSet;
    use std::collections::HashSet;

    #[test]
    fn empty_set_has_one_subset() {
        assert_eq!(PowerSet::new(0).collect::<Vec<_>>(), vec![vec![]]);
    }

    #[test]
    fn one_element_has_empty_and_full_subsets() {
        assert_eq!(PowerSet::new(1).collect::<Vec<_>>(), vec![vec![], vec![0]]);
    }

    #[test]
    fn three_elements_produce_all_eight_unique_subsets() {
        let subsets = PowerSet::new(3).collect::<Vec<_>>();
        let unique = subsets.iter().cloned().collect::<HashSet<_>>();

        assert_eq!(subsets.len(), 8);
        assert_eq!(unique.len(), 8);
        assert!(subsets
            .iter()
            .all(|subset| subset.iter().all(|element| *element < 3)));
    }
}
