
use itertools::Itertools;
use itertools::{Dedup, KMerge};
use crate::utilities::cell_complexes::simplices_unweighted::simplex::{Simplex};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;


//  ===========================================================================
//  ===========================================================================
//  SIMPLEX - AS - SORTED - VECTOR
//  ===========================================================================
//  ===========================================================================


//  ---------------------------------------------------------------------------
//  FACES FROM FACETS-OF-THE-COMPLEX (NEW)
//  ---------------------------------------------------------------------------


/// Assuming the complex facets have vertices sorted in ascending order, returns
/// an iterator that runs over `dim`-dimensional subsimplices in lexicographic 
/// order.
pub fn ordered_subsimplices_fixed_dim_iter< Vertex >(
    complex_facets: & Vec< Vec< Vertex >>, 
    dim: usize 
) 
    -> 
    Dedup< KMerge<   itertools::Combinations<std::iter::Cloned<std::slice::Iter<'_, Vertex>>  > > >
    where Vertex: Ord + Clone
{
    complex_facets
        .iter()
        .map( |x| x.iter().cloned().combinations( dim + 1 )  )
        .kmerge()
        .dedup()
}


pub fn  ordered_subsimplices_up_thru_dim_vec< Vertex >( 
    complex_facets: & Vec< Vec< Vertex >>, 
    max_dim: usize 
) 
-> 
Vec< Vec< Vec< Vertex >>> 
where Vertex: Ord + Clone
{
    let mut seq             =   Vec::with_capacity( max_dim );
    for dim in 0 .. max_dim + 1  {
        let vec: Vec<_>     =   ordered_subsimplices_fixed_dim_iter(
                                    complex_facets,
                                    dim
                                )
                                .collect();
        seq.push( vec );
    }
    seq
}


pub fn  ordered_subsimplices_up_thru_dim_concatenated_vec< Vertex >( 
    complex_facets: & Vec< Vec< Vertex >>, 
    max_dim: usize 
) 
-> 
Vec< Vec< Vertex >>
    where Vertex: Ord + Clone
{
    let mut a = ordered_subsimplices_up_thru_dim_vec( complex_facets, max_dim );
    let mut b = Vec::new();
    for i in 0 .. a.len() {
        b.append( &mut a[ i ] );
    }
    b
}



//  ===========================================================================
//  ===========================================================================
//  SIMPLEX - AS - STRUCT
//  ===========================================================================
//  ===========================================================================



//  ---------------------------------------------------------------------------
//  FACES FROM FACETS-OF-THE-COMPLEX ( OLD )
//  ---------------------------------------------------------------------------

/// Given something that iterates over vectors (each of which represents a strictly 
/// ascending sequence of vertices), return a HashSet containing all nonempty subsequences.
pub fn  set_of_subsequences< IterFacet, Vertex >( facets: IterFacet ) -> HashSet< Vec< Vertex > > 
    where   IterFacet:      IntoIterator< Item = Vec< Vertex > >,
            Vertex:    Ord + Hash + Clone
{
    println!("THIS FUNCTION COULD PROBABLY BE MADE MUCH MORE EFFICIENT");    
    let mut faces       =   HashSet::new();
    for facet in facets {
        for seq_length in 1 .. facet.len() {
            for comb in facet.iter().cloned().combinations( seq_length ) {
                faces.insert( comb );
            }
        }
    }
    faces
}

/// Given something that iterates over vectors (each of which represents a strictly 
/// ascending sequence of vertices), return a vector V containing all nonempty ordered
/// subsequences; V is strictly ascending under the order that first compares length of 
/// a sequence, then compares equal-length sequences lexicographically.
/// 
//  NB: THE USE OF SIMPLICES RATHER THAN VECTORS IS IMPORTANT HERE, BECAUSE THE TWO STRUCTS HAVE
//      **DIFFERENT** TOTAL ORDERS
pub fn  ordered_sequence_of_faces< IterFacet, Vertex >( facets: IterFacet ) -> Vec< Simplex< Vertex > > 
    where   IterFacet:  IntoIterator< Item = Vec< Vertex > >,
            Vertex:     Ord + Hash + Clone
{
    println!("THIS FUNCTION COULD PROBABLY BE MADE MUCH MORE EFFICIENT");
    let mut faces   =   set_of_subsequences(facets);
    let mut faces   =   Vec::from_iter( faces.drain().map(|x| Simplex{vertices: x}) );
    faces.sort();
    faces
}   

//  ---------------------------------------------------------------------------
//  FACETS-OF-A-SIMPLEX
//  ---------------------------------------------------------------------------

/// Maintains an "inner state" that steps through the facets of a simplex in 
/// ascending lexicographic order; only returns `Some(())` or `None`.
/// 
/// # Examples
/// 
/// ```
/// use solar::utilities::cell_complexes::simplices_unweighted::simplex::{Simplex, FacetIteratorNoReturnAscending};

