//! Base entity trait

use std::fmt::Debug;

/// Entity with unique identity
pub trait Entity: Debug {
    type Id: Clone + PartialEq + Eq + Debug;

    fn id(&self) -> &Self::Id;
}
