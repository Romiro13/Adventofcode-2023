use itertools::Itertools;

use crate::Puzzle;

pub fn play(input: &str, puzzle: &Puzzle) -> i32 {
  if puzzle.eq(&Puzzle::Part01) {
    get_scratchcards_points(input)
  } else {
    get_win_more_scratchcards(input)
  }
}

fn get_win_more_scratchcards(input: &str) -> i32 {
  let lines = input
    .trim()
    .lines()
    .map(|l| l.trim())
    .collect::<Vec<&str>>();

  let games = lines
    .iter()
    .map(|l| {
      let v: Vec<&str> = l.trim().split(" | ").collect();
      let my_numbers = v[0].split(": ").collect::<Vec<&str>>();
      (my_numbers[1], v[1])
    })
    .collect::<Vec<(&str, &str)>>();

  let games = games
    .iter()
    .map(|t| {
      let numbers = t.0.split_whitespace().collect::<Vec<&str>>();
      let winning_numbers = t.1.split_whitespace().collect::<Vec<&str>>();
      (numbers, winning_numbers)
    })
    .collect::<Vec<_>>();

  let possible_instances = games.len();
  let mut instances = Vec::from_iter(std::iter::repeat(1).take(possible_instances));

  let mut i = 1; // start on second line
  for (my_numbers, winning_numbers) in games {
    let v = my_numbers
      .into_iter()
      .filter(|v| winning_numbers.contains(v))
      .collect::<Vec<&str>>();

    let copies = v.len() + i;
    let cards = instances[i - 1];
    for _ in 0..cards {
      (i..copies).for_each(|card| {
        instances[card] += 1;
      });
    }
    i += 1;
  }

  instances.iter().sum()
}

fn get_scratchcards_points(input: &str) -> i32 {
  let lines = input
    .trim()
    .lines()
    .map(|l| l.trim())
    .collect::<Vec<&str>>();

  let games = lines
    .iter()
    .map(|line| {
      let v: Vec<&str> = line.trim().split(" | ").collect();
      let my_numbers = v[0].split(": ").collect::<Vec<&str>>();
      (my_numbers[1], v[1])
    })
    .collect::<Vec<(&str, &str)>>();

  let games = games
    .iter()
    .map(|t| {
      let numbers = t.0.split_whitespace().collect::<Vec<&str>>();
      let winning_numbers = t.1.split_whitespace().collect::<Vec<&str>>();
      (numbers, winning_numbers)
    })
    .collect::<Vec<_>>();

  let mut points: Vec<i32> = Vec::new();

  for (my_numbers, winning_numbers) in games {
    let v = my_numbers
      .into_iter()
      .filter(|v| winning_numbers.contains(v))
      .collect::<Vec<&str>>();
    let exp = v.len() as u32;
    if exp > 0 {
      let exp = 2_i32.pow(exp - 1);
      points.push(exp);
    }
  }

  points.iter().sum()
}

// region:    --- Tests
#[cfg(test)]
mod tests {

  use super::*;
  use anyhow::Result;

  #[test]
  fn test_day04_play_part01_ok() -> Result<()> {
    let fx_input = r#"
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    let fx_expected = 13;

    let rs = play(fx_input, &Puzzle::Part01);

    assert_eq!(rs, fx_expected);

    Ok(())
  }

  #[test]
  fn test_day04_play_part02_ok() -> Result<()> {
    let fx_input = r#"
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    let fx_expected = 30;

    let rs = play(fx_input, &Puzzle::Part02);

    assert_eq!(rs, fx_expected);

    Ok(())
  }
}
// endregion: --- Tests
