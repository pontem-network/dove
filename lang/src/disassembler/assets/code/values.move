module M {
    struct S {}
    resource struct R {}

    fun t() {
        (0x1: address);
        000001u8;
        (0000001: u128);
        (0: u64);
        (10000: u64);
        (true: bool);
        (false: bool);
        b"(false: bool);";
        x"0101";
    }
}
