#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::collections::HashMap;

// Deal hand to player one
// Deal hand to player two
// Set remaining cards

// --------------------

// input card list and return true false. If true, assign to that number
// e.g. Royal Flush = 1
// use a function for each check

// if royal flush
// elif straight flush (includes wrap arounds, e.g. Q, K, A, 1, 2)
// elif four of a kind
// elif full house
// elif flush
// elif straight
// elif three of a kind
// elif two pairs
// elif pair
// elif high card

// --------------------

// compare two hands
// if numbers are equal use following tie breakers:
//   straight flush      = high card (A is highest)
//   four of a kind      = 1 kicker
//   full house          = highest three of a kind, (3 in community, use highest pairs)
//   flush               = high card. If same, next highest & so on
//   straight            = high card
//   three of a kind     = highest three of a kind, (3 in commnity, use highest card)
//   two pair            = highest pair. If tie, second highest pair. If tie kicker
//   one pair            = highest pair. If tie, highest kicker. If still tie, next highest kicker, etc.
//   high card           = highest card, If tie, highest kicker. If still tie, next highest kicker, etc.

// ------------------

// determine winner
// determine suit
// return list

/******************************************************** HELPERS START ********************************************************/

/************************
  get_n_highest
    params: vector, n
    return: vector with n cards (ascending left to right)
************************/

fn get_n_highest(vec: &Vec<u32>, n: u32) -> Vec<u32> {

  let mut iter: Vec<(u32, u32)> = Vec::new();

  for card in vec.iter() {
    let mut ord = card_ord(*card);
    if card_ord(*card) == 1 {
      ord += 99;  // artificially set ord to 100 for aces
    }
    if card_ord(*card) == 0 {
      ord += 99;  // artificially set ord to 99 for kings
    }
    iter.push((*card, ord));
  }

  iter.sort_by(|a, b| b.1.cmp(&a.1));

  let mut result: Vec<u32> = iter
      .iter()
      .cloned()
      .take(n as usize)
      .map(|x| x.0)
      .collect();
    result.reverse();
    
  return result;
}

/************************
  is_higher
    finds the higher of two cards
    params: card we're comparing against, card we want to compare with
    return: boolean
************************/

fn is_higher(first: i32, second: i32) -> bool {
  
  // 53 is default value
  if first == 53 { return true }
  
  let o1 = card_ord(first as u32) as i32;
  let o2 = card_ord(second as u32) as i32;

  // compare ace high
  if o1 == 1 && o2 != 1 {
    return false
  } else if o2 == 1 && o1 != 1 {
    return true
  }
  // compare king high
  if o1 == 0 && o2 != 0 {
    return false
  } else if o2 == 0 && o1 != 0 {
    return true
  }

  if o1 > o2 {
    return false
  } else if o1 < o2 {
    return true
  }
  // TODO: handle tie? (return a tuple)
  // satisfy compiler
  false
}

/************************
  has_ace
    determines if a vector has an ace
*************************/

fn check_ace(vec: &Vec<u32>) -> (bool, u32) {
  let has_ace: Vec<u32> = vec
        .iter()
        .filter(|&card| card % 13 == 1)
        .cloned()
        .collect();
  if has_ace.len() > 0 {
    return (has_ace.len() > 0, has_ace[0]); // TODO: use Some/None check instead of directly accessing has_ace[0]
  } else {
    return (false, 0);  // 0 is meaningless - find better way
  }
}

/************************
  get_card_count
    determines if a vector has an ace
*************************/

fn get_card_count(pool: &Vec<u32>) -> HashMap<u32,u32>{
  let mut card_map: HashMap<u32, u32> = HashMap::new();
  let pool_ord: Vec<u32> = pool
    .iter()
    .cloned()
    .map(|card| card % 13)
    .collect();
  
  // use Entry API: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry
  for card in pool_ord.iter() {
    // get card's count entry; if doesn't exist set card_count & dict = 0
    let card_count: &mut u32 = 
      card_map
        .entry(*card)
        .or_insert(0);
    // card_count is a mut reference; we can change its value directly
    *card_count += 1;
  }
  
  return card_map;
}


