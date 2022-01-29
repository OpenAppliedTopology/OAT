//! Place a sequence of iterators into a **heap of iterators** (HIT); the
//! result is a new iterator that returns items in sorted order, *provided 
//! that the original iterators are sorted.*
//!
//! **[ Adapted from itertools ]**
//! This module is simlar to (and adapted from) the `kmerge_by` module from 
//! itertools.  The key difference is that **new iterators** can merge into an
//! existing merged iterator.
//!
//! # All you need to know
//!
//! Most people who use this module will only need to use the functions:
//!  [`hit_merge_ascend`], [`hit_bulk_insert`], etc.   
//! The other items in the module are primarily just "internal machinery."



use crate::utilities::heaps::heap::{ heapify, heapify_tail, sift_down };



// ----------------------------------------------------------------------------
// COPIED MACROS    
// ----------------------------------------------------------------------------


// Implementation's internal macros

macro_rules! debug_fmt_fields {
    ($tyname:ident, $($($field:ident).+),*) => {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.debug_struct(stringify!($tyname))
                $(
              .field(stringify!($($field).+), &self.$($field).+)
              )*
              .finish()
        }
    }
}

macro_rules! clone_fields {
    ($($field:ident),*) => {
        fn clone(&self) -> Self {
            Self {
                $($field: self.$field.clone(),)*
            }
        }
    }
}


// ----------------------------------------------------------------------------
// COPIED SIZE HINT FUNCTIONS (from other itertools file)
// ----------------------------------------------------------------------------


/// Add **x** correctly to a **SizeHint**.
#[inline]
fn hacked_size_hint_add_scalar(sh: (usize, Option<usize>), x: usize) -> (usize, Option<usize>)
{
    let (mut low, mut hi) = sh;
    low = low.saturating_add(x);
    hi = hi.and_then(|elt| elt.checked_add(x));
    (low, hi)
}

/// Add **SizeHint** correctly.
#[inline]
fn hacked_size_hint_add(a: (usize, Option<usize>), b: (usize, Option<usize>)) -> (usize, Option<usize>)
{
    let min = a.0.checked_add(b.0).unwrap_or(usize::MAX);
    let max = match (a.1, b.1) {
        (Some(x), Some(y)) => x.checked_add(y),
        _ => None,
    };

    (min, max)
}


// ----------------------------------------------------------------------------
// COPIED ORIGINAL FILE CONTENT
// ----------------------------------------------------------------------------


use std::mem::replace;
use std::fmt;
use itertools::Itertools;



//  HEAD/TAIL 
//  ---------------------------------------------------------------------------

/// Iterator wrapper having two fields: `head` (an item)  and `tail` (an iterator). 
///
/// If an iterator represents a sequence of elements, then `head` corresponds
/// to the first element, and `tail` corresponds to the sequence of all
/// remaining elements.
///
/// `PartialEq`, `Eq`, `PartialOrd` and `Ord` are implemented by comparing sequences based on
/// first items (which are guaranteed to exist).
///
#[derive(Debug)]
pub struct HeadTail<I>
    where I: Iterator
{
    pub head: I::Item,
    pub tail: I,
}

impl<I> HeadTail<I>
    where I: Iterator
{
    /// Constructs a `HeadTail` from an `Iterator`. Returns `None` if the `Iterator` is empty.
    fn new(mut it: I) -> Option<HeadTail<I>> {
        let head = it.next();
        head.map(|h| {
            HeadTail {
                head: h,
                tail: it,
            }
        })
    }

    /// Get the next element and update `head`, returning the old head in `Some`.
    ///
    /// Returns `None` when the tail is exhausted (only `head` then remains).
    fn next(&mut self) -> Option<I::Item> {
        if let Some(next) = self.tail.next() {
            Some(replace(&mut self.head, next))
        } else {
            None
        }
    }

    // ADDAPTED FROM:
    // /// Hints at the size of the sequence, same as the `Iterator` method.
    // fn size_hint(&self) -> (usize, Option<usize>) {
    //     size_hint::add_scalar(self.tail.size_hint(), 1)
    // }

    /// Hints at the size of the sequence, same as the `Iterator` method.
    fn size_hint(&self) -> (usize, Option<usize>) {
        hacked_size_hint_add_scalar(self.tail.size_hint(), 1)
    }

}


