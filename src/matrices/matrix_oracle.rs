//! The traits that define a matrix oracle.
//! 
//! A matrix oracle is an object that can provide two types of information:
//! 
//! * **Rows and columns** What are the entries in the ith row or jth column?  
//! * **Major dimension** With many (but not all) sparse matrices, it's easier to access
//! one dimension than another, e.g. easier to access rows than columns.  It's important
//! to know which is easier when you're working with the matrix.  
//! 
//! When you ask a matrix oracle for information about a row or column vector, it returns
//! something called a "view."  A view is an iterator that runs over the entries of that
//! row/column.  We call it a view because it doesn't allow you to re-write those entries,
//! just to "see" them.
//! 
//! **Example** If a matrix is row-major, then 
//! 
//! * the ith major view is the ith row of the matrix, 
//! * the jth minor view is the jth column of the matrix.
//! 
//! 
//! 
//! # How to write your own matrix oracle
//! 
//! Writing your own matrix oracle is easier than you might think.
//! 
//! ### Example of scary source code
//! 
//! The first time you see source code for a matrix oracle, it can look
//! a bit daunting.  For example, the following is an excerpt from 
//! code that defines a vector-of-vector matrix, and implements the 
//! `OracleMajor` trait; it looks like a real mess.
//! 
//!
//! ```ignore
//! // Define the object
//! 
//! pub struct VecOfVec
//! 
//!     < 'a, IndexCoeffPair >
//! 
//!     where   IndexCoeffPair:    KeyValGet,
//!             Self:           'a
//! 
//! {
//!     pub major_dimension: MajorDimension, 
//!     pub vec_of_vec: Vec< Vec< IndexCoeffPair > >,
//!     pub phantom: PhantomData<&'a IndexCoeffPair >
//! }
//! 
//! // Implement the trait
//! 
//! impl < 'a, IndexCoeffPair > 
//!     
//!     OracleMajor
//!     <   
//!         'a,
//!         usize, 
//!         < IndexCoeffPair as KeyValGet >::Key, 
//!         < IndexCoeffPair as KeyValGet >::Val, 
//!     > 
//!     
//!     for 
//!     
//!     VecOfVec < 'a, IndexCoeffPair > 
//! 
//!     where   IndexCoeffPair:    KeyValGet + Clone + 'a,
//!             Self: 'a
//! {
//!     type PairMajor = IndexCoeffPair;
//!     type ViewMajor = Cloned<std::slice::Iter<'a, IndexCoeffPair>>; 
//!         
//!     fn view_major<'b: 'a>( &'b self, index: usize ) -> Self::ViewMajor {
//!         return self.vec_of_vec[index].iter().cloned()
//!     } 
//! }
//! ```
//!
//! 
//! ### Easily modify the scary example to do what you want
//!
//! Suppose we want to write a matrix oracle that represents a scalar matrix. 
//! The only data we need to define this matrix are: (i) the scalar value, `alpha`,
//! and (ii) the major dimension of the matrix.  Let's define a struct to house
//! this data:
//! 
//! ```
//! // Import the object that formally encodes the two symbols for major dimension (row and col)
//! use solar::matrices::matrix_oracle::MajorDimension; 
//! 
//! // Define the struct that represents a scalar matrix with a specified major dimension.
//! pub struct ScalarMatrixDemo
//! {
//!     scalar: f64,                            // the scalar must be a float
//!     major_dimension: MajorDimension,        // row-major or col-major
//! }
//! ```
//! 
//! 
//! The `m`th row or column of a scalar matrix is equal to `alpha` times
//! the `m`th standard unit vector.  This vector has at most one nonzero entry, 
//! namely `(m, alpha)`.  So an oracle representing this matrix should return an 
//! iterator that runs over `(m, alpha)` exactly once, for any `m` (for convenience
//! we'll assume the matrix has infinite size, so any nonnegative integer `m` is 
//! allowed). We can write a function that returns such an iterator, for any `m`:
//! 
//! ```
//! # // Import the object that formally encodes the two symbols for major dimension (row and col)
//! # use solar::matrices::matrix_oracle::MajorDimension; 
//! # 
//! # // A struct representing a scalar matrix.
//! # pub struct ScalarMatrixDemo
//! # {
//! #     scalar: f64,                              // the scalar must be a float
//! #     major_dimension: MajorDimension,          // row-major or col-major
//! # }
//! 
//! /// Given a scalar matrix `M` and an index `i`, return a view of the `i`th 
//! /// row or column vector of `M`.
//! fn get_vector( matrix: &ScalarMatrixDemo, index: usize ) -> Vec< (usize, f64) > 
//! {
//!     let alpha = matrix.scalar.clone();          // make a copy of the scalar
//!     return vec![ (index, alpha) ]
//! }  
//! ```
//! 
//! 
//! Now we can modify the source code in the example above (used for vec-of-vec matrices)
//! to implement the `OracleMajor` trait for our new struct.
//! 
//! ```
//! // ORIGINAL CODE
//! // impl < 'a, IndexCoeffPair > 
//! //     
//! //     OracleMajor
//! //     <   
//! //         'a,
//! //         usize, 
//! //         < IndexCoeffPair as KeyValGet >::Key, 
//! //         < IndexCoeffPair as KeyValGet >::Val, 
//! //     > 
//! //     
//! //     for 
//! //     
//! //     VecOfVec < 'a, IndexCoeffPair > 
//! // 
//! //     where   IndexCoeffPair:    KeyValGet + Clone + 'a,
//! //             Self: 'a
//! // {
//! //     type PairMajor = IndexCoeffPair;
//! //     type ViewMajor = Cloned<std::slice::Iter<'a, IndexCoeffPair>>; 
//! //         
//! //     fn view_major<'b: 'a>( &'b self, index: usize ) -> Self::ViewMajor {
//! //         return self.vec_of_vec[index].iter().cloned()
//! //     } 
//! // }
//! 
//! // MODIFIED CODE
//! 
//! # // Import the object that formally encodes the two symbols for major dimension (row and col)
//! # use solar::matrices::matrix_oracle::*;
//! # 
//! # // A struct representing a scalar matrix.
//! # pub struct ScalarMatrixDemo
//! # {
//! #     scalar: f64,                            // the scalar must be a float
//! #     major_dimension: MajorDimension,        // row-major or col-major
//! # }
//! 
//! impl < 'a >  // delete `IndexCoeffPair`, since our scalar matrix doesn't use this
//!     
//!     OracleMajor
//!     <   
//!         'a,     // we don't have to worry about this
//!         usize,  // our major dimension is indexed by keys of type `usize`
//!         usize,  // our minor dimension is indexed by keys of type `usize`
//!         f64,    // our coefficients are f64
//!     > 
//!     
//!     for 
//!     
//!     ScalarMatrixDemo
//! 
//!     where   Self: 'a    // we deleted `IndexCoeffPiar` so we remove the associated type constraints
//! {
//!     type PairMajor = (usize, f64);              // our vector entries are represented by objects of type `(usize, f64)`
//!     type ViewMajor = Vec< (usize, f64) >;       // our vectors are represented by objects of type `Vec< (usize, f64) >`
//!         
//!     // To define the `major_view` function, we essentially copy/paste the body of
//!     // our `get_vector` into the body of the original `major_view` function.  
//!     // Note that we replace `matrix` with `self`.
//!     fn view_major<'b: 'a>( &'b self, index: usize ) -> Self::ViewMajor {
//!         let alpha = self.scalar.clone();        // make a copy of the scalar
//!         return vec![ (index, alpha) ]  
//!     } 
//! }
//! ```
//! 
//! That's it!  Other traits can be implemented similarly.
//! 
//! **Note** Most functions that take matrix oracles as inputs do not require 
//! their inputs to implement *all* of the oracle traits -- only a *subset*.

