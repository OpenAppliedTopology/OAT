//! Transformations on sparse vector iterators: [`Gather`] , [`Scale`], [`DropZeros`].
//!
// //! By definition, a *sparse vector iterator* (SVI) is struct that implements `Iterator< Item = KeyValItem< Index, 
// //! Coeff > >`.

use crate::utilities::iterators::utility::{PeekUnqualified};
use crate::vector_entries::vector_entries::{KeyValGet, KeyValSet};
use crate::rings::ring::{Semiring};
use std::fmt::{Debug};


// //  ---------------------------------------------------------------------------
// //  KEY-VALUE KeyValItemS
// //  ---------------------------------------------------------------------------
// 
// 
// /// Struct encoding a key/value KeyValItem.
// ///
// /// Preferred to a tuple `(key, val)`, since the latter may require 
// /// [rewriting in memory](https://www.reddit.com/r/rust/comments/79ry4s/tuple_performance/), 
// /// and also has memory overhead for length.
// #[derive( Clone )]
// pub struct KeyValItem< Key, Val > 
//    // where Key : Clone + Debug,
//    //       Val : Clone + Debug
// {   
//     key: Key, 
//     val: Val 
// }
// 
// impl < Key, Val >
//     Debug for KeyValItem 
//     < Key, Val > 
// 
//     where Key : Clone + Debug,
//           Val : Clone + Debug
// 
// {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_tuple("P")
//          .field(&self.key)
//          .field(&self.val)
//          .finish()
//     }
// }



//  ---------------------------------------------------------------------------
//  ITERATOR WRAPPER / TRANSFORMATIONS 
//  ---------------------------------------------------------------------------



//  ---------------------------------------------------------------------------
//  DROP ZEROS


/// Iterates over the same items as `self.undropped`, skipping any items with coefficient 0.
/// 
/// Formally, we skip any item `x` such that `self.ring.is_0( x.val() )==true`.
/// 
#[derive(Debug, Clone)]
pub struct DropZeros 
    < Sprs, Ring > 
    where   Sprs:           Iterator,
            Sprs::Item:     KeyValGet,
            Ring:           Semiring< <Sprs::Item as KeyValGet>::Val >,
            // <Sprs::Item as KeyValGet>::Key: Debug + Clone,
            // <Sprs::Item as KeyValGet>::Val: Debug + Clone,
    
{
    undropped: Sprs,
    ring: Ring,
}

impl    < Sprs, Ring > 
        
        Iterator for DropZeros 
        
        < Sprs, Ring > 

        where   Sprs:           Iterator,
                Sprs::Item:     KeyValGet,
                Ring:           Semiring< <Sprs::Item as KeyValGet>::Val >,
                // <Sprs::Item as KeyValGet>::Key: Debug + Clone,
                // <Sprs::Item as KeyValGet>::Val: Debug + Clone,

{
    type Item = Sprs::Item;

    fn next( &mut self) -> Option< Self::Item > 
    {
        let mut next = self.undropped.next();

        while let Some( ref x ) = next {
            if self.ring.is_0( x.val() ) { next = self.undropped.next(); }
            else {break} 
        }
        return next 
    }
}



//  ---------------------------------------------------------------------------
//  SCALE


/// Iterates over the same items as `self.unscaled`, with all coefficients scaled by `self.scale`.
#[derive(Debug, Clone)]
pub struct Scale      
    
    < Sprs, Ring > 
    
    where   Sprs:           Iterator,
            Sprs::Item:     KeyValGet + KeyValSet,
            Ring:           Semiring< <Sprs::Item as KeyValGet>::Val >,
            // <Sprs::Item as KeyValGet>::Key: Debug + Clone,
            <Sprs::Item as KeyValGet>::Val: Debug + Clone,          
{
    unscaled:   Sprs,
    ring:       Ring,
    scale:      <Sprs::Item as KeyValGet>::Val
}

