#[derive(Clone, Debug, PartialEq)]
pub enum RnaColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RnaAlpha {
    Transparent,
    Opaque,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Rna {
    AddColor(RnaColor),
    AddAlpha(RnaAlpha),
    EmptyBucket,
    Move,
    TurnCounterClockwise,
    TurnClockwise,
    Mark,
    Line,
    TryFill,
    AddBitmap,
    Compose,
    Clip,
    Unknown(Vec<Dna>),
}

use super::{HEIGHT, WIDTH};

use super::dna::{
    Dna::{self, *},
    DnaRopeIter,
};

use self::{Dir::*, Rna::*, RnaAlpha::*, RnaColor::*};

impl Rna {
    pub fn from_dna_iter(iter: &mut DnaRopeIter) -> Self {
        if let (Some(a1), Some(a2), Some(a3), Some(a4), Some(a5), Some(a6), Some(a7)) = (
            iter.next(),
            iter.next(),
            iter.next(),
            iter.next(),
            iter.next(),
            iter.next(),
            iter.next(),
        ) {
            match (a1, a2, a3, a4, a5, a6, a7) {
                (P, I, P, I, I, I, C) => AddColor(Black),
                (P, I, P, I, I, I, P) => AddColor(Red),
                (P, I, P, I, I, C, C) => AddColor(Green),
                (P, I, P, I, I, C, F) => AddColor(Yellow),
                (P, I, P, I, I, C, P) => AddColor(Blue),
                (P, I, P, I, I, F, C) => AddColor(Magenta),
                (P, I, P, I, I, F, F) => AddColor(Cyan),
                (P, I, P, I, I, P, C) => AddColor(White),
                (P, I, P, I, I, P, F) => AddAlpha(Transparent),
                (P, I, P, I, I, P, P) => AddAlpha(Opaque),
                (P, I, I, P, I, C, P) => EmptyBucket,
                (P, I, I, I, I, I, P) => Move,
                (P, C, C, C, C, C, P) => TurnCounterClockwise,
                (P, F, F, F, F, F, P) => TurnClockwise,
                (P, C, C, I, F, F, P) => Mark,
                (P, F, F, I, C, C, P) => Line,
                (P, I, I, P, I, I, P) => TryFill,
                (P, C, C, P, F, F, P) => AddBitmap,
                (P, F, F, P, C, C, P) => Compose,
                (P, F, F, I, C, C, F) => Clip,
                _ => Unknown(vec![*a1, *a2, *a3, *a4, *a5, *a6, *a7]),
            }
        } else {
            Unknown(vec![])
        }
    }
}

struct BucketColor(u8, u8, u8);

impl From<&RnaColor> for BucketColor {
    fn from(value: &RnaColor) -> Self {
        match value {
            Black => BucketColor(0, 0, 0),
            Red => BucketColor(1, 0, 0),
            Green => BucketColor(0, 1, 0),
            Yellow => BucketColor(1, 1, 0),
            Blue => BucketColor(0, 0, 1),
            Magenta => BucketColor(1, 0, 1),
            Cyan => BucketColor(0, 1, 1),
            White => BucketColor(1, 1, 1),
        }
    }
}

type BucketAlpha = u8;

impl From<&RnaAlpha> for BucketAlpha {
    fn from(value: &RnaAlpha) -> Self {
        match value {
            Transparent => 0,
            Opaque => 1,
        }
    }
}

pub enum Dir {
    N,
    E,
    S,
    W,
}

const DIRS: [Dir; 4] = [N, E, S, W];

type Position = (u32, u32);

type Pixel = (u8, u8, u8, u8);

pub struct RnaRenderer {
    bitmaps: Vec<Vec<Pixel>>,
    bucket_color: Vec<BucketColor>,
    bucker_alpha: Vec<BucketAlpha>,
    dir_index: usize,
    position: Position,
    mark: Position,
}

const BITMAP_SIZE: usize = WIDTH as usize * HEIGHT as usize;

fn new_bitmap() -> Vec<Pixel> {
    vec![(0, 0, 0, 0); BITMAP_SIZE]
}

impl Default for RnaRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl RnaRenderer {
    pub fn new() -> RnaRenderer {
        RnaRenderer {
            bitmaps: vec![new_bitmap()],
            bucket_color: vec![],
            bucker_alpha: vec![],
            dir_index: 1,
            position: (0, 0),
            mark: (0, 0),
        }
    }

    pub fn render(&mut self, rna: &[Rna]) {
        for command in rna {
            self.render_command(command);
        }
    }

