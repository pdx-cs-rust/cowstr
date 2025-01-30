use std::borrow::Cow;
use std::collections::BTreeMap;

fn has_uppercase(s: &str) -> bool {
    s.chars().any(|c| c.is_uppercase())
}

fn main() {
    let mut s = Cow::from("Hello");
    if has_uppercase(&s) {
        *s.to_mut() = s.to_lowercase();
    }
    assert!(!has_uppercase(&s));
    println!("{}", s);

    let mut map = BTreeMap::new();
    let t1 = Cow::from("t1");
    assert!(matches!(t1, Cow::Borrowed(_)));
    map.insert(t1, 1usize);
    let t2 = Cow::from("t2".to_string());
    assert!(matches!(t2, Cow::Owned(_)));
    // *t2.to_mut() = t2.as_ref().to_string();
    map.insert(t2, 1);
    println!("{:?}", map);
}
