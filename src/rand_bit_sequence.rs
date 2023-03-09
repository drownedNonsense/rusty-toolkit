//#########################
// D E P E N D E N C I E S
//#########################

    use crate::{BitField, WrappingFrom};

    use std::ops::{RangeBounds, Bound, Add, Sub, Rem};


//#######################
// D E F I N I T I O N S
//#######################

    /// A thread of pseudo-random bit sequences.
    pub struct RandBitSequence<B: BitField> {
        sequence: (B, B),
        index:    u8,
    } // struct RandBitSequence


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl<B: BitField> RandBitSequence<B> {

        /// Instantiates a new RBS thread.
        pub fn new(seed: (B, B)) -> Self {
            RandBitSequence {
                sequence: seed,
                index:    0u8,
            } // RandBitSequence
        } // fn new()


        /// Generates a pseudo-random bit sequence.
        pub fn generate_raw(&mut self) -> B {

            let a =  self.sequence.0 & B::TAIL_BIT;
            let b = (self.sequence.0 & B::HEAD_BIT << self.index) >> self.index;
            let c = (self.sequence.1 << 2u8) >> 2u8;

            self.index      = (self.index + 1u8) & (B::BITS);
            self.sequence.0 = (((self.sequence.0 << 1u8) ^ a) ^ b) ^ c;
            self.sequence.1 = (((self.sequence.0 >> 1u8) ^ a) ^ b) ^ c;

            self.sequence.0

        } // fn generate_raw()


        /// Generates a pseudo-random bit sequence as a `T` bit field.
        pub fn generate<T>(&mut self) -> T
        where T: WrappingFrom<B>, B: Sub<B, Output=B> + TryFrom<T> { T::wrapping_from(self.generate_raw()) }


        /// Generates a pseudo-random bit sequence as a `T` integer inside `range` bounds.
        pub fn generate_irange<T: BitField>(&mut self, range: impl RangeBounds<T>) -> T
        where T: WrappingFrom<B> + Add<T, Output=T> + Sub<T, Output=T> + Rem<T, Output=T>,
              B: Sub<B, Output=B> + TryFrom<T>  {
            
            let start = match range.start_bound() {
                Bound::Included(start) => *start,
                Bound::Excluded(start) => *start + T::HEAD_BIT,
                Bound::Unbounded       =>  T::MIN,
            }; // let ..
            let end = match range.end_bound() {
                Bound::Included(end) => *end,
                Bound::Excluded(end) => *end - T::HEAD_BIT,
                Bound::Unbounded     =>  T::MAX,
            }; // let ..
            
            start + self.generate() % (end + T::HEAD_BIT - start)

        } // fn generate_irange()
    } // impl RandBitSequence


    impl<B: BitField> Default for RandBitSequence<B>
    where B: WrappingFrom<u128>, u128: From<B> {
        fn default() -> Self {
            RandBitSequence::new((
                B::MAX,
                B::wrapping_from(
                    std::time::SystemTime::now()
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .expect("Could not get UNIX time!")
                        .as_millis()) // wrapping_from()
            )) // new()
        } // fn default()
    } // impl Default ..
