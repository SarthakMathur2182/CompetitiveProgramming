/*
    Author: WORTH
    Problem:
*/
// Solution at end

#![allow(unused_variables)]
#![allow(unused_imports)]
use std::io::{BufWriter, StdoutLock, Write};
use std::thread;

fn main() {
    let builder = thread::Builder::new()
        .name("main".into())
        .stack_size(1 << 28);
    builder
        .spawn(|| {
            let sc = Scanner::new();
            let out = BufWriter::with_capacity(custom_io::IO_BUF_SIZE, std::io::stdout().lock());
            let mut sol = Solution::with_io(sc, out);

            loop {
                sol.solve();
                // let testcases: i32 = sol.sc.input();
                // for testcase in 1..=testcases {
                //     // dbg!("......", testcase);
                //
                //     // write!(sol.out, "Case #{}: ", testcase).unwrap();
                //     let ans = sol.solve();
                //     // writeln!(sol.out, "{}", ans).unwrap();
                //
                //     // if ans {
                //     //     writeln!(sol.out, "Yes").unwrap();
                //     // } else {
                //     //     writeln!(sol.out, "No").unwrap();
                //     // }
                //
                //     // for x in ans {
                //     //     write!(sol.out, "{} ", x).unwrap();
                //     //     writeln!(sol.out).unwrap();
                //     // }
                // }

                sol.sc.skip_whitespaces();
                if sol.sc.stored_next_byte.is_none() {
                    break;
                }
                match std::env::args().collect::<Vec<_>>().get(1) {
                    Some(val) if *val == "-DEBUG".to_string() => {
                        writeln!(sol.out, "Next Input:").unwrap();
                        dbg!("Next Input:");
                    }
                    _ => {
                        break;
                    }
                }
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

pub mod custom_io {
    use std::io::{BufRead, BufReader, StdinLock};

    /// This is used in both input and output (check the main function).
    pub const IO_BUF_SIZE: usize = 1 << 16;

    /// This is not tested on UTF-8, only ASCII.
    pub struct Scanner<'a> {
        reader: BufReader<StdinLock<'a>>,
        input_helper: String,
        pub stored_next_byte: Option<u8>,
    }
    impl<'a> Scanner<'a> {
        pub fn new() -> Self {
            let mut reader = BufReader::with_capacity(IO_BUF_SIZE, std::io::stdin().lock());
            let first_byte = reader.fill_buf().unwrap().get(0).copied();
            if first_byte.is_some() {
                reader.consume(1);
            }
            Self {
                reader,
                input_helper: String::with_capacity(32),
                stored_next_byte: first_byte,
            }
        }

        // BufReader::peek() and BufReader::has_data_left() are not available yet
        fn next_byte(&mut self) -> Option<u8> {
            let b = self.stored_next_byte;
            self.stored_next_byte = self.reader.fill_buf().unwrap().get(0).copied();
            if b.is_some() {
                self.reader.consume(1);
            }
            b
        }

        pub fn skip_whitespaces(&mut self) {
            while let Some(b) = self.stored_next_byte {
                if b.is_ascii_whitespace() {
                    self.next_byte();
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
            assert!(self.stored_next_byte.is_some(), "Input not found!");

            self.input_helper.clear();
            while let Some(c) = self.next_byte() {
                if c.is_ascii_whitespace() {
                    break;
                }
                self.input_helper.push(c as char);
            }

            self.input_helper
                .parse::<T>()
                .expect("Failed to parse input")
        }

        pub fn input_line(&mut self) -> String {
            self.skip_whitespaces();
            assert!(self.stored_next_byte.is_some(), "Input not found!");

            self.input_helper.clear();
            while let Some(c) = self.next_byte() {
                if c == b'\n' || c == b'\r' {
                    break;
                }
                self.input_helper.push(c as char);
            }
            self.input_helper.trim().to_string()
        }
    }
}
use custom_io::Scanner;

struct Solution<'io> {
    sc: Scanner<'io>,
    out: BufWriter<StdoutLock<'io>>,
}
impl<'io> Solution<'io> {
    pub fn with_io(sc: Scanner<'io>, out: BufWriter<StdoutLock<'io>>) -> Self {
        Self { sc, out }
    }

    pub fn solve(&mut self) {}
}
