use std::collections::BTreeSet;

/// As `Map`, but for sets.
pub type Set<K> = BTreeSet<K>;

#[allow(dead_code)]
pub fn set<K: Ord>() -> Set<K> {
    Set::<K>::default()
}
