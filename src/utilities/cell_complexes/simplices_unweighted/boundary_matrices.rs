
use crate::utilities::sequences_and_ordinals::{BiMapSequential};
use crate::utilities::ring::{MinusOneToPower};
use crate::rings::ring::{Ring, Semiring};
use crate::utilities::cell_complexes::simplices_unweighted::facets::{ordered_subsimplices_up_thru_dim_concatenated_vec};
use crate::utilities::cell_complexes::simplices_unweighted::simplex::{Simplex, FacetIteratorNoReturnAscending};
use itertools::Itertools;
use std::hash::Hash;
use std::fmt::Debug;


//  ===========================================================================
//  ===========================================================================
//  SIMPLEX - AS - SORTED - VECTOR
//  ===========================================================================
//  ===========================================================================


//  ---------------------------------------------------------------------------
//  SIMPLEX BIMAP TO BOUNDARY
//  ---------------------------------------------------------------------------


pub fn  boundary_matrix_from_complex_facets< Vertex, RingOp, RingElt >( 
            simplex_bimap:  & BiMapSequential< Vec < Vertex > >,
            ring:           RingOp
        ) 
        ->
        Vec< Vec < (usize, RingElt) >>

        where   Vertex:    Ord + Hash + Clone + Debug,      
                RingOp:     Semiring< RingElt > + Ring< RingElt >,
{
    if simplex_bimap.ord_to_val.is_empty() { return vec![] }

    let mut boundary            =   Vec::with_capacity( simplex_bimap.ord_to_val.len() );  
    
    let mut simplex_dim         =   0;
    let mut simplex_num_verts   =   0;

    for simplex in simplex_bimap.ord_to_val.iter().cloned() {

        simplex_num_verts       =   simplex.len();
        simplex_dim             =   simplex_num_verts - 1;

        // no need to calculate boundaries of dim-0 cells
        if simplex_dim == 0 {
            boundary.push( Vec::with_capacity(0) );
            continue;
        }  

        let mut vec             =   Vec::with_capacity( simplex_num_verts );    // num_vertices = NUMBER OF FACETS

        for (facet_count, facet)  in simplex.iter().cloned().combinations( simplex_dim ).enumerate() {
            vec.push( 
                (
                    simplex_bimap.ord( &facet ).unwrap(),
                    ring.minus_one_to_power( simplex_dim - facet_count )
                ) 
            )            
        }
        boundary.push( vec );
    }

    boundary

}



//  ===========================================================================
//  ===========================================================================
//  SIMPLEX - AS - STRUCT
//  ===========================================================================
//  ===========================================================================






pub fn  boundary_matrix_from_complex_facets_simplexform< Vertex, RingOp, RingElt >( 
            simplex_bimap:  BiMapSequential< Simplex< Vertex > >,
            ring:           RingOp
        ) 
        ->
        Vec< Vec < (usize, RingElt) >>

        where   Vertex:    Ord + Hash + Clone + Debug,      
                RingOp:     Semiring< RingElt > + Ring< RingElt >,
{
    if simplex_bimap.ord_to_val.is_empty() { return vec![] }

    let mut boundary            =   Vec::with_capacity( simplex_bimap.ord_to_val.len() );  
    
    let mut state_iter          =   FacetIteratorNoReturnAscending{
                                        simplex: Simplex{ vertices: vec![] },
                                        facet: Simplex{ vertices: vec![] },
                                        deleted_vertex_index: None
                                    };

    let mut global_int_index    =   0;
    let mut simplex_dim         =   0;
    let mut simplex_num_verts   =   0;

    for simplex in simplex_bimap.ord_to_val.iter().cloned() {

        simplex_dim             =   simplex.dim();
        simplex_num_verts       =   simplex.num_vertices();
        state_iter.reinitialize_with_simplex( simplex );

        let mut vec             =   Vec::with_capacity( simplex_num_verts );    // num_vertices = NUMBER OF FACETS
        
        for i in 0 .. simplex_num_verts {
            state_iter.next();
            
            println!("{:?}", &state_iter);
            println!("{:?}", &simplex_bimap);            

            global_int_index    =   simplex_bimap.ord( &state_iter.facet ).unwrap();
            vec.push( 
                (
                    global_int_index.clone(),
                    ring.minus_one_to_power( simplex_dim - i )
                ) 
            )
        }
        boundary.push( vec );
    }

    boundary

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
    fn test_bimap_to_boundary () {

        let ring                    =   crate::rings::ring_native::NativeDivisionRing::< f64 >::new();
        let complex_facets          =   vec![ vec![0,1,2] ];


        let bimap_sequential        =   BiMapSequential::from_vec(
                                            ordered_subsimplices_up_thru_dim_concatenated_vec( & complex_facets, 2 )
                                        );  

        let boundary                =   boundary_matrix_from_complex_facets( & bimap_sequential, ring );

        assert_eq!(     &   boundary,
                        &   vec![
                                    vec![],
                                    vec![],
                                    vec![],
                                    vec![(0, -1.0), (1, 1.0)],
                                    vec![(0, -1.0), (2, 1.0)],
                                    vec![(1, -1.0), (2, 1.0)],
                                    vec![(3, 1.0), (4, -1.0), (5, 1.0)]
                            ]
        )
    }    


}    