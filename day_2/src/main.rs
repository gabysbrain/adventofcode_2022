
use std::fs;

const GUIDEPATH: &str = "input";

#[derive(Clone, Copy)]
enum Play {
  Rock,
  Paper,
  Scissors
}

fn read_guide(path: &str) -> Vec<(Play, Play)> {
  let contents = fs::read_to_string(path)
      .expect("Can't read the file :(");

  let mut plays = Vec::new();
  for line in contents.lines() {
    let vals: Vec<&str> = line.split(" ").collect();
    let them = if vals[0] == "A" {
      Play::Rock
    } else if vals[0] == "B" {
      Play::Paper
    } else if vals[0] == "C" {
      Play::Scissors
    } else {
      panic!("Can't process line {line}");
    };
    // X = lose, Y = draw, Z = win
    let me = match vals[1] { 
      "X" => match them { // need to lose
        Play::Rock     => Play::Scissors,
        Play::Paper    => Play::Rock,
        Play::Scissors => Play::Paper
      },
      "Y" => them.clone(), // tie
      "Z" => match them {
        Play::Rock     => Play::Paper,
        Play::Paper    => Play::Scissors,
        Play::Scissors => Play::Rock
      }
      _ => panic!("Can't process line {line}")
    };
    plays.push((me, them));
  }

  return plays;
}

fn score(me: &Play, them: &Play) -> i64 {
  let shape = match &me {
    Play::Rock => 1,
    Play::Paper => 2,
    Play::Scissors => 3
  };

  let outcome = match (me, them) {
    (Play::Rock, Play::Rock) => 3,
    (Play::Rock, Play::Paper) => 0,
    (Play::Rock, Play::Scissors) => 6,
    (Play::Paper, Play::Rock) => 6,
    (Play::Paper, Play::Paper) => 3,
    (Play::Paper, Play::Scissors) => 0,
    (Play::Scissors, Play::Rock) => 0,
    (Play::Scissors, Play::Paper) => 6,
    (Play::Scissors, Play::Scissors) => 3,
  };

  return shape + outcome;
}

fn main() {
  let guide = read_guide(GUIDEPATH);
  let scores: Vec<i64> = guide.iter().map(|(me, them)| score(me, them)).collect();
  let ttl_score: i64 = scores.iter().sum();
  println!("total score: {}", ttl_score);
}

