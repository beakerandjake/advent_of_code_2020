use std::collections::HashMap;
use std::error::Error;

type BagMap<'a> = HashMap<&'a str, Bag<'a>>;
type ChildBag<'a> = (u32, &'a str);

#[derive(Debug)]
struct Bag<'a> {
    input: &'a str,
    word_breaks: Vec<usize>,
    name: &'a str,
}

impl<'a> Bag<'a> {
    fn new(input: &'a str) -> Bag {
        // in order to parse the input string we just need to collect the word breaks so
        // we can jump from word to word.
        let mut word_breaks = Vec::new();

        for (index, character) in input.chars().enumerate() {
            if character.is_whitespace() {
                word_breaks.push(index);
            }
        }

        // expect name is first two words.
        let name = &input[..word_breaks[1]];

        Bag {
            input,
            word_breaks,
            name,
        }
    }

    // returns an iterator that iterates over all of the child bags
    fn children(&self) -> BagIter {
        BagIter {
            bag: self,
            child_index: 0,
        }
    }

    fn get_child(&self, child_index: usize) -> Option<ChildBag<'a>> {
        // children start at break 4 and repeat every 4 breaks
        let word_break_index = child_index * 4 + 4;

        // bail if requested child doesn't exist.
        if word_break_index >= self.word_breaks.len() {
            return None;
        }
        // expect count is first word of child.
        let count_str = &self.input
            [self.word_breaks[word_break_index - 1] + 1..self.word_breaks[word_break_index]];

        // bail if no children.
        if count_str == "no" {
            return None;
        }

        // expect count is a number.
        let parsed_count: u32 = count_str.parse().unwrap();

        // expect next two words after count are the child name.
        let name = &self.input
            [self.word_breaks[word_break_index] + 1..self.word_breaks[word_break_index + 2]];

        Some((parsed_count, name))
    }
}

// lets us iterate over the children without pre-parsing
pub struct BagIter<'a> {
    bag: &'a Bag<'a>,
    child_index: usize,
}

impl<'a> Iterator for BagIter<'a> {
    type Item = ChildBag<'a>;
    fn next(&mut self) -> Option<ChildBag<'a>> {
        let next = self.bag.get_child(self.child_index);
        self.child_index += 1;
        next
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let bags: BagMap = input
        .lines()
        .map(|line| {
            let bag = Bag::new(line);
            (bag.name, bag)
        })
        .collect();

    let mut visited = HashMap::new();
    visited.insert("shiny gold", true);

    let count = bags
        .keys()
        .filter(|bag| search_part1(bag, &bags, &mut visited))
        .count()
        - 1;

    Ok(count.to_string())
}

fn search_part1<'a>(
    current_bag: &'a str,
    all_bags: &'a HashMap<&'a str, Bag<'a>>,
    visited: &mut HashMap<&'a str, bool>,
) -> bool {
    match visited.get(current_bag) {
        Some(&v) => return v,
        None => {
            let found = all_bags[current_bag]
                .children()
                .any(|(_, name)| search_part1(name, all_bags, visited));

            visited.insert(current_bag, found);

            return found;
        }
    }
}

pub fn part2(_input: &str) -> Result<String, Box<dyn Error>> {
    unimplemented!();
}
