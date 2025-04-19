#[derive(Debug, Clone, Copy)]
pub enum RoundingMode {
    Nearest, // .round() | Rounds to nearest, ties away from zero | 2.625 → 2.63
    Floor,   // .floor() | Always rounds down | 2.625 → 2.62, -2.625 → -2.63
    Ceil,    // .ceil()  | Always rounds up | 2.625 → 2.63, -2.625 → -2.62
}
