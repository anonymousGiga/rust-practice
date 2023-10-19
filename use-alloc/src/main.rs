use bumpalo::Bump;
use hashbrown::HashMap;

fn main() {
    let bump = Bump::new();
    let mut map = HashMap::new_in(&bump);

    assert_eq!(map.len(), 0);
    assert_eq!(map.capacity(), 0);

    map.insert("One", 1);
    assert_eq!(map.len(), 1);
    assert!(map.capacity() > 1);
}
