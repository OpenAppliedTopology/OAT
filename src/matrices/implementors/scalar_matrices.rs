
use std::marker::PhantomData;
use crate::matrices::matrix_oracle::{   OracleMajor,
                                        OracleMajorAscend,
                                        OracleMajorDescend,
                                        OracleMinor, 
                                        OracleMinorAscend,
                                        OracleMinorDescend,
                                        WhichMajor,
                                        MajorDimension};
use std::iter;



//  ---------------------------------------------------------------------------
//  SCALAR MATRICES (INDEXED ONLY BY INTEGERS; SEE BELOW FOR A GENERALIZATION)
//  ---------------------------------------------------------------------------


//  STRUCT
//  ------

/// Represents a scalar matrix indexed by integers (see below for a more general struct).
///
/// Concretely, for any `index` (whether major or minor), each major/minor view
/// of the matrix returns an interator of form `Once< (index, scalar) >` out, 
/// where `scalar` is the given scalar. 
///
/// # Examples
///
/// ```
/// use solar::matrices::implementors::scalar_matrices::ScalarMatrixOracleUsize;
/// use solar::matrices::matrix_oracle::{OracleMajor, MajorDimension};
///
/// let a = < ScalarMatrixOracleUsize::< f64 > >::new(
///                                                 2.,
///                                                 MajorDimension::Row,
///                                                 );
/// let mut b : <ScalarMatrixOracleUsize<f64> as OracleMajor<usize, usize, f64>>::ViewMajor  = a.view_major( 2 ); 
/// // let mut c : ScalarMatrixOracleUsize<f64>::MajorSlice  = a.view_major( 2 ); // THIS THROWS AN ERROR ASKING FOR FULLY QUALIFIED SYNTAX
/// let mut d = <ScalarMatrixOracleUsize<f64> as OracleMajor<usize, usize, f64>>::view_major(
/// &a, 2 );
/// ```
pub struct ScalarMatrixOracleUsize < Val >
{
    scalar: Val,
    major_dimension: MajorDimension,
}

impl    < Val >
        ScalarMatrixOracleUsize
        < Val > 
{
    /// Create new scalar matrix.
    pub fn new( scalar: Val, major_dimension: MajorDimension ) -> Self  
    {
        ScalarMatrixOracleUsize { scalar: scalar,
                             major_dimension: major_dimension,
                            }
    }
}


//  ---------------------
//  TRAIT IMPLEMENTATIONS
//  ---------------------


//  MAJOR DIMENSION
//  ---------------------------------------------------------------------------


//  WHICH MAJOR 
//  

impl     < Val >
        WhichMajor 
        for 
        ScalarMatrixOracleUsize < Val > 
{ fn major_dimension( &self ) -> MajorDimension { self.major_dimension.clone() } }


//  MAJORS
//  ---------------------------------------------------------------------------


