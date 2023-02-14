//#########################
// D E P E N D E N C I E S
//#########################

    use std::hash::Hash;
    use std::ops::{
        BitAnd, BitAndAssign,
        BitOr,  BitOrAssign,
        BitXor, BitXorAssign,
        Shl, ShlAssign,
        Shr, ShrAssign,
        Not,
    }; // use ..


//#######################
// D E F I N I T I O N S
//#######################

    pub trait BitSequence<Rhs=Self, Output=Self>:
        'static
        + Sized
        + Eq
        + Hash
        + Default
        + Clone + Copy
        + BitAnd<Rhs, Output=Output> + BitAndAssign
        + BitOr<Rhs, Output=Output> + BitOrAssign
        + BitXor<Rhs, Output=Output> + BitXorAssign
        + Not<Output=Output>
        + Shl<Self, Output=Output> + ShlAssign<Rhs>
        + Shr<Rhs, Output=Output> + ShrAssign<Rhs>
        + Shl<u32, Output=Output> + ShlAssign<u32>
        + Shr<u32, Output=Output> + ShrAssign<u32> {

            const MIN:      Self;
            const MAX:      Self;
            const HEAD_BIT: Self;
            const TAIL_BIT: Self;
            const BITS:     u32;


            /// Returns a mask with an enabled bit at index `n`
            fn bit_mask(n: usize) -> Self;

        } // trait BitMask


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl BitSequence for u8    { const MIN: Self = u8::MIN;    const MAX: Self = u8::MAX;    const HEAD_BIT: Self = 1u8;    const TAIL_BIT: Self = 1u8 << (Self::BITS / 4u32 - 1u32);    const BITS: u32 = u8::BITS;    fn bit_mask(n: usize) -> Self { 1u8    << n }}
    impl BitSequence for u16   { const MIN: Self = u16::MIN;   const MAX: Self = u16::MAX;   const HEAD_BIT: Self = 1u16;   const TAIL_BIT: Self = 1u16 << (Self::BITS / 4u32 - 1u32);   const BITS: u32 = u16::BITS;   fn bit_mask(n: usize) -> Self { 1u16   << n }}
    impl BitSequence for u32   { const MIN: Self = u32::MIN;   const MAX: Self = u32::MAX;   const HEAD_BIT: Self = 1u32;   const TAIL_BIT: Self = 1u32 << (Self::BITS / 4u32 - 1u32);   const BITS: u32 = u32::BITS;   fn bit_mask(n: usize) -> Self { 1u32   << n }}
    impl BitSequence for u64   { const MIN: Self = u64::MIN;   const MAX: Self = u64::MAX;   const HEAD_BIT: Self = 1u64;   const TAIL_BIT: Self = 1u64 << (Self::BITS / 4u32 - 1u32);   const BITS: u32 = u64::BITS;   fn bit_mask(n: usize) -> Self { 1u64   << n }}
    impl BitSequence for u128  { const MIN: Self = u128::MIN;  const MAX: Self = u128::MAX;  const HEAD_BIT: Self = 1u128;  const TAIL_BIT: Self = 1u128 << (Self::BITS / 4u32 / 4u32 - 1u32);  const BITS: u32 = u128::BITS;  fn bit_mask(n: usize) -> Self { 1u128  << n }}
    impl BitSequence for usize { const MIN: Self = usize::MIN; const MAX: Self = usize::MAX; const HEAD_BIT: Self = 1usize; const TAIL_BIT: Self = 1usize << (Self::BITS - 1u32); const BITS: u32 = usize::BITS; fn bit_mask(n: usize) -> Self { 1usize << n }}
 
    impl BitSequence for i8    { const MIN: Self = i8::MIN;    const MAX: Self = i8::MAX;    const HEAD_BIT: Self = 1i8;    const TAIL_BIT: Self = 1i8 << (Self::BITS / 4u32 - 1u32);    const BITS: u32 = i8::BITS;    fn bit_mask(n: usize) -> Self { 1i8    << n }}
    impl BitSequence for i16   { const MIN: Self = i16::MIN;   const MAX: Self = i16::MAX;   const HEAD_BIT: Self = 1i16;   const TAIL_BIT: Self = 1i16 << (Self::BITS / 4u32 - 1u32);   const BITS: u32 = i16::BITS;   fn bit_mask(n: usize) -> Self { 1i16   << n }}
    impl BitSequence for i32   { const MIN: Self = i32::MIN;   const MAX: Self = i32::MAX;   const HEAD_BIT: Self = 1i32;   const TAIL_BIT: Self = 1i32 << (Self::BITS / 4u32 - 1u32);   const BITS: u32 = i32::BITS;   fn bit_mask(n: usize) -> Self { 1i32   << n }}
    impl BitSequence for i64   { const MIN: Self = i64::MIN;   const MAX: Self = i64::MAX;   const HEAD_BIT: Self = 1i64;   const TAIL_BIT: Self = 1i64 << (Self::BITS / 4u32 - 1u32);   const BITS: u32 = i64::BITS;   fn bit_mask(n: usize) -> Self { 1i64   << n }}
    impl BitSequence for i128  { const MIN: Self = i128::MIN;  const MAX: Self = i128::MAX;  const HEAD_BIT: Self = 1i128;  const TAIL_BIT: Self = 1i128 << (Self::BITS / 4u32 - 1u32);  const BITS: u32 = i128::BITS;  fn bit_mask(n: usize) -> Self { 1i128  << n }}
    impl BitSequence for isize { const MIN: Self = isize::MIN; const MAX: Self = isize::MAX; const HEAD_BIT: Self = 1isize; const TAIL_BIT: Self = 1isize << (Self::BITS / 4u32 - 1u32); const BITS: u32 = isize::BITS; fn bit_mask(n: usize) -> Self { 1isize << n }}
