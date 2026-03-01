/// Wire encoding: `b'B'` for buy, `b'S'` for sell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Buy,
    Sell,
}

/// 4 implied decimal places: raw `1_500_000` = $150.0000.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Price4(pub u32);

/// Per-day locate code. Usable as a compact array index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StockLocate(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrackingNumber(pub u16);

/// Nanoseconds since midnight. Wire format is 6 bytes big-endian,
/// widened to `u64` during parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderRefNumber(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MatchNumber(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Shares(pub u32);

/// 8-byte stock symbol, right-padded with ASCII spaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Stock(pub [u8; 8]);
