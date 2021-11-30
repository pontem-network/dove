script {
    use 0x1::Bar;
    use 0x1134::Foo;

    fun main() {
        Foo::foo();
        Bar::bar(0x13);
    }
}