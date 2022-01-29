
use crate::utilities::indexing_and_bijection::{compose_f_after_g, sort_perm, inverse_perm};
use crate::utilities::cell_complexes::simplices_unweighted::facets::ordered_subsimplices_up_thru_dim_concatenated_vec;
use std::cmp::Ordering;
use std::iter::FromIterator;


//  ===========================================================================
//  ===========================================================================
//  SIMPLEX - AS - SORTED VECTOR
//  ===========================================================================
//  ===========================================================================

//  ---------------------------------------------------------------------------
//  PERMUTATION ON SIMPLICES INDUCED BY PERMUTATION ON VERTICES
//  ---------------------------------------------------------------------------


/// Given an vector f = [f0, .., fn] representing a function of form 
/// f: old_vertex_number -> new_vertex_number, obtain the vector 
/// g: old_simplex_number -> new_simplex_number; here "number" refers to 
/// lexicographic order. 
/// 
/// This function does not assume that the `simplex_sequence` has lexicogrphic 
/// order, but it **does** assume that the new simplex sequence has lexicographic
/// order.
pub fn  simplex_perm_o2n_from_vertex_perm_o2n( 
    simplex_sequence:           &   Vec< Vec< usize >>,
    vertex_perm_old_to_new:     &   Vec< usize >
    ) 
    ->
    Vec< usize >
{
    // Create vector of new simplices
    let mut new_simplex_sequence =  Vec::from_iter(
                                        simplex_sequence
                                            .iter()
                                            .cloned()
                                            .map(
                                                |x|
                                                Simplex{ 
                                                    vertices:  compose_f_after_g( &vertex_perm_old_to_new, &x )
                                                }
                                            )
                                    );

    // We must remember to sort the new vertices                                    
    for simplex in new_simplex_sequence.iter_mut() { simplex.vertices.sort()}

    // Obtain the sort permutation
    sort_perm( &new_simplex_sequence )
}


//  ===========================================================================
//  ===========================================================================
//  SIMPLEX - AS - STRUCT
//  ===========================================================================
//  ===========================================================================


//  ---------------------------------------------------------------------------
//  COMBINATORIAL SIMPLEX (UNWEIGHTED) -- DEFINITION
//  ---------------------------------------------------------------------------


/// An unweighted simplex; the vertices should sorted in ascending order.
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Simplex< Vertex > 
{ 
    pub vertices: Vec< Vertex >     //  vertices should be sorted in ascending order
} 

impl    < Vertex > 
        Simplex
        < Vertex >   
        {
    
    pub fn num_vertices( &self ) -> usize { self.vertices.len() }
    pub fn dim( &self ) -> usize { self.vertices.len() - 1 }
}        