/// // Create the iterator
/// let mut facet_iterator_noreturn     =   FacetIteratorNoReturnAscending::new(
///                                             Simplex{ vertices: vec![0, 1, 2] },
///                                             None
///                                         );
///
/// // Test it                                                
/// let mut answers = vec![
///     FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 1, 2] }, facet: Simplex { vertices: vec![0, 1] }, deleted_vertex_index: Some(2) },
///     FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 1, 2] }, facet: Simplex { vertices: vec![0, 2] }, deleted_vertex_index: Some(1) },
///     FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 1, 2] }, facet: Simplex { vertices: vec![1, 2] }, deleted_vertex_index: Some(0) },
///     FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 1, 2] }, facet: Simplex { vertices: vec![]     }, deleted_vertex_index: None    },            
///     FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 1, 2] }, facet: Simplex { vertices: vec![0, 1] }, deleted_vertex_index: Some(2) },                                                                        
/// ];
///
/// for i in 0..5 {
///     facet_iterator_noreturn.next();
///     assert_eq!( &facet_iterator_noreturn, &answers[ i ] )    
/// }      
//
/// // Re-initialize with a new simplex
///
/// facet_iterator_noreturn.reinitialize_with_simplex( Simplex{ vertices: vec![0 ,3]} );
///
/// // Test again        
///
/// answers = vec![
///     FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 3] }, facet: Simplex { vertices: vec![0] }, deleted_vertex_index: Some(1) },
///     FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 3] }, facet: Simplex { vertices: vec![3] }, deleted_vertex_index: Some(0) },
///     FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 3] }, facet: Simplex { vertices: vec![]  }, deleted_vertex_index: None    },
///     FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 3] }, facet: Simplex { vertices: vec![0] }, deleted_vertex_index: Some(1) },                                                                                                      
/// ];    
///
/// for i in 0..4 {
///     facet_iterator_noreturn.next();
///     assert_eq!( &facet_iterator_noreturn, &answers[ i ] )    
/// }   
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct  FacetIteratorNoReturnAscending< Vertex >
{
    pub simplex: Simplex< Vertex > ,
    pub facet: Simplex< Vertex >,
    pub deleted_vertex_index: Option<usize>
}

impl < Vertex > FacetIteratorNoReturnAscending < Vertex > 
    where Vertex: Clone
{

    /// Initialize a no-return facet iterator.  
    /// 
    /// Its internal state does not represent a facet; rather, the internal state will
    /// represent the first facet after `next()` is called for the first time.
    pub fn new( simplex: Simplex< Vertex >, buffer: Option< Simplex<Vertex> > ) -> Self {
        let buff = 
            if let Some( vec ) = buffer { vec } 
            else { 
                Simplex{ 
                    vertices: Vec::with_capacity( simplex.dim() ) // = 1 less than num_vertices = NUM VERTICES OF A FACET
                }             
            };
        
        FacetIteratorNoReturnAscending {
            simplex: simplex,
            facet: buff,
            deleted_vertex_index: None
        }
    }

    pub fn reinitialize_with_simplex( &mut self, simplex: Simplex< Vertex > ) {
        // if necessary, expand the capacity of the facet vector
        if simplex.dim() > self.facet.vertices.capacity() { 
            self.facet.vertices.reserve_exact(
                simplex.dim() - self.facet.vertices.capacity()
            ) 
        }
        // replace the old simplex with the new
        self.simplex    =   simplex;
        // update the state to indicate that it does not represent a facet
        self.facet.vertices.clear();
        self.deleted_vertex_index = None;
    }
}


impl < Vertex >
    Iterator for 
    FacetIteratorNoReturnAscending < Vertex >     
    where Vertex : Clone
{
    type Item    =   ();

    fn next( &mut self ) -> Option<()> {

        if let Some( deleted_index ) = self.deleted_vertex_index {

            if deleted_index == 0 {
                // if we start from the facet obtained by deleting vertex 0, then the 
                // next state should **not** represent a facet
                self.deleted_vertex_index   =   None;
                self.facet.vertices.clear();
                return None
                
            } else {
                // if we start from the facet obtained by deleting vertex k > 0, then 
                // the next state should represent the facet obtained by deleting vertex k-1
                let next_deleted_index  =   deleted_index - 1;
                self.facet.vertices[ next_deleted_index ] = self.simplex.vertices[ deleted_index ].clone(); // replace the deleted index and remove the next one
                self.deleted_vertex_index = Some( next_deleted_index );
                return Some( () )
            }
        
        } else {

            self.facet.vertices.clear();
            for i in 0..self.simplex.dim() {   // dim = 1 less than num_vertices = INDEX OF LAST VERTEX IN SIMPLEX
                self.facet.vertices.push( self.simplex.vertices[ i ].clone() ) 
            }      
            // set deleted vertex equal to last
            self.deleted_vertex_index = Some( self.simplex.dim() );  // dim = 1 less than num_vertices = INDEX OF LAST VERTEX IN SIMPLEX
            return Some(())            
        }
    }
}




//  ===========================================================================
//  ===========================================================================
//  TESTS
//  ===========================================================================
//  ===========================================================================





#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test_ordered_subsimplices_up_thru_dim() {

        let complex_facets          =   vec![ vec![0, 1, 2] ];

        assert_eq!(         ordered_subsimplices_up_thru_dim_vec( & complex_facets, 2),
                            vec![
                                vec![   vec![0],     vec![1],    vec![2]         ],                                
                                vec![   vec![0,1],   vec![0,2],  vec![1,2]       ],
                                vec![   vec![0,1,2]                              ]
                            ]
        );

        assert_eq!(         ordered_subsimplices_up_thru_dim_concatenated_vec( & complex_facets, 2),
                            vec![
                                        vec![0],     vec![1],    vec![2],                                         
                                        vec![0,1],   vec![0,2],  vec![1,2],       
                                        vec![0,1,2]                              
                            ]
        ) ;       


    }
}