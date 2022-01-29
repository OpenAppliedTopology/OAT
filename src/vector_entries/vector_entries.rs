//! Basic definitions for sparse vector entries


use std::fmt;
use std::fmt::{Debug};



//  ---------------------------------------------------------------------------
//  KEY-VALUE TRAIT -- GETTING
//  ---------------------------------------------------------------------------


//  DESIGN NOTES
//
//  NOTE 1
//
//  There are least 3 things you might want to get from Val: the variable
//  itself, an immutable ref to the variable, and a mutable ref to the
//  variable.  There are circumstances that call for / exclude any of these
//  three:
//  1)  geting values from a tuple (tuples only let you take ref's out to their
//      values; if you want a non-ref, you might have to clone?)
//  2)  a function that spits out a new value (namely a value that's not stored
//      inside a struct and can't be reference).
//  Currently we're sticking with the simplest option.
//
//  NOTE 2
//
//  Deciding to make Key and Val associated types for the following reasons
//  1)  One runs into type problems (unconstrained types, unused types, etc.)
//      if one makes Key and Val into type parameters
//  2)  This trait is primarily intended to be implemented on the Items of
//      an iterator.  Since Item is uniquely determined by the iterator, and
//      the Key/Val types are uniquely determiend by the item, this doesn't
//      seem to impose any constraints that one would get implicitly if one
//      were using generics




/// Get the key or coefficient from an (key, val) KeyValItem.
pub trait KeyValGet

{
    type Key;
    type Val;

    /// Get the key in the `(key, val)` pair.
    fn key( &self ) -> Self::Key;

    /// Get the val in the `(key, val)` pair.    
    fn val( &self ) -> Self::Val;
}


// Auto-implement for tuples of length 2.
// --------------------------------------

impl< Key, Val >
    KeyValGet
    for 
    ( Key, Val )
    where
        Key: Clone, // this is basically required, since o/w have to implement copy
        Val: Clone  // this is basically required, since o/w have to implement copy
{
    type Key = Key;
    type Val = Val;
    fn key( &self ) -> Key { self.0.clone() }
    fn val( &self ) -> Val { self.1.clone() }
}


//  ---------------------------------------------------------------------------
//  KEY-VALUE TRAIT -- SETTTNG 
//  ---------------------------------------------------------------------------


/// Set the key or valicient from an (key, val) KeyValItem.
pub trait KeyValSet : KeyValGet

{
    /// Set the key in the `(key, val)` pair.    
    fn set_key( &mut self, key: <Self as KeyValGet>::Key ) ;

    /// Set the val in the `(key, val)` pair.        
    fn set_val( &mut self, val: <Self as KeyValGet>::Val ) ;
}


//  Auto-implement for tuples of length 2.
//  --------------------------------------

impl< Key, Val >
    KeyValSet
    for 
    ( Key, Val )
    where
        Key: Clone,
        Val: Clone
{
    fn set_key( &mut self, key: Key ) { self.0 = key }
    fn set_val( &mut self, val: Val ) { self.1 = val }
}





//  ---------------------------------------------------------------------------
//  KEY-VALUE ITEM STRUCT
//  ---------------------------------------------------------------------------


/// Struct encoding a key/value KeyValItem.
///
/// Preferred to a tuple `(key, val)`, since the latter may require 
/// [rewriting in memory](https://www.reddit.com/r/rust/comments/79ry4s/tuple_performance/), 
/// and also has memory overhead for length.
#[derive( Clone )]
pub struct KeyValItem< Key, Val > 
   // where Key: Clone + Debug,
   //       Val: Clone + Debug
{   
    pub key: Key, 
    pub val: Val 
}


//  Custom implementaiton of debug
//  ------------------------------
impl < Key, Val >
    Debug for KeyValItem 
    < Key, Val > 

    where Key:  Debug,
          Val:  Debug

{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("P")
         .field(&self.key)
         .field(&self.val)
         .finish()
    }
}

//  Implement KeyValGet 
//  ------------------------------

impl< Key, Val >
    KeyValGet 
    for 
    KeyValItem< Key, Val > 
    where
        Key: Clone,
        Val: Clone
{
    type Key = Key;
    type Val = Val;
    fn key( &self ) -> Key { self.key.clone() }
    fn val( &self ) -> Val { self.val.clone() }
}

//  Implement KeyValSet
//  --------------------------------------

impl< Key, Val >
    KeyValSet
    for 
    KeyValItem< Key, Val > 
    where
        Key: Clone,
        Val: Clone
{
    fn set_key( &mut self, key: Key ) { self.key = key }
    fn set_val( &mut self, val: Val ) { self.val = val }
}