use std::collections::HashMap;

#[derive(Debug)]
pub struct LinearConflictGraph {
    pub conflicts: HashMap<u16, Vec<u16>>,
}

impl LinearConflictGraph {
    pub fn new() -> LinearConflictGraph {
        LinearConflictGraph {
            conflicts: HashMap::new(),
        }
    }

    fn push(&mut self, t1: u16, t2: u16) {
        // if self.conflicts.contains_key(&t1) {
        //     self.conflicts.get_mut(&t1).unwrap().push(t2);
        // } else {
        //     self.conflicts.insert(t1, vec![t2]);
        // }
        self.conflicts.entry(t1).or_insert(vec![]);
        self.conflicts.get_mut(&t1).unwrap().push(t2);
    }

    pub fn push_conflict(&mut self, t1: u16, t2: u16) {
        self.push(t1, t2);
        self.push(t2, t1);
    }

    pub fn is_conflicts(&self) -> bool {
        self.conflicts.len() > 0
    }

    pub fn remove_conflict_with(&mut self, tile: u16) {
        let links = self.conflicts.remove(&tile).unwrap();
        for l in links {
            let others = self.conflicts.get_mut(&l).unwrap();
            let idx =
                others.iter().position(|other| *other == tile).unwrap();
            others.remove(idx);

            if others.len() == 0 {
                self.conflicts.remove(&l);
            }
        }
    }

    pub fn most_conflicts(&self) -> u16 {
        *self
            .conflicts
            .iter()
            .max_by(|(_, av), (_, bv)| av.len().cmp(&bv.len()))
            .map(|(k, _)| k)
            .unwrap()
    }
}

impl PartialEq for LinearConflictGraph {
    fn eq(&self, other: &Self) -> bool {
        self.conflicts == other.conflicts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_conflict() {
        let mut lng = LinearConflictGraph::new();
        lng.push_conflict(1, 2);
        lng.push_conflict(1, 3);

        let mut cmp = LinearConflictGraph::new();
        cmp.conflicts.insert(1, vec![2, 3]);
        cmp.conflicts.insert(2, vec![1]);
        cmp.conflicts.insert(3, vec![1]);

        assert_eq!(lng, cmp);
    }

    #[test]
    fn test_conflict_sum() {
        let mut lng = LinearConflictGraph::new();
        lng.push_conflict(1, 2);
        lng.push_conflict(1, 3);

        assert_eq!(lng.is_conflicts(), true);

        lng.remove_conflict_with(2);
        lng.remove_conflict_with(3);

        assert_eq!(lng.is_conflicts(), false);
    }

    #[test]
    fn test_remove_conflict() {
        let mut lng = LinearConflictGraph::new();
        lng.push_conflict(1, 2);
        lng.push_conflict(1, 3);
        lng.remove_conflict_with(1);

        let cmp = LinearConflictGraph::new();

        assert_eq!(lng, cmp);
    }

    #[test]
    fn test_most_conflicts() {
        let mut lng = LinearConflictGraph::new();
        lng.push_conflict(1, 2);
        lng.push_conflict(1, 3);
        lng.push_conflict(2, 3);
        lng.push_conflict(1, 4);

        assert_eq!(lng.most_conflicts(), 1);
    }
}