/************************
  card_ord
************************/

fn card_ord(card: u32) -> u32 {
  card % 13
}

/************************
  high_from_suits
    finds the highest suit of a given card
************************/
fn high_from_suits(pool: &Vec<u32>, test: u32) -> (bool, u32) {
  let mut test_vec: Vec<u32> = Vec::new();
  let factor = 13;
  let mut testc = test;

  // set up comparison vector
  while testc > 13 {
    testc -= factor;
  }
  for i in 0..4 {
    test_vec.push(testc + (factor * i));
  }

  let mut highest: u32 = 0;
  for card in test_vec.iter() { // iterate immutably & do not consume vector
    if pool.contains(&card) && *card > highest { 
      highest = *card;
    }
  }
  
  if highest == 0 { (false, 0) }
  else { (true, highest) }
}

/************************
  is_king_high_straight
    return format: 10 11 12 13 9
************************/

fn is_king_high_straight(straight: &Vec<u32>) -> bool {
  let straight_ord: Vec<u32> = straight.iter().cloned().map(|x| x % 13).collect();
  if 
  straight_ord.contains(&9) &&
  straight_ord.contains(&0) &&
  straight_ord.contains(&12) &&
  straight_ord.contains(&11) &&
  straight_ord.contains(&10) {
    return true
  } else {
    return false
  }
}

/************************
  trim
************************/

fn trim(vec: &Vec<u32>, hand: u32) -> Vec<u32>{
  let mut result = vec.to_vec();
  match hand {
    7 => { result.truncate(4); return result },     // four-oak: trim last
    3 => { result.truncate(3); return result },     // three-oak: trim last 2
    2 => { result.truncate(4); return result },     // two-pair: trim last
    1 => { result.truncate(2); return result },     // one-pair: trim last 3
    0 => { return vec![result[4]] },                // high card: trim last 4
    _ => return result
  }
}

/************************
  display_hand
************************/

fn display_hand(hand: u8) -> &'static str {
  match hand {
    9 => "(Royal Flush)",
    8 => "(Straight Flush)",
    7 => "(Four of a Kind)",
    6 => "(Full House)",
    5 => "(Flush)",
    4 => "(Straight)",
    3 => "(Three of a Kind)",
    2 => "(Two Pair)",
    1 => "(One Pair)",
    0 => "(High Card)",
    _ => "Woops! Something went wrong."
  }
}

/******************************************************** HELPERS END ********************************************************/

/************************
is_royal_flush
************************/

fn is_royal_flush(pool: &Vec<u32>) -> (bool, Vec<u32>) {
  let mut result = false;
  let mut result_vec = Vec::new();

  for x in 0..4 {
    let factor = 13 * x;
    if pool.contains(&(10 + factor)) &&
        pool.contains(&(11 + factor)) &&
        pool.contains(&(12 + factor)) &&
        pool.contains(&(13 + factor)) &&
        pool.contains(&(1 + factor)) {
        result = true;
        for x in 0..5 {
          if x == 4 { result_vec.push(1 + factor) }
          else { result_vec.push(10 + x + factor) }
        }
        break; // not necessary?
        }
  }

  (result, result_vec)
}

/************************
  is_straight_flush
************************/

fn is_straight_flush(pool: &Vec<u32>) -> (bool, Vec<u32>) {
  let (is_flush, flush) = is_flush(pool);

  if is_flush {
    let (is_straight, straight) = is_straight(&flush);
    if is_straight { return (true, straight) } 
  }

  (false, Vec::new())
}

/************************
  is_four_oak
    return format: 1 1 1 1 2
************************/

