address 0x0 {

/// Enum-like module to make generic type-matching possible, every coin which is
/// officially supported by blockchain (or peg-zone specifically) is added here.
/// Ideally this module should be auto-generated and rarely updated via consensus
module Coins {
    struct ETH {}
    struct BTC {}
    struct USDT {}
}
}
