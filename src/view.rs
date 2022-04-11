use std::cmp::max;
use std::cmp::min;
use std::ops::Range;

use bevy::prelude::*;

#[derive(Copy, Clone)]
pub struct SpriteGridRect {
    pub left: usize,
    pub right: usize,
    pub bottom: usize,
    pub top: usize,
}

impl SpriteGridRect {
    pub fn xs(self) -> Range<usize> {
        self.left .. self.right
    }

    pub fn ys(self) -> Range<usize> {
        self.bottom .. self.top
    }

    pub fn intersect_with(self, other: Self) -> Option<Self> {
        let left = max(self.left, other.left);
        let right = min(self.right, other.right);
        if right <= left {
            return None;
        }
        let bottom = max(self.bottom, other.bottom);
        let top = min(self.top, other.top);
        if top <= bottom {
            None
        } else {
            Some(Self {
                left,
                right,
                bottom,
                top,
            })
        }
    }
}


#[derive(Copy, Clone)]
#[derive(Component)]
pub struct SpriteGridView(pub SpriteGridRect);