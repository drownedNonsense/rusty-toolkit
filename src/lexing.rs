//#########################
// D E P E N D E N C I E S
//#########################

    use std::error::Error;
    use std::fmt::Display;
    use std::fmt;
    use std::ops::Range;


//#######################
// D E F I N I T I O N S
//#######################

    /// Handles the conversion of a string input into a vector of tokens.
    pub struct Lexer;

    pub trait Token : Clone + Sized {
        const EOF: Self;
        const STR_LIT_KEY:      Option<char>;
        const COMMENT_KEY:      Option<char>;
        const LINE_COMMENT_KEY: Option<char>;
    } // trait ..


    pub trait ReadToken : Token {
        fn read_digit_lit(input: &str, peek: Range<usize>) -> Result<Self, LexingErr>;
        fn read_keyword(input: &str, peek: Range<usize>)   -> Result<Self, LexingErr>;
        fn read_ident(input: &str, peek: Range<usize>)     -> Self;
        fn read_str_lit(input: &str, peek: Range<usize>)   -> Self;
    } // trait ..


    impl Error for LexingErr {}
    impl Display for LexingErr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", match self {
                LexingErr::NotADigit(i)                      => format!("Failed to read digit at index `{}`!", i),
                LexingErr::NotAKeyword(i)                    => format!("Failed to read keyword at index `{}`!", i),
                LexingErr::MissingEnclosingStrLitKey(i)      => format!("Missing enclosing string literal key at index `{}`!", i),
                LexingErr::MissingEnclosingLineCommentKey(i) => format!("Missing enclosing line comment key at index `{}`!", i),
            }) // write()
        } // fn ..
    } // impl ..


    #[derive(Debug)]
    /// An enumeration of errors that may come from the lexing process.
    pub enum LexingErr {
        NotADigit(usize),
        NotAKeyword(usize),
        MissingEnclosingStrLitKey(usize),
        MissingEnclosingLineCommentKey(usize),
    } // enum ..


//###############################
// I M P L E M E N T A T I O N S
//###############################

    impl Lexer {
        /// Reads the input and outputs a vector of tokens.
        pub fn run<READER: ReadToken>(input: &str) -> Result<Vec<READER>, LexingErr> {

            let mut tokens = Vec::new();
            let     chars  = input.chars().collect::<Vec<char>>();
            let mut peek   = 0usize..1usize;


            while let Some(&char) = chars.get(peek.start) {
                match char {
                    ' ' | '\n' | '\r' => (),
                    '0'..='9'         => {
                        
                        let mut token = READER::read_digit_lit(&input, peek.clone())?;
                        while let Some(&char) = chars.get(peek.end) {

                            match char.is_ascii_digit() {
                                true  => peek.end += 1usize,
                                false => {

                                    token = READER::read_digit_lit(&input, peek.clone())?;
                                    break;

                                }, // => ..
                            } // match ..
                        } // while ..


                        tokens.push(token);

                    }, // => ..
                    _ => if Some(char) == READER::STR_LIT_KEY {


                        while let Some(&char) = chars.get(peek.end) {
                            if Some(char) == READER::STR_LIT_KEY {

                                peek.end += 2usize;
                                tokens.push(READER::read_str_lit(&input, peek.clone()));
                                break;

                            } else { peek.end += 1usize; }
                        } // while ..
                    } else if Some(char) == READER::LINE_COMMENT_KEY {


                        peek.end              += 1usize;
                        while let Some(&char)  = chars.get(peek.end) {
                            if Some(char) == READER::LINE_COMMENT_KEY {

                                peek.end += 2usize;
                                break;
                            
                            } else { peek.end += 1usize; }
                        } // while ..
                    } else if Some(char) == READER::COMMENT_KEY {


                        while let Some(&char) = chars.get(peek.end) {
                            match char {
                                '\r' | '\n' => { peek.end += 1usize; break; }
                                _           => { peek.end += 1usize; }
                            } // match ..
                        } // while ..
                    } else {

                        let mut token           = READER::read_keyword(&input, peek.clone());
                        let     first_alpha     = char.is_ascii_alphabetic();
                        let mut last_valid_peek = peek.end;

                        while let Some(&char) = chars.get(peek.end) {
                            if first_alpha {
                                if !char.is_ascii_alphabetic() && char != '_' {

                                    token = match READER::read_keyword(&input, peek.clone()) {
                                        Ok(token) => Ok(token),
                                        Err(_)    => Ok(READER::read_ident(&input, peek.clone()))
                                    }; // let ..

                                    break;
    
                                } // if ..
                            } else {
                                
                                if READER::read_keyword(&input, peek.clone()).is_ok()
                                { last_valid_peek = peek.end; }

                                if char.is_ascii_alphanumeric()
                                || char.is_ascii_whitespace()
                                || peek.end == input.len() - 1usize {

                                    peek.end = last_valid_peek;
                                    token    = READER::read_keyword(&input, peek.clone());
                                    break;
                                    
                                } // if ..
                            } // if ..


                            peek.end += 1usize;

                        } // while ..


                        tokens.push(token?);

                    } // if ..
                } // match ..


                peek.start =  peek.end;
                peek.end   += 1usize;

            } // while ..


            tokens.push(READER::EOF);
            Ok(tokens)
            
        } // fn ..
    } // impl ..
