use std::fs::File;
use std::io::{Read, Write};

enum SubstringRepetition {
    /// twice means the number must be representable
    /// by a substring which is repeacted twice
    Twice,
    /// the number in digit form can be a substring
    /// repeated as many tiems as you want (at least two)
    AtLeastTwice,
}

/// parses the input file and returns a
/// iterator whose items tuple which is a (usize, usize)
fn parse_input(fname: impl AsRef<std::path::Path>) -> impl Iterator<Item = (u64, u64)> {
    let mut f = File::open(fname.as_ref()).unwrap();
    let mut buf = String::new();
    // just read the whole damn thing
    f.read_to_string(&mut buf).unwrap();

    let buf: Vec<_> = buf.trim().split(',').map(|s| s.to_string()).collect();
    buf.into_iter().map(|id_range| {
        id_range
            .split_once('-')
            .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
            .unwrap()
    })
}

/// sees if the number is valid
fn is_valid(id: u64, substring: SubstringRepetition) -> bool {
    let mut as_string = Vec::new();

    // we know we are ascii here so gonna do some silly stuff
    write!(&mut as_string, "{id}").unwrap();
    let start = match substring {
        SubstringRepetition::Twice if as_string.len() == 1 => return true,
        SubstringRepetition::Twice => as_string.len() / 2,
        SubstringRepetition::AtLeastTwice => 1,
    };
    for j in start..=as_string.len() / 2 {
        if as_string.len() % j != 0 {
            continue;
        }
        let (all_equal, _) =
            as_string
                .chunks(j)
                .fold((true, None), |(all_equal, mut maybe_prior), cur_chunk| {
                    let all_equal = all_equal
                        && maybe_prior
                            .map(|prev_chunk| prev_chunk == cur_chunk)
                            .unwrap_or(true);
                    maybe_prior.replace(cur_chunk);
                    (all_equal, maybe_prior)
                });
        if all_equal {
            return false;
        }
    }
    true
}

fn main() {
    // Part a
    let mut sum = 0;
    for (start, end) in parse_input("input.txt") {
        for i in start..=end {
            if !is_valid(i, SubstringRepetition::Twice) {
                sum += i;
            }
        }
    }
    println!("Part A: `{sum}`");

    // Part b
    let mut sum = 0;
    for (start, end) in parse_input("input.txt") {
        for i in start..=end {
            if !is_valid(i, SubstringRepetition::AtLeastTwice) {
                sum += i;
            }
        }
    }
    println!("Part B: `{sum}`")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        for i in [12] {
            assert!(is_valid(i, SubstringRepetition::Twice))
        }
        for i in [11, 22, 1188511885] {
            assert!(!is_valid(i, SubstringRepetition::Twice))
        }
    }
}
