//! Sparse vector iterators (SVI's).
//!
//! Sparse vectors in SOLAR are represented as iterators that run over vector entries
//! (i.e. that run over object which implement `KeyValGet` and possibly `KeyValSet`)
//! 

//! 
//!
//! **Example**
//! 
//! ```
//! let v = vec![ (1, 1.5), (2, 2.5) ];
//! 
//! for (index, coeff) in v {
//!     println!("(index, coeff) = {:?}", (index, coeff));
//! }
//! 
//! // Should display:
//! // (index, coeff) = (1, 1.5)
//! // (index, coeff) = (2, 2.5)
//! ```
//! 
//! -----------------------------------
//! 
//! DEVELOPER NOTES
//!
//! **REMARK** The `Pair< Index, Coeff >` items are preferred to tuples of form `( Index, Coeff )` (at least
//! for now) because tuples demand a certain memory structure that may impede performance, c.f. 
//! [rewriting in memory](https://www.reddit.com/r/rust/comments/79ry4s/tuple_performance/).
//!
//! 
//! **TO DO LIST**
//! 
//! - add the following functions for convenience:
//!     - (low priority) add vectors (with or without specifying precidence function)
//!     - (low priority) a "tuple merge" method allowing one to merge iterators of several different types
//! - use Rust 'Cells' to re-work the iterator heap to work by reference


// pub mod svi;
pub mod vector_transforms;
// pub mod svi_discussion;