fn is_four_oak(pool: &Vec<u32>) -> (bool, Vec<u32>) {
  let card_map: HashMap<u32, u32> = get_card_count(pool);
  let mut result_vec: Vec<u32> = Vec::new();

  // build foak hand
  for (ord, count) in card_map.iter() {
    if *count == 4 {

      for card in pool.iter() {
        if card_ord(*card) == *ord {
          result_vec.push(*card);
        }
      }
      result_vec.sort();

      let mut no_four_oak = pool.iter().cloned().collect::<Vec<u32>>();
      for card in result_vec.iter() {
        if no_four_oak.contains(card) {
          no_four_oak.retain(|x| x != card);  // retain = opposite of removal; operates on vector in place
        }
      }

      result_vec.append(&mut get_n_highest(&no_four_oak, 1));
      return (true, result_vec)
    }
  }

  (false, Vec::new())
}

/************************
  is_full_house
    return format: 1 1 1 2 2
************************/

fn is_full_house(pool: &Vec<u32>) -> (bool, Vec<u32>) {
  let (has_three_oak, three_oak) = is_three_oak(pool);

  if has_three_oak {
    let no_three_oak: Vec<u32> = pool
      .iter()
      .cloned()
      .filter(|x| (*x != three_oak[0]) && (*x!= three_oak[1]) && (*x != three_oak[2]))  // TODO: optimize
      .collect();
    
    let (has_two_pair, two_pair) = is_two_pair(&no_three_oak);
    let (has_one_pair, one_pair) = is_one_pair(&no_three_oak);
    let mut new_pair;
    let mut result = three_oak[0..3].to_vec();
    result.sort();

    if has_two_pair {
      new_pair = vec![two_pair[2], two_pair[3]];
    } else if !has_two_pair && has_one_pair {
      new_pair = vec![one_pair[0], one_pair[1]];
    } else {
      return (false, Vec::new())
    }

    new_pair.sort();  // TODO: double check if sorting already done in is_two_pair & is_one_pair
    result.append(&mut new_pair);
    return (true, result)

  }

  (false, Vec::new())
}

/************************
  is_flush
*************************/

fn is_flush(pool: &Vec<u32>) -> (bool, Vec<u32>){
  let mut result_vec: Vec<u32>;
  // rev: reverses 0 to 3 -> 3 to 0
  for x in (0..4).rev() {
    let factor = 13 * x;

    // https://doc.rust-lang.org/std/iter/index.html
    let mut potential_flush: Vec<u32> = pool
      .into_iter()
      .filter(|&card| (card >= &(1 + factor)) && (card <= &(13 + factor))) // why pass in references? Is this just API?
      .cloned()   // need to clone because otherwise will have vector of references
      .collect(); // produces a new collection
    potential_flush.sort_by(|a, b| b.cmp(a));

    if potential_flush.len() >= 5 {
      result_vec = potential_flush[0..4].to_vec();
      let (has_ace, ace) = check_ace(&potential_flush);
      let (has_straight, straight) = is_straight(&potential_flush);

      if !has_straight {  // TODO: should be in another fn
        if has_ace {
          result_vec.reverse();
          result_vec.push(ace);
        } else {
          result_vec.push(potential_flush[4]);
          result_vec.reverse();
        }
        return (true, result_vec)
      } else {
        return (true, straight)
      }
      
      // return (true, result_vec)
    }
  }

  (false, Vec::new())
}

/************************
  is_straight
************************/

