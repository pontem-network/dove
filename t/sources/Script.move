script {
    use 0x1::Foo;
    fun main(s: signer) {
        Foo::test_d(&s, 10);
    }
}
