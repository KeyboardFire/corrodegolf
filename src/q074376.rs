// http://codegolf.stackexchange.com/q/74376/3808

const NUM_DOTS: i32 = 6;

pub fn q074376(dominoes: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    // first, get the dominoes in a predictable order:
    // the first number should always be less than the second...
    let mut normalized_dominoes = dominoes.iter().map(|domino|
            if domino[0] < domino[1] { [domino[0], domino[1]] }
            else                     { [domino[1], domino[0]] }
        ).collect::<Vec<[i32; 2]>>();
    // ... and the dominoes should be sorted (by first number and then second)
    // Rust's default .cmp() does this nicely
    normalized_dominoes.sort();

    // now that we have the dominoes sorted, we can simply loop through all
    //   possible dominoes in order for the output array
    let mut missing_dominoes: Vec<[i32; 2]> = Vec::new();
    let mut dominoes_iter = normalized_dominoes.iter().peekable();
    for a in 0..NUM_DOTS+1 {
        for b in a..NUM_DOTS+1 {
            if dominoes_iter.peek().map_or(false, |domino| domino == &&[a, b]) {
                // this domino was present in the input
                // hence, do not add it to the output array
                dominoes_iter.next();
            } else {
                missing_dominoes.push([a, b]);
            }
        }
    }

    // a nice side effect of the way we're doing things is that the output
    //   is guaranteed to come in the "normalized" format
    missing_dominoes
}

#[test]
fn test074376() {
    assert_eq!(q074376(vec![[0, 0], [0, 1], [0, 2], [0, 3], [0, 4], [0, 5], [0, 6], [1, 1], [1, 2], [1, 3], [1, 4], [1, 5], [1, 6], [2, 2], [2, 3], [2, 4], [2, 5], [2, 6], [3, 3], [3, 4], [3, 5], [3, 6], [4, 4], [4, 5], [4, 6], [5, 5], [5, 6], [6, 6]]), Vec::<[i32; 2]>::new());
    assert_eq!(q074376(vec![[0, 0], [1, 0], [1, 1], [2, 0], [2, 1], [2, 2], [3, 0], [3, 1], [3, 2], [3, 3], [4, 0], [4, 1], [4, 2], [4, 3], [4, 4], [5, 0], [5, 1], [5, 2], [5, 3], [5, 4], [5, 5], [6, 0], [6, 1], [6, 2], [6, 3], [6, 4], [6, 5], [6, 6]]), Vec::<[i32; 2]>::new());
    assert_eq!(q074376(vec![[0, 0], [0, 1], [0, 2], [0, 3], [0, 4], [0, 5], [0, 6], [1, 1], [1, 2], [1, 3], [1, 4], [1, 5], [1, 6], [2, 2], [2, 3], [2, 4], [2, 5], [2, 6], [3, 4], [3, 5], [3, 6], [4, 4], [4, 5], [4, 6], [5, 5], [5, 6], [6, 6]]), vec![[3, 3]]);
    assert_eq!(q074376(vec![[0, 0], [0, 2], [0, 3], [0, 4], [0, 5], [0, 6], [1, 1], [1, 3], [1, 4], [1, 5], [1, 6], [2, 2], [2, 4], [2, 5], [2, 6], [3, 3], [3, 5], [3, 6], [4, 4], [4, 6], [5, 5], [6, 6]]), vec![[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [5, 6]]);
    assert_eq!(q074376(vec![]), vec![[0, 0], [0, 1], [0, 2], [0, 3], [0, 4], [0, 5], [0, 6], [1, 1], [1, 2], [1, 3], [1, 4], [1, 5], [1, 6], [2, 2], [2, 3], [2, 4], [2, 5], [2, 6], [3, 3], [3, 4], [3, 5], [3, 6], [4, 4], [4, 5], [4, 6], [5, 5], [5, 6], [6, 6]]);
}