fn is_straight(pool: &Vec<u32>) -> (bool, Vec<u32>) {

  for card in pool.iter() {
    // ace high
    let (has_ace, ace) =     high_from_suits(pool, 1);
    let (has_king, king) =   high_from_suits(pool, 13);
    let (has_queen, queen) = high_from_suits(pool, 12);
    let (has_jack, jack) =   high_from_suits(pool, 11);
    let (has_ten, ten) =     high_from_suits(pool, 10);
    if has_ace &&
        has_king &&
        has_queen &&
        has_jack &&
        has_ten {
      return (true, vec![ten, jack, queen, king, ace])
    } else {
      // for any given card, want see if there's 4 sequential ones down
      let ord = card_ord(*card);
      if ord >= 5 || ord == 0 {
        let (has_first, fifth) =    high_from_suits(pool, *card);
        let (has_second, fourth) =  high_from_suits(pool, *card - 1);
        let (has_third, third) =    high_from_suits(pool, *card - 2);
        let (has_fourth, second) =  high_from_suits(pool, *card - 3);
        let (has_fifth, first) =    high_from_suits(pool, *card - 4);
  
        if has_first && 
          has_second && 
          has_third && 
          has_fourth && 
          has_fifth {
            let result = vec![first, second, third, fourth, fifth];
            if is_king_high_straight(&result) {
              return (true, vec![second, third, fourth, fifth, first])
            }
            return (true, result)
        }
      }
    }
  }
  
  (false, Vec::new())
}

/************************
  is_three_oak
    return format: 1 1 1 2 3
************************/

fn is_three_oak(pool: &Vec<u32>) -> (bool, Vec<u32>) {
  let card_map: HashMap<u32, u32> = get_card_count(pool);
  let mut result_vec: Vec<u32> = Vec::new();

  for (ord, count) in card_map.iter() {
    if *count == 3 {
      
      // build 3-oak
      for card in pool.iter() {
        if card_ord(*card) == *ord {
          result_vec.push(*card);
        }
      }
      result_vec.sort();

      let mut no_three_oak = pool.iter().cloned().collect::<Vec<u32>>();
      for card in result_vec.iter() {
        if no_three_oak.contains(card) {
          no_three_oak.retain(|x| x != card);
        }
      }

      result_vec.append(&mut get_n_highest(&no_three_oak, 2));  // &mut - need to pass mutable reference to append
      return (true, result_vec)
    }
  }

  (false, Vec::new())
}

/************************
  is_two_pair
    return format: 1 1 2 2 3
************************/

fn is_two_pair(pool: &Vec<u32>) -> (bool, Vec<u32>) {
  let (has_first_pair, fst_pair) = is_one_pair(pool);
  let mut first_pair: Vec<u32> = Vec::new();
  let mut second_pair: Vec<u32> = Vec::new();
  
  if has_first_pair {
    // remove the first pair
    let mut no_first_pair = pool.iter().cloned().collect::<Vec<u32>>();
    for card in pool.iter() {
      if *card == fst_pair[0] || *card == fst_pair[1] {
        no_first_pair.retain(|x| x != card);
        first_pair.push(*card);
      }
    }
    first_pair.sort();

    let (has_scnd_pair, scnd_pair) = is_one_pair(&no_first_pair);
    let mut no_scnd_pair = scnd_pair.iter().cloned().collect::<Vec<u32>>(); // pool w/o 1st pair

    if has_scnd_pair {
      // remove the second pair
      for card in no_first_pair.iter() {
        if *card == scnd_pair[0] || *card == scnd_pair[1] {
          no_scnd_pair.retain(|x| x != card);
          second_pair.push(*card);
        }
      }
      second_pair.sort();

      // add kicker
      if is_higher(card_ord(second_pair[0]) as i32, card_ord(first_pair[0]) as i32) {
        second_pair.append(&mut first_pair);
        second_pair.append(&mut get_n_highest(&no_scnd_pair, 1)); // no_scnd_pair = pool w/o 1st & 2nd pairs
        return (true, second_pair);
      } else {
        first_pair.append(&mut second_pair);
        first_pair.append(&mut get_n_highest(&no_scnd_pair, 1));
        return (true, first_pair);
      }

    } else {
      return (false, Vec::new())
    }
  }
  
  (false, Vec::new())
}

/************************
  is_one_pair
    return format: 1 1 2 3 4
************************/

