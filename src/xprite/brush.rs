use xprite::Block;
use xprite::Color;
use xprite::BlockOffset;
use std::collections::HashSet;

pub struct Brush {
    pub shape: BlockOffset,
    pub size: (u32, u32),
}

impl Brush {
    pub fn pixel() -> Self {
        let mut hs = HashSet::new();
        hs.insert(blocks!(0, 0));

        Self {
            shape: hs,
            size: (1, 1),
        }
    }

    pub fn cross() -> Self {
        let mut hs = HashSet::new();
        hs.insert(blocks!(0, 1));
        hs.insert(blocks!(1, 0));
        hs.insert(blocks!(1, 1));
        hs.insert(blocks!(1, 2));
        hs.insert(blocks!(2, 1));

        Self {
            shape: hs,
            size: (3, 3),
        }
    }
}
