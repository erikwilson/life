use std::collections::{BTreeMap, BTreeSet};

pub struct StateCache<T> {
    pub transitions: BTreeMap<BTreeSet<T>, BTreeSet<T>>,
    pub last_state: BTreeSet<T>,
}

impl<T> Default for StateCache<T> {
    fn default() -> Self {
        Self {
            transitions: BTreeMap::new(),
            last_state: BTreeSet::new(),
        }
    }
}

impl<T> StateCache<T>
where
    T: Clone + Ord,
{
    pub fn insert(&mut self, state: &BTreeSet<T>) -> bool {
        let last_state = self.last_state.clone();
        self.last_state = state.to_owned();

        if self.transitions.contains_key(&last_state) {
            return false;
        }

        if !last_state.is_empty() {
            self.transitions.insert(last_state, state.to_owned());
        }
        true
    }
}