fn is_one_pair(pool: &Vec<u32>) -> (bool, Vec<u32>) {
  let card_map: HashMap<u32, u32> = get_card_count(pool);
  let mut result_vec: Vec<u32> = Vec::new();
  
  for (ord, count) in card_map.iter() {
    if *count == 2 {
      
      // build 2-oak
      for card in pool.iter() {
        if card_ord(*card) == *ord {
          result_vec.push(*card);
        }
      }
      result_vec.sort();

      let mut no_two_oak = pool.iter().cloned().collect::<Vec<u32>>();
      for card in result_vec.iter() {
        if no_two_oak.contains(card) {
          no_two_oak.retain(|x| x != card);
        }
      }

      result_vec.append(&mut get_n_highest(&no_two_oak, 3));
      return (true, result_vec)
    }
  }

  return (false, Vec::new())
}

fn high_card(pool: &Vec<u32>) -> Vec<u32> {
  return get_n_highest(pool, 5);
}
  
/************************
  hand_strength
************************/

fn hand_strength(pool: &Vec<u32>) -> (u8, Vec<u32>) {

  if is_royal_flush(pool).0         { (9, is_royal_flush(pool).1) }
  else if is_straight_flush(pool).0 { (8, is_straight_flush(pool).1) }
  else if is_four_oak(pool).0       { (7, is_four_oak(pool).1) }
  else if is_full_house(pool).0     { (6, is_full_house(pool).1) }
  else if is_flush(pool).0          { (5, is_flush(pool).1) }
  else if is_straight(pool).0       { (4, is_straight(pool).1) }
  else if is_three_oak(pool).0      { (3, is_three_oak(pool).1) }
  else if is_two_pair(pool).0       { (2, is_two_pair(pool).1) }
  else if is_one_pair(pool).0       { (1, is_one_pair(pool).1) }
  else                              { (0, high_card(pool)) }

}

/************************
  tie_break
************************/

//   9 royal flush         = by suit: S > H > D > C (not necessary for assignment; also Poker does not compare suits)
//   8 straight flush      = high card (A is highest)
//   7 four of a kind      = 1 kicker
//   6 full house          = highest three of a kind, (3 in community, use highest pairs)
//   5 flush               = high card. If same, next highest & so on
//   4 straight            = high card (never use suit; no split pot = never have 2x 45678)
//   3 three of a kind     = highest three of a kind, (3 in commnity, use highest card)
//   2 two pair            = highest pair. If tie, second highest pair. If tie kicker
//   1 one pair            = highest pair. If tie, highest kicker. If still tie, next highest kicker, etc.
//   0 high card           = highest card, If tie, highest kicker. If still tie, next highest kicker, etc.

fn tie_break(p1: &Vec<u32>, p2: &Vec<u32>, strength: u8) -> Vec<u32>{
  let p1_cpy = p1.to_vec();
  let p2_cpy = p2.to_vec();
  
  match strength {
    9 => { 
      if p1[4] > p2[4] { return p1_cpy } 
      else { return p2_cpy }
    },
    8 | 7 | 4 => {
      if is_higher(p2[4] as i32, p1[4] as i32) { return p1_cpy } 
      else { return p2_cpy }
    },
    6 => {
      if is_higher(p2[0] as i32, p1[0] as i32) {
        return p1_cpy
      } else if is_higher(p1[0] as i32, p2[0] as i32) {
        return p2_cpy
      } else {
        if is_higher(p2[3] as i32, p1[3] as i32) {
          return p1_cpy
        } else {
          return p2_cpy
        }
      }
    },
    5 => {
      if zip_biggest(p1, p2) == 1 { return p1_cpy }
      else { return p2_cpy }
    },
    3 => {
      if is_higher(p2[0] as i32, p1[0] as i32) {
        return p1_cpy
      } else if is_higher(p1[0] as i32, p2[0] as i32) {
        return p2_cpy
      } else {
        if zip_biggest(&p1[3..5].to_vec(), &p2[3..5].to_vec()) == 1 { return p1_cpy }
        else { return p2_cpy }
      }
    },
    2 => {
      if is_higher(p2[2] as i32, p1[2] as i32) { return p1_cpy }
      else if is_higher(p1[2] as i32, p2[2] as i32) { return p2_cpy } 
      else {
        if is_higher(p2[0] as i32, p1[0] as i32) { return p1_cpy }
        else if is_higher(p1[0] as i32, p2[0] as i32) { return p2_cpy } 
        else {
          if is_higher(p2[4] as i32, p1[4] as i32) { return p1_cpy }
          else { return p2_cpy } 
        }
      }
    },
    1 => {
      if is_higher(p2[0] as i32, p1[0] as i32) { return p1_cpy }
      else if is_higher(p1[0] as i32, p2[0] as i32) { return p2_cpy } 
      else {
        if zip_biggest(&p1[2..5].to_vec(), &p2[2..5].to_vec()) == 1 { return p1_cpy }
        else { return p2_cpy }
      }
    },
    0 => {
      if zip_biggest(p1, p2) == 1 { return p1_cpy }
      else { return p2_cpy }
    }
    _ => Vec::new()
  }
}

