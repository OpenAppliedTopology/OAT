


use std::iter::{Iterator, Peekable};



//  ---------------------------------------------------------------------------
//  PEEKING


/// Similar to itertools::PeakingNext, but without the `F` parameter
/// (we hypothesize that omitting this closure will help with type
/// inference)
pub trait PeekUnqualified : Iterator {
    fn peek_unqualified( &mut self ) -> Option < & Self::Item >;
}

impl < I : Iterator > PeekUnqualified for Peekable< I >
{
    fn peek_unqualified( &mut self ) -> Option < &<Self as Iterator>::Item > { self.peek() }
}

