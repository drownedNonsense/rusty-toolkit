//###############
// M O D U L E S
//###############

    mod bit_field;
    mod fixed_string;
    mod wrapping_conversion;
    mod rand_bit_sequence;
    mod lexing;

    pub use bit_field::{BitField, Capacity, L8, L16, L32, L64, L128, L256};
    pub use fixed_string::FixedStr;
    pub use wrapping_conversion::{WrappingInto, WrappingFrom};
    pub use rand_bit_sequence::RandBitSequence;
    