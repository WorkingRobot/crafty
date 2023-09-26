#[repr(u16)]
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub enum Condition {
    #[default]
    Normal      = 0x0001,
    Good        = 0x0002,
    Excellent   = 0x0004,
    Poor        = 0x0008,

    Centered    = 0x0010,
    Sturdy      = 0x0020,
    Pliant      = 0x0040,
    Malleable   = 0x0080,
    Primed      = 0x0100,
    GoodOmen    = 0x0200,
}
