use crate::matrices::matrix_oracle::{   OracleMajor,
    OracleMajorAscend,
    OracleMajorDescend,
    OracleMinor, 
    OracleMinorAscend,
    OracleMinorDescend,
    WhichMajor,
    MajorDimension};
use crate::vector_entries::vector_entries::KeyValGet;
use crate::rings::ring::{Semiring, Ring, DivisionRing};



//  ---------------------------------------------------

/// Returns a U-match factorization.
/// 
/// For details on this factorization, see [this preprint](https://arxiv.org/pdf/2108.08831.pdf).
pub fn  umatch_factorization< MatrixOracle, IndexItr >
        ( 
            matrix: &MatrixOracle, 
            reduction_indices: IndexItr // the indices to reduce the matrix
            coeff_ring: RingOperations
        ) 
        ->  
            UmatchR< MatrixOracle > 
        where   MatrixOracle: < 'a, KeyMaj, KeyMin >,
                IndexItr: Iterator< KeyMaj >
                RingOperations: Semiring, Ring, DivisionRing
{

}


//  ---------------------------------------------------
//
//  FOUR STRUCTS (ONE FOR EACH OF THE FOUR MATRICES R, Ri, C, Ci
//  DEFINED IN THE UMATCH PAPER).
//  
//  !!!! NOTE:  FOR NOW, LET'S MAKE EVERY MATRIX HAVE THE SAME MAJOR
//              DIMENSION THAT THE ORIGINAL MATRIX M HAS.


pub struct UmatchR< MatrixOracle > 
    where   MatrixOracle: OracleMajorAscend, OracleMajorDescend
{
    umatch:     Umatch< MatrixOracle > ,
    maj_dim:    MajorDimension,
}

pub struct UmatchRi< MatrixOracle > 
    where   MatrixOracle: OracleMajorAscend, OracleMajorDescend
{
    umatch:     Umatch< MatrixOracle >,
    maj_dim:    MajorDimension,    
}

pub struct UmatchC< MatrixOracle > 
    where   MatrixOracle: OracleMajorAscend, OracleMajorDescend
{
    umatch:     Umatch< MatrixOracle >,
    maj_dim:    MajorDimension,    
}

pub struct UmatchCi< MatrixOracle > 
    where   MatrixOracle: OracleMajorAscend, OracleMajorDescend
{
    umatch:     Umatch< MatrixOracle >,
    maj_dim:    MajorDimension,    
}

//  ---------------------------------------------------

/// The U-match struct.
/// 
/// This struct contains a reference to a matrix oracle, `M`.
/// It also contains all the information needed to recover 
/// the matrices involved in a proper U-match factorization 
/// of `M`.
pub struct Umatch< MatrixOracle > 
    where   MatrixOracle: OracleMajorAscend, OracleMajorDescend
{
    M:          &'a MatrixOracle,
    Rip:        CSM, // the square invertible submatrix of Ri indexed by pivots
    indexing:   Indexing

impl Umatch {

    /// Returns the matrix $R$.
    fn R( &self ) -> UmatchR< MatrixOracle >    

    /// Returns the matrix $R^{-1}$.
    fn Ri( &self ) -> UmatchRi< MatrixOracle >

    /// Returns the matrix $C$.
    fn C( &self ) -> UmatchR< MatrixOracle >    

    /// Returns the matrix $C^{-1}$.
    fn Ci( &self ) -> UmatchRi< MatrixOracle >

    /// Returns a clone of the object that stores information about
    /// pivot pairs.
    fn pivot_indices( &self ) -> Indexing
}