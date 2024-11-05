use super::token::Token;

#[derive(Debug, Clone, PartialEq)]
/// Errors the tokenizer can yield.
pub enum TokenizerError {
    /// The tokenizer could not read the associated sequence.
    UnmatchedSequence(String),
}

/// A reader used to wrap the `TokenStream`.
/// When reading, it starts with the staging buffer of the stream, once
/// it's end is reached, the input stream is copied character wise to
/// the staging buffer.
struct StagingReader<'a, I> {
    head: usize,
    stream: &'a mut TokenStream<I>,
}

impl<'a, I> StagingReader<'a, I>
where
    I: Iterator<Item = char>,
{
    /// Create a new StagingReader for a stream.
    fn new(stream: &'a mut TokenStream<I>) -> StagingReader<'a, I> {
        StagingReader { head: 0, stream }
    }
    /// Step back the reader's head by `n` chars, stopping at 0
    fn step_back(&mut self, n: usize) {
        if self.head >= n {
            self.head -= n;
        }
    }
}

impl<'a, I> Iterator for StagingReader<'a, I>
where
    I: Iterator<Item = char>,
{
    type Item = char;

    /// Get the char at `self.head`. If it is in the staging buffer, return it and increase `self.head` by 1.
    /// It it is not in the staging buffer, copy one char from the input stream to the staging buffer.
    /// Returns `None` when the input stream is empty and `self.head` points after the staging buffer.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.stream.staging.get(self.head) {
            self.head += 1;
            Some(*c)
        } else {
            let next_char = self.stream.input.next()?;
            self.stream.staging.push(next_char);
            self.head += 1;
            Some(next_char)
        }
    }
}

/// An iterator yielding tokens scanned from a stream of characters.
pub struct TokenStream<InputStream> {
    staging: Vec<char>,
    input: InputStream,
    error: bool,
}

impl<I> TokenStream<I>
where
    I: Iterator<Item = char>,
{
    fn new(input: I) -> TokenStream<I> {
        TokenStream {
            staging: Vec::new(),
            input,
            error: false,
        }
    }

    fn skip_whitespace(&mut self) {
        // Drop whitespace of the staging buffer
        while let Some(c) = self.staging.first() {
            if c.is_whitespace() {
                self.staging.remove(0);
            } else {
                return; // Readable character next, keep input untouched
            }
        }

        // Staging buffer is empty, drop whitespace from input
        while let Some(c) = self.input.next() {
            if !c.is_whitespace() {
                self.staging.push(c);
                return;
            }
        }
    }

    fn run_scanners(&mut self) -> Option<(Token, usize)> {
        let scanners = [
            scan_symbol,
            scan_string_literal,
            scan_integer,
            scan_float,
            scan_true,
            scan_quote,
            scan_dot,
            scan_nil,
            scan_par_close,
            scan_par_open,
        ];

        scanners
            .iter()
            .filter_map(|scanner| {
                let mut reader = StagingReader::new(self);
                let token = scanner(&mut reader)?;
                Some((token, reader.head))
            })
            .max_by_key(|pair| pair.1)
    }
}

impl<I> Iterator for TokenStream<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Token, TokenizerError>;

    /// Get the next scanned token, consuming as much characters from the
    /// wrapped input stream as neccessary. If nothing could be scanned and the input
    /// stream has still elements an error is returned. Each successive call to
    /// `next` will then return `None`.
    fn next(&mut self) -> Option<Self::Item> {
        if self.error {
            return None;
        }

        self.skip_whitespace();

        match self.run_scanners() {
            Some((tkn, n_read)) => {
                self.staging.drain(0..n_read);
                Some(Ok(tkn))
            }
            None if self.staging.is_empty() => None,
            None => {
                let remaining = self.staging.iter().collect();
                self.staging.clear();
                self.error = true;
                Some(Err(TokenizerError::UnmatchedSequence(remaining)))
            }
        }
    }
}

/// Run the tokenizer on an iterator of chars and return an
/// iterator of tokens as a result.
pub fn tokenize<I>(input: I) -> TokenStream<I>
where
    I: Iterator<Item = char>,
{
    TokenStream::new(input)
}

// ================== Scanner definitions ================== //

fn scan_par_open<I>(reader: &mut StagingReader<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    match reader.next()? {
        '(' => Some(Token::ParOpen),
        _ => {
            reader.step_back(1);
            None
        }
    }
}