impl    < Sprs, Ring > 
        
        Iterator for Scale
        
        < Sprs, Ring > 
   
        where   Sprs:           Iterator,
                Sprs::Item:     KeyValGet + KeyValSet,
                Ring:           Semiring< <Sprs::Item as KeyValGet>::Val >,
                <Sprs::Item as KeyValGet>::Key: Debug + Clone,
                <Sprs::Item as KeyValGet>::Val: Debug + Clone,

{
    type Item = Sprs::Item;

    fn next( &mut self) -> Option< Self::Item > 
        {
            if let Some( mut x ) = self.unscaled.next() { 
                x.set_val( 
                    self.ring.multiply( 
                        x.val().clone(), 
                        self.scale.clone() 
                    )
                );
                Some(x)
            }
            else { None }
        }
}


//  ---------------------------------------------------------------------------
//  GATHER COEFFICIENTS 


/// Iterates over the same items as `self.ungathered`, except that 
/// consecutive entries with equal indices are merged into a single entry whose
/// coefficient is the sum of the coefficients.
#[derive(Debug, Clone)]
pub struct Gather
    
    < Sprs, Ring > 

    where   Sprs:           Iterator + PeekUnqualified,
            Sprs::Item:     KeyValGet + KeyValSet,
            Ring:           Semiring< <Sprs::Item as KeyValGet>::Val >,
            // <Sprs::Item as KeyValGet>::Key: Debug + Clone,
            // <Sprs::Item as KeyValGet>::Val: Debug + Clone,    

{
    ungathered: Sprs,
    ring: Ring,
}



impl    < Sprs, Ring > 

        Iterator for Gather
    
        < Sprs, Ring > 
   
        where   Sprs:           Iterator + PeekUnqualified,
                Sprs::Item:     KeyValGet + KeyValSet,
                Ring:           Semiring< <Sprs::Item as KeyValGet>::Val >,
                <Sprs::Item as KeyValGet>::Key: PartialEq,
                // <Sprs::Item as KeyValGet>::Key: Debug + Clone + PartialEq,
                // <Sprs::Item as KeyValGet>::Val: Debug + Clone,  
{
    type Item = Sprs::Item;

    fn next( &mut self) -> Option< Self::Item > 
    {
        if let Some( mut x ) = self.ungathered.next() {
            while let Some( peek ) = self.ungathered.peek_unqualified() {
                if peek.key() == x.key() { 
                    x.set_val(
                        self.ring.add( 
                            x.val(), 
                            peek.val() 
                        )
                    );
                    let _ = self.ungathered.next(); // we have already gotten what we need
                }
                else { break }
            }
            return Some( x )
        }
        else 
        { None }
    }
}



//  ---------------------------------------------------------------------------
//  SPARSE VECTOR TRAIT
//  ---------------------------------------------------------------------------


/// Transformations on sparse vector iterators.
///
/// The methods in this trait are implemented automatically for structs that
/// implement `Iterator< Item : KeyValGet >`.  They are not complicated
/// in general; rather, they're provided as a convenience for 
/// chained transformations.
pub trait Transforms

    where   Self:           Iterator,
            Self::Item:     KeyValGet,

{

    /// Returns an interator that iterates over the same items as `self`, 
    /// skipping any items with coefficient 0.
    fn  drop_zeros< Ring >
        ( self, ring: Ring ) 
        -> 
        DropZeros< Self, Ring >
        
        where   Self:           Iterator + Sized,
                Self::Item:     KeyValGet,
                Ring:           Semiring< <Self::Item as KeyValGet>::Val >,
                // <Self::Item as KeyValGet>::Key: Debug + Clone,
                // <Self::Item as KeyValGet>::Val: Debug + Clone,

    {
        DropZeros{ undropped: self, ring: ring }
    }

    /// Returns an interator that iterates over the same items as `self`, 
    /// with all coefficients scaled by `scalar`.
    fn  scale 
        < Ring > 
        ( self, ring: Ring, scalar: <Self::Item as KeyValGet>::Val )
        -> 
        Scale< Self, Ring >
        
        where   Self:           Iterator + Sized,
                Self::Item:     KeyValGet + KeyValSet,
                Ring:           Semiring< <Self::Item as KeyValGet>::Val >,
                // <Self::Item as KeyValGet>::Key: Debug + Clone,
                <Self::Item as KeyValGet>::Val: Debug + Clone,
        {
            Scale{ unscaled: self, ring: ring, scale: scalar }
        }

    /// Returns an interator that iterates over the same items as `self`, except that 
    /// consecutive entries with equal indices are merged into a single entry whose
    /// coefficient is the sum of the coefficients.  
    fn gather < Ring > ( self, ring: Ring )
        -> Gather< Self, Ring >

        where   Self:           Iterator + Sized + PeekUnqualified,
                Self::Item:     KeyValGet + KeyValSet,
                Ring:           Semiring< <Self::Item as KeyValGet>::Val >,
                <Self::Item as KeyValGet>::Key:PartialEq,
                // <Self::Item as KeyValGet>::Key: Debug + Clone,
                // <Self::Item as KeyValGet>::Val: Debug + Clone,               
        {
            Gather{ ungathered: self, ring: ring  } 
        }
}

