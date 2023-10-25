#[cfg(feature = "alloc-bump")]
pub mod bump;
#[cfg(feature = "alloc-dlmalloc")]
pub mod dlmalloc;
#[cfg(feature = "alloc-galloc")]
pub mod galloc;
#[cfg(feature = "alloc-slab")]
pub mod slab;
