use std::collections::VecDeque;
use std::iter::{FromIterator, IntoIterator};

pub fn index_to_dice(index: u16) -> String {
    let mut out = VecDeque::new();
    let mut index = index;
    while index > 0 {
        let c = match index % 6 {
            0 => '1',
            1 => '2',
            2 => '3',
            3 => '4',
            4 => '5',
            5 => '6',
            _ => panic!("wtf?!"),
        };
        out.push_front(c);
        index /= 6;
    }
    while out.len() < 5 {
        out.push_front('1');
    }
    String::from_iter(out.into_iter())
}

pub fn dice_to_index(dice: &str) -> u16 {
    dice.chars().fold(0, |mut index, c| {
        index *= 6;
        index
            + match c {
                '1' => 0,
                '2' => 1,
                '3' => 2,
                '4' => 3,
                '5' => 4,
                '6' => 5,
                _ => panic!("Invalid character {}", c),
            }
    })
}
