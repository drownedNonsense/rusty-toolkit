//#########################
// D E P E N D E N C I E S
//#########################

    use std::{error::Error, fmt::Display};
    use std::fmt;


//#######################
// D E F I N I T I O N S
//#######################

    /// Handles the conversion of a string input into a vector of tokens.
    pub struct Lexer;

    pub trait Token : Sized {
        const EOF: Self;
        const STR_LIT_KEY:      Option<char>;
        const COMMENT_KEY:      Option<char>;
        const LINE_COMMENT_KEY: Option<char>;
    } // trait ..

    pub trait ReadToken : Token {
        fn read_digit_lit(value: &str) -> Result<Self, LexingErr>;
        fn read_str_lit(value: &str)   -> Self;
        fn read_ident(value: &str)     -> Self;
    } // trait ..

    impl Error for LexingErr {}
    impl Display for LexingErr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        } // fn ..
    } // impl ..

    #[derive(Debug)]
    pub enum LexingErr {
        NotADigit,
        MissingEnclosingStrLitKey,
        MissingEnclosingLineCommentKey,
    } // enum ..


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl Lexer {
        /// Reads the input and outputs a vector of tokens.
        pub fn run<READER: ReadToken>(input: &str) -> Result<Vec<READER>, LexingErr> {

            let mut tokens = Vec::new();
            let mut input  = input.chars().peekable();

            while let Some(char) = input.next() {
                match char {
                    ' '       => (),
                    '0'..='9' => {

                        let mut number = String::new();
                                number.push(char);

                        while let Some(&char) = input.peek() {
                            match char.is_ascii_digit() {
                                true  => number.push(input.next().unwrap()),
                                false => break,
                            } // match ..
                        } // while ..


                    tokens.push(READER::read_digit_lit(&number)?);

                    } _ => {

                        if Some(char) == READER::STR_LIT_KEY {

                            let mut string = String::new();
                                    string.push(input.next().unwrap());
    
                            while let Some(&char) = input.peek() {
                                if Some(char) == READER::STR_LIT_KEY
                                 { input.next(); input.next(); break; }
                                else
                                 { string.push(input.next().unwrap()); }
                            } // while ..
    
    
                            tokens.push(READER::read_str_lit(&string));
    
                        } else if Some(char) == READER::LINE_COMMENT_KEY {

                            input.next();
                            while let Some(&char) = input.peek() {
                                if Some(char) == READER::LINE_COMMENT_KEY
                                 { input.next(); input.next(); break; }
                                else
                                 { input.next(); }
                            } // while ..
                        } else if Some(char) == READER::COMMENT_KEY {
                            while let Some(&char) = input.peek() {
                                match char {
                                    '\r' | '\n' => { input.next(); break; }
                                    _           => { input.next(); }
                                } // match ..
                            } // while ..
                        } else {
    
                            let mut identifier = String::new();
                                    identifier.push(char);
    
                            while let Some(&char) = input.peek() {

                                if char.is_ascii_alphanumeric() || char == '_'
                                 { identifier.push(input.next().unwrap()); }
                                else
                                 { break; }
                            } // while ..
    
    
                            tokens.push(READER::read_ident(&identifier));
    
                        } // if ..
                    } // => ..
                } // match ..
            } // while ..

            tokens.push(READER::EOF);
            Ok(tokens)
            
        } // fn ..
    } // impl ..
