module M {
//    public fun l() {
//        l1();
//        loop {
//            cond();
//            cond();
//            cond();
//        }
//    }
//
    public fun l1() {
        loop {}
    }
//
//
//    public fun l3(g: u64): u64 {
//        loop {
//            l1();
//            if (g > 100) {
//                break
//            } else {
//                l1();
//            }
//        };
//        1
//    }
//
//    public fun l4(g: u64): u64 {
//        loop {
//            l1();
//            if (g > 100) {
//                l1();
//            } else {
//                break
//            }
//        };
//        1
//    }
//
//    public fun l2() {
//        loop {
//            loop {
//                l1();
//                loop {
//                    l();
//                }
//            }
//        }
//    }
//
//    fun l5() {
//        loop {
//            if (cond()) {
//                continue
//            };
//            l1();
//        }
//    }
//
//    fun l6() {
//        loop {
//            l1();
//            continue
//        }
//    }
//
//    fun l7() {
//        loop {
//            continue
//        }
//    }

    public fun cond(): bool {
        true
    }

    fun l8() {
    l1();
        loop {
            if (cond()) {
                l1();
            } else {
                l1();
                continue
            };
            l8();
        }
    }
}