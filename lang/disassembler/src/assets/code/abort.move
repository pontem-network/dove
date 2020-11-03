module M {
   use 0x00000000000000000000000000000001::Base;

    fun _abort() {
        abort 0
    }

    fun _abort1(code: u64) {
        abort code
    }

    fun _abort2(code: u8) {
        abort (code as u64)
    }

    fun _abort3(_x: u64, _z: u128, _i: address) {
        abort Base::code()
    }
}