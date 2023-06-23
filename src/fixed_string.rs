//#########################
// D E P E N D E N C I E S
//#########################

    use crate::Capacity;

    use std::ops::Deref;
    use std::str;
    use std::ptr::copy_nonoverlapping;
    use std::fmt::{Debug, Display};
    use std::fmt;


//#######################
// D E F I N I T I O N S
//#######################

    #[derive(Hash, PartialEq, Eq, Clone)]
    /// A fixed sized string equivalent.
    pub struct FixedStr<CAPACITY: Capacity<u8>> {
        array: CAPACITY::Array,
        len:   u8,
    } // struct FixedStr


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl<CAPACITY: Capacity<u8>> Default for FixedStr<CAPACITY> {
        fn default() -> Self {
            FixedStr {
                array: CAPACITY::uninit(),
                len:   0u8
            } // FixedStr ..
        } // fn ..
    } // impl ..


    impl<CAPACITY: Capacity<u8>> AsRef<str> for FixedStr<CAPACITY> {
        fn as_ref(&self) -> &str {
            
            &str::from_utf8(&self.as_bytes()).unwrap()

        } // fn as_ref()
    } // impl AsRef ..


    impl<CAPACITY: Capacity<u8>> Deref for FixedStr<CAPACITY> {

        type Target = str;
        fn deref(&self) -> &Self::Target { self.as_ref() }
    } // impl Deref ..


    impl<CAPACITY: Capacity<u8>> FixedStr<CAPACITY> {

        /// Converts a fixed string to an array of bytes.
        pub fn as_bytes(&self)         -> &[u8]    { &CAPACITY::slice(&self.array)[0usize..self.len as usize] }
        
        /// Converts a fixed string to an array of mutable bytes.
        pub fn as_mut_bytes(&mut self) -> &mut[u8] { &mut CAPACITY::slice_mut(&mut self.array)[0usize..self.len as usize] }


        /// Returns an unsafe fixed string from a `String` slice.
        pub unsafe fn from_str_unchecked(string: &str) -> Self {

            let mut fixed_string = FixedStr::<CAPACITY>::default();
            let count = string.len();

            assert!(
                count < CAPACITY::LEN,
                "ERROR string is out of bound ({} > {})!", count, CAPACITY::LEN,
            ); // assert()

            copy_nonoverlapping(
                string.as_ptr(),
                fixed_string.as_mut_bytes().as_mut_ptr(),
                count,
            ); // copy_nonoverlapping()

            fixed_string.len = count as u8;
            fixed_string

        } // fn from_str_unchecked()
    } // impl FixedStr


    impl<CAPACITY: Capacity<u8> + Default> From<&str> for FixedStr<CAPACITY> {
        fn from(string: &str) -> Self { unsafe { Self::from_str_unchecked(string) }}
    } // impl From ..


    impl<CAPACITY: Capacity<u8> + Default> TryFrom<&String> for FixedStr<CAPACITY> {

        type Error = String;
        fn try_from(string: &String) -> Result<Self, Self::Error> {

            let count = string.len();
            match count <= CAPACITY::LEN {
                true  => Ok(unsafe { Self::from_str_unchecked(string) }),
                false => Err(format!("ERROR string is out of bound ({} > {})!", count, CAPACITY::LEN))
            } // match ..
        } // fn from()
    } // impl From ..


    impl<CAPACITY: Capacity<u8> + Default> Display for FixedStr<CAPACITY> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.as_ref())
        } // fn fmt()
    } // impl Display ..


    impl<CAPACITY: Capacity<u8> + Default> Debug for FixedStr<CAPACITY> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FixedStr")
                .field("str", &self.as_ref())
                .field("capacity", &CAPACITY::LEN)
                .finish()
        } // fn fmt()
    } // impl Debug ..
