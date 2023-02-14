//#########################
// D E P E N D E N C I E S
//#########################

    use std::fmt::Debug;
    use crate::{BitSequence, WrappedInto};


//#######################
// D E F I N I T I O N S
//#######################

    pub struct RandBitSequence<B: BitSequence> {
        sequence: (B, B),
        index:    u32,
    } // struct RandBitSequence


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl<B: BitSequence> RandBitSequence<B> {

        /// Instantiates a new RBS thread.
        pub fn new(seed: (B, B)) -> Self {
            RandBitSequence {
                sequence: seed,
                index:    0u32,
            } // RandBitSequence
        } // fn new()


        /// Generates a pseudo random bit sequence.
        pub fn generate(&mut self) -> B {

            let a =  self.sequence.0 & B::HEAD_BIT;
            let b = (self.sequence.0 & B::HEAD_BIT << self.index) >> self.index;
            let c = (self.sequence.1 << 2u32) >> 2u32;

            self.index      = (self.index + 1u32) & (B::BITS - 1u32);
            self.sequence.0 = (((self.sequence.0 << 1u32) ^ a) ^ b) ^ c;
            self.sequence.1 = (((self.sequence.0 >> 1u32) ^ a) ^ b) ^ c;

            self.sequence.0

        } // fn generate()
    } // impl RandBitSequence


    impl<B: BitSequence> Default for RandBitSequence<B>
    where B: TryFrom<u128> + TryInto<u128>,
          <B as TryFrom<u128>>::Error: Debug,
          <B as TryInto<u128>>::Error: Debug {
        fn default() -> Self {
            RandBitSequence::new(
                (
                    B::MAX,
                    std::time::SystemTime::now()
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .expect("Could not get UNIX time!")
                        .as_millis().wrapped_into::<B>()
                )
            ) // new()
        } // fn default()
    } // impl Default ..