fn zip_biggest(p1: &Vec<u32>, p2: &Vec<u32>) -> u32 {
  let mut reverse_zip: Vec<(&u32,&u32)> = p1.iter().zip(p2).collect();
  reverse_zip.reverse();
  
  for (c1, c2) in reverse_zip.iter() {
    let o1 = card_ord(**c1); // ** b/c reverse_zip stores pointers, then iter() also creates refs
    let o2 = card_ord(**c2);
    
    if o1 != o2 {
      if is_higher(o2 as i32, o1 as i32) {
        return 1
      } else {
        return 2
      }
    }
  }
  return 0
}

/************************
  deal
************************/

pub fn deal(perm:[u32;9]) -> Vec<String> {
  let c_map: [&str;52] = [
    "1C", "2C", "3C", "4C", "5C", "6C", "7C", "8C", "9C", "10C", "11C", "12C", "13C",
    "1D", "2D", "3D", "4D", "5D", "6D", "7D", "8D", "9D", "10D", "11D", "12D", "13D",
    "1H", "2H", "3H", "4H", "5H", "6H", "7H", "8H", "9H", "10H", "11H", "12H", "13H",
    "1S", "2S", "3S", "4S", "5S", "6S", "7S", "8S", "9S", "10S", "11S", "12S", "13S", 
  ];
  let mut p_one: Vec<u32> = Vec::new();
  let mut p_two: Vec<u32> = Vec::new();

  for (i, card) in perm.iter().enumerate() {
    if i == 0 || i == 2 { p_one.push(*card); } 
    else if i == 1 || i == 3 { p_two.push(*card); } 
    else {
      p_one.push(*card);
      p_two.push(*card);
    }
  }

  let (p1_str, p1_hand) = hand_strength(&p_one);
  let (p2_str, p2_hand) = hand_strength(&p_two);
  
  // Uncomment to debug
  println!("-------------------------------------------------------------");
  println!("p1 pool: {:?}", p_one);
  println!("p2 pool: {:?}", p_two);
  println!("p1 str: {} {}", p1_str, display_hand(p1_str));
  println!("p2 str: {} {}", p2_str, display_hand(p2_str));
  println!("p1 hand: {:?}", p1_hand);
  println!("p2 hand: {:?}", p2_hand);

  // results must be in increasing order (CDHS)
    
  if p1_str > p2_str {
    println!("Winning Hand: {:?}", p1_hand);
    println!("Winning Player: P1");
    let result: Vec::<String> = 
      trim(&p1_hand, p1_str as u32)
        .iter()
        .cloned()
        .map(|x| String::from(c_map[(x - 1) as usize]))
        .collect();
    return result;
  } else if p2_str > p1_str {
    println!("Winning Hand: {:?}", p2_hand);
    println!("Winning Player: P2");
    let result: Vec::<String> =
      trim(&p2_hand, p2_str as u32)
        .iter()
        .cloned()
        .map(|x| String::from(c_map[(x - 1) as usize]))
        .collect();
    return result;
  } else {
    return 
    // trim(&tie_break(&p1_hand, &p2_hand, p1_str), p1_str as u32)
    tie_break(&p1_hand, &p2_hand, p1_str)
        .iter()
        .cloned()
        .map(|x| String::from(c_map[(x - 1) as usize]))
        .collect()
  }
}

