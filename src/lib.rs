
//! Sparse matrix algebra: you define the matrix, SOLAR does the rest.
//! 
//! # Start here
//! 
//! This page provides a high-level overview to help orient you to the package.
//! 
//! If you're new to Rust, you might find the VS Code editor useful (it has an
//! extension for rust, with syntax highlighting and convenient error-checking
//! features).
//! 
//! # Traits for matrices, vectors, and vector entries
//! 
//! SOLAR revolves around three objects: matrices, 
//! vectors, and vector entries.  However, we don't define a matrix object, 
//! vector object, and entry object.  Instead we use the Rust notion of a 
//! [trait](https://doc.rust-lang.org/book/ch10-02-traits.html).  You can write
//! your own object, and as long as it implements the appropriate traits, the 
//! library will be able to use it as a matrix, vector, or vector entry.
//! 
//! * **Vector** traits
//! 
//!     SOLAR has no traits for sparse vectors.  In SOLAR, a sparse vector is simply 
//! represented as an [iterator](https://doc.rust-lang.org/book/ch13-02-iterators.html) 
//! that loops over sparse vector entries (defined next).
//! 
//! * **Vector entry**  traits
//! 
//!     An entry in a sparse vector `v`, is a pair `(i, a)` such that `v[i] = a`.  
//!     There are many ways to store a vector entry in computer memory, e.g. in a tuple, 
//!     a list, a dictionary, etc.  There are some things we'd like to do with an entry,
//!     no matter the data structure stores it:
//! 
//!     * [KeyValGet](vector_entries::vector_entries::KeyValGet) 
//!         allows one to determine the value of  `i` or `a`.  
//! 
//! 
//!     * [KeyValSet](vector_entries::vector_entries::KeyValSet)
//!         allows one to change the value of  `i` or  `a`.  
//!     
//!     ```
//!     // Import the KeyValGet and KeyValSet traits, so that we can use them.
//!     use solar::vector_entries::vector_entries::{KeyValGet, KeyValSet}; 
//!
//!     // Define a vector entry.
//!     // Every tuple of length 2 (that meets certain requirements) implements the KeyValGet and KeyValSet traits, automatically.
//!     let mut vector_entry = (1, 0.);
//!
//!     // Use the methods associated with these traits.
//!     // The associated methods are `.key()`, `.val()`, `.set_key`, and `.set_val`
//!     assert_eq!( vector_entry.key(), 1  ); // the .key() method retreives the index
//!     assert_eq!( vector_entry.val(), 0. ); // the .val() method retreives the coefficient
//!
//!     vector_entry.set_key( 2 );            // the .set_key() method sets the index
//!     assert_eq!( vector_entry.key(), 2  ); 
//!     ```
//! 
//! 
//! * **Matrix**  traits
//!       
//! 
//!     *   In many, though not all, sparse 
//!         matrices, it's easier to look up rows than to look up columns, or vice versa.  
//!         We call the easy dimension the *major dimension*.
//!         The [WhichMajor](matrices::matrix_oracle::WhichMajor) traight allows one to
//!         determine the major dimension.
//! 
//!     * The so-called "oracle" traits allow one to look up a row or column of a matrix. 
//!       These traits are described in the [matrix_oracle](matrices::matrix_oracle)
//!       module. They are:
//! 
//!         [OracleMajor](matrices::matrix_oracle::OracleMajor): returns the entries in a row (if the matrix is row-major) or 
//!         column (if the matrix is column-major).  Entries may not appear in sorted order. <br />
//!         [OracleMajorAscend](matrices::matrix_oracle::OracleMajorAscend): returns entries in ascending order of index <br />
//!         [OracleMajorDescend](matrices::matrix_oracle::OracleMajorDescend): returns entries in descending order of index <br />
//!         [OracleMinor](matrices::matrix_oracle::OracleMinor): returns the entries in a row (if the matrix is row-major) or 
//!         column (if the matrix is column-major).  Entries may not appear in sorted order. <br />
//!         [OracleMinorAscend](matrices::matrix_oracle::OracleMinorAscend): returns entries in ascending order of index, <br />
//!         [OracleMinorDescend](matrices::matrix_oracle::OracleMinorDescend): returns entries in descending order of index, <br />
//!        
//!        
//!     ```
//!     // Import the definition of a sparse vec-of-vec matrix, as well as some other traits.
//!     use solar::matrices::implementors::vec_of_vec::VecOfVec;
//!     use solar::matrices::matrix_oracle::{MajorDimension, OracleMajor};
//!     use std::iter::FromIterator;
//! 
//!     // Create a vector-of-vectors sparse matrix.  
//!     // In particular, we will construct a 2x2 upper triangular matrix with 1's above the diagonal.  
//!     // The matrix is row-major; for vec-of-vec matrices, that means that each vector represents a row.
//!     let matrix =    VecOfVec::new(
//!                         MajorDimension::Row, // this declares that the matrix is row-major
//!                         vec![ vec![(0, 1.), (1, 1.)], vec![(1, 1.)] ], // the vector of vectors,
//!                     );
//! 
//!     // Access a row.
//!     // Since this matrix is row-major, we use the OracleMajor trait.  This trait accesses
//!     // vectors along the major dimension via the command `view_major`
//!     let row_iterator    =   matrix.view_major( 0 ); // access the 0th row.  the result is an iterator.
//!     let row_vector      =   Vec::from_iter( row_iterator ); // collect the elements of the iterator into a Rust vector
//!     assert_eq!( row_vector, vec![(0, 1.), (1, 1.)] );
//! 
//!     ```