impl    < Vertex >           
        PartialOrd for Simplex
        < Vertex >

    where   Vertex: Ord     {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl    < Vertex >   
        Ord for Simplex
        < Vertex >

    where Vertex: Ord   {

    fn cmp(&self, other: &Self) -> Ordering {

        // next compare simplex dimensions
        let comp = self.num_vertices().cmp( & other.vertices.len() );
        if comp != Ordering::Equal { return comp }

        // finally, compare simplices lexicographically
        return self.vertices.cmp( & other.vertices )
    }
}

impl    < Vertex >   
        IntoIterator for Simplex
        < Vertex >      {

    type Item = Vertex;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter { self.vertices.into_iter() }
}



//  ---------------------------------------------------------------------------
//  FACETS-OF-A-SIMPLEX: ASCENDING ITERATOR WITH **NO** RETURN VALUE
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







#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;



    #[test]
    fn test_ascending_facet_iterator_no_return()
    {

        // Create the iterator
        let mut facet_iterator_noreturn     =   FacetIteratorNoReturnAscending::new(
                                                    Simplex{ vertices: vec![0, 1, 2] },
                                                    None
                                                );

        // Test it                                                
        let mut answers = vec![
            FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 1, 2] }, facet: Simplex { vertices: vec![0, 1] }, deleted_vertex_index: Some(2) },
            FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 1, 2] }, facet: Simplex { vertices: vec![0, 2] }, deleted_vertex_index: Some(1) },
            FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 1, 2] }, facet: Simplex { vertices: vec![1, 2] }, deleted_vertex_index: Some(0) },
            FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 1, 2] }, facet: Simplex { vertices: vec![]     }, deleted_vertex_index: None    },            
            FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 1, 2] }, facet: Simplex { vertices: vec![0, 1] }, deleted_vertex_index: Some(2) },                                                                        
        ];

        for i in 0..5 {
            facet_iterator_noreturn.next();
            assert_eq!( &facet_iterator_noreturn, &answers[ i ] )    
        }      

        // Re-initialize with a new simplex

        facet_iterator_noreturn.reinitialize_with_simplex( Simplex{ vertices: vec![0 ,3]} );

        // Test again        

        answers = vec![
            FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 3] }, facet: Simplex { vertices: vec![0] }, deleted_vertex_index: Some(1) },
            FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 3] }, facet: Simplex { vertices: vec![3] }, deleted_vertex_index: Some(0) },
            FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 3] }, facet: Simplex { vertices: vec![]  }, deleted_vertex_index: None    },
            FacetIteratorNoReturnAscending { simplex: Simplex { vertices: vec![0, 3] }, facet: Simplex { vertices: vec![0] }, deleted_vertex_index: Some(1) },                                                                                                      
        ];    

        for i in 0..4 {
            facet_iterator_noreturn.next();
            assert_eq!( &facet_iterator_noreturn, &answers[ i ] )    
        }     
                
    }       





    #[test]
    fn test_simplex_perm_o2n_from_vertex_perm_o2n() {

        // sequence_old:          [[0], [1], [2], [3], [0, 1], [0, 2], [0, 3], [1, 2]]
        // sequence_old_permuted: [[0], [1], [3], [2], [0, 1], [0, 3], [0, 2], [1, 2]]
        // new_sequence:          [[0], [1], [2], [3], [0, 1], [0, 2], [0, 3], [1, 3]]
        // permutation: simplex old -> new 
        //                        [ 0,   1,   3,   2,   4,      6,      5,      7]


        let complex_facets          =   vec![  vec![0,1,2], vec![0, 3] ];
        let simplex_sequence_old    =   ordered_subsimplices_up_thru_dim_concatenated_vec( &complex_facets, 1);   
        let perm_v_o2n              =   vec![0, 1, 3, 2];

        let mut simplex_sequence_new =  Vec::from_iter(
            simplex_sequence_old
                .iter()
                .cloned()
                .map(
                    |x|
                    Simplex{ 
                        vertices:  compose_f_after_g( &perm_v_o2n, &x )
                    }
                )
        );        

        // We must remember to sort the new vertices                                    
        for simplex in simplex_sequence_new.iter_mut() { simplex.vertices.sort() }        

        // perm: simplex OLD -> NEW
        let perm_s_o2n              =   simplex_perm_o2n_from_vertex_perm_o2n( &simplex_sequence_old, &perm_v_o2n );
        // perm: simplex NEW -> OLD
        let perm_s_n2o               =   inverse_perm( &perm_s_o2n );

        let mut simplex_sequence_permuted    =   compose_f_after_g( &simplex_sequence_old, &perm_s_n2o );

        let mut simplex_sequence_permuted_vertex_translated     =   simplex_sequence_permuted.clone();
        for i in 0..simplex_sequence_permuted_vertex_translated.len() { simplex_sequence_permuted_vertex_translated[i] = compose_f_after_g( & perm_v_o2n, & simplex_sequence_permuted[i]) };
        for i in 0..simplex_sequence_permuted_vertex_translated.len() { simplex_sequence_permuted_vertex_translated[i].sort() };        
        

        println!("sequence_old:          {:?}",     & simplex_sequence_old );
        println!("sequence_old_permuted: {:?}",     & simplex_sequence_permuted );        
        println!("new_sequence:          {:?}",     & simplex_sequence_permuted_vertex_translated );     
        println!("permutation: simplex old -> new {:?}", & perm_s_o2n);           

    }


}    
