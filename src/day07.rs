use std::collections::HashMap;
use std::error::Error;

type BagName<'a> = (&'a str, &'a str);
type ChildBag<'a> = (u32, BagName<'a>);
type BagMap<'a> = HashMap<BagName<'a>, Bag<'a>>;

#[derive(Eq, Hash)]
struct Bag<'a> {
    words: Vec<&'a str>,
}

impl<'a> Bag<'a> {
    fn new(input: &'a str) -> Bag {
        Bag {
            words: input.split_whitespace().collect(),
        }
    }

    fn name(&self) -> BagName<'a> {
        (self.words[0], self.words[1])
    }

    fn children(&self) -> BagIter {
        BagIter {
            bag: self,
            child_index: 0,
        }
    }
}

impl<'a> PartialEq for Bag<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.words[0] == other.words[0] && self.words[1] == other.words[1]
    }
}

pub struct BagIter<'a> {
    bag: &'a Bag<'a>,
    child_index: usize,
}

impl<'a> Iterator for BagIter<'a> {
    type Item = ChildBag<'a>;
    fn next(&mut self) -> Option<ChildBag<'a>> {
        let index = self.child_index * 4 + 4;

        if index >= self.bag.words.len() {
            return None;
        }

        let count = self.bag.words[index];

        if count == "no" {
            return None;
        }

        let parsed_count: u32 = count.parse().unwrap();
        let name = (self.bag.words[index + 1], self.bag.words[index + 2]);

        self.child_index += 1;

        Some((parsed_count, name))
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let bags: BagMap = input
        .lines()
        .map(|line| {
            let bag = Bag::new(line);
            (bag.name(), bag)
        })
        .collect();

    let target_bag = ("shiny", "gold");
    let mut found_count = 0;
    let mut visited: HashMap<BagName, bool> = HashMap::new();

    for bag in &bags {
        if *bag.0 == target_bag {
            continue;
        }

        if search_part1(bag.1, target_bag, &bags, &mut visited) {
            visited.insert(*bag.0, true);
            found_count += 1;
        }
    }

    Ok(found_count.to_string())
}

fn search_part1<'a>(
    current_bag: &'a Bag<'a>,
    target_bag_name: BagName<'a>,
    all_bags: &'a BagMap<'a>,
    visited: &mut HashMap<BagName<'a>, bool>,
) -> bool {
    let current_bag_name = current_bag.name();

    if let Some(&v) = visited.get(&current_bag_name) {
        return v;
    }

    if current_bag_name == target_bag_name {
        return true;
    }

    current_bag.children().any(|x| {
        let child_bag = all_bags.get(&x.1).unwrap();
        let found = search_part1(child_bag, target_bag_name, &all_bags, visited);
        visited.insert(x.1, found);
        found
    })
}

pub fn part2(_input: &str) -> Result<String, Box<dyn Error>> {
    unimplemented!();
}
