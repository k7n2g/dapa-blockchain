use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::{BuildHasher, BuildHasherDefault, Hasher}
};

use anyhow::{Result, Context as AnyContext};

// A hasher for `TypeId`s that takes advantage of its known characteristics.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoOpHasher(u64);

impl Hasher for NoOpHasher {
    fn write(&mut self, _: &[u8]) {
        unimplemented!("This NoOpHasher can only handle u64s")
    }

    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

#[derive(Clone, Default)]
pub struct NoOpBuildHasher;

impl BuildHasher for NoOpBuildHasher {
    type Hasher = NoOpHasher;

    fn build_hasher(&self) -> Self::Hasher {
        NoOpHasher::default()
    }
}

/// A static Context to store/retrieve data by type
pub struct Context {
    values: HashMap<TypeId, Box<dyn Any + Send + Sync>, BuildHasherDefault<NoOpHasher>>,
}

impl Context {
    /// Create a new empty Context
    #[inline]
    pub fn new() -> Self {
        Self {
            values: HashMap::default()
        }
    }

    /// Store a value in the Context
    pub fn store<T: Send + Sync + 'static>(&mut self, data: T) {
        self.values.insert(TypeId::of::<T>(), Box::new(data));
    }

    /// Remove a value from the Context
    pub fn remove<T: 'static>(&mut self) {
        self.values.remove(&TypeId::of::<T>());
    }

    /// Check if a value of type T exists in the Context
    pub fn has<T: 'static>(&self) -> bool {
        self.values.contains_key(&TypeId::of::<T>())
    }

    /// Get an optional reference to a value of type T from the Context
    pub fn get_optional<T: 'static>(&self) -> Option<&T> {
        self.values.get(&TypeId::of::<T>()).and_then(|b| b.downcast_ref())
    }

    /// Get a reference to a value of type T from the Context or return an error if not found
    pub fn get<T: 'static>(&self) -> Result<&T> {
        self.get_optional().context("Requested type not found")
    }

    /// Get a copy of a value of type T from the Context or return an error if not found
    pub fn get_copy<T: 'static + Copy>(&self) -> Result<T> {
        self.get().copied()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}