// We implement this trait automatically on all iterators.
impl    < Sprs > 

        Transforms 
        
        for Sprs  

        where   Sprs:           Iterator,
                Sprs::Item:     KeyValGet,
                // <Sprs::Item as KeyValGet>::Key: Debug + Clone,
                // <Sprs::Item as KeyValGet>::Val: Debug + Clone,          
{} // everything implemented automatically


//     DEFINE SPARSE VECTOR ITERATOR AS
//
//     struct Svi< Iter, Index, Ring, Coeff > 
//         where   Iter: Iterator< Item = KeyValItem<Index,Coeff> >,
//                 Ring: Semiring< Coeff >,
// 
//         {    
// }
// 
// 
//     svi.transform( DropZeros::y,
//                    Scale::Coeff(3)
//                    Merge::y,
//                    ring
//                 )
// 
// 
//     M0 + M1 + M4 + sum(M[2..4,:]) + sum(2 * M[4..6,:]) + 
// 
//     R = gen_ring();
//     C = gen_comparator();
// 
//     let svi0  = M.maj(majkeys[0]).simplify(&R);
//     let svi1  = M.maj(majkeys[1]).simplify(&R);
//     let svi2  = (2..4)
//                     .map(|x| 
//                          M.maj(majkeys[x].simplify(&R)
//                     )
//     let svi3  = (4..6)
//                     .map(|x| 
//                          M.maj(majkeys[x].simplify(&R)
//                         )
//                     .kmerge_by(&C)
//                     .simplify(&R)
//                     .map(|y|
//                             (y.0, R.multiply( &y.1, &2)
//                     )
// 
//     let svi4  = M.maj(majkeys[6]).simplify(&R);
// 
//     let agg = (svi0, svi1, svi4)
//                 .hit_merge(             &C )
//                 .hit_bulk_insert( svi2, &C )
//                 .merge_by( svi3,        &C )
//                 .simplify()
// 
// 
//     !!!!!!
//     drain_monomials
//     drain_monomials_ordered
// 
//     let agg = LciSimplified::combine( ( (svi0, t0), (svi1, t1), (svi3, t3) ), R, C )
//                 .add_combination(( (svi4, t4), (svi5, t5) ))// universal input format
//                 .add_svi( svi4, None    )
//                 .add_svi( svi5, Some(2) )
//                 .add_lci( lci2, None )                            // option for other lci
//                 .add_lci( lci2, Some(3) )                            // option for other lci
//                 .add_sum( (svi6, svi7,  svi8 ), None    )            // avoid 1; avoid nested parentheses 
//                 .add_sum( (svi9, svi10, svi11), Some(2) )  // option to scale
// 
// 
//                 .add_svi( svi4 )                            // avoid 1; avoid nested parentheses 
//                 .add_svi_scaled( svi5, 2 )                  // option to scale
//               
//                 .add_sum( (svi6, svi7, svi8) )            // avoid 1; avoid nested parentheses 
//                 .add_sum_scaled( (svi9, svi10, svi11), 2 )  // option to scale
//                 
//                 .add_lci( lci2 )                            // option for other lci
//                 .add_lci_scaled( lci3, 4 )
// 
//                 .add_svi( svi4 )                            // avoid 1; avoid nested parentheses 
//                 .add_lci( lci2 )                            // option for other lci
//                 .add_sum( (svi6, svi7, svi8) )              // avoid 1 
//                 .add_svi_scaled( svi5, 2 )                  // option to scale
//                 .add_lci_scaled( lci3, 4 )                  // option to scale
//                 .add_sum_scaled( (svi9, svi10, svi11), 2 )  // option to scale
//                 
// 
// 
//                 .add_svi( svi4, R.one() )
//                 .add_svi( svi7, 2       )
//                 .add_lci( lci0, R.one() )
//                 .add_lci( lci8, 3
//                 .add_sum( (svi6, svi7,  svi8 ), R.one() )
//                 .add_sum( (svi9, svi10, svi11), 4       )
// 
//                 .add( Term::Svi(svi4) )
//                 .add( Term::Lci(lci5) )
// 
// 
//                 .add( Term::Svi(svi4), STerm::Svi(svi5, 2), STerm::Lci(lci, 4) )
//                 .add_svi(svi1).add_lci(lci2).add_sum_scaled( (svi2, svi3), 2)
//                 .add_svi(svi1, None).add_svi(svi
// 
// 
//                 .add_combination( ((svi0, 1), (svi2, 2), (svi3, 3)) ).add_sum( (svi4,) ).add
//                 .add_lci( lci, 4 )
//                 .add_sum(
// 
//                 X = LciSimplified::sum( (svi0, svi1), R, C ).add_scaled( (svi
// 
//                 X = LciSimplified::new().add( (
//                         Term::Svi( svi0 ),
//                         Term::Svi( svi1 ).x(2),
//                         Term::Lci( lci0 ),
//                         Term::Lci( lci1 ).x(3),
//                         Term::Sum( (svi2, svi3, svi4) ),
//                         Term::Sum( (svi5, svi6, svi7) ).x(4),
//                         Term::Com( ( (svi10, 3), (svi11, 4) ),
//                         Term::Com( ( (svi12, 3), (svi13, 4) ).x(7),
//                         Term::Com( iter1 ).x(7),
//                         Term::Sum( iter0 ).x(4),
//                         )
//                     )
// 
//                 X.add( Term::Sum(( svi0, svi1, svi2 )), Term::Svi( svi3 ).x(2) );
// 
//                 x.change_add( y, C);
//                 x.change_add_simplify( y, C);
// 
// 
//     enum LTerms< I, Index, Coeff >
//         where I: IntoIterator< Item = KeyValItem<Index,Coeff> >
// 
//     {
//         Scale(   I       ),
//         Unscaled( I, Coeff),
//         Combination( 
//     }
// 
//     lc.add( LTerm::Unscaled( svi    ) )
//     lc.add( LTerm::Scale(   svi, x ) )
//    
//     lc.add_n( LTerms::Unscaled( ( svi0, svi1, svi2 )    ) )
//     lc.add_n( Lterms::Scale(   ( svi0, svi1, svi2 ), y ) )
//   
//     lc.add_n( Lterms::Combination( ( (svi0, t0), (svi1, t1), (svi2, t2) ) )
// 
// 
//     
//     lc.add(        svi0    )
//     lc.add_scaled( svi1, 2 )
// 
//     lc.add_n(        (svi2, svi3)    )
//     lc.add_n_scaled( (svi4, svi5), 2 )
//     
//     lc.add_n_multiscaled(( (svi6, 2), (svi7, 3) ))
//     lc.add_lci( lci )
// //--------------------------------------------------------------
// 
//     lc.add_1( svi );
//     lc.add_n( (svi0, svi1) );
//     lc.add_1_times( svi, 2 );
//     lc.add_n_times( ( svi0, svi1 ), 2 )
//     
//     lc.add_combination( ( (svi0, 2), (svi1, 3) ) );
// 
// 
//     lc.add( LTerm::times_1( svi0    ) )
//     lc.add( LTerm::times_x( svi0, 2 ) )
// 
//     lc.add( LTerms::times_1( (svi0, svi1)    ) )
//     lc.add( LTerms::times_x( (svi0, svi1), 3 ) )
// 
//     lc.add( LTerms::combine( ( (svi0, 2), (svi1, 3) ) )
//     lc.add( lci.into_forgetful_terms() )
// 
// //--------------------------------------------------------------
//     lc.add( svi )
//     lc.add_scaled(svi, 2)
// 
// 
//     lc.add( LTerm::new(svi).x(2) )
//     lc.add_n( (svi0, svi1).map(|x| (x, 2) ) )
// 
// 
//     lc.add( LTerm::scaled( svi, 2 ) )
//     lc.add( LTerm::unscaled( x )
// 
// 
//     lc.add( LTerm::scaled(   svi    ) )
//     lc.add( LTerm::unscaled( svi, x ) )
//    
//     lc.add( LTerms::scaled(   ( svi0, svi1, svi2 )    ) )
//     lc.add( LTerms::unscaled( ( svi0, svi1, svi2 ), y ) )
//   
//     lc.add( LTerms::combination( ( (svi0, t0), (svi1, t1), (svi2, t2) ) )
//     lc.add( LTerms::lci( lci0 ) )
// 
//     lc.add( LTerm::scaled(svi) )
//     lc.add( svi )
// 
//     lc.add_mn( ..) // FOR THESE COMPLEX LARGE SCALE OPERATIONS -- JUST EXPLAIN HOW TO DO A BULK
//     INSERT AND HEAPIFY
// 
// 
//     X.add( LTerm::Scale(   (svi0,), y )
//     X.add( Lterm::Scale(   (svi0, svi1, svi2), y )
//     X.add( LTerm::Unscaled( (svi0,) )
//     X.add( LTerm::Unscaled( (svi0, svi1, svi2) ) )
//     X.add( Lterm::Combination( (svi0, t0), (svi1, t1), (svi2, t2) )
// 
// 
//     !!!!! NOPE -- JUST .merge(y, C).simplify();
// 
//     heter_add( svi0, svi1, C )
//     heter_add_simplify( svi0, svi1, C, R)
// 
//     
// 
// 
//     // Generally speaking, better to build/add in bulk.
//     let agg = simple_lc::sum( (svi0, svi1, svi2), R, C)
//                 .add_vector( svi3    );
//                 .add_scaled( svi3, 2 );
//                 .add_combination( svi4, Coeff::Scalars((3, 2, 1)) )
//                 .add_combination( svi4, Coeff::Uniform(3) )
//                 .add_combination( svi4, Coeff::Unit )
//                 .add_combination( svi4, Coeff::Raw( Some(1), None, Some(2) )
//                 .add_combination( ( (svi5, Some(3)), (svi6, Some(2)), (svi7, None) )
// 
//     let agg = 
// 
// 
//                 .add_vector(      svi3, Coeff::unit )
//                 .add_vector(      svi3, Coeff::scalar(3))
//     let agg = svi2.bulk_insert( (svi0, svi1) ). 
// 
// (svi,)kk
//                 .hit_merge( (svi2,), C )
//                 .simplify(&R)
//                 .merge(  hit_merge(svii, C).simplify(&R)  )
// 
//     svi
//         .gather_terms(R.clone())
//         .drop(R.zero())
//         .hit_merge( 
//     
// 
//     svi + svi2 + sum(svi_iter)
//     svi 
//         .add( svi2, &R)
//         .gather_terms(R.clone())
//         .drop_zeros(R.clone())
//         .add( 
//             hit_merge(svi_iter)
//             .gather_terms(R.clone())
//             );
//         .gather()
//         .drop_zeros(&R)
//    
// 
//     spi
//         .into_svi(R)
//         .gather_terms()
//         .
// 
//     kk
// 
//     svir.add( svi );
//     svir.add_simple( svi );
// 
// 
//     vec.add( vec2, ring ).drop_zero(ring).
// 
//     add_1_simple
//     add_k_simple
//     
//     vec.add_1_simple( vec2, ring )
//     vec.add_k_simple(
// 
//     simple_add
//     simple_sum_several
// 
//     simple_add_k_other
//     simple_add_k_same
// 
//     
//     // UNDER CONSTRUCTION
//    //  fn add < Ring, Svi2> (mut self, ring: Ring, svi2: Svi2) -> 
// 
// 
//    //  svi.add( svi2, ring ).gather(ring)
//    //  svi.add_several( svi_set, ring ).gather(ring)
// 



