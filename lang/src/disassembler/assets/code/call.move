module M {
   use 0x00000000000000000000000000000001::Base;

    fun r() : u8 {
        1
    }

    fun r1(): u64 {
        (r() as u64)
    }

    fun c(_code: u64, _data: u8) {

    }

    fun c1(code: u64) {
        c(code, r())
    }

    fun c2(code: u8) {
        c1((code as u64))
    }

    fun c3(code: u64) {
        c(code, (Base::code() as u8));
        c(code, (Base::code() as u8))
    }

    fun g<G, B, R>() {
    }

    fun g1() {
        g<u8, Base::Test, vector<u8>>();
    }
}