/************************
  main
************************/

// remove main after completion
fn main() {
  let perms:[[u32;9]; 15] = [
        [ 9,  8,  7,  6,  5,  4,  3,  2,  1  ],  // 1   2-6 Straight flush VS 1-5 straight flush  (tie)
        [ 40, 41, 42, 43, 48, 49, 50, 51, 52 ],  // 2   Royal flush VS straight flush             (no tie)
        [ 40, 41, 27, 28, 1,  14, 15, 42, 29 ],  // 3   Four aces VS 2-full-of-A                  (no tie)
        [ 30, 13, 27, 44, 12, 17, 33, 41, 43 ],  // 4   3-fours VS 2-fours                        (no tie)
        [ 27, 45, 3,  48, 44, 43, 41, 33, 12 ],  // 5   Flush VS straight                         (no tie)
        [ 17, 31, 30, 51, 44, 43, 41, 33, 12 ],  // 6   3-fours VS 2-queens-2-fives               (no tie)
        [ 17, 39, 30, 52, 44, 25, 41, 51, 12 ],  // 7   Q-full-of-K VS Q-full-of-4                (tie)
        [ 11, 25, 9,  39, 50, 48, 3,  49, 45 ],  // 8   9-K straight VS 9-J-two-pair              (no tie)
        [ 50, 26, 39, 3,  11, 27, 20, 48, 52 ],  // 9   J-K-two-pair VS K-pair                    (no tie)
        [ 40, 52, 46, 11, 48, 27, 29, 32, 37 ],  // 10                                            (tie)
        [ 1,  20, 13, 43, 48, 12, 25, 38, 51 ],  // 11                                            (tie) - four of a kind
        [ 14, 15, 26, 22, 16, 17, 21, 33, 48 ],  // 12                                            (tie) - flush
        [ 1,  20, 47, 14, 4,  5,  6,  27, 40 ],  // 13                                            (tie) - three of a kind
        [ 1,  2,  3,  4,  7,  10, 23, 37, 50 ],  // 14                                            (tie) - two pair
        [ 41, 3,  11, 51, 18, 6,  7,  9, 14 ]    // 15                                            (tie) - high card
    ];

  // Ties tested:
  //   9 royal flush     []
  //   8 straight flush  [x]
  //   7 four of a kind  []
  //   6 full house      [x]
  //   5 flush           []
  //   4 straight        []
  //   3 three of a kind []
  //   2 two pair        []
  //   1 one pair        [x]
  //   0 high card       []
  
  for (i, perm) in perms.iter().enumerate() {
    // if i != 0 && i != 6 && i != 9 {
      let winner: Vec<String> = deal(*perm);
      println!("{:?}", winner);
      println!("{}", i + 1);
      println!("-------------------------------------------------------------");
    // }
  }

  // let perm_vec = vec![6, 13, 41, 12, 11, 10, 1]; // ace high
  // let perm_vec = vec![46, 13, 3, 44, 31, 17, 45];  // 34567 straight
  // let perm_vec = vec![15, 44, 15, 29, 43, 28, 13];
  // let perm_vec = vec![25, 1, 9, 10, 11, 12, 13];  // royal flush
  // let perm_vec = vec![22, 17, 18, 19, 21, 14, 20]; // flush

  // let perm_vec = vec![3, 16, 29, 42, 21, 18, 24]; // foak (3333J)
  // let perm_vec = vec![13, 26, 39, 52, 21, 18, 40]; // foak (KKKKA)
  // let perm_vec = vec![13, 46, 45, 1, 14, 27, 40]; // foak (AAAAK )
  // let perm_vec = vec![3, 16, 29, 41, 21, 40, 24]; // three-oak
  // let perm_vec = vec![40, 16, 11, 41, 24, 37, 47]; // three-oak (JJJ8A)
  // let perm_vec = vec![14, 27, 40, 49, 50, 51, 52]; // three-oak (AAAQK)
  // let perm_vec = vec![14, 27, 45, 49, 50, 51, 52]; // one-pair (AA6QK)
  // let perm_vec = vec![13, 26, 45, 3, 17, 28, 46]; // one-pair (KK467)
  // let perm_vec = vec![12, 25, 26, 15, 16, 20, 22]; // one-pair (QQ78K)
  // let perm_vec = vec![25, 12, 26, 15, 16, 20, 22]; // one-pair (QQ78K)
  // let perm_vec = vec![12, 25, 26, 15, 33, 20, 22]; // two-pair (77QQK) (20, 33, 12, 25, 26)
  // let perm_vec = vec![1, 13, 17, 19, 21, 52, 40]; // two-pair (KKAA8) (13, 52, 1, 40, 21)
  // let perm_vec = vec![1, 13, 17, 19, 27, 52, 40]; // full-house (AAAKK) (1, 27, 40, 13, 52)
  // let perm_vec = vec![26, 52, 13, 27, 30, 40, 7]; // full-house (KKKAA) (13, 26, 52, 27, 40)
  // let perm_vec = vec![26, 52, 13, 17, 4, 8, 21]; // full-house (KKK88) (13, 26, 52, 8, 21) - two pairs
  // let perm_vec = vec![4, 17, 30, 1, 40, 13, 26]; // full-house (444AA) (4, 17, 30, 1, 40) - two pairs
  // let perm_vec = vec![40, 2, 3, 18, 19, 49, 51]; // high card (5,6,10,Q,A) (18, 19, 49, 51, 40) - ace high
  // let perm_vec = vec![52, 2, 3, 18, 19, 49, 51]; // high card (5,6,10,Q,K) (18, 19, 49, 51, 52) - king high
  // let perm_vec = vec![40, 27, 1, 14, 15, 42, 29]; // four-oak (AAAA3)


  // println!("{:?}", high_from_suits(&perm_vec, 1));
  // println!("{:?}", is_royal_flush(&perm_vec));
  // println!("{:?}", is_straight_flush(&perm_vec));
  // println!("{:?}", is_straight(&perm_vec));
  // println!("{:?}", is_flush(&perm_vec));
  // println!("{:?}", is_four_oak(&perm_vec));
  // println!("{:?}", is_three_oak(&perm_vec));
  // println!("{:?}", is_one_pair(&perm_vec));
  // println!("{:?}", is_two_pair(&perm_vec));
  // println!("{:?}", is_full_house(&perm_vec));
  // println!("{:?}", high_card(&perm_vec));
 
}

// to build: 'cargo build' @ '/Users/JAAI/Desktop/RY/F2020/CCPS506/Assignment/Rust/Poker/src'
// to build & run: 'cargo run' @ '/Users/JAAI/Desktop/RY/F2020/CCPS506/Assignment/Rust/Poker/src'

// to debug: f5
// in launch.json (config)
// {
//   "type": "lldb",
//   "request": "launch",
//   "name": "Debug",
//   "program": "${workspaceFolder}/Rust/Poker/target/debug/Poker",
//   "args": [],
//   "cwd": "${workspaceFolder}",
//   "sourceLanguages": ["rust"],
// }

// Questions
// 1. How to return equivalent of 'null'?

// General Improvements
// - create struct for cards
// - replace is_higher using the Ord trait for abovementioned structs
