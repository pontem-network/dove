module M {
   use 0x00000000000000000000000000000001::Base;

    fun abort0() {
        abort 0
    }

    fun abort1(code: u64) {
        abort code
    }

    public fun abort2(code: u8) {
        abort (code as u64)
    }

    public(friend) fun abort3(_x: u64, _z: u128, _i: address) {
        abort Base::code()
    }
}