//  OracleMajor
//  
impl     < 'a, Val >
        OracleMajor < 'a, usize, usize, Val >
        for 
        ScalarMatrixOracleUsize < Val > 
        
        where   Val: 'a + Clone, // hard to drop this requirement (tuples give move errors if no clone) 
{
    type PairMajor =   (usize, Val)  ;
    type ViewMajor =   iter::Once< Self::PairMajor >;

    fn view_major<'b: 'a>( &'b self, index: usize ) -> Self::ViewMajor 
    { 
        iter::once( ( index, self.scalar.clone() ) )
    }
}

//  OracleMajorAscend
//  
impl     < 'a, Val >
        OracleMajorAscend < 'a, usize, usize, Val >
        for 
        ScalarMatrixOracleUsize < Val > 
        
        where   Val: Clone, // hard to drop this requirement (tuples give move errors if no clone) 
{
    type PairMajorAscend =   (usize, Val)  ;
    type ViewMajorAscend =   iter::Once< Self::PairMajorAscend >;

    fn view_major_ascend<'b: 'a>( &'b self, index: usize ) -> Self::ViewMajorAscend
    { 
        iter::once( ( index, self.scalar.clone() ) )
    }
}


//  OracleMajorDescend
//  
impl     < 'a, Val >
        OracleMajorDescend < 'a, usize, usize, Val >
        for 
        ScalarMatrixOracleUsize < Val > 
        
        where   Val: Clone, // hard to drop this requirement (tuples give move errors if no clone) 
{
    type PairMajorDescend =   (usize, Val)  ;
    type ViewMajorDescend =   iter::Once< Self::PairMajorDescend >;

    fn view_major_descend<'b: 'a>( &'b self, index: usize ) -> Self::ViewMajorDescend
    { 
        iter::once( ( index, self.scalar.clone() ) )
    }
}


//  MINORS
//  ---------------------------------------------------------------------------


//  OracleMinor
//  
impl     < 'a, Val >
        OracleMinor < 'a, usize, usize, Val >
        for 
        ScalarMatrixOracleUsize < Val > 
        
        where   Val: Clone, // hard to drop this requirement (tuples give move errors if no clone) 
{
    type PairMinor =   (usize, Val)  ;
    type ViewMinor =   iter::Once< Self::PairMinor >;

    fn view_minor<'b: 'a>( &'b self, index: usize ) -> Self::ViewMinor 
    { 
        iter::once( ( index, self.scalar.clone() ) )
    }
}

//  OracleMinorAscend
//  
impl     < 'a, Val >
        OracleMinorAscend < 'a, usize, usize, Val >
        for 
        ScalarMatrixOracleUsize < Val > 
        
        where   Val: Clone, // hard to drop this requirement (tuples give move errors if no clone) 
{
    type PairMinorAscend =   (usize, Val)  ;
    type ViewMinorAscend =   iter::Once< Self::PairMinorAscend >;

    fn view_minor_ascend<'b: 'a>( &'b self, index: usize ) -> Self::ViewMinorAscend
    { 
        iter::once( ( index, self.scalar.clone() ) )
    }
}


//  OracleMinorDescend
//  
impl     < 'a, Val >
        OracleMinorDescend < 'a, usize, usize, Val >
        for 
        ScalarMatrixOracleUsize < Val > 
        
        where   Val: Clone, // hard to drop this requirement (tuples give move errors if no clone) 
{
    type PairMinorDescend =   (usize, Val)  ;
    type ViewMinorDescend =   iter::Once< Self::PairMinorDescend >;

    fn view_minor_descend<'b: 'a>( &'b self, index: usize ) -> Self::ViewMinorDescend
    { 
        iter::once( ( index, self.scalar.clone() ) )
    }
}










//  ---------------------------------------------------------------------------
//  SCALAR MATRICES (INDICES CAN BE OF ANY TYPE)
//  ---------------------------------------------------------------------------


//  STRUCT
//  ------

/// Represents a scalar matrix.
///
/// Concretely, for any `index` (whether major or minor), each major/minor view
/// of the matrix returns an interator of form `Once< (index, scalar) >` out, 
/// where `scalar` is the given scalar. 
///
/// # Examples
///
/// ```
/// use solar::matrices::implementors::scalar_matrices::ScalarMatrixOracle;
/// use solar::matrices::matrix_oracle::{OracleMajor, MajorDimension};
///
/// let a = < ScalarMatrixOracle::< usize, usize > >::new(
///                                                 2,
///                                                 MajorDimension::Row,
///                                                 );
/// let mut b : <ScalarMatrixOracle<usize,usize> as OracleMajor<usize, usize, usize>>::ViewMajor  = a.view_major( 2 ); 
/// // let mut c : ScalarMatrixOracle<usize,usize>::MajorSlice  = a.view_major( 2 ); // THIS THROWS AN ERROR ASKING FOR FULLY QUALIFIED SYNTAX
/// let mut d = <ScalarMatrixOracle<usize,usize> as OracleMajor<usize, usize, usize>>::view_major(
/// &a, 2 );
/// ```
pub struct ScalarMatrixOracle < Key, Val >
{
    scalar: Val,
    major_dimension: MajorDimension,
    phantom: PhantomData<*const Key> 
}

impl    < Key, Val >
        ScalarMatrixOracle 
        < Key, Val > 
{
    /// Create new scalar matrix.
    pub fn new( scalar: Val, major_dimension: MajorDimension ) -> Self  
    {
        ScalarMatrixOracle { scalar: scalar,
                             major_dimension: major_dimension,
                             phantom: PhantomData 
                            }
    }
}


//  ---------------------
//  TRAIT IMPLEMENTATIONS
//  ---------------------


//  MAJOR DIMENSION
//  ---------------------------------------------------------------------------


//  WHICH MAJOR 
//  

impl     < Key, Val >
        WhichMajor 
        for 
        ScalarMatrixOracle < Key, Val > 
{ fn major_dimension( &self ) -> MajorDimension { self.major_dimension.clone() } }


//  MAJORS
//  ---------------------------------------------------------------------------


//  OracleMajor
//  
impl     < 'a, Key, Val >
        OracleMajor < 'a, Key, Key, Val >
        for 
        ScalarMatrixOracle < Key, Val > 
        
        where   Val: 'a + Clone, // hard to drop this requirement (tuples give move errors if no clone) 
                Key: 'a + Clone  // hard to drop this requirement (tuples give move errors if no clone) 
{
    type PairMajor =   (Key, Val)  ;
    type ViewMajor =   iter::Once< Self::PairMajor >;

    fn view_major<'b: 'a>( &'b self, index: Key ) -> Self::ViewMajor 
    { 
        iter::once( ( index, self.scalar.clone() ) )
    }
}

//  OracleMajorAscend
//  
impl     < 'a, Key, Val >
        OracleMajorAscend < 'a, Key, Key, Val >
        for 
        ScalarMatrixOracle < Key, Val > 
        
        where   Val: Clone, // hard to drop this requirement (tuples give move errors if no clone) 
                Key: Clone  // hard to drop this requirement (tuples give move errors if no clone) 
{
    type PairMajorAscend =   (Key, Val)  ;
    type ViewMajorAscend =   iter::Once< Self::PairMajorAscend >;

    fn view_major_ascend<'b: 'a>( &'b self, index: Key ) -> Self::ViewMajorAscend
    { 
        iter::once( ( index, self.scalar.clone() ) )
    }
}


//  OracleMajorDescend
//  
impl     < 'a, Key, Val >
        OracleMajorDescend < 'a, Key, Key, Val >
        for 
        ScalarMatrixOracle < Key, Val > 
        
        where   Val: Clone, // hard to drop this requirement (tuples give move errors if no clone) 
                Key: Clone  // hard to drop this requirement (tuples give move errors if no clone) 
{
    type PairMajorDescend =   (Key, Val)  ;
    type ViewMajorDescend =   iter::Once< Self::PairMajorDescend >;

    fn view_major_descend<'b: 'a>( &'b self, index: Key ) -> Self::ViewMajorDescend
    { 
        iter::once( ( index, self.scalar.clone() ) )
    }
}


//  MINORS
//  ---------------------------------------------------------------------------


//  OracleMinor
//  
impl     < 'a, Key, Val >
        OracleMinor < 'a, Key, Key, Val >
        for 
        ScalarMatrixOracle < Key, Val > 
        
        where   Val: Clone, // hard to drop this requirement (tuples give move errors if no clone) 
                Key: Clone  // hard to drop this requirement (tuples give move errors if no clone) 
{
    type PairMinor =   (Key, Val)  ;
    type ViewMinor =   iter::Once< Self::PairMinor >;

    fn view_minor<'b: 'a>( &'b self, index: Key ) -> Self::ViewMinor 
    { 
        iter::once( ( index, self.scalar.clone() ) )
    }
}

//  OracleMinorAscend
//  
impl     < 'a, Key, Val >
        OracleMinorAscend < 'a, Key, Key, Val >
        for 
        ScalarMatrixOracle < Key, Val > 
        
        where   Val: Clone, // hard to drop this requirement (tuples give move errors if no clone) 
                Key: Clone  // hard to drop this requirement (tuples give move errors if no clone) 
{
    type PairMinorAscend =   (Key, Val)  ;
    type ViewMinorAscend =   iter::Once< Self::PairMinorAscend >;

    fn view_minor_ascend<'b: 'a>( &'b self, index: Key ) -> Self::ViewMinorAscend
    { 
        iter::once( ( index, self.scalar.clone() ) )
    }
}


//  OracleMinorDescend
//  
impl     < 'a, Key, Val >
        OracleMinorDescend < 'a, Key, Key, Val >
        for 
        ScalarMatrixOracle < Key, Val > 
        
        where   Val: Clone, // hard to drop this requirement (tuples give move errors if no clone) 
                Key: Clone  // hard to drop this requirement (tuples give move errors if no clone) 
{
    type PairMinorDescend =   (Key, Val)  ;
    type ViewMinorDescend =   iter::Once< Self::PairMinorDescend >;

    fn view_minor_descend<'b: 'a>( &'b self, index: Key ) -> Self::ViewMinorDescend
    { 
        iter::once( ( index, self.scalar.clone() ) )
    }
}






