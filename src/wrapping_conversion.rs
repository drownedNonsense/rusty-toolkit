//#########################
// D E P E N D E N C I E S
//#########################

    use crate::BitField;
    
    use std::ops::Sub;


//#######################
// D E F I N I T I O N S
//#######################
            
    /// A lossy into conversion between two differently sized bit fields.
    pub trait WrappingInto<INTO: BitField>: BitField
    where Self: Sub<Self, Output=Self>, INTO: TryFrom<Self> {
        
        /// Integer lossy into conversion. Discards overflowing bits.
        fn wrapping_into(self) -> INTO {
            match INTO::BITS > Self::BITS {
                true  => unsafe { INTO::try_from(self).unwrap_unchecked() },
                false => unsafe { INTO::try_from(self & Self::n_bits(INTO::BITS)).unwrap_unchecked() }
            } // match ..
        } // fn wrapping_into()
    } // trait WrappingInto


    /// A lossy from conversion between two differently sized bit fields.
    pub trait WrappingFrom<FROM>: BitField
    where FROM: WrappingInto<Self>, Self: TryFrom<FROM> {
        
        /// Integer lossy from conversion. Discards overflowing bits.
        fn wrapping_from(value: FROM) -> Self { FROM::wrapping_into(value) }
    } // trait WrappingFrom


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl<FROM, INTO> WrappingFrom<FROM> for INTO where FROM: BitField + Sub<FROM, Output=FROM> + TryFrom<INTO>, INTO: WrappingInto<FROM> + TryFrom<FROM> {}
    impl<FROM, INTO> WrappingInto<INTO> for FROM where FROM: BitField + Sub<FROM, Output=FROM> + TryFrom<INTO>, INTO: BitField + TryFrom<FROM> {}
