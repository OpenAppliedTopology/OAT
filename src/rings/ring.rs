//! Traits for semirings, rings, and division rings.
//!
//!
//! TO-DO LIST FOR DEVELOPERS:
//! * MAKE ALL FUNCTIONS TAKE INPUTS BY VAL, NOT BY REFERENCE
//! (POINTERS CAN TAKE MORE MEMORY THAN INPUTS)
//! * MAKE AUTO-TRAIT IMPLEMENTATIONS TO HANDLE
//! INPUTS BY REFERENCE


//  ---------------------------------------------------------------------------
//  DESIGN NOTES
//  ---------------------------------------------------------------------------

//  * Advantage of this nested structure: makes it straightforward to define matrix multipication
//  over semirings.
//
//  * Reason for deprecating the function "field name" that tells you the underlying mathematical
//  field: 
//  in general, you always know what struct you're working with; so it suffices to describe the
//  mathematical object underlying the struct in the struct's documentation

use auto_impl::auto_impl;





//  ---------------------------------------------------------------------------
//  THE SEMIRING TRAIT
//  ---------------------------------------------------------------------------

/// Basic operations for semirings.
#[auto_impl(&)] // auto-implement this trait on references to objects that implement the trait
pub trait Semiring < Element > {


    // IDENTITY ELEMENTS

    /// Return the additive identity.
    fn is_0( &self, x : Element ) -> bool;

    /// Return the multiplicative identity.
    fn is_1( &self, x : Element ) -> bool;

    /// Return the additive identity.
    fn zero() -> Element;

    /// Return the multiplicative identity.
    fn one() -> Element;


    // OPERATIONS 
    
    // DESIGN NOTE: if we changed these functions to take
    // non-references as input, we would have to think about
    // wether to require ring elements to implement the
    // copy trait.

    /// Add
    fn add( &self, x : Element, y : Element ) -> Element;

    /// Multiply
    fn multiply( &self, x : Element, y: Element ) -> Element;

}


//  ---------------------------------------------------------------------------
//  THE RING TRAIT
//  ---------------------------------------------------------------------------


/// Basic operations for **unital** rings.
pub trait Ring <Element> : Semiring < Element > {

    /// Subtract y from x.
    fn subtract( &self, x : Element, y: Element ) -> Element;

    /// Reverse the sign of x.
    fn negate( &self, x : Element ) -> Element;

}


//----------------------------------------------------------
//  THE DIVISION RING TRAIT 
//----------------------------------------------------------

/// Basic operations for division rings.
pub trait DivisionRing <Element> : Ring < Element > {
    
    /// Divide 
    fn divide( &self, x : Element, y: Element ) -> Element;

    /// Invert 
    fn invert( &self, x : Element ) -> Element;

}