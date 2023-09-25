use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Recipe {
    pub rlvl: u32,
    pub level: u8,
    pub progress_max: u32,
    pub quality_max: u32,
    pub durability_max: i8,
    pub progress_divider: u32,
    pub progress_modifier: u32,
    pub quality_divider: u32,
    pub quality_modifier: u32,
    pub is_expert: bool,
    pub conditions_flag: u16,
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:>3}) lv{:>2} / {:>5} progress / {:>5} quality / {:>2} durability",
            self.rlvl, self.level, self.progress_max, self.quality_max, self.durability_max
        )
    }
}
