address 0x1134 {
module Foo {
    use 0x1::Bar;

    const F: address = 0x881134;

    public fun foo() {
        Bar::bar(F);
        Bar::bar(0x13);
    }
}
}