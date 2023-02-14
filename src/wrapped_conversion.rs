//#########################
// D E P E N D E N C I E S
//#########################

    use std::fmt::Debug;
    use crate::BitSequence;


//#######################
// D E F I N I T I O N S
//#######################

    pub trait WrappedInto: BitSequence {

        /// Integer lossy into conversion. Discards overflowing bits.
        fn wrapped_into<T>(self) -> T
        where T: BitSequence + TryInto<Self> + TryFrom<Self>,
            <T as TryFrom<Self>>::Error: Debug,
            <T as TryInto<Self>>::Error: Debug,
            { T::try_from(self & T::try_into(T::MAX).unwrap()).unwrap() }}

            
    pub trait WrappedFrom: BitSequence {
        
        /// Integer lossy from conversion. Discards overflowing bits.
        fn wrapped_from<T>(value: T) -> Self
        where T: BitSequence + TryInto<Self> + TryFrom<Self>,
            Self: TryFrom<T>,
            <Self as TryFrom<T>>::Error: Debug,
            <Self as TryInto<T>>::Error: Debug,
            { Self::try_from(value & Self::try_into(Self::MAX).unwrap()).unwrap() }}


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl<T> WrappedInto for T where T: BitSequence {}
    impl<T> WrappedFrom for T where T: BitSequence {}
