use crate::parser::types::{
    MatchNumber, OrderRefNumber, Price4, Shares, Side, Stock, StockLocate, Timestamp,
    TrackingNumber,
};

/// Non-Cross Trade (type `b'P'`), ITCH 5.0 §1.5.1.
///
/// Execution of a non-displayable order. Does not affect order book state.
///
/// # Wire format (44 bytes)
///
/// | Field                  | Offset | Length | Type       |
/// |------------------------|--------|--------|------------|
/// | Message Type           | 0      | 1      | `b'P'`    |
/// | Stock Locate           | 1      | 2      | Integer    |
/// | Tracking Number        | 3      | 2      | Integer    |
/// | Timestamp              | 5      | 6      | Integer    |
/// | Order Reference Number | 11     | 8      | Integer    |
/// | Buy/Sell Indicator     | 19     | 1      | Alpha      |
/// | Shares                 | 20     | 4      | Integer    |
/// | Stock                  | 24     | 8      | Alpha      |
/// | Price                  | 32     | 4      | Price (4)  |
/// | Match Number           | 36     | 8      | Integer    |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonCrossTrade {
    pub stock_locate: StockLocate,
    pub tracking_number: TrackingNumber,
    pub timestamp: Timestamp,
    /// Always zero since December 6, 2010 for binary TotalView-ITCH feeds.
    pub order_ref_number: OrderRefNumber,
    /// Always `Side::Buy` since July 14, 2014, regardless of actual resting side.
    pub side: Side,
    pub shares: Shares,
    pub stock: Stock,
    pub price: Price4,
    pub match_number: MatchNumber,
}

/// Wire size excluding the 2-byte transport length prefix.
#[allow(dead_code)]
pub const NON_CROSS_TRADE_LEN: usize = 44;

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn struct_size() {
        assert_eq!(size_of::<NonCrossTrade>(), 48);
    }

    #[test]
    fn wire_length_matches_spec() {
        // Last field: match_number at offset 36, 8 bytes → 44 total.
        assert_eq!(NON_CROSS_TRADE_LEN, 44);
    }
}
