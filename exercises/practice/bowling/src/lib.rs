#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const TOTAL_FRAMES: usize = 10;
const PINS_PER_FRAME: u16 = 10;

#[derive(Debug, Clone, Copy)]
enum Frame {
    Strike,
    Spare(u16, u16),
    Open(u16, u16),
    FillBall(u16),
    Unfinished(u16),
}

use Frame::*;

pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: Vec::with_capacity(TOTAL_FRAMES + 2) }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > PINS_PER_FRAME {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.frames.len() >= TOTAL_FRAMES {
            let last_frame = &self.frames[TOTAL_FRAMES - 1];

            match (last_frame, self.frames.len()) {
                (Strike, len)                        if len < TOTAL_FRAMES + 2 => (),
                (Spare(_, _) | Unfinished(_), len)   if len < TOTAL_FRAMES + 1 => (),
                _ => return Err(Error::GameComplete),
            }
        }

        let frames = &mut self.frames;
        match frames.last().cloned() {
            Some(Strike | Spare(..)) if frames.len() >= TOTAL_FRAMES => frames.push(FillBall(pins)),
            None | Some(Strike | Spare(..) | Open(..)) if pins == PINS_PER_FRAME => frames.push(Strike),
            None | Some(Strike | Spare(..) | Open(..)) if pins <  PINS_PER_FRAME => frames.push(Unfinished(pins)),
            Some(Unfinished(a)) if pins + a == PINS_PER_FRAME => { frames.pop(); frames.push(Spare(a, pins)) },
            Some(Unfinished(a)) if pins + a <  PINS_PER_FRAME => { frames.pop(); frames.push(Open(a, pins)) },
            Some(FillBall(a))   if pins + a <= PINS_PER_FRAME || a == PINS_PER_FRAME => frames.push(FillBall(pins)),
            _ => return Err(Error::NotEnoughPinsLeft),
        };
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.len() < TOTAL_FRAMES {
            return None;
        }

        let mut res = 0;
        for i in 0..self.frames.len() {
            res += match self.frames[i..] {
                [Strike, Strike, Strike, ..] => 3 * PINS_PER_FRAME,
                [Strike, Strike, Open(a, _) | Spare(a, _) | FillBall(a), ..] => 2 * PINS_PER_FRAME + a,
                [Strike, FillBall(a), FillBall(b), ..] => PINS_PER_FRAME + (a + b),
                [Strike, Open(a, b) | Spare(a, b), ..] => PINS_PER_FRAME + (a + b),
                [Spare(a, b), Open(c, _) | Spare(c, _) | FillBall(c), ..] => (a + b) + c,
                [Spare(a, b), Strike, ..] => (a + b) + PINS_PER_FRAME,
                [Open(a, b), ..] => a + b,
                [Unfinished(_), ..] |
                  [Strike | Spare(_, _), ..] => return None,
                [..] => 0,
            }
        }
        Some(res)
    }
}
