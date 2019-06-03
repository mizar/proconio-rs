use super::Source;
use std::io::BufRead;
use std::iter::Peekable;
use std::str::SplitWhitespace;

pub struct LineSource<R: BufRead> {
    // FIXME: This is actually not 'static but it is treated as 'static for the
    // same reason with crate::source::once::Source.  Also there is no way to
    // separate context and tokens since they are private field, this is safe.
    tokens: Peekable<SplitWhitespace<'static>>,

    current_context: Box<str>,

    reader: R,
}

impl<R: BufRead> LineSource<R> {
    pub fn new(reader: R) -> LineSource<R> {
        // dummy values.
        LineSource {
            current_context: "".to_string().into_boxed_str(),
            tokens: "".split_whitespace().peekable(),
            reader,
        }
    }
}

impl<R: BufRead> Source<R> for LineSource<R> {
    fn next_token(&mut self) -> Option<&str> {
        while self.tokens.peek().is_none() {
            let mut line = String::new();
            self.reader
                .read_line(&mut line)
                .expect("failed to get line");
            self.current_context = line.into_boxed_str();
            self.tokens = unsafe { std::mem::transmute::<_, &'static str>(&*self.current_context) }
                .split_whitespace()
                .peekable();
        }

        self.tokens.next()
    }
}

use std::io::BufReader;
impl<'a> From<&'a str> for LineSource<BufReader<&'a [u8]>> {
    fn from(s: &'a str) -> LineSource<BufReader<&'a [u8]>> {
        LineSource::new(BufReader::new(s.as_bytes()))
    }
}
