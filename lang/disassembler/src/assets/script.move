script {
    use 0x1::Base;
    fun main<Ta>(_a: &signer) {
        Base::code();
        abort 1
    }
}