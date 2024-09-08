use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;

struct ECNode {
    len: usize,
    suffix_link: Option<usize>,
    transitions: HashMap<char, usize>,
    terminal: bool,
}

fn build_suffix_automaton(word: &str) -> Vec<ECNode> {
    let mut automaton: Vec<ECNode> = Vec::new();
    automaton.push(ECNode {
        len: 0,
        suffix_link: None,
        transitions: HashMap::new(),
        terminal: false,
    });

    let mut last = 0;
    for c in word.chars() {
        // add new Equivalence Class (EC)
        automaton.push(ECNode {
            len: automaton[last].len + 1,
            suffix_link: None,
            transitions: HashMap::new(),
            terminal: false,
        });
        let curr = automaton.len() - 1;

        // find if suffix exists for new EC
        // check all suffixes of last EC for transition with c char
        // * if no transition then add transition to new EC
        // * if transition exists then break
        let mut last_suffix = Some(last);
        while let Some(pos) = last_suffix {
            match automaton[pos].transitions.entry(c) {
                Entry::Occupied(_) => break,
                Entry::Vacant(e) => {
                    e.insert(curr);
                    last_suffix = automaton[pos].suffix_link;
                }
            }
        }
        if let Some(p) = last_suffix {
            // Suffix q for new EC exists
            let &q = automaton[p]
                .transitions
                .get(&c)
                .expect("Node should contain current transition");

            // p is a proper suffix of last EC
            // check if q is a proper suffix of new EC
            if automaton[p].len + 1 == automaton[q].len {
                // q is a proper suffix of new EC
                automaton[curr].suffix_link = Some(q);
            } else {
                // clone q as it is a part of some more specific (longer) EC
                automaton.push(ECNode {
                    len: automaton[p].len + 1,
                    suffix_link: automaton[q].suffix_link,
                    transitions: automaton[q].transitions.clone(),
                    terminal: false,
                });
                let clone = automaton.len() - 1;

                //clone is a suffix of q
                automaton[q].suffix_link = Some(clone);

                // update remaining suffixes of last EC to transition to clone
                while let Some(pos) = last_suffix {
                    if let Entry::Occupied(mut e) = automaton[pos].transitions.entry(c) {
                        if *e.get() == q {
                            e.insert(clone);
                            last_suffix = automaton[pos].suffix_link;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                automaton[curr].suffix_link = Some(clone);
            }
        } else {
            // Suffix for new EC does not exist
            automaton[curr].suffix_link = Some(0);
        }
        last = curr;
    }
    automaton[last].terminal = true;
    while let Some(next_pos) = automaton[last].suffix_link {
        automaton[next_pos].terminal = true;
        last = next_pos;
    }
    automaton
}

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Input should be valid UTF-8 string");
    let message = buf.trim();
    let size = message.chars().count();
    let automaton = build_suffix_automaton(message);
    let mut merge_detected = false;
    let mut state = 0;
    for (i, c) in message.chars().enumerate() {
        state = *automaton[state]
            .transitions
            .get(&c)
            .expect("Automaton should contain all valid transitions.");
        if i + 1 < size && i >= size / 2 && automaton[state].terminal {
            println!("Yes");
            let word = &message[0..automaton[state].len];
            println!("{word}");
            merge_detected = true;
            break;
        }
    }
    if !merge_detected {
        println!("No")
    }
}
