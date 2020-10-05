module M {
    resource struct Pair { f: u64, g: u64}

   public fun test_eq(addr1: address, addr2: address): bool acquires Pair {
       let p1 = borrow_global<Pair>(addr1);
       let p2 = borrow_global<Pair>(addr2);
       let _b = (p1 == p2);
       let _x: u8;
       p1 = borrow_global<Pair>(addr1);
       p2 = borrow_global<Pair>(addr2);
       p1 == p2
   }
}