
use crate::matrices::matrix_oracle::{   OracleMajor,
                                        OracleMajorAscend,
                                        OracleMajorDescend,
                                        OracleMinor, 
                                        OracleMinorAscend,
                                        OracleMinorDescend,
                                        WhichMajor,
                                        MajorDimension};
use std::iter;


pub struct VecCsv< MinKey, SnzVal >
{
    major_dimension: MajorDimension, 
    min_ind: Vec< Vec< MinKey > > ,
    snz_val: Vec< Vec< SnzVal > >
}


impl    < MinKey, SnzVal >
        VecOfVec 
        < MinKey, SnzVal > 
{
    // Make new (empty) VecOfVec. 
    pub fn new( major_dimension: MajorDimension ) -> Self  
    {
        atrixOracle { scalar: scalar,
                             major_dimension: major_dimension,
                             phantom: PhantomData 
                            }
    }
}