    pub fn render_command(&mut self, command: &Rna) {
        match command {
            AddColor(c) => self.bucket_color.push(BucketColor::from(c)),
            AddAlpha(a) => self.bucker_alpha.push(BucketAlpha::from(a)),
            EmptyBucket => {
                self.bucket_color.clear();
                self.bucker_alpha.clear();
            }
            Move => match DIRS[self.dir_index] {
                N => self.position = (self.position.0, (self.position.1 + HEIGHT - 1) % HEIGHT),
                E => self.position = ((self.position.0 + 1) % WIDTH, self.position.1),
                S => self.position = (self.position.0, (self.position.1 + 1) % HEIGHT),
                W => self.position = ((self.position.0 + WIDTH - 1) % WIDTH, self.position.1),
            },
            TurnCounterClockwise => self.dir_index = (self.dir_index + DIRS.len() - 1) % DIRS.len(),
            TurnClockwise => self.dir_index = (self.dir_index + 1) % DIRS.len(),
            Mark => self.mark = self.position,
            Line => {
                let position = self.position;
                let mark = self.mark;
                let current_pixel = self.current_pixel();
                self.line(position, mark, current_pixel)
            }
            TryFill => self.try_fill(),
            AddBitmap => self.add_bitmap(),
            Compose => self.compose(),
            Clip => self.clip(),
            Unknown(_) => (),
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn mark(&self) -> &Position {
        &self.mark
    }

    pub fn dir(&self) -> &Dir {
        &DIRS[self.dir_index]
    }

    pub fn current_pixel(&self) -> Pixel {
        let c = self
            .bucket_color
            .iter()
            .fold((0, 0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2));
        let a: u8 = if self.bucker_alpha.is_empty() {
            255
        } else {
            ((u16::from(self.bucker_alpha.iter().sum::<u8>()) * 255)
                / self.bucker_alpha.len() as u16) as u8
        };

        let len = if self.bucket_color.is_empty() {
            1
        } else {
            self.bucket_color.len() as u16
        };
        let avg = |x| (u16::from(x) * u16::from(a) / len) as u8;
        (avg(c.0), avg(c.1), avg(c.2), a)
    }

    fn line(&mut self, from: Position, to: Position, pixel: Pixel) {
        let deltax = to.0 as i32 - from.0 as i32;
        let deltay = to.1 as i32 - from.1 as i32;
        let d = deltax.abs().max(deltay.abs());
        let c = if deltax * deltay <= 0 { 1 } else { 0 };
        let mut x = from.0 as i32 * d + (d - c) / 2;
        let mut y = from.1 as i32 * d + (d - c) / 2;
        for _ in 0..d {
            self.pixel_set(((x / d) as u32, (y / d) as u32), pixel);
            x += deltax;
            y += deltay;
        }
        self.pixel_set(to, pixel);
    }

    fn pixel_set(&mut self, p: Position, c: Pixel) {
        let index = p.1 * HEIGHT + p.0;
        self.bitmaps[0][index as usize] = c;
    }

    fn pixel_get(&self, p: Position) -> Pixel {
        let index = p.1 * HEIGHT + p.0;
        self.bitmaps[0][index as usize]
    }

    fn try_fill(&mut self) {
        let new = self.current_pixel();
        let old = self.pixel_get(self.position);
        if old != new {
            let position = self.position;
            self.fill(position, old, new);
        }
    }

    fn fill(&mut self, p: Position, initial: Pixel, new: Pixel) {
        let p_vec = &mut vec![p];
        while !p_vec.is_empty() {
            self.fill_vec(p_vec, initial, new)
        }
    }

    fn fill_vec(&mut self, p_vec: &mut Vec<Position>, initial: Pixel, new: Pixel) {
        if let Some(p) = p_vec.pop() {
            if self.pixel_get(p) == initial {
                self.pixel_set(p, new);
                if p.0 > 0 {
                    p_vec.push((p.0 - 1, p.1));
                }
                if p.0 < WIDTH - 1 {
                    p_vec.push((p.0 + 1, p.1));
                }
                if p.1 > 0 {
                    p_vec.push((p.0, p.1 - 1));
                }
                if p.1 < HEIGHT - 1 {
                    p_vec.push((p.0, p.1 + 1));
                }
            }
        }
    }

    fn add_bitmap(&mut self) {
        if self.bitmaps.len() != 10 {
            self.bitmaps.insert(0, new_bitmap());
        }
    }

    fn compose(&mut self) {
        if self.bitmaps.len() < 2 {
            return;
        }

        let bitmap0 = &self.bitmaps.drain(0..=0).next().unwrap();
        let bitmap1 = &mut self.bitmaps[0];

        for (x1, x0) in bitmap1.iter_mut().zip(bitmap0.iter()) {
            let a0_diff = 255 - x0.3 as u16;

            let xx = (
                x0.0 as u16 + (x1.0 as u16 * a0_diff / 255u16),
                x0.1 as u16 + (x1.1 as u16 * a0_diff / 255u16),
                x0.2 as u16 + (x1.2 as u16 * a0_diff / 255u16),
                x0.3 as u16 + (x1.3 as u16 * a0_diff / 255u16),
            );
            *x1 = (xx.0 as u8, xx.1 as u8, xx.2 as u8, xx.3 as u8)
        }
    }

    fn clip(&mut self) {
        if self.bitmaps.len() < 2 {
            return;
        }

        let bitmap0 = &self.bitmaps.drain(0..=0).next().unwrap();
        let bitmap1 = &mut self.bitmaps[0];

        for (x1, x0) in bitmap1.iter_mut().zip(bitmap0.iter()) {
            let a0 = x0.3;

            *x1 = (
                x1.0 * a0 / 255,
                x1.1 * a0 / 255,
                x1.2 * a0 / 255,
                x1.3 * a0 / 255,
            );
        }
    }

    pub fn to_vec(&self, index: usize) -> Vec<u8> {
        if let Some(bitmap) = self.bitmaps.get(index) {
            let mut result = vec![];
            bitmap.iter().for_each(|pixel| {
                result.push(pixel.0);
                result.push(pixel.1);
                result.push(pixel.2);
                result.push(if index == 0 { 255 } else { pixel.3 });
            });
            result
        } else {
            vec![0; BITMAP_SIZE * 4]
        }
    }
}
