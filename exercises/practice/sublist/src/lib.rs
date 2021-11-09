use std::cmp::Ordering::{ Equal, Less, Greater };

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contains_all_of<T: PartialEq>(first: &[T], second: &[T]) -> bool {
    second.is_empty() || first.windows(second.len()).any(|window| window == second)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match first.len().cmp(second.len()) {
        Equal   => if first == second                { Comparison::Equal }     else { Comparison::Unequal },
        Less    => if contains_all_of(second, first) { Comparison::Sublist }   else { Comparison::Unequal },
        Greater => if contains_all_of(first, second) { Comparison::Superlist } else { Comparison::Unequal }
    }
}
