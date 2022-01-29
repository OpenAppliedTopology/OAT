
use crate::matrices::matrix_oracle::{   OracleMajor,
                                        OracleMajorAscend,
                                        OracleMajorDescend,
                                        OracleMinor, 
                                        OracleMinorAscend,
                                        OracleMinorDescend,
                                        WhichMajor,
                                        MajorDimension};
use crate::vector_entries::vector_entries::KeyValGet;
use std::marker::PhantomData;
use std::iter::{Rev, Cloned};


/// A vector of vectors, representing a sparse matrix.  
/// 
/// Each of the internal vectors should have entries sorted in asecneding order of index.
/// 
/// # Examples
/// 
/// ```
/// use solar::matrices::implementors::vec_of_vec::*;
/// use solar::matrices::matrix_oracle::*;
/// use std::marker::PhantomData;
/// 
/// // Streamlined method to create a row-major vec-of-vec matrix.
/// let matrix  =   VecOfVec::new(
///                     MajorDimension::Row,
///                     vec![ vec![(1,1.)], vec![], vec![(2,2.)]  ],
///                 );
/// 
/// // Naive method to create a row-major vec-of-vec matrix (note we have to use "PhantomData").
/// let matrix  =   VecOfVec {
///                     major_dimension: MajorDimension::Row,
///                     vec_of_vec: vec![ vec![(1,1.)], vec![], vec![(2,2.)]  ],
///                     phantom: PhantomData
///                 };
/// 
/// 
/// ```
pub struct VecOfVec

    < 'a, IndexCoeffPair >

    where   IndexCoeffPair:    KeyValGet,
            Self:           'a

{
    pub major_dimension: MajorDimension, 
    pub vec_of_vec: Vec< Vec< IndexCoeffPair > >,
    pub phantom: PhantomData<&'a IndexCoeffPair >
}


impl    < 'a, IndexCoeffPair >
        VecOfVec 
        < 'a, IndexCoeffPair > 
        
        where   IndexCoeffPair:    KeyValGet        

{
    // Make new (empty) VecOfVec. 
    pub fn new( major_dimension: MajorDimension, vecvec: Vec<Vec<IndexCoeffPair>> ) -> Self  
    {
        VecOfVec{   major_dimension: major_dimension,
                    vec_of_vec: vecvec,                    
                    phantom: PhantomData 
                }
    }
}


impl < 'a, IndexCoeffPair > 
    
    OracleMajor
    <   
        'a,
        usize, 
        < IndexCoeffPair as KeyValGet >::Key, 
        < IndexCoeffPair as KeyValGet >::Val, 
    > 
    
    for 
    
    VecOfVec < 'a, IndexCoeffPair > 

    where   IndexCoeffPair:    KeyValGet + Clone + 'a,
            Self: 'a
{
    type PairMajor = IndexCoeffPair;
    type ViewMajor = Cloned<std::slice::Iter<'a, IndexCoeffPair>>; 
        
    fn view_major<'b: 'a>( &'b self, index: usize ) -> Self::ViewMajor {
        return self.vec_of_vec[index].iter().cloned()
    } 
}

impl < 'a, IndexCoeffPair > 
    
    OracleMajorAscend
    <   
        'a,
        usize, 
        < IndexCoeffPair as KeyValGet >::Key, 
        < IndexCoeffPair as KeyValGet >::Val, 
    > 
    
    for 
    
    VecOfVec < 'a, IndexCoeffPair > 

    where   IndexCoeffPair:    KeyValGet + Clone + 'a,
            Self: 'a
{
    type PairMajorAscend = IndexCoeffPair;
    type ViewMajorAscend = Cloned<std::slice::Iter<'a, IndexCoeffPair>>; 
        
    /// Assumes that entries in each vector are sorted in ascending order.
    fn view_major_ascend<'b: 'a>( &'b self, index: usize ) -> Self::ViewMajorAscend {
        return self.view_major( index )
    } 
}

impl < 'a, IndexCoeffPair > 
    
    OracleMajorDescend
    <   
        'a,
        usize, 
        < IndexCoeffPair as KeyValGet >::Key, 
        < IndexCoeffPair as KeyValGet >::Val, 
    > 
    
    for 
    
    VecOfVec < 'a, IndexCoeffPair > 

    where   IndexCoeffPair:    KeyValGet + Clone + 'a,
            Self: 'a
{
    type PairMajorDescend = IndexCoeffPair;
    type ViewMajorDescend = Cloned<Rev<std::slice::Iter<'a, IndexCoeffPair>>>; 
        
    /// Assumes that entries in each vector are sorted in ascending order.    
    fn view_major_descend<'b: 'a>( &'b self, index: usize ) -> Self::ViewMajorDescend {
        return self.vec_of_vec[index].iter().rev().cloned()
    } 
}






#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test_vec_of_vec_construction() {
        

        let matrix  =   VecOfVec {
                            major_dimension: MajorDimension::Row,
                            vec_of_vec: vec![ vec![(1,1.)], vec![], vec![(2,2.)]  ],
                            phantom: PhantomData
                        };
                 

    }

}

