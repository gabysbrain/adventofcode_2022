
use std::fs;
use std::collections::BinaryHeap;

const INPUTPATH: &str = "input";

type Elf = i64;

fn read_input(path: &str) -> Vec<Elf> {
  let contents = fs::read_to_string(path)
      .expect("Can't read the file :(");

  let mut elves = Vec::new();
  let mut elf = 0;
  for line in contents.lines() {
    if line != "" {
      let cals: i64 = line.parse()
          .expect("cannot parse {line} into number");
      elf += cals;
    } else {
      elves.push(elf);
      elf = 0; // reset
    }
  }

  // may be one more
  if elf > 0 {
    elves.push(elf);
  }

  return elves;
}

fn max_cals(elves: &Vec<Elf>) -> i64 {
  let mut cur_max = 0i64;
  for elf in elves {
    if elf > &cur_max {
      cur_max = *elf;
    }
  }

  return cur_max;
}

fn top3_cals(elves: &Vec<Elf>) -> [i64; 3] {
  // use a heap because the top 3 elements will be in the first 3 elements 
  // of the heap array (root and 2 children)
  let mut heap = BinaryHeap::from(elves.clone());
  let top3 = [ heap.pop().expect("heap is size 0"), 
               heap.pop().expect("heap is size 1"), 
               heap.pop().expect("heap is size 2") ];
  //println!("{:?}", top3);
  return top3;
}

fn main() {
  let elves = read_input(INPUTPATH);
  //println!("{}", elves.len());
  //println!("{elves:?}");
  let max_cals = max_cals(&elves);
  println!("max cal elf: {max_cals}");

  let top3 = top3_cals(&elves);
  let top3_ttl: i64 = top3.iter().sum();
  println!("top3 elves: {:?}: {}", top3, top3_ttl);
}

