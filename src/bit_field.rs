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
        RangeBounds, Bound,
        Sub,
    }; // use ..


//#######################
// D E F I N I T I O N S
//#######################

    /// A sequence of bits.
    pub trait BitField:
        Static
        + Copy
        + BitAnd<Self, Output=Self> + BitAndAssign
        + BitOr<Self,  Output=Self> + BitOrAssign
        + BitXor<Self, Output=Self> + BitXorAssign
        + Not<Output=Self>
        + Shl<Self, Output=Self> + ShlAssign<Self>
        + Shr<Self, Output=Self> + ShrAssign<Self>
        + Shl<u8,   Output=Self> + ShlAssign<u8>
        + Shr<u8,   Output=Self> + ShrAssign<u8> {

        const MIN:      Self;
        const MAX:      Self;
        const HEAD_BIT: Self;
        const TAIL_BIT: Self;
        const BITS:     u8;
        

        /// Returns a bit field with the `n`th bit enabled.
        fn bit(n: impl Into<u8>) -> Self { Self::HEAD_BIT << n.into() }

        /// Returns a bit field with the `n` first bits enabled.
        fn bits(n: impl Into<u8>) -> Self
        where Self: Sub<Self, Output=Self> { Self::bit(n) - Self::HEAD_BIT }

        /// Returns a bit mask inside `range`'s bounds.
        fn bit_mask(range: impl RangeBounds<u8>) -> Self
        where Self: Sub<Self, Output=Self> {
            let start = match range.start_bound() {
                Bound::Included(start) => *start,
                Bound::Excluded(start) => *start + 1u8,
                Bound::Unbounded       =>  0u8,
            }; // let ..
            let end = match range.end_bound() {
                Bound::Included(end) => *end,
                Bound::Excluded(end) => *end - 1u8,
                Bound::Unbounded     =>  255u8,
            }; // let ..
            (Self::bit(end - start) - Self::HEAD_BIT) << start
        } // fn bit_mask()


        /// Returns `true` if the `n`th bit is enabled.
        fn get_bit(&self, n: impl Into<u8>) -> bool { *self & Self::bit(n) == Self::HEAD_BIT }

        /// Returns the `n` first bits.
        fn get_n_bits(&self, n: impl Into<u8>) -> Self
        where Self: Sub<Self, Output=Self> { *self & Self::bits(n) }


        /// Returns a sub bit field inside `range`'s bounds.
        fn get_range(&self, range: impl RangeBounds<u8> + Clone) -> Self
        where Self: Sub<Self, Output=Self> {
            (*self & Self::bit_mask(range.clone())) >> match range.start_bound() {
                Bound::Included(start) => *start,
                Bound::Excluded(start) => *start + 1u8,
                Bound::Unbounded       =>  0u8,
            } // match ..
        } // fn get_range()
    } // trait BitField


    pub trait Static:
        'static
        + Sized
        + Eq
        + Hash
        + Clone {}

        
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct L8;
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct L16;
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct L32;
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct L64;
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct L128;
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct L256;


    pub trait Capacity<T> {

        const LEN: usize;
        type Array;
        
        fn uninit()                           -> Self::Array;
        fn slice(array: &Self::Array)         -> &[T];
        fn slice_mut(array: &mut Self::Array) -> &mut[T];

    } // trait Capacity


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl BitField for u8    { const MIN: Self = u8::MIN;    const MAX: Self = u8::MAX;    const HEAD_BIT: Self = 1u8;    const TAIL_BIT: Self = 1u8    << (Self::BITS - 1u32); const BITS: u8 = u8::BITS as u8; }
    impl BitField for u16   { const MIN: Self = u16::MIN;   const MAX: Self = u16::MAX;   const HEAD_BIT: Self = 1u16;   const TAIL_BIT: Self = 1u16   << (Self::BITS - 1u32); const BITS: u8 = u16::BITS as u8; }
    impl BitField for u32   { const MIN: Self = u32::MIN;   const MAX: Self = u32::MAX;   const HEAD_BIT: Self = 1u32;   const TAIL_BIT: Self = 1u32   << (Self::BITS - 1u32); const BITS: u8 = u32::BITS as u8; }
    impl BitField for u64   { const MIN: Self = u64::MIN;   const MAX: Self = u64::MAX;   const HEAD_BIT: Self = 1u64;   const TAIL_BIT: Self = 1u64   << (Self::BITS - 1u32); const BITS: u8 = u64::BITS as u8; }
    impl BitField for u128  { const MIN: Self = u128::MIN;  const MAX: Self = u128::MAX;  const HEAD_BIT: Self = 1u128;  const TAIL_BIT: Self = 1u128  << (Self::BITS - 1u32); const BITS: u8 = u128::BITS as u8; }
    impl BitField for usize { const MIN: Self = usize::MIN; const MAX: Self = usize::MAX; const HEAD_BIT: Self = 1usize; const TAIL_BIT: Self = 1usize << (Self::BITS - 1u32); const BITS: u8 = usize::BITS as u8; }

    impl BitField for i8    { const MIN: Self = i8::MIN;    const MAX: Self = i8::MAX;    const HEAD_BIT: Self = 1i8;    const TAIL_BIT: Self = 1i8    << (Self::BITS - 2u32); const BITS: u8 = i8::BITS as u8 - 1u8; }
    impl BitField for i16   { const MIN: Self = i16::MIN;   const MAX: Self = i16::MAX;   const HEAD_BIT: Self = 1i16;   const TAIL_BIT: Self = 1i16   << (Self::BITS - 2u32); const BITS: u8 = i16::BITS as u8 - 1u8; }
    impl BitField for i32   { const MIN: Self = i32::MIN;   const MAX: Self = i32::MAX;   const HEAD_BIT: Self = 1i32;   const TAIL_BIT: Self = 1i32   << (Self::BITS - 2u32); const BITS: u8 = i32::BITS as u8 - 1u8; }
    impl BitField for i64   { const MIN: Self = i64::MIN;   const MAX: Self = i64::MAX;   const HEAD_BIT: Self = 1i64;   const TAIL_BIT: Self = 1i64   << (Self::BITS - 2u32); const BITS: u8 = i64::BITS as u8 - 1u8; }
    impl BitField for i128  { const MIN: Self = i128::MIN;  const MAX: Self = i128::MAX;  const HEAD_BIT: Self = 1i128;  const TAIL_BIT: Self = 1i128  << (Self::BITS - 2u32); const BITS: u8 = i128::BITS as u8 - 1u8; }
    impl BitField for isize { const MIN: Self = isize::MIN; const MAX: Self = isize::MAX; const HEAD_BIT: Self = 1isize; const TAIL_BIT: Self = 1isize << (Self::BITS - 2u32); const BITS: u8 = isize::BITS as u8 - 1u8; }


    impl<T> Static for T where T:
        'static
        + Sized
        + Eq
        + Hash
        + Clone {}


    impl<T: Copy + Default> Capacity<T> for L8   { const LEN: usize = 8usize;   type Array = [T; 8usize];   fn uninit() -> Self::Array { [T::default(); 8usize] }   fn slice(array: &Self::Array) -> &[T] { array } fn slice_mut(array: &mut Self::Array) -> &mut[T] { array }}
    impl<T: Copy + Default> Capacity<T> for L16  { const LEN: usize = 16usize;  type Array = [T; 16usize];  fn uninit() -> Self::Array { [T::default(); 16usize] }  fn slice(array: &Self::Array) -> &[T] { array } fn slice_mut(array: &mut Self::Array) -> &mut[T] { array }}
    impl<T: Copy + Default> Capacity<T> for L32  { const LEN: usize = 32usize;  type Array = [T; 32usize];  fn uninit() -> Self::Array { [T::default(); 32usize] }  fn slice(array: &Self::Array) -> &[T] { array } fn slice_mut(array: &mut Self::Array) -> &mut[T] { array }}
    impl<T: Copy + Default> Capacity<T> for L64  { const LEN: usize = 64usize;  type Array = [T; 64usize];  fn uninit() -> Self::Array { [T::default(); 64usize] }  fn slice(array: &Self::Array) -> &[T] { array } fn slice_mut(array: &mut Self::Array) -> &mut[T] { array }}
    impl<T: Copy + Default> Capacity<T> for L128 { const LEN: usize = 128usize; type Array = [T; 128usize]; fn uninit() -> Self::Array { [T::default(); 128usize] } fn slice(array: &Self::Array) -> &[T] { array } fn slice_mut(array: &mut Self::Array) -> &mut[T] { array }}
    impl<T: Copy + Default> Capacity<T> for L256 { const LEN: usize = 256usize; type Array = [T; 256usize]; fn uninit() -> Self::Array { [T::default(); 256usize] } fn slice(array: &Self::Array) -> &[T] { array } fn slice_mut(array: &mut Self::Array) -> &mut[T] { array }}