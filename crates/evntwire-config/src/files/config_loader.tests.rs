use super::{Table, merge_tables};

#[test]
fn merges_nested_tables_and_replaces_arrays() {
    let mut base: Table = "[source]\nkeep = true\nkeys = [1, 2]".parse().unwrap();
    let overlay = "[source]\nkeys = [3]".parse().unwrap();

    merge_tables(&mut base, overlay);

    let expected: Table = "[source]\nkeep = true\nkeys = [3]".parse().unwrap();
    assert_eq!(base, expected);
}
