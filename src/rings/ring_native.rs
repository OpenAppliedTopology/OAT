//! Convenient wrappers for rings that already exist in the Rust language.
//! 
//! To define a semiring (or ring, or division ring) in SOLAR, you define an 
//! object `R` that implements the semiring trait.  You can then use `R` to 
//! perform basic operations on the elements of the ring (addition, multiplication, etc.)
//! 
//! Rust already has a number of rings "built in."  The current module provides a 
//! convenient way to generate a ring operation object `R` for one of these built-in rings.
//! The objects defined in this way use zero memory!
//! `

// //! There are a number of concepts related to rings and ring operations
// //! built into the Rust language.  This module allows one to create a 
// //! "ring-operation object" has a number of 
// //! This module provides wrappers and auto-implementations that 
// //! Zero-memory structs representing semirings/rings/division rings that are native to Rust.


use crate::rings::ring::{Semiring, Ring, DivisionRing};
use std::marker::PhantomData;

//----------------------------------------------------------
//  SEMIRINGS NATIVE TO RUST
//----------------------------------------------------------


/// Zero-memory struct encoding structure of native Rust semirings.
///
/// # Examples
///
/// ```
/// use solar::rings::ring_native::NativeSemiring;
/// use solar::rings::ring::{Semiring};
///
/// let ring  =  < NativeSemiring::<usize> >::new();
///
/// assert_eq!( 3, ring.add( 1, 2 ) ); 
/// assert_eq!( 2, ring.multiply( 1, 2 ) );
/// assert_eq!( 0, NativeSemiring::<usize>::zero() );
/// assert_eq!( 1, NativeSemiring::<usize>::one()  );
/// assert!( ! ring.is_0( 1 ) );
/// assert!(   ring.is_0( 0 ) );
/// assert!(   ring.is_1( 1 ) );
/// assert!( ! ring.is_1( 0 ) );
/// ```
#[derive(Debug, Clone)]
pub struct NativeSemiring< Element >
    where 
        Element:    //num::traits::Num + 
                    num::traits::Zero +
                    num::traits::One +
                    core::ops::Add < Output = Element >  +
                    //core::ops::Sub < Output = Element > +
                    core::ops::Mul < Output = Element >  +
                    //core::ops::Div < Output = Element > +
                    //std::ops::Neg  < Output = Element > +
                    std::cmp::PartialEq +
                    std::clone::Clone
{ 
    // This phantom field uses zero memory; it is here only 
    // because rust otherwise complains that `Element` is
    // unused.  See the documentation on `PhantomData` for
    // more details.  **Note** that `*const` appears because
    // there is no relevant lifetime parameter for the 
    // struct.  Again, see the docs for `PhantomData`.
    phantom: PhantomData<*const Element> 
}

impl    < Element >
        NativeSemiring 
        < Element > 
    where 
        Element:    //num::traits::Num + 
                    num::traits::Zero +
                    num::traits::One +
                    core::ops::Add < Output = Element >  +
                    //core::ops::Sub < Output = Element > +
                    core::ops::Mul < Output = Element >  +
                    //core::ops::Div < Output = Element > +
                    //std::ops::Neg  < Output = Element > +
                    std::cmp::PartialEq +
                    std::clone::Clone
{
    // Generate a `NativeSemiring`.
    pub fn new( ) -> Self  
    {
        NativeSemiring { phantom: PhantomData }
    }
}


impl    < Element > 
        Semiring < Element > for NativeSemiring 
        < Element >  
    where 
        Element:    //num::traits::Num + 
                    num::traits::Zero +
                    num::traits::One +
                    core::ops::Add < Output = Element >  +
                    //core::ops::Sub < Output = Element > +
                    core::ops::Mul < Output = Element >  +
                    //core::ops::Div < Output = Element > +
                    //std::ops::Neg  < Output = Element > +
                    std::cmp::PartialEq +
                    std::clone::Clone
{
    /// Identity elements
    fn is_0( &self, x: Element ) -> bool { x.is_zero() }
    fn is_1( &self, x: Element ) -> bool { x.is_one() }
    fn zero() -> Element { Element::zero() }
    fn one()  -> Element { Element::one() }

    /// Add
    fn add( &self, x: Element, y: Element ) -> Element { x + y }

    /// Multiply
    fn multiply( &self, x: Element, y: Element ) -> Element { x * y }
}



//----------------------------------------------------------
//  RINGS NATIVE TO RUST
//----------------------------------------------------------

