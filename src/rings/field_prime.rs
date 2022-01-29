//! Objects representing prime fields.

use crate::rings::ring::{Semiring, Ring, DivisionRing};





//  ---------------------------------------------------------
//  2   ELEMENT FIELD
//  ---------------------------------------------------------

#[derive(Debug, Clone)]
pub struct GF2{}

impl GF2 {
    /// Create a new instance of `GF2`.
    /// 
    /// The commands `GF2::new()` and `GF2{}` are equivalent.  The primary advantage of having this
    /// `new` function is that overlaps with the syntax of other ring objects that are harder to construct; 
    /// this makes it easier to interchange one ring object with another.
    pub fn new() -> GF2 { GF2{} }
}

impl Semiring<bool> for GF2 
{
    fn is_0( &self, x: bool ) -> bool { ! x         }
    fn is_1( &self, x: bool ) -> bool {   x.clone() }
    fn zero() -> bool { false }
    fn one()  -> bool { true  }

    fn add( &self, x : bool, y : bool ) -> bool { x ^ y }
    fn multiply( &self, x : bool, y: bool ) -> bool { x && y }
}

impl Ring<bool> for GF2
{
    fn subtract( &self, x : bool, y: bool ) -> bool { x ^ y }
    fn negate( &self, x : bool ) -> bool { x }  // this one is tricky; you want to try logical negation, but you relly need to perform additive negation
}

impl DivisionRing<bool> for GF2
{
    /// NOTE: THIS DIVISION IS UNSAFE; DESIGNERS MAY WANT TO ADD OPTION TO CHECK FOR DIVISION BY
    /// ZERO
    fn divide( &self, x : bool, _y: bool ) -> bool { x.clone() }
    
    /// NOTE: THIS DIVISION IS UNSAFE; DESIGNERS MAY WANT TO ADD OPTION TO CHECK FOR DIVISION BY
    /// ZERO
    fn invert( &self, x : bool ) -> bool { x.clone() }
}






#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test_GF2() {
        
        let ring                        =   GF2{};

        assert!(    !   ring.is_0( true     ) );
        assert!(        ring.is_0( false    ) );
        assert!(        ring.is_1( true     ) );
        assert!(    !   ring.is_1( false    ) );        
        assert!(        ring.invert( true   ) );
        assert!(        ring.negate( true     ) );
        assert!(    !   ring.negate( false    ) );        
        assert!(    !   ring.add( false, false ) );        
        assert!(        ring.add( false, true  ) );        
        assert!(        ring.add( true,  false ) );                
        assert!(    !   ring.add( true,  true  ) );
        assert!(    !   ring.subtract( false, false ) );        
        assert!(        ring.subtract( false, true  ) );        
        assert!(        ring.subtract( true,  false ) );                
        assert!(    !   ring.subtract( true,  true  ) );  
        assert!(    !   ring.multiply( false, false ) );        
        assert!(    !   ring.multiply( false, true  ) );        
        assert!(    !   ring.multiply( true,  false ) );                
        assert!(        ring.multiply( true,  true  ) );                 
        assert!(    !   ring.divide( false, true  ) );            
        assert!(        ring.divide( true,  true  ) );                  

    }

}