impl<I> Clone for HeadTail<I>
    where I: Iterator + Clone,
          I::Item: Clone
{
    clone_fields!(head, tail);
}


//  HEAP
//  ---------------------------------------------------------------------------

//  NB: this was original content of the file; we extracted and moved to a
//      separate file/folder dedicated to heaps.


// /// Make `data` a heap (min-heap w.r.t the sorting).
// fn heapify<T, S>(data: &mut [T], mut less_than: S)
//     where S: FnMut(&T, &T) -> bool
// {
//     for i in (0..data.len() / 2).rev() {
//         sift_down(data, i, &mut less_than);
//     }
// }
// 
// /// Sift down element at `index` (`heap` is a min-heap wrt the ordering)
// pub fn sift_down<T, S>(heap: &mut [T], index: usize, mut less_than: S)
//     where S: FnMut(&T, &T) -> bool
// {
//     debug_assert!(index <= heap.len());
//     let mut pos = index;
//     let mut child = 2 * pos + 1;
//     // the `pos` conditional is to avoid a bounds check
//     while pos < heap.len() && child < heap.len() {
//         let right = child + 1;
// 
//         // pick the smaller of the two children
//         if right < heap.len() && less_than(&heap[right], &heap[child]) {
//             child = right;
//         }
// 
//         // sift down is done if we are already in order
//         if !less_than(&heap[child], &heap[pos]) {
//             return;
//         }
//         heap.swap(pos, child);
//         pos = child;
//         child = 2 * pos + 1;
//     }
// }

//  "Predicate" trait for comparison
//  ---------------------------------------------------------------------------

//  REMARK: This was copied from the itertools library.  The exhact developers
//  suspect that the trait was introduced in order to supply type information 
//  that ordinary closures might miss.  At least, we tried removing the trait
//  and ran into some type erros at compilation.

/// A trait with a single function, `ordering_predicate`, which compares two
/// elements.
///
/// Most users won't use this trait; it's coppied from itertools, and seems
/// to be used primarily for formatting purposes.
pub trait OrderingPredicate<T> {
    fn ordering_predicate(&mut self, a: &T, b: &T) -> bool;
}


//  Implementors of the predicate 
//  ---------------------------------------------------------------------------


//  Less-than
//  ---------

/// Empty struct representing the "less than" relation; usec exclusively
/// `hit_merge_ascend`.
#[derive(Clone)]
pub struct HitOrderLt;

impl<T: PartialOrd> OrderingPredicate<T> for HitOrderLt {
    fn ordering_predicate(&mut self, a: &T, b: &T) -> bool {
        a < b
    }
}


//  Greater-than object
//  -------------------

/// Empty struct representing the "greater than" relation; usec exclusively
/// `hit_merge_descend`.
#[derive(Clone)]
pub struct HitOrderGt;

impl<T: PartialOrd> OrderingPredicate<T> for HitOrderGt {
    fn ordering_predicate(&mut self, a: &T, b: &T) -> bool {
        a > b
    }
}

//  Mutable closure 
//  ---------------

impl<T, F: FnMut(&T, &T)->bool> OrderingPredicate<T> for F {
    fn ordering_predicate(&mut self, a: &T, b: &T) -> bool {
        self(a, b)
    }
}


//  HitMerge object
//  ---------------------------------------------------------------------------


/// An iterator adaptor that merges an abitrary number of base iterators
/// according to an ordering function.
///
/// Iterator element type is `I::Item`.
///
/// See [`hit_merge_by`] for more information.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct HitMerge<I, F>
    where I: Iterator,
{
    pub heap: Vec<HeadTail<I>>,
    pub less_than: F,
}

