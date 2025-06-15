pub mod types {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Color {
        Red,
        Black,
        Green,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RouletteSpin {
        pub number: u8, // 0–36
        pub color: Color,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum BetType {
        Straight(u8),
        Color(Color),
        EvenOdd(bool),
        Dozen(u8),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Bet {
        pub bet_type: BetType,
        pub amount: f64,
    }
}

