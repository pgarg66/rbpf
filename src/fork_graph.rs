use std::cmp::Ordering;

/// Identity of a fork
pub type ForkId = u64;

/// Relationship between two fork IDs
pub enum ForkIdRelation {
    /// The fork is an ancestor of the other fork
    Ancestor,
    /// The two fork IDs are same or point to the same fork
    Equal,
    /// The fork is a descendant of the other fork
    Descendant,
}

/// Maps relationship between two ForkIds.
pub trait ForkGraph<ForkId>: PartialOrd<ForkId> {
    /// Returns whether the current forkId is an ancestor or descendant of the other forkId.
    /// Returns None if the two forkIds are not related.
    fn relationship(&self, other: &ForkId) -> Option<ForkIdRelation> {
        match self.partial_cmp(other)? {
            Ordering::Less => Some(ForkIdRelation::Ancestor),
            Ordering::Equal => Some(ForkIdRelation::Equal),
            Ordering::Greater => Some(ForkIdRelation::Descendant),
        }
    }
}
