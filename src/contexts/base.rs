multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub struct StorageCache<'a, C>
where
    C: crate::storage::Storage,
{
    _sc_ref: &'a C,
}

impl<'a, C> StorageCache<'a, C>
where
    C: crate::storage::Storage,
{
    pub fn new(sc_ref: &'a C) -> Self {
        StorageCache {
            _sc_ref: sc_ref,
        }
    }
}
