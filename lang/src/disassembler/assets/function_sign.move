module M {
   struct C {}

   struct B<G> {}

   resource struct R {}

   resource struct D {}

   native fun n1();
   native public fun n2();
   native fun n3<T, G: resource, V: copyable>();

   native fun a1(): R acquires R, D;

   native fun n4(a: u64): u64;
   native fun n5<T>(t: T): T;
   native fun n6(v: vector<u8>): vector<u8>;
   native fun n7<T>(r: &T): &T;
   native fun n8(c: &mut C): &mut C;
   native fun n9<T>(t: 0x00000000000000000000000000000001::Base::Test): 0x00000000000000000000000000000001::Base::Test;
   native fun n10<G>(t: 0x00000000000000000000000000000001::Base::Test, b: B<G>): (0x00000000000000000000000000000001::Base::Test, B<G>);

   fun p() {
   }

   public fun p1() {

   }
}