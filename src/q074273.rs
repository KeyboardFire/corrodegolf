// http://codegolf.stackexchange.com/q/74273/3808

// this is implemented as an infinite iterator
// letters is the input array, curr is the thing we're about to yield
// curr is stored as an easier format and converting to a string on the fly
pub struct AllStrings {
    letters: Vec<char>,
    curr: Vec<usize>
}

impl Iterator for AllStrings {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        // since we have to return curr and then do more processing, store
        //   this first
        // convert the indices to an actual string
        let prev: String = self.curr.iter().rev().map(|ch| self.letters[*ch])
            .collect();

        // add one to the "least significant" index, carrying for as long as
        //   needed
        // add_char is a flag signifying whether the end of the list was
        //   reached while carrying, which means we need to add another zero
        //   to the end of curr
        let mut add_char = true;
        for idx in 0..self.curr.len() {
            self.curr[idx] += 1;
            // is there a "carry"/"overflow"?
            if self.curr[idx] == self.letters.len() {
                // reset the current index to zero
                self.curr[idx] = 0;
            } else {
                // no carry, didn't reach the end, don't add a char
                add_char = false;
                break;
            }
        }

        // increase the length of curr if necessary
        if add_char { self.curr.push(0); }

        // yield whatever we had before this
        Some(prev)
    }
}

pub fn q074273(letters: String) -> AllStrings {
    AllStrings { letters: letters.chars().collect(), curr: Vec::new() }
}

#[test]
fn test074376() {
    assert_eq!(q074273("ab".to_string()).take(9).collect::<Vec<String>>(), vec!["", "a", "b", "aa", "ab", "ba", "bb", "aaa", "aab"]);
}
