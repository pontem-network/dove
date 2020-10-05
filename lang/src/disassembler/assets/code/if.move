module M {
    struct S { f: u64, g: u64 }

    fun r() {}

    fun r1() {}

    fun cond(): bool {
        true
    }

    fun t0(cond: bool) {
        let s = S { f: 0, g: 0 };
        let f;
        if (cond) f = &s.f else f = &s.g;
        *f;
        s = S { f: 0, g: 0 };
        s;
    }

    fun f(cond: bool) {
        if (cond) { if (cond) () };
        if (cond) ( if (cond) () );
        if (cond) if (cond) ()
    }

    fun i1(cond: bool) {
        if (!cond) {
            r();
        }
    }

    fun i01(cond: bool) {
        if (cond) {
        } else {
            r();
        };
        r1();
    }

    fun i001(cond: bool) {
        if (cond) {
        } else {
            r();
        }
    }

    fun i3() {
        if (cond()) {
            r();
            r1();
        } else {
            if (cond()) {
                r1();
            };
            r();
        };
        r1();
    }

    fun i2(): u64 {
        if (cond()) {
            10
        } else {
            11
        }
    }

    fun i4(): u64 {
        if (cond()) {
            10
        } else if (cond()) {
            11
        } else {
            12
        }
    }

    fun i5() {
        if (cond()) {
        } else {
            r();
        }
    }

    fun i6() {
        if (cond()) {
            return
        } else {
            r();
        }
    }

    fun i7() {
        if (cond()) {
            i7();
            return
        } else {
            r();
        }
    }

//    fun i8() {
//        if (cond()) {
//            i7();
//        } else {
//            return
//        }
//    }

    fun i9() {
        if (cond()) {
            i7();
        } else {
            r();
            return
        }
    }

    fun i10(): u64 {
        if (cond()) {
            return 0
        };
        1
    }

    fun i11(): u64 {
        if (cond()) {
            return 0
        };
        if (cond()) {
            return 3
        };
        1
    }

    fun i(cond: bool) {
        if (cond) {
            r();
            r1();
        } else {
            r1();
            if (cond()) {
               r1();
            } else {
               if (true) {
                r1();
               };
               if (!cond) {
                cond();
               } else {
                cond();
               };
               r();
            };
            r1();
        };
        r();
    }
}