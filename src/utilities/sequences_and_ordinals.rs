

use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use serde::{Deserialize, Serialize};




//  ---------------------------------------------------------------------------
//  BIMAPS WITH {0, .., N}
//  ---------------------------------------------------------------------------

/// Represents a surjective map {0,..,N} -> S and another map S -> {0, .., N}; if
/// one of these maps is a bijection, then the other should be its inverse.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BiMapSequential< T >
    where T : Hash + Eq
{ 
    pub ord_to_val: Vec< T >, 
    pub val_to_ord: HashMap< T, usize > 
}

impl < T > BiMapSequential < T > 
    where T: Clone + Hash + Eq
{
    // /// Evaluate the function {0, ..., N} -> S
    // pub fn elt( &self, ord: usize ) -> Option< T >   { if ord < self.ord_to_val.len()self.ord_to_val[ ord ].clone() }

    // /// Evaluate the function S -> {0, ..., N}
    // pub fn ord( &self, elt_ref: &T     ) -> usize { self.val_to_ord.get( elt_ref ).unwrap().clone() }   

    /// Evaluate the function {0, ..., N} -> S
    pub fn ord( &self, a: &T ) -> Option< usize > { 
        self.val_to_ord.get( a ).map(|x| x.clone()) 
    }

    /// Evaluate the function S -> {0, ..., N}
    pub fn val( &self, a: usize ) -> Option< T > { 
        if a < self.ord_to_val.len() { Some( self.ord_to_val[ a ].clone() ) } else { None }
    }      
    
    /// Create sequential bimap
    pub fn from_vec( vec: Vec< T > ) -> BiMapSequential< T >
    {
        let hash    =   HashMap::from_iter(
                            vec.iter().cloned().enumerate().map(|x| (x.1, x.0) )
                        );
        BiMapSequential{ ord_to_val: vec, val_to_ord: hash}
    }
}

impl    < T > 
        FromIterator< T > 
        for 
        BiMapSequential < T > 
    where   T:  Clone + Hash + std::cmp::Eq
{
    fn from_iter< I: IntoIterator<Item=T>>(iter: I) -> Self {

        let vec     =   Vec::from_iter( iter );

        BiMapSequential::from_vec( vec )
    }
}


//  ---------------------------------------------------------------------------
//  PRIMITIVE ORDINALS
//  ---------------------------------------------------------------------------


// #[derive(Clone, Debug, PartialEq)]
// pub struct OrdinalData < T : Ord + Eq + PartialOrd + PartialEq + Hash > {
//     pub ord_to_val:  Vec< T >,
//     pub val_to_ord:  HashMap< T, usize >
// }

// impl    < T >
//         OrdinalData
//         < T >
//         where T : Ord + Eq + PartialOrd + PartialEq + Hash + Clone
// {
//     /// The ordinal of the raw filtration value
//     pub fn ord( &self, a: &T ) -> Option< usize > { 
//         self.val_to_ord.get( a ).map(|x| x.clone()) 
//     }
//     /// The raw filtration value of the ordinal
//     pub fn val( &self, a: usize ) -> Option< T > { 
//         if a < self.ord_to_val.len() { Some( self.ord_to_val[ a ].clone() ) } else { None }
//     }    
// }


/// Given a vector of elements of a poset, first sort the vector and delete 
/// duplicate entries; the resulting vector represents a bijection from 
/// {0, .., n} to the set of unique values in the vector.  Store this new vector
/// in an OrdinalData struct together with a hashmap representing the inverse bijection
pub fn ordinate_unique_vals < FilRaw > ( v: & Vec< FilRaw > ) -> BiMapSequential< FilRaw > 
    where FilRaw: Ord + Hash + Clone
{
    let mut a       =   v.clone();
    let mut b       =   HashMap::new();
    a.sort();       // sort entries
    a.dedup();      // remove duplicates

    for (i, t) in a.iter().enumerate() {
        b.insert( t.clone(), i.clone() );
    }

    BiMapSequential { ord_to_val: a, val_to_ord: b }
}


pub fn  reverse_hash_sequential< T: Hash + std::cmp::Eq + Clone >( 
            vec: & Vec< T >
        ) 
        -> 
        HashMap< T, usize >
{
    let mut rev_hash    =   HashMap::new();

    for (i, t) in vec.iter().enumerate() {
        rev_hash.insert( t.clone(), i.clone() );
    }

    rev_hash
}