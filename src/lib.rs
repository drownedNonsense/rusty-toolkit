//###############
// M O D U L E S
//###############

    mod bit_sequence;
    mod wrapped_conversion;
    mod rand_bit_sequence;

    pub use bit_sequence::BitSequence;
    pub use wrapped_conversion::{WrappedInto, WrappedFrom};
    pub use rand_bit_sequence::RandBitSequence;
    