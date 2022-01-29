//! Rings, semirings, division rings.
//!
//! SOLAR uses ring objects to perform basic ring operations.  This is different from many
//! other platforms.  This issue concerns the relationship between strongly typed languages and how
//! they can/can't cope with infinite families of rings (like the family of finite fields).  The
//! main advantage of our approach is that it allows one to effectively work with infinitely many
//! rings, in principle, without defining infinitely many types.


// pub mod field;
pub mod ring;
pub mod ring_native;
pub mod field_prime;