use crate::vector_entries::vector_entries::{KeyValGet};
use std::fmt::Debug;
use std::iter::IntoIterator;
use auto_impl::auto_impl; // auto-implement a trait on references to objects that implement the trait

//  DESIGN NOTES
//  ------------
//
//  * REASON TO BAKE ORDERED SLICES INTO TRAITS, RATHER THAN ASKING USER TO IMPLEMENT SEPARATE
//  ORACLE STRUCTS
//  In the U-match decomposition you naturally want to access rows in one order and columns in the
//  opposite order.  You want to input a single matrix to the U-match decomposition.  This means
//  that at a minimum you want a forward row order and a backward column order; the two other
//  orders follow naturally, by symmetry.
//
//  * REASON FOR USING DIFFERENT TYPE + METHODS NAMES FOR EACH VARIANT OF THE
//  ORACLT TRAIT
//  The alternative would be to force the user to write
//  `let vec = < InplementorType< TypeParams > as OracleTrait< MajKey, MinKey, SnzVal > >slice( &matrix, key )`
//  versus
//  `let vec = matrix.major_slice( key );


/// An enum with two values: `Row` and `Col`.
#[derive(Clone, Debug)]
pub enum MajorDimension{
    Row,
    Col
}


//  ---------------------------------------------------------------------------
//  MAJOR DIMENSION 
//  ---------------------------------------------------------------------------

pub trait WhichMajor{ fn major_dimension( &self ) -> MajorDimension; }


//  ---------------------------------------------------------------------------
//  ORACLE MAJOR
//  ---------------------------------------------------------------------------


