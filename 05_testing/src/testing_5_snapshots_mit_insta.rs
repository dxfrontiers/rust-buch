#[test]
fn test_hello_world() {
    insta::assert_snapshot!(
        "Ich bin ein Snapshot, wenn auch \
    nur ein kleiner!"
    );
}