//! 
//! 
//! 
//! # Operations on sparse vector iterators
//! 
//! Let `iter_a`, `iter_b`, and `iter_c` be sparse vectors, i.e. iterators that run over 
//! sparse matrix entries.  For example, we could define `iter_a`, `iter_b`, `iter_c` as follows
//! 
//! ```
//! // First define the entries
//! // Note that `vec!` creates a standard rust vector, which is different 
//! // from the sort of vector we care about)
//! let entries_a   =   vec![ (1, 1.), (4, 4.) ];
//! let entries_b   =   vec![ (2, 2.), (3, 3.) ];
//! let entries_c   =   vec![ (1, 1.), (2, 2.), (3, 3.), (3, 3.), (4, 0.) ];
//! 
//! // Now define the sparse vector iterators
//! // Note that `iter()` creates an interator, and `cloned()` reformats 
//! // the entries of each iterator.
//! let iter_a      =   entries_a.iter().cloned(); 
//! let iter_b      =   entries_b.iter().cloned();
//! let iter_c      =   entries_c.iter().cloned();
//! ```
//! 
//! Let's also define an object that represents the coefficient ring we want to work with.
//!     
//! ```
//! // Load the module that allows us to define our coefficient ring.
//! use solar::rings::ring_native::*;

//! // Define a coefficient ring (in this case, floating point real numbers)
//! let ring = NativeDivisionRing::<f64>::new();   
//! ```
//!     
//! * **Scale, drop zeros, gather**
//! 
//!     We can scale, drop zero entries, and gather terms as follows
//!     
//!     ```
//!     use solar::vectors::vector_transforms::*;
//!     use solar::rings::ring_native::*;
//! 
//!     # // Define the vector
//!     # let entries_a   =   vec![ (1, 1.), (4, 4.) ];
//!     # let entries_c   =   vec![ (1, 1.), (2, 2.), (3, 3.), (3, 3.), (4, 0.) ];
//!     # let iter_a      =   entries_a.iter().cloned();
//!     # let iter_c      =   entries_c.iter().cloned();
//!     #
//!     # // Define a coefficient ring (in this case, floating point real numbers)
//!     # let ring = NativeDivisionRing::<f64>::new();        
//!       
//!     // SCALE A VECTOR BY 2.
//!     // Example: convert [ (1, 1.), (4, 4.) ] into [ (1, 2.), (4, 8.) ]
//!     let scaled : Vec<_> = iter_a
//!                             .clone() // this makes a copy of the iterator, so the original stays unchanged
//!                             .scale( ring.clone(), 2. )
//!                             .collect(); // this collects the entries of the iterator into a standard Rust vector
//!     assert_eq!( scaled, vec![ (1, 2.), (4, 8.) ]);
//!       
//!     // DROP ZERO ENTRIES
//!     // Example: convert [ (1, 1.), (2, 2.), (3, 3.), (3, 3.), (4, 0.) ] into [ (1, 1.), (2, 2.), (3, 3.), (3, 3.) ]
//!     let dropped : Vec<_> = iter_c
//!                             .clone() // this makes a copy of the iterator, so the original stays unchanged
//!                             .drop_zeros( ring.clone() )
//!                             .collect(); // this collects the entries of the iterator into a standard Rust vector
//!     
//!     assert_eq!( dropped, vec![ (1, 1.), (2, 2.), (3, 3.), (3, 3.) ]);
//!       
//!     // GATHER CONSECUTIVE ENTRIES THAT SHARE THE SAME INDEX
//!     // The resulting vector has no repeating consecutive indices; each index gets 
//!     // the sum of the corresponding coefficients.
//!     // Example: convert [(1,1.), (1,0.5), (2,0.), (1,0.)] into [(1,1.5), (2,0.), (1,0.)]
//!     let gathered : Vec<_> = iter_c
//!                             .clone() // this makes a copy of the iterator, so the original stays unchanged
//!                             .peekable() // this puts the iterator in a slightly different form, which is compatible with gather
//!                             .gather( ring.clone() )
//!                             .collect(); // this collects the entries of the iterator into a standard Rust vector
//!     assert_eq!( gathered, vec![ (1, 1.), (2, 2.), (3, 6.), (4, 0.) ]);   
//!     ```
//! * **Combine iterators in sorted order** (basic)
//! 
//!   We can combine two iterators, `A` and `B`, into a single iterator `C` using the 
//!     [merge](https://docs.rs/itertools/0.7.2/itertools/fn.merge.html) function from [itertools](https://docs.rs/itertools/latest/itertools/). 
//!     The resulting iterator, `C`, is a 
//!     [Merge struct](https://docs.rs/itertools/0.7.8/itertools/structs/struct.Merge.html).
//!     Iterator `C` will iterate over all the entries in `A` and `B`.
//!     If the items of `A` and `B` appear in sorted order, then the items of `C` will also 
//!     appear in sorted order.
//!     ```
//!     # let entries_a   =   vec![ (1, 1.), (4, 4.) ];
//!     # let entries_b   =   vec![ (2, 2.), (3, 3.) ];
//!     # let iter_a      =   entries_a.iter().cloned(); 
//!     # let iter_b      =   entries_b.iter().cloned();
//! 
//!     use itertools::merge;
//!     use std::iter::FromIterator;
//!     
//!     // Merge [ (1, 1.), (4, 4.) ] and [ (2, 2.), (3, 3.) ].
//!     // The entries in these vectors are in sorted order, so the resulting iterator will 
//!     // also produce items in sorted order.
//!     let iter_merged   =   merge(iter_a, iter_b);
//!     let entries_mrgd  =   Vec::from_iter(iter_merged);
//!     assert_eq!( entries_mrgd, vec![ (1, 1.), (2, 2.), (3, 3.), (4, 4.) ])
//!     ```
//! 
//!     We can merge any `k` iterators of the same kind using the 
//!     [merge](https://docs.rs/itertools/0.7.2/itertools/fn.merge.html) 
//!     function from 
//!     [itertools](https://docs.rs/itertools/latest/itertools/).  
//! 
//! * **Combine iterators in sorted order** (advanced)
//!  
//!     For advanced usage (eg matrix reduction), we also provide a
//!     customized merge process in the [hit_merge](utilities::iterators::hit_merge) module.
//! 
//! * **Add**
//! 
//!     We can add the vectors represented by `iter_a` and `iter_b` by 
//!     first combining (e.g., with the `merge` function discussed above), 
//!     then applying the `gather` method.
//! 
//! * **Subtract**
//! 
//!     We can subtract  `iter_a` from `iter_b` by first scaling `iter_a` by `-1`, then adding.
//!̦
//! 
//! # Where to learn more
//! 
//! The list of modules shown below gives further details on many 
//! of the topics introduced above.
//! 
//! If you can't find what you need, feel free to reach out to the ExHACT team!


pub mod rings;
pub mod vectors;
pub mod matrices;
pub mod matrix_factorization;
pub mod utilities;
pub mod vector_entries;
//pub mod iterators::itertools_kmerge_impl;
//pub mod itertools_kmerge_impl;
