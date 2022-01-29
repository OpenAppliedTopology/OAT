

use crate::rings::ring::{Semiring, Ring};
use num::integer::Integer;


pub trait MinusOneToPower< RingElt > {
    fn minus_one_to_power( &self, k: usize ) -> RingElt;
}

impl    < RingOp, RingElt > 
        MinusOneToPower
        < RingElt > 
        for 
        RingOp
        where   RingOp: Semiring< RingElt > + Ring< RingElt >
{
    fn minus_one_to_power( &self, k: usize ) -> RingElt {
        if k.is_even() { RingOp::one() }
        else { self.negate( RingOp::one() ) }
    }
}        