/// Zero-memory struct encoding structure of native Rust rings.
///
/// # Examples
///
/// ```
/// use solar::rings::ring_native::NativeRing;
/// use solar::rings::ring::{Semiring, Ring};
///
/// let ring = NativeRing::<i64>::new();
/// let a : i64 = 1;
/// let b : i64 = 2;
///
/// assert_eq!( -1, ring.subtract( a, b ) );
/// assert_eq!( -1, ring.negate( a ) );
/// ```
#[derive(Debug, Clone)]
pub struct NativeRing< Element >
    where 
        Element:    num::traits::Num + 
                    num::traits::Zero +
                    num::traits::One +
                    core::ops::Add < Output = Element >  +
                    core::ops::Sub < Output = Element > +
                    core::ops::Mul < Output = Element >  +
                    core::ops::Div < Output = Element > +
                    std::ops::Neg  < Output = Element > +
                    std::cmp::PartialEq +
                    std::clone::Clone
{ 
    // This phantom field uses zero memory; it is here only 
    // because rust otherwise complains that `Element` is
    // unused.  See the documentation on `PhantomData` for
    // more details.  **Note** that `*const` appears because
    // there is no relevant lifetime parameter for the 
    // struct.  Again, see the docs for `PhantomData`.
    phantom: PhantomData<*const Element> 
}

impl    < Element >
        NativeRing 
        < Element > 
    where 
        Element:    num::traits::Num + 
                    num::traits::Zero +
                    num::traits::One +
                    core::ops::Add < Output = Element >  +
                    core::ops::Sub < Output = Element > +
                    core::ops::Mul < Output = Element >  +
                    core::ops::Div < Output = Element > +
                    std::ops::Neg  < Output = Element > +
                    std::cmp::PartialEq +
                    std::clone::Clone
{
    // Generate a `NativeRing`.
    pub fn new( ) -> Self  
    {
        NativeRing { phantom: PhantomData }
    }
}


impl    < Element > 
        Semiring < Element > for NativeRing 
        < Element >  
    where 
        Element:    num::traits::Num + 
                    num::traits::Zero +
                    num::traits::One +
                    core::ops::Add < Output = Element >  +
                    core::ops::Sub < Output = Element > +
                    core::ops::Mul < Output = Element >  +
                    core::ops::Div < Output = Element > +
                    std::ops::Neg  < Output = Element > +
                    std::cmp::PartialEq +
                    std::clone::Clone
{
    /// Identity elements
    fn is_0( &self, x: Element ) -> bool { x.is_zero() }
    fn is_1( &self, x: Element ) -> bool { x.is_one() }
    fn zero() -> Element { Element::zero() }
    fn one()  -> Element { Element::one() }

    /// Add
    fn add( &self, x: Element, y: Element ) -> Element { x + y }

    /// Multiply
    fn multiply( &self, x: Element, y: Element ) -> Element { x * y }
}

impl    < Element > 
        Ring < Element > for NativeRing 
        < Element >  
    where 
        Element:    num::traits::Num + 
                    num::traits::Zero +
                    num::traits::One +
                    core::ops::Add < Output = Element >  +
                    core::ops::Sub < Output = Element > +
                    core::ops::Mul < Output = Element >  +
                    core::ops::Div < Output = Element > +
                    std::ops::Neg  < Output = Element > +
                    std::cmp::PartialEq +
                    std::clone::Clone
{
    /// Subtract `x-y`.
    fn subtract( &self, x: Element, y: Element ) -> Element { x - y }

    /// Additive inverse `-x`. 
    fn negate( &self, x: Element ) -> Element { - x }
}


//----------------------------------------------------------
//  DIVISION RINGS NATIVE TO RUST
//----------------------------------------------------------

