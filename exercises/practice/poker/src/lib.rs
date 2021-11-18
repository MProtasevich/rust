/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use Combination::{ StraightFlush, FourOfAKind, FullHouse, Flush, Straight, ThreeOfAKind, TwoPairs, OnePair, HighCard };
use Rank::{ N, Jack, Queen, King, Ace };
use std::cmp::{ Ord, Ordering, Reverse };
use std::collections::{ BTreeMap, HashSet };

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq)]
enum Rank {
    N(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl From<&Rank> for u8 {
    fn from(val: &Rank) -> Self {
        match val {
            &N(v) => v,
            Jack => 11,
            Queen => 12,
            King => 13,
            Ace => 14
        }
    }
}

impl Ord for Rank {
    fn cmp(&self, other: &Self) -> Ordering {
        let val: u8 = self.into();
        val.cmp(&other.into())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Copy, Clone, Debug)]
struct Card {
    rank: Rank,
    suit: Suit
}

impl Card {
    fn new(card: &str) -> Self {
        let chars: Vec<char> = card.chars().collect();
        if let (Some(&rank_char), Some(&suit_char)) = (chars.first(), chars.last()) {
            let rank = match rank_char {
                '2'..='9' => N(rank_char as u8 - '0' as u8),
                '1' => N(10), // 10 doesn't fit to char, so had to use this workaround
                'J' => Jack,
                'Q' => Queen,
                'K' => King,
                'A' => Ace,
                _   => unreachable!("Unknown rank char: {:?}", rank_char),
            };
            let suit = match suit_char {
                'H' => Suit::Hearts,
                'D' => Suit::Diamonds,
                'S' => Suit::Spades,
                'C' => Suit::Clubs,
                _   => unreachable!("Unknown suit char: {:?}", suit_char),
            };
            Self { rank, suit }
        } else {
            unreachable!("Card string should consist of rank and suit. Provided card: {:?}", card);
        }
    }
}

#[derive(PartialEq, Eq, Debug, PartialOrd)]
enum Combination {
    StraightFlush(Rank),
    FourOfAKind(Box<[Rank]>),
    FullHouse(Box<[Rank]>),
    Flush(Box<[Rank]>),
    Straight(Rank),
    ThreeOfAKind(Box<[Rank]>),
    TwoPairs(Box<[Rank]>),
    OnePair(Box<[Rank]>),
    HighCard(Box<[Rank]>),
}

macro_rules! match_combinations {
    ( $e:expr, $( $comb:ident ),* $(,)*) => {
        match $e {
            $(
                ($comb(a), $comb(b)) => a.cmp(&b),
                ($comb(_), _) => Ordering::Greater,
                (_, $comb(_)) => Ordering::Less,
            )*
        }
    };
}

impl Ord for Combination {
    #[allow(unreachable_patterns)]
    fn cmp(&self, other: &Self) -> Ordering {
        match_combinations!((self, other),
            StraightFlush,
            FourOfAKind,
            FullHouse,
            Flush,
            Straight,
            ThreeOfAKind,
            TwoPairs,
            OnePair,
            HighCard
        )
    }
}

impl Combination {
    fn parse(hand: &str) -> Self {
        let hand_cards: Vec<Card> = hand.split(" ").map(Card::new).collect();
        if hand_cards.len() != 5 {
            panic!("There should be 5 cards. Cards are: `{}`", hand);
        }

        let canonical_symb_view: BTreeMap<Rank, Vec<Suit>> = hand_cards.iter().fold(BTreeMap::new(), |mut acc, card| {
            acc.entry(card.rank)
                .and_modify(|suits| suits.push(card.suit))
                .or_insert_with(|| vec![card.suit]);
            acc
        });

        let mut suits_by_rank: Vec<(&Rank, &Vec<Suit>)> = canonical_symb_view.iter().collect();
        suits_by_rank.sort_unstable_by_key(|&(rank, suits)| (Reverse(suits.len()), Reverse(rank)));

        // Check, whether this can be refactored with pattern matching
        let (vls, num_of_suits): (Vec<_>, Vec<_>) = suits_by_rank.iter().map(|&(rank, suits)| (rank, suits.len())).unzip();
        let vls = vls.into_boxed_slice();

        if (2..=4).contains(&num_of_suits.len()) {
            return match num_of_suits[..] {
                [4, 1] => FourOfAKind(vls),
                [3, 2] => FullHouse(vls),
                [3, 1, 1] => ThreeOfAKind(vls),
                [2, 2, 1] => TwoPairs(vls),
                [2, 1, 1, 1] => OnePair(vls),
                [..] => unreachable!("Incorrect sortion. Can't instantiate ThreeOfAKind or TwoPairs"),
            };
        } else if num_of_suits.len() == 5 {
            let min_rank = vls.iter().min().cloned().unwrap();
            let has_aces = vls.iter().any(|&rank| rank == Ace);
            let sequential_values = vls.iter()
                .filter(|&&rank| rank != Ace)
                .rev().enumerate()
                .all(|(i, rank)| u8::from(rank) - i as u8 == u8::from(&min_rank));
            let same_suit = suits_by_rank.iter()
                .flat_map(|(_, suits)| suits.iter())
                .collect::<HashSet<_>>().len() == 1;
            return match (sequential_values, same_suit) {
                (true, true)   => if has_aces && min_rank == N(2) { StraightFlush(N(5)) } else { StraightFlush(vls[0]) },
                (true, false)  => if has_aces && min_rank == N(2) { Straight(N(5)) } else { Straight(vls[0]) },
                (false, true)  => Flush(vls),
                (false, false) => HighCard(vls),
            }
        }
        unreachable!("Impossible card deck")
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let combinations: Vec<_> = hands.iter().cloned().map(|hand| (Combination::parse(hand), hand)).collect();
    let (hand_with_higher_score, _) = combinations.iter().max_by_key(|&(comb, _)| comb).unwrap();

    println!("Zipped vals: {:?}\nBest hand: {:?}", combinations, hand_with_higher_score);

    combinations.iter()
        .filter(|(combination, _)| combination == hand_with_higher_score)
        .map(|&(_, hand_str)| hand_str)
        .collect()
}
