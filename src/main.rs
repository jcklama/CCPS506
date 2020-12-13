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
      return: vector with n cards (ascending with highest right to left)
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
    return (has_ace.len() > 0, has_ace[0]); // TODO: use Some/None check instead of directly accessing has_ace[0]
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
        // let mut kicker: i32 = 53;

        for card in pool.iter() {
          if card_ord(*card) == *ord {
            result_vec.push(*card);
          }
        }
        // result_vec.push(kicker as u32); // TODO: return array instead of vector (all hands)

        // find kicker (highest single)
        // TODO: refactor to use get_n_highest?
        // for card in pool.iter() {
        //   if card_ord(*card) != card_ord(*ord) {
        //     if card_ord(*card) == 1 {
        //       result_vec[4] = *card;
        //       return (true, result_vec)
        //     } else if is_higher(kicker, *card as i32) {
        //       kicker = *card as i32;
        //       result_vec[4] = kicker as u32;
        //     }
        //   }
        // }
        // return (true, result_vec)
        let mut no_four_oak = pool.iter().cloned().collect::<Vec<u32>>();
        for card in result_vec.iter() {
          if no_four_oak.contains(card) {
            no_four_oak.retain(|x| x != card);  // retain = opposite of removal
          }
        }

        result_vec.append(&mut get_n_highest(&no_four_oak, 1));
        return (true, result_vec)
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
      return format: 1 1 2 3 4
  ************************/

  fn is_two_pair(pool: &Vec<u32>) -> (bool, Vec<u32>) {

    
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
              return (true, vec![first, second, third, fourth, fifth])
          }
        }
      }
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
        .cloned()  // need to clone because otherwise will have vector of references
        .collect(); // produces a new collection
      potential_flush.sort_by(|a, b| b.cmp(a));

      if potential_flush.len() >= 5 {
        result_vec = potential_flush[0..4].to_vec();

        let (has_ace, ace) = check_ace(&potential_flush);
        if has_ace {
          result_vec.reverse();
          result_vec.push(ace);  
        } else {
          result_vec.push(potential_flush[4]);
          result_vec.reverse();
        }
        
        return (true, result_vec)
      }
    }
  
    (false, Vec::new())
  }
    
  /************************
    hand_strength
  ************************/
  
  fn hand_strength(pool: &Vec<u32>) -> (u8, Vec<u32>) {
    // sorts in place
    // pool.sort();
  
    if is_royal_flush(pool).0         { (9, is_royal_flush(pool).1) }
    else if is_straight_flush(pool).0 { (8, is_straight_flush(pool).1) }
    else if is_four_oak(pool).0       { (7, is_four_oak(pool).1) }
    // else if is_full_house(pool) { 6 }
    else if is_flush(pool).0          { (5, is_flush(pool).1) }
    else if is_straight(pool).0       { (4, is_straight(pool).1) }
    else if is_three_oak(pool).0      { (3, is_three_oak(pool).1) }
    else if is_two_pair(pool).0       { (2, is_two_pair(pool).1) }
    else if is_one_pair(pool).0       { (1, is_one_pair(pool).1) }
    else { (0, pool.to_vec()) }
  }
  
  /************************
    deal
  ************************/
  
  pub fn deal(perm:[u32;9]) -> Vec<String> {
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
    
    println!("p1 pool: {:?}", p_one);
    println!("p2 pool: {:?}", p_two);
    println!("p1 str: {}", p1_str);
    println!("p2 str: {}", p2_str);
    println!("p1 hand: {:?}", p1_hand);
    println!("p2 hand: {:?}", p2_hand);

    // results must be in increasing order (CDHS)
    // no tie-breaker:
      // four-oak: trim last
      // three-oak: trim last 2
      // two-pair: trim last
      // one-pair: trim last 3
      // high card: trim last 4
    if p1_str > p2_str {
      // TODO: map function to get result
    } else if p1_str < p2_str {
      // TODO: map function to get result
    } else {
      // TODO: call tie breaker function
    }
  
    let mut v = Vec::new();
    v.push(String::from("string"));
    v
  }
  
  /************************
    main
  ************************/
  
  // remove main after completion
  fn main() {
    let perm: [u32;9] = [28,8,3,35,9,36,37,38,39];
    
    // let winner: Vec<String> = deal(perm);
    
    // let perm2: [u32;9] = [1,8,3,9,15,29,43,5,14];
    // deal(perm2);
  
    // let perm_vec = vec![6, 13, 41, 12, 11, 10, 1]; // ace high
    // let perm_vec = vec![46, 13, 3, 44, 31, 17, 45];  // 34567 straight
    // let perm_vec = vec![15, 44, 15, 29, 43, 28, 13];
    // let perm_vec = vec![25, 1, 9, 10, 11, 12, 13];  // royal flush
    // let perm_vec = vec![22, 17, 18, 19, 21, 14, 20]; // flush

    // let perm_vec = vec![3, 16, 29, 42, 21, 18, 24]; // foak
    let perm_vec = vec![13, 26, 39, 52, 21, 18, 40]; // foak (KKKKA)
    // let perm_vec = vec![13, 46, 45, 1, 14, 27, 40]; // foak (AAAAK )
    // let perm_vec = vec![3, 16, 29, 41, 21, 40, 24]; // three-oak
    // let perm_vec = vec![40, 16, 11, 41, 24, 37, 47]; // three-oak (JJJ8A)
    // let perm_vec = vec![14, 27, 40, 49, 50, 51, 52]; // three-oak (AAAQK)
    // let perm_vec = vec![14, 27, 45, 49, 50, 51, 52]; // one-pair (AA6QK)
    // let perm_vec = vec![13, 26, 45, 3, 17, 28, 46]; // one-pair (KK467)
    // let perm_vec = vec![12, 25, 26, 15, 16, 20, 22]; // one-pair (QQ78K)


    // println!("{:?}", high_from_suits(&perm_vec, 1));
    // println!("{:?}", is_royal_flush(&perm_vec));
    // println!("{:?}", is_straight_flush(&perm_vec));
    // println!("{:?}", is_straight(&perm_vec));
    // println!("{:?}", is_flush(&perm_vec));
    println!("{:?}", is_four_oak(&perm_vec));
    // println!("{:?}", is_three_oak(&perm_vec));
    // println!("{:?}", is_one_pair(&perm_vec));

  
    
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
  
  