/// Zero-memory struct encoding structure of native Rust division rings.
///
/// # Examples
///
/// ```
/// use solar::rings::ring_native::NativeDivisionRing;
/// use solar::rings::ring::{Semiring, Ring, DivisionRing};
/// use num::rational::Ratio;
///
///
/// // The `< .. >` brackets around `NativeDivisionRing::<Ratio<i64>>`
/// // are used to disambiguate which `new` function should be used
/// // (Rust throws an error if these aren't used)
/// let ring  =     < 
///                     NativeDivisionRing::<Ratio<i64>> 
///                 >
///                 ::new();
/// let a = Ratio::new( 2, 3 );
/// let b = Ratio::new( 3, 1 );
/// let c = Ratio::new( 2, 9 );
/// let d = Ratio::new( 3, 2 );
///
/// assert_eq!( c, ring.divide( a, b ) );
/// assert_eq!( d, ring.invert( a ) );
/// ```
///
/// ```
/// use solar::rings::ring_native::NativeDivisionRing;
/// use solar::rings::ring::{Semiring, Ring, DivisionRing};
/// use num::rational::Ratio;
/// 
/// 
/// let ring : NativeDivisionRing< f64 > =  NativeDivisionRing::<f64>::new();
/// let a = 2.0 ;
/// let b = 4.0 ;
///
/// assert_eq!( 0.5, ring.divide( a, b ) );
/// assert_eq!( 0.5, ring.invert( a ) );
///
/// ```
#[derive(Debug, Clone)]
pub struct NativeDivisionRing< Element >
    where 
        Element:    num::traits::Num + 
                    num::traits::Zero +
                    num::traits::One +
                    core::ops::Add < Output = Element >  +
                    core::ops::Sub < Output = Element > +
                    core::ops::Mul < Output = Element >  +
                    core::ops::Div < Output = Element > +
                    std::ops::Neg  < Output = Element > +
                    std::cmp::PartialEq +
                    std::clone::Clone
{ 
    // This phantom field uses zero memory; it is here only 
    // because rust otherwise complains that `Element` is
    // unused.  See the documentation on `PhantomData` for
    // more details.  **Note** that `*const` appears because
    // there is no relevant lifetime parameter for the 
    // struct.  Again, see the docs for `PhantomData`.
    phantom: PhantomData<*const Element> 
}
//{
//    zero: Element, // keep this on hand so it never has to be (de)allocated
//    one: Element, // keep this on hand so it never has to be (de)allocated
//}

impl    < Element >
        NativeDivisionRing 
        < Element > 
    where 
        Element:    num::traits::Num + 
                    num::traits::Zero +
                    num::traits::One +
                    core::ops::Add < Output = Element >  +
                    core::ops::Sub < Output = Element > +
                    core::ops::Mul < Output = Element >  +
                    core::ops::Div < Output = Element > +
                    std::ops::Neg  < Output = Element > +
                    std::cmp::PartialEq +
                    std::clone::Clone

{
    // Generate a `NativeDivisionRing`.
    pub fn new( ) -> Self  
    {
        NativeDivisionRing { phantom: PhantomData }
    }
}


impl    < Element > 
        Semiring < Element > for NativeDivisionRing 
        < Element >  
    where 
        Element:    num::traits::Num + 
                    num::traits::Zero +
                    num::traits::One +
                    core::ops::Add < Output = Element >  +
                    core::ops::Sub < Output = Element > +
                    core::ops::Mul < Output = Element >  +
                    core::ops::Div < Output = Element > +
                    std::ops::Neg  < Output = Element > +
                    std::cmp::PartialEq +
                    std::clone::Clone
{
    /// Identity elements
    fn is_0( &self, x: Element ) -> bool { x.is_zero() }
    fn is_1( &self, x: Element ) -> bool { x.is_one() }
    fn zero() -> Element { Element::zero() }
    fn one()  -> Element { Element::one() }

    /// Add
    fn add( &self, x: Element, y: Element ) -> Element { x + y }

    /// Multiply
    fn multiply( &self, x: Element, y: Element ) -> Element { x * y }
}

impl    < Element > 
        Ring < Element > for NativeDivisionRing 
        < Element >  
    where 
        Element:    num::traits::Num + 
                    num::traits::Zero +
                    num::traits::One +
                    core::ops::Add < Output = Element >  +
                    core::ops::Sub < Output = Element > +
                    core::ops::Mul < Output = Element >  +
                    core::ops::Div < Output = Element > +
                    std::ops::Neg  < Output = Element > +
                    std::cmp::PartialEq +
                    std::clone::Clone
{
    /// Subtract y from x.
    fn subtract( &self, x: Element, y: Element ) -> Element { x - y }

    /// Additive inverse `-x`. 
    fn negate( &self, x: Element ) -> Element { - x }
}

impl    < Element > 
        DivisionRing < Element > for NativeDivisionRing 
        < Element >  
    where 
        Element:    num::traits::Num + 
                    num::traits::Zero +
                    num::traits::One +
                    core::ops::Add < Output = Element >  +
                    core::ops::Sub < Output = Element > +
                    core::ops::Mul < Output = Element >  +
                    core::ops::Div < Output = Element > +
                    std::ops::Neg  < Output = Element > +
                    std::cmp::PartialEq +
                    std::clone::Clone
{
    /// `x/y` if `y` is nonzero.  
    fn divide( &self, x: Element, y: Element ) -> Element { x / y }

    /// `1/x` if `x` is nonzero.  
    fn invert( &self, x: Element ) -> Element { Element::one() / x }
}


//----------------------------------------------------------
//  CREATORS
//----------------------------------------------------------

// pub fn field_f64() -> NativeDivisionRing < f64 > 
//     { NativeDivisionRing{ zero: 0.0 as f64, one: 1.0 as f64 } }