impl<I, F> fmt::Debug for HitMerge<I, F>
    where I: Iterator + fmt::Debug,
          I::Item: fmt::Debug,
{
    debug_fmt_fields!(HitMerge, heap);
}


impl<I, F> Clone for HitMerge<I, F>
    where I: Iterator + Clone,
          I::Item: Clone,
          F: Clone,
{
    clone_fields!(heap, less_than);
}

impl<I, F> Iterator for HitMerge<I, F>
    where I: Iterator,
          F: OrderingPredicate<I::Item>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.heap.is_empty() {
            return None;
        }
        let result = if let Some(next) = self.heap[0].next() {
            next
        } else {
            self.heap.swap_remove(0).head
        };
        let less_than = &mut self.less_than;
        sift_down(&mut self.heap, 0, |a, b| less_than.ordering_predicate(&a.head, &b.head));
        Some(result)
    }

//    Original version, which uses private methods from itertools library.    
//    fn size_hint(&self) -> (usize, Option<usize>) {
//        self.heap.iter()
//                 .map(|i| i.size_hint())
//                 .fold1(size_hint::add)
//                 .unwrap_or((0, Some(0)))
//    }  
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.heap.iter()
                 .map(|i| i.size_hint())
                 .fold1(hacked_size_hint_add)
                 .unwrap_or((0, Some(0)))
    }  
}      


//  HitMerge makers
//  ---------------------------------------------------------------------------


/// Merge a sequence of iterators into a single iterator; result is sorted by
/// `less_than` if each iterator in the original sequences is sorted by 
/// `less_than`.
///
/// This is very similar to `hit_merge_by` but the type constraint on 
/// the third parameter is different.  Moreover, `hit_merge_by` is 
/// esseentially just a wrapper around this function.  We believe that
/// the original reason for splitting it into two had to do with 
/// engineering type constraints.
fn hit_merge_by_predicate<I, F>(iterable: I, mut less_than: F)
    -> HitMerge<<I::Item as IntoIterator>::IntoIter, F>
    where I: IntoIterator,
          I::Item: IntoIterator,
          F: OrderingPredicate<<<I as IntoIterator>::Item as IntoIterator>::Item>,
{
    let iter = iterable.into_iter();
    let (lower, _) = iter.size_hint();
    let mut heap: Vec<_> = Vec::with_capacity(lower);
    heap.extend(iter.filter_map(|it| HeadTail::new(it.into_iter())));
    heapify(&mut heap, |a, b| less_than.ordering_predicate(&a.head, &b.head));
    HitMerge { heap, less_than }
}


/// Merge a sequence of iterators into a single iterator; result is sorted by
/// `less_than` if each iterator in the original sequence is sorted by 
/// `less_than`.

/// ```
/// use solar::utilities::iterators::hit_merge::hit_merge_by;
/// use num_traits::sign::Signed;
/// 
/// // Result may not respect the order function if the input sequences do not.
/// let unordered_sequences =  vec![ vec![ 2, 0, -4 ] ];
/// let x : Vec<_> = hit_merge_by( unordered_sequences, |a, b| &a.abs() < &b.abs() ).collect();
/// assert_eq!( x , vec![ 2, 0, -4 ] );
///
/// // Result *will* respect the order function if the input sequences do.
/// let ordered_sequences = vec![ vec![1, -2], vec![0, -3] ];
/// let y : Vec<_> = hit_merge_by( ordered_sequences, |a, b| &a.abs() < &b.abs() ).collect();
/// assert_eq!( y, vec![ 0, 1, -2, -3 ] )
/// ```
pub fn hit_merge_by<I, F>(iter: I, less_than: F)
    -> HitMerge<<I::Item as IntoIterator>::IntoIter, F>
    where I: Sized + IntoIterator,
          I::Item: IntoIterator,
          F: FnMut(&<I::Item as IntoIterator>::Item,
                   &<I::Item as IntoIterator>::Item) -> bool
{
    hit_merge_by_predicate(iter, less_than)
}


/// Merge a sequence of iterators into a single iterator; result is sorted in
/// ascending order 
/// if each iterator in the original sequence is sorted in ascending order.
///
/// ```
/// use solar::utilities::iterators::hit_merge::hit_merge_ascend;
/// 
/// // Result may not respect order if the input sequences do not.
/// let data_unordered = vec![ vec![2, 0, 4]];
/// let x : Vec<usize> = hit_merge_ascend( data_unordered ).collect();
/// assert_eq!( x, vec![ 2, 0, 4 ] );
///
/// let data_ordered = vec![ vec![1, 2], vec![0, 3] ];
/// let y : Vec<usize> = hit_merge_ascend( data_ordered ).collect();
/// assert_eq!( y, vec![ 0, 1, 2, 3 ] )
/// ```
pub fn hit_merge_ascend<I>(iterable: I) 
    -> HitMerge<<I::Item as IntoIterator>::IntoIter, HitOrderLt>
    
    where I: IntoIterator,
          I::Item: IntoIterator,
          <<I as IntoIterator>::Item as IntoIterator>::Item: PartialOrd
{
    hit_merge_by_predicate(iterable, HitOrderLt)
}

/// Merge a sequence of iterators into a single iterator; result is sorted in
/// descending order 
/// if each iterator in the original sequence is sorted in descending order.
///
/// ```
/// use solar::utilities::iterators::hit_merge::hit_merge_descend;
/// 
/// // Result may not respect order if the input sequences do not.
/// let data_unordered = vec![ vec![2, 0, 4]];
/// let merged_unordered : Vec<usize> = hit_merge_descend( data_unordered ).collect();
/// assert_eq!( merged_unordered, vec![ 2, 0, 4 ] );
///
/// let data_ordered = vec![ vec![6, 4], vec![5, 3] ];
/// let merged_ordered : Vec<usize> = hit_merge_descend( data_ordered ).collect();
/// assert_eq!( merged_ordered, vec![ 6, 5, 4, 3 ] )
/// ```
pub fn hit_merge_descend<I>(iterable: I) 
    -> HitMerge<<I::Item as IntoIterator>::IntoIter, HitOrderGt>
    
    where I: IntoIterator,
          I::Item: IntoIterator,
          <<I as IntoIterator>::Item as IntoIterator>::Item: PartialOrd
{
    hit_merge_by_predicate(iterable, HitOrderGt)
}


//  ---------------------------------------------------------------------------
//  NEW CODE: MODIFY HEAP POST-HOC
//  ---------------------------------------------------------------------------


/// Append a new seqeunce of iterators to the merge heap.
///
/// ```
/// use solar::utilities::iterators::hit_merge::{hit_merge_ascend, hit_bulk_insert};
/// 
/// // Create a HIT iterator, and pop off some elements
/// let ordered_sequences = vec![ vec![1, 2], vec![0, 3] ];
/// let mut hit = hit_merge_ascend( ordered_sequences );
/// assert_eq!( Some(0), hit.next() );
/// assert_eq!( Some(1), hit.next() );
/// 
/// // Insert new iterators into the heap.
/// hit_bulk_insert( &mut hit, vec![ vec![ 4, 5 ], vec![ 6 ] ] );
/// let vec : Vec<usize> = hit.collect();
/// assert_eq!( vec, vec![ 2, 3, 4, 5, 6 ] )
/// ```
pub fn hit_bulk_insert< I, F >( 
    merged : &mut HitMerge<<I::Item as IntoIterator>::IntoIter, F>, 
    iterable: I,
    )
    where I: IntoIterator,
          I::Item: IntoIterator,
          F: OrderingPredicate<<<I as IntoIterator>::Item as IntoIterator>::Item>
{
    // this is where we'll start the bulk heapify
    let tail_base = merged.heap.len();     

    // push the new iterators onto the heap
    let iter = iterable.into_iter();
    merged.heap.extend(iter.filter_map(|it| HeadTail::new(it.into_iter())));
    
    // heapify
    let less_than = &mut merged.less_than;
    heapify_tail(&mut merged.heap, |a, b| less_than.ordering_predicate( &a.head, &b.head),
    & tail_base);
}


