use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TimeSignature {
    FourOnFour,    // 4/4 time signature (common time)
    ThreeOnFour,   // 3/4 time signature (waltz time)
    SixOnEight,    // 6/8 time signature (duple compound time)
    TwoOnFour,     // 2/4 time signature (duple time)
    SevenOnEight,  // 7/8 time signature (irregular time)
    FiveOnFour,    // 5/4 time signature (irregular time)
}