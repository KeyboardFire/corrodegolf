// http://codegolf.stackexchange.com/q/74648/3808

pub fn q074648(s: &str) -> String {
    // step 1 (easy step: just remove ` and ')
    let deapostrophed = s.replace("`", "").replace("'", "");

    // step 2, split into words: the hard part is splitting on uppercase
    //   letters surrounded by lowercase letters
    // first, let's find their indices:
    let mut split_indices = deapostrophed
        // look for any uppercase letter...
        .match_indices(char::is_uppercase)
        // then choose only the ones surrounded by lowercase letters...
        .filter_map(|(idx, _)| {
        // avoid errors due to letters at either end of the string
        if idx > 0 && idx < deapostrophed.len() - 1 {
            let mut ch_iter = deapostrophed.chars().skip(idx - 1);
            if ch_iter.next().unwrap().is_lowercase() &&
                ch_iter.skip(1).next().unwrap().is_lowercase() {
                // found one, so add this index to the vec
                Some(idx)
            } else { None }
        } else { None }
    }).collect::<Vec<usize>>();
    // these indices specify boundaries, so add the ends of the string too
    split_indices.insert(0, 0);
    split_indices.push(deapostrophed.len());

    // now split the string at these indices
    // more specifically, get slices for every pair of indices
    let parts: Vec<&str> = split_indices.windows(2).flat_map(|x| {
        // also, split at non-alphanumberics while we're here
        let slice = &deapostrophed[x[0]..x[1]];
        slice.split(|ch: char| !ch.is_alphanumeric())
    }).collect();

    // now we have the list of parts/words
    // all that's left to do is fix case and join
    parts.iter().enumerate().map(|(part_idx, part)|
        if part_idx == 0 {
            // the first word is entirely lowercase...
            part.to_lowercase()
        } else {
            // all other words are title case
            part.chars().enumerate().map(|(i, c)|
                if i == 0 { c.to_uppercase().next() }
                else { c.to_lowercase().next() }
                    .unwrap()
            ).collect()
        }
    ).collect::<Vec<String>>().join("")
}

#[test]
fn test074648() {
    assert_eq!(q074648("Programming Puzzles & Code Golf"), "programmingPuzzlesCodeGolf");
    assert_eq!(q074648("XML HTTP request"), "xmlHttpRequest");
    assert_eq!(q074648("supports IPv6 on iOS?"), "supportsIpv6OnIos");
    assert_eq!(q074648("SomeThing w1th, apo'strophe's and' punc]tuation"), "someThingW1thApostrophesAndPuncTuation");
    assert_eq!(q074648("nothing special"), "nothingSpecial");
    assert_eq!(q074648("5pecial ca5e"), "5pecialCa5e");
    assert_eq!(q074648("1337"), "1337");
    assert_eq!(q074648("1337-spEAk"), "1337Speak");
    assert_eq!(q074648("abcD"), "abcd");
    assert_eq!(q074648("a"), "a");
    assert_eq!(q074648("B"), "b");
}