//  ---------------------------------------------------------------------------
//  TEST
//  ---------------------------------------------------------------------------


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::rings::ring_native::NativeDivisionRing;
    use crate::vector_entries::vector_entries::KeyValItem;

    //  THIS TEST IS REDUNDANT -- OK TO DELETE
    // #[test]
    // pub fn test() {

    //     // Define three copies of the coefficient ring.
    //     let ring_a = NativeDivisionRing::<f64>::new();
    //     let ring_b = ring_a.clone();
    //     let ring_c = ring_a.clone();        
        
    //     // Define a sequence of vector entries.
    //     let entry_data = vec![ (1, 1.), (2, 2.), (3, 3.), (3, 3.), (4, 0.) ];
    
    //     // Define three copies of a sparse vector iterator (i.e., an iterator that runs over entries)       
    //     let vec_a = entry_data.iter().cloned();
    //     let vec_b = vec_a.clone();
    //     let vec_c = vec_a.clone();

    //     // Scale the iterator by 2.  
    //     let scaled      = vec_a.scale( ring_a , 2. );
    
    //     // Drop zero entries.        
    //     let dropped     = vec_b.drop_zeros( ring_b );
        
    //     // Gather entries with equal indices
    //     let gathered    = vec_c.peekable().gather( ring_c );
        
    //     // Print the results
    //     println!("Original:\n{:?}", entry_data.clone());     
    //     println!("Scaled:\n{:?}", &scaled);    
    //     println!("Dropped zeros:\n{:?}", dropped.clone());    
    //     println!("Gathered:\n{:?}", gathered.clone());                   
        
    // }   

    #[test]
    pub fn test_2() {

        // Define the coefficient ring
        let ring = NativeDivisionRing::<f64>::new();        

        // Define a sequence of vector entries.
        let entry_data = vec![ (1, 1.), (2, 2.), (3, 3.), (3, 3.), (4, 0.) ];
    
        // Define a sparse vector iterator (i.e., an iterator that runs over entries)       
        let sparse_vec = entry_data.iter().cloned();
    
        // SCALE THE VECTOR BY 2.
        let scaled : Vec<_> = sparse_vec
                                .clone() // this makes a copy of the iterator, so the original stays unchanged
                                .scale( ring.clone(), 2. )
                                .collect(); // this collects the entries of the iterator into a standard Rust vector
        assert_eq!( scaled, vec![ (1, 2.), (2, 4.), (3, 6.), (3, 6.), (4, 0.) ]);
    
        // DROP ZERO ENTRIES
        let dropped : Vec<_> = sparse_vec
                                .clone() // this makes a copy of the iterator, so the original stays unchanged
                                .drop_zeros( ring.clone() )
                                .collect(); // this collects the entries of the iterator into a standard Rust vector
        
        assert_eq!( dropped, vec![ (1, 1.), (2, 2.), (3, 3.), (3, 3.) ]);
    
        // MERGE CONSECUTIVE ENTRIES THAT SHARE THE SAME INDEX
        let gathered : Vec<_> = sparse_vec
                                .clone() // this makes a copy of the iterator, so the original stays unchanged
                                .peekable() // this puts the iterator in a slightly different form, which is compatible with gather
                                .gather( ring.clone() )
                                .collect(); // this collects the entries of the iterator into a standard Rust vector
        assert_eq!( gathered, vec![ (1, 1.), (2, 2.), (3, 6.), (4, 0.) ]);        
        
    }       

}


