// THIS MODULE HAS BEEN BROKEN UP INTO PARTS; IT IS UNUSED AS OF 2021-06-12

// //! Fields in abstract algebra.
// 
// 
// use num::rational::Ratio;
// use crate::rings::ring;
// 
// 
// 
// 
// 
// //----------------------------------------------------------
// //  THE FIELD TRAIT 
// //----------------------------------------------------------
// 
// 
// pub trait Field <Element> : ring::Ring {
//     
// 
// //    // IDENTITY ELEMENTS
// //
// //    /// Return the additive identity.
// //    fn is_0( &self, x : &Element ) -> bool;
// //
// //    /// Return the multiplicative identity.
// //    fn is_1( &self, x : &Element ) -> bool;
// //
// //    /// Return the additive identity.
// //    fn zero( &self ) -> Element;
// //
// //    /// Return the multiplicative identity.
// //    fn one( &self ) -> Element;
// //
// //
// //
// //    // FIELD OPERATIONS 
// //    
// //    // DESIGN NOTE: if we changed these functions to take
// //    // non-references as input, we would have to think about
// //    // wether to require ring elements to implement the
// //    // copy trait.
// //
// //    /// Add
// //    fn add( &self, x : &Element, y : &Element ) -> Element;
// //
// //    /// Subtract y from x.
// //    fn subtract( &self, x : &Element, y: &Element ) -> Element;
// //
// //    /// Multiply
// //    fn multiply( &self, x : &Element, y: &Element ) -> Element;
// //
// //    /// `Some(x/y)` if `y` is nonzero.  Otherwise return `None`.
// //    fn divide( &self, x : &Element, y: &Element ) -> Element;
// //
// //    /// Reverse the sign of x.
// //    fn negate( &self, x : &Element ) -> Element;
// //
// //    /// `Some(1/x)` if `x` is nonzero.  Otherwise return `None`.
// //    fn invert( &self, x : &Element ) -> Element;
// //    
//     /// Divide 
//     fn divide( &self, x : &Element, y: &Element ) -> Element;
// 
//     /// Invert 
//     fn invert( &self, x : &Element ) -> Element;
// 
// }
// 
// 
// 
// //----------------------------------------------------------
// //  NATIVE FIELDS (GENERAL)
// //----------------------------------------------------------
// 
// 
// pub struct NativeField< Element >
//     where 
//         Element:    num::traits::num + 
//                     num::traits::Zero +
//                     num::traits::One +
//                     core::ops::Add < Output = Element >  +
//                     core::ops::Sub < Output = Element > +
//                     core::ops::Mul < Output = Element >  +
//                     core::ops::Div < Output = Element > +
//                     std::ops::Neg  < Output = Element > +
//                     std::cmp::PartialEq +
//                     std::clone::Clone
// {
//     zero: Element, // keep this on hand so it never has to be (de)allocated
//     one: Element, // keep this on hand so it never has to be (de)allocated
// }
// 
// impl <Element> Semiring  <Element> for NativeField < Element >
//     where 
//         Element:    core::ops::Add +
//                     core::ops::Sub < Output = Element > +
//                     core::ops::Mul +
//                     core::ops::Div < Output = Element > +
//                     std::ops::Neg < Output = Element > +
//                     num::traits::Zero +
//                     num::traits::One +
//                     std::cmp::PartialEq +
//                     std::clone::Clone
// {
//     // IDENTITY ELEMENTS
// 
//     fn is_0( &self, x : &Element ) -> bool { x.is_zero() }
//     fn is_1( &self, x : &Element ) -> bool { x.is_one() }
//     fn zero( &self ) -> Element { self.zero.clone() }
//     fn one( &self ) -> Element { self.one.clone() }
// 
//     // FIELD OPERATIONS
// 
//     /// Add
//     fn add( &self, x : &Element, y : &Element ) -> Element { x.clone() + y.clone() }
// 
//     /// Subtract y from x.
//     fn subtract( &self, x : &Element, y: &Element ) -> Element { x.clone() - y.clone() }
// 
//     /// Multiply
//     fn multiply( &self, x : &Element, y: &Element ) -> Element { x.clone() * y.clone() }
// 
//     /// `Some(x/y)` if `y` is nonzero.  Otherwise return `None`.
//     fn divide_maybe( &self, x : &Element, y: &Element ) -> Option<Element> { Some(x.clone() / y.clone()) }
// 
//     /// Reverse the sign of x.
//     fn negate( &self, x : &Element ) -> Element { -x.clone() }
// 
//     /// `Some(1/x)` if `x` is nonzero.  Otherwise return `None`.
//     fn invert_maybe( &self, x : &Element ) -> Option<Element> { Some(self.one() / x.clone()) }
//     
// }
// 
// 
// 
// 
// impl <Element> Field <Element> for NativeField < Element > 
//     where 
//         Element:    core::ops::Add +
//                     core::ops::Sub < Output = Element > +
//                     core::ops::Mul +
//                     core::ops::Div < Output = Element > +
//                     std::ops::Neg < Output = Element > +
//                     num::traits::Zero +
//                     num::traits::One +
//                     std::cmp::PartialEq +
//                     std::clone::Clone
// {
// 
//     /// `Some(x/y)` if `y` is nonzero.  Otherwise return `None`.
//     fn divide( &self, x : &Element, y: &Element ) -> Element { x.clone() / y.clone() }
// 
//     /// `Some(1/x)` if `x` is nonzero.  Otherwise return `None`.
//     fn invert( &self, x : &Element ) -> Element { self.one() / x.clone() }
//     
// }
// 
// 
// //----------------------------------------------------------
// //  IMPORTED FIELDS (SPECIFIC)
// //----------------------------------------------------------
// 
// /// Generate a field operator for `i32` rationals.
// ///
// /// ```
// /// use num::rational::Ratio;
// /// use solar::field::{Field, field_r32};
// /// use std::assert_eq;
// /// 
// /// fn main() {
// ///     let a = Ratio::from_integer(2 as i32);
// ///     let b = Ratio::from_integer(3 as i32);
// ///     let op = field_r32();
// ///     
// ///     assert_eq!( op.add(      &a, &b), Ratio::from_integer(5 as i32)  );
// ///     assert_eq!( op.subtract( &a, &b), Ratio::from_integer(-1 as i32) );
// ///     assert_eq!( op.multiply( &a, &b), Ratio::from_integer(6 as i32)  );
// ///     assert_eq!( op.divide(   &a, &b), Ratio::new(2 as i32, 3 as i32) );
// ///     assert_eq!( op.negate(   &a    ), Ratio::from_integer(-2 as i32) );
// ///     assert_eq!( op.invert(   &a    ), Ratio::new(1 as i32, 2 as i32) );
// /// }
// /// ```
// pub fn field_r32() -> NativeField < Ratio < i32 > > {
//     NativeField {
//         field_name: FieldName::Rational,
//         zero: Ratio::from_integer(0 as i32),
//         one: Ratio::from_integer(1 as i32)
//     }
// }
// 
// pub fn field_f64() -> NativeField < f64 > {
//     NativeField{
//         field_name: FieldName::Real,
//         zero: 0.,
//         one: 1.
//     }
// }
// 
// 
// 
// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn test_field_r32() {
//         let a = Ratio::from_integer(2 as i32);
//         let b = Ratio::from_integer(3 as i32);
//         let op = field_r32();
// 
//         assert_eq!( op.add(      &a, &b), Ratio::from_integer(5 as i32)  );
//         assert_eq!( op.subtract( &a, &b), Ratio::from_integer(-1 as i32) );
//         assert_eq!( op.multiply( &a, &b), Ratio::from_integer(6 as i32)  );
//         assert_eq!( op.divide(   &a, &b), Ratio::new(2 as i32, 3 as i32) );
//         assert_eq!( op.negate(   &a    ), Ratio::from_integer(-2 as i32) );
//         assert_eq!( op.invert(   &a    ), Ratio::new(1 as i32, 2 as i32) );
// 
//         //    println!("{:?}", op.add( &a, &b) );
//         //    println!("{:?}", op.subtract( &a, &b) );
//         //    println!("{:?}", op.multiply( &a, &b) );
//         //    println!("{:?}", op.divide( &a, &b) );
//         //    println!("{:?}", op.negate( &a) );
//         //    println!("{:?}", op.invert( &a) );
//     }
// }
// 
// 