// #[auto_impl(&)] 
// pub trait Oracle2< 'a, MajKey, MinKey, SnzVal>
// {
//     type ViewMajor: IntoIterator< Item: KeyValGet<MinKey, SnzVal>>;
//     /// Get a major vector.
//     ///
//     /// The order in which terms appear should be the same every time the
//     /// function is called; however, the order need not be sorted.
//     fn   view_major<'b: 'a>( &'b self, index: MajKey ) -> ViewMajor;
// }


// #[auto_impl(&)] 
// pub trait Oracle< MajKey, ViewMajor>
//     where   ViewMajor: IntoIterator,
//             <ViewMajor as IntoIterator>::Item: KeyValGet
// {
//     /// Get a major vector.
//     ///
//     /// The order in which terms appear should be the same every time the
//     /// function is called; however, the order need not be sorted.
//     fn   view_major<'b: 'a>( &'b self, index: MajKey ) -> ViewMajor;
// }


/// Entries may not appear in sorted order.
#[auto_impl(&)] 
pub trait OracleMajor< 'a, MajKey, MinKey, SnzVal>
{
    type PairMajor: KeyValGet< Key=MinKey, Val=SnzVal >;
    type ViewMajor: IntoIterator< Item = Self::PairMajor > + 'a;
    /// Get a major vector.
    ///
    /// The order in which terms appear should be the same every time the
    /// function is called; however, the order need not be sorted.
    fn   view_major<'b: 'a>( &'b self, index: MajKey ) -> Self::ViewMajor;
}

/// Entries appear in ascending order, according to index.
#[auto_impl(&)] 
pub trait OracleMajorAscend< 'a, MajKey, MinKey, SnzVal>
{
    type PairMajorAscend: KeyValGet< Key=MinKey, Val=SnzVal >;
    type ViewMajorAscend: IntoIterator< Item = Self::PairMajorAscend >;
    /// Get a major vector with entries sorted in ascending order of index.
    fn   view_major_ascend<'b: 'a>( &'b self, index: MajKey ) -> Self::ViewMajorAscend;
}

/// Entries appear in descending order, according to index.
#[auto_impl(&)] 
pub trait OracleMajorDescend< 'a, MajKey, MinKey, SnzVal>
{
    type PairMajorDescend: KeyValGet< Key=MinKey, Val=SnzVal >;
    type ViewMajorDescend: IntoIterator< Item = Self::PairMajorDescend >;
    /// Get a major vector with entries sorted in descending order of index.
    fn   view_major_descend<'b: 'a>( &'b self, index: MajKey ) -> Self::ViewMajorDescend;
}

// FOR FUTURE CONSIDERATION
// pub trait OracleMajorAscendScoped< 'a, MajKey, MinKey, SnzVal>
// {
//     type PairMajorAscendScoped: KeyValGet< Key=MinKey, Val=SnzVal >;
//     type ViewMajorAscendScoped: IntoIterator< Item = Self::PairMajorAscendScoped >;
//     /// Get a major vector with entries sorted in ascending order of index, clipped to range [min,
//     /// max).
//     fn   view_major_ascend_scoped<'b: 'a>( &'b self, index: MajKey, min: MinKey, max: MinKey ) -> Self::ViewMajorAscendScoped;
// }

//  ---------------------------------------------------------------------------
//  ORACLE MINOR
//  ---------------------------------------------------------------------------

/// Entries may not appear in sorted order.
#[auto_impl(&)] 
pub trait OracleMinor< 'a, MajKey, MinKey, SnzVal>
{
    type PairMinor: KeyValGet< Key=MinKey, Val=SnzVal >;
    type ViewMinor: IntoIterator< Item = Self::PairMinor >;
    /// Get a minor vector.
    ///
    /// The order in which terms appear should be the same every time the
    /// function is called; however, the order need not be sorted.
    fn   view_minor<'b: 'a>( &'b self, index: MajKey ) -> Self::ViewMinor;
}

/// Entries appear in ascending order, according to index.
#[auto_impl(&)] 
pub trait OracleMinorAscend< 'a, MajKey, MinKey, SnzVal>
{
    type PairMinorAscend: KeyValGet< Key=MinKey, Val=SnzVal >;
    type ViewMinorAscend: IntoIterator< Item = Self::PairMinorAscend >;
    /// Get a minor vector with entries sorted in ascending order of index.
    fn   view_minor_ascend<'b: 'a>( &'b self, index: MajKey ) -> Self::ViewMinorAscend;
}

/// Entries appear in descending order, according to index.
#[auto_impl(&)] 
pub trait OracleMinorDescend< 'a, MajKey, MinKey, SnzVal>
{
    type PairMinorDescend: KeyValGet< Key=MinKey, Val=SnzVal >;
    type ViewMinorDescend: IntoIterator< Item = Self::PairMinorDescend >;
    /// Get a minor vector with entries sorted in descending order of index.
    fn   view_minor_descend<'b: 'a>( &'b self, index: MajKey ) -> Self::ViewMinorDescend;
}


