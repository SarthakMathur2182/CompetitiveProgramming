/*
I've tried to write this template on my own, so don't expect much performance from this.
Maybe I'll work more on this in the future.

You can read these discussions to get some idea: https://codeforces.com/blog/entry/144590
*/

#[allow(unused_imports)]
use std::io::{BufRead, BufReader, BufWriter, StdinLock, StdoutLock, Write};

/// - This is not tested on UTF-8, only ASCII.
/// - This implementation is not yet safe in case of no input found exception.
/// So please follow the input format.
pub struct Scanner<'a> {
    reader: BufReader<StdinLock<'a>>,
    input_helper: String,
}
impl<'a> Scanner<'a> {
    /// This is used in both input and output (check the main function).
    const IO_BUF_SIZE: usize = 1 << 16;

    pub fn new() -> Self {
        Self {
            reader: BufReader::with_capacity(Scanner::IO_BUF_SIZE, std::io::stdin().lock()),
            input_helper: String::with_capacity(32),
        }
    }

    // BufReader::peek() and BufReader::has_data_left() are not available yet
    fn peek_next_byte(&mut self) -> u8 {
        unsafe { *self.reader.fill_buf().unwrap().get_unchecked(0) }
    }

    fn next_byte(&mut self) -> u8 {
        let b = self.peek_next_byte();
        self.reader.consume(1);
        b
    }

    fn skip_whitespaces(&mut self) {
        loop {
            if self.peek_next_byte().is_ascii_whitespace() {
                self.reader.consume(1);
            } else {
                break;
            }
        }
    }

    pub fn input<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.skip_whitespaces();
        self.input_helper.clear();
        loop {
            let c = self.next_byte() as char;
            if c.is_ascii_whitespace() {
                return self.input_helper.parse::<T>().unwrap();
            }
            self.input_helper.push(c);
        }
    }

    pub fn input_line(&mut self) -> String {
        self.skip_whitespaces();
        self.input_helper.clear();
        loop {
            let c = self.next_byte() as char;
            if c == '\n' {
                return self.input_helper.trim().to_string();
            }
            self.input_helper.push(c);
        }
    }
}

fn solve(sc: &mut Scanner, out: &mut BufWriter<StdoutLock>) {}

fn main() {
    let mut sc = Scanner::new();
    let mut out = BufWriter::with_capacity(Scanner::IO_BUF_SIZE, std::io::stdout().lock());

    solve(&mut sc, &mut out);

    // let testcases: i32 = sc.input();
    // for testcase in 1..=testcases {
    //     eprintln!("Testcase: {testcase}");
    //     solve(&mut sc, &mut out);
    // }
}
