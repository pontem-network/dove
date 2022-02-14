#[test_only]
module Demo::Test3 {
    #[test]
    fun error() {
        assert!(false, 3);
    }
}
