// Utility for converting a Move value to its binary representation in LCS (diem Canonical
// Serialization). LCS is the binary encoding for Move resources and other non-module values
// published on-chain. See https://github.com/diem/diem/tree/master/common/lcs for more
// details on LCS (TODO: link to spec once we have one)

address 0x1 {

module LCS {
    // Return the binary representation of `v` in LCS (diem Canonical Serialization) format
    native public fun to_bytes<MoveValue>(v: &MoveValue): vector<u8>;
}
}