fn scan_par_close<I>(reader: &mut StagingReader<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    match reader.next()? {
        ')' => Some(Token::ParClose),
        _ => {
            reader.step_back(1);
            None
        }
    }
}

fn scan_dot<I>(reader: &mut StagingReader<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    match reader.next()? {
        '.' => Some(Token::Dot),
        _ => {
            reader.step_back(1);
            None
        }
    }
}

fn scan_string_literal<I>(reader: &mut StagingReader<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    let mut lit = String::new();

    if reader.next()? == '"' {
        for c in reader {
            match c {
                '"' => {
                    return Some(Token::StringLiteral(lit));
                }
                c => {
                    lit.push(c);
                }
            }
        }
    }

    return None;
}

fn scan_nil<I>(reader: &mut StagingReader<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    if reader.next()? == 'n' && reader.next()? == 'i' && reader.next()? == 'l' {
        Some(Token::Nil)
    } else {
        reader.step_back(3);
        None
    }
}

fn scan_quote<I>(reader: &mut StagingReader<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    if let Some('\'') = reader.next() {
        Some(Token::Quote)
    } else {
        reader.step_back(1);
        None
    }
}

fn scan_symbol<I>(reader: &mut StagingReader<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    let mut sym = String::new();

    // Allow some special chars and alphanumeric
    while let Some(c) = reader.next() {
        match c {
            '_' | '-' | '<' | '>' | '=' | '*' | '/' | '+' | '%' | '!' | '?' => sym.push(c),
            c if c.is_ascii_alphanumeric() => sym.push(c),
            _ => {
                reader.step_back(1);
                break;
            }
        }
    }

    if sym.len() > 0 {
        Some(Token::Symbol(sym))
    } else {
        None
    }
}

fn scan_true<I>(reader: &mut StagingReader<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    if reader.next()? == 't'
        && reader.next()? == 'r'
        && reader.next()? == 'u'
        && reader.next()? == 'e'
    {
        Some(Token::True)
    } else {
        reader.step_back(4);
        None
    }
}

fn scan_integer<I>(reader: &mut StagingReader<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    let mut buf = String::new();

    while let Some(c) = reader.next() {
        if c.is_ascii_digit() {
            buf.push(c);
        } else {
            reader.step_back(1);
            break;
        }
    }

    if buf.len() > 0 {
        buf.parse().map(Token::IntLiteral).ok()
    } else {
        None
    }
}

fn scan_float<I>(reader: &mut StagingReader<I>) -> Option<Token>
where
    I: Iterator<Item = char>,
{
    let mut buf = String::new();
    let mut has_dot = false;

    while let Some(c) = reader.next() {
        if c.is_ascii_digit() {
            buf.push(c);
        } else if c == '.' && !has_dot {
            buf.push(c);
            has_dot = true;
        } else {
            reader.step_back(1);
            break;
        }
    }

    if buf.len() > 0 && has_dot {
        buf.parse().map(Token::FloatLiteral).ok()
    } else {
        None
    }
}

#[test]
fn test_tokenize() {
    let test_str = "(\"abcdefg( )123\" )(\n\t 'nil true \"true\")00987463 123.125 . 0+-*/go=";

    let result: Vec<_> = tokenize(&mut test_str.chars()).collect();

    assert_eq!(result.len(), 13);
    assert_eq!(result[0].clone().unwrap(), Token::ParOpen);
    assert_eq!(
        result[1].clone().unwrap(),
        Token::StringLiteral(String::from("abcdefg( )123"))
    );
    assert_eq!(result[2].clone().unwrap(), Token::ParClose);
    assert_eq!(result[3].clone().unwrap(), Token::ParOpen);
    assert_eq!(result[4].clone().unwrap(), Token::Quote);
    assert_eq!(result[5].clone().unwrap(), Token::Nil);
    assert_eq!(result[6].clone().unwrap(), Token::True);
    assert_eq!(
        result[7].clone().unwrap(),
        Token::StringLiteral(String::from("true"))
    );
    assert_eq!(result[8].clone().unwrap(), Token::ParClose);
    assert_eq!(result[9].clone().unwrap(), Token::IntLiteral(987463));
    assert_eq!(result[10].clone().unwrap(), Token::FloatLiteral(123.125));
    assert_eq!(result[11].clone().unwrap(), Token::Dot);
    assert_eq!(
        result[12].clone().unwrap(),
        Token::Symbol("0+-*/go=".to_string())
    );
}
