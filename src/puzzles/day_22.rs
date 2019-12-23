use crate::input::Input;
use std::error::Error;

type Card = usize;

fn deal_into_new_stack(deck: &mut [Card]) {
    deck.reverse();
}

fn cut_n_cards(deck: &mut [Card], mut n: isize) {
    if n < 0 {
        n += deck.len() as isize;
    }
    if n <= 0 || n >= deck.len() as isize {
        return;
    }

    let n = n as usize;

    let l = deck.len() - n;
    let mut cards = deck[..n].to_vec();
    for i in 0..l {
        deck.swap(i, i + n);
    }

    deck[l..].swap_with_slice(&mut cards);
}

fn deal_with_increment(deck: &mut [Card], n: usize) {
    let mut tmp = vec![0; deck.len()];

    let mut pos = 0;
    for i in 0..deck.len() {
        tmp[pos] = deck[i];

        pos += n;
        if pos >= deck.len() {
            pos %= deck.len();
        }
    }

    deck.swap_with_slice(&mut tmp);
}

fn modular_pow(mut base: i128, mut exponent: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0;
    }

    debug_assert!({
        let (_, overflowed) = (modulus - 1).overflowing_pow(2);
        !overflowed
    });

    let mut result = 1;
    base %= modulus;
    while exponent > 0 {
        if exponent.rem_euclid(2) == 1 {
            result *= base;
            result %= modulus;
        }

        exponent >>= 1;
        base *= base;
        base %= modulus;
    }

    result
}

fn invert_n(deck_len: usize, n: i128) -> i128 {
    let deck_len = deck_len as i128;
    modular_pow(n, deck_len - 2, deck_len)
}

#[derive(Clone)]
enum ShuffleType {
    DealIntoNewStack,
    CutNCards(isize),
    DealWithIncrement(usize),
}

impl ShuffleType {
    fn perform(&self, deck: &mut [Card]) {
        match self {
            Self::DealIntoNewStack => deal_into_new_stack(deck),
            Self::CutNCards(n) => cut_n_cards(deck, *n),
            Self::DealWithIncrement(n) => deal_with_increment(deck, *n),
        }
    }

    fn simulate_step(&self, deck_len: usize, incr: &mut i128, off: &mut i128) {
        match self {
            Self::DealIntoNewStack => {
                *incr *= -1;
                *off += *incr;
                *off %= deck_len as i128;
            }
            Self::CutNCards(n) => {
                *off += *incr * *n as i128;
                *off %= deck_len as i128;
            }
            Self::DealWithIncrement(n) => {
                *incr *= invert_n(deck_len, *n as i128);
                *incr %= deck_len as i128;
            }
        }
    }
}

fn iter_shuffles<'a>(
    lines: impl Iterator<Item = &'a str> + 'a,
) -> impl Iterator<Item = ShuffleType> + 'a {
    lines.filter_map(|line| {
        let mut words = line.split_whitespace();
        let first_word = {
            if let Some(word) = words.next() {
                word
            } else {
                return None;
            }
        };
        let last_word = {
            if let Some(word) = words.last() {
                word
            } else {
                return None;
            }
        };

        match first_word {
            "deal" => {
                if let Ok(n) = last_word.parse() {
                    return Some(ShuffleType::DealWithIncrement(n));
                } else {
                    return Some(ShuffleType::DealIntoNewStack);
                }
            }
            "cut" => {
                if let Ok(n) = last_word.parse() {
                    return Some(ShuffleType::CutNCards(n));
                }
            }
            _ => (),
        }

        None
    })
}

fn perform_shuffles<'a>(deck: &mut [Card], lines: impl IntoIterator<Item = &'a str> + 'a) {
    iter_shuffles(lines.into_iter()).for_each(|s| s.perform(deck));
}

pub fn first(i: &Input) -> Result<String, Box<dyn Error>> {
    let mut deck: Vec<_> = (0..10_007).collect();
    perform_shuffles(&mut deck, i.lines());

    Ok(deck
        .iter()
        .enumerate()
        .find(|(_, &v)| v == 2019)
        .map(|(i, _)| i.to_string())
        .unwrap())
}

pub fn second(i: &Input) -> Result<String, Box<dyn Error>> {
    const DECK_LEN: usize = 119_315_717_514_047;
    const TOTAL_SHUFFLES: usize = 101_741_582_076_661;

    let mut increment = 1;
    let mut offset = 0;
    for shuffle in iter_shuffles(i.lines()) {
        shuffle.simulate_step(DECK_LEN, &mut increment, &mut offset);
    }

    let total_increment = modular_pow(increment, TOTAL_SHUFFLES as i128, DECK_LEN as i128);
    let mut total_offset = offset * (1 - total_increment) % DECK_LEN as i128;
    total_offset *= invert_n(DECK_LEN, (1 - increment) % DECK_LEN as i128);
    total_offset %= DECK_LEN as i128;

    let index = (total_offset + 2020 * total_increment).rem_euclid(DECK_LEN as i128);
    Ok(index.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cut_n_cards() {
        let mut deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        cut_n_cards(&mut deck, 3);
        assert_eq!(deck, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);

        let mut deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        cut_n_cards(&mut deck, -4);
        assert_eq!(deck, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deal_with_increment() {
        let mut deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        deal_with_increment(&mut deck, 3);
        assert_eq!(deck, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn test_perform_shuffles() -> Result<(), Box<dyn Error>> {
        let mut deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        perform_shuffles(
            &mut deck,
            "
            deal with increment 7
            deal into new stack
            deal into new stack
            "
            .lines(),
        );
        assert_eq!(deck, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);

        Ok(())
    }
}
