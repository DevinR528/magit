//! All credit goes to https://github.com/A1-Triard/dyn-fmt

use std::fmt::{self, Display};

use indexmap::IndexMap;

/// This is the dynamic equivalent to the `format!` macro.
///
/// This macro takes a format string and any number of the same type arguments.
/// The format arguments must implement [`Display`](std::fmt::Arguments) and match
/// the number of positional arguments in the format string.
///
/// ## Examples
///
/// ```rust
/// use magit::str_fmt;
/// let mut format_string = "Some user input with braces {} {} {}";
/// assert_eq!(
///     "Some user input with braces 1 2 3",
///     str_fmt!(format_string, 1, 2, 3)
/// );
/// ```
#[macro_export]
macro_rules! str_fmt {
    ($fmt:expr, $($args:expr),+ $(,)?) => {
        format!("{}", $crate::strfmt::Arguments::new($fmt, &[ $( ( stringify!($args), $args ), )* ]))
    };
}

/// This structure represents a format string combined with its arguments.
/// In contrast with [`fmt::Arguments`](std::fmt::Arguments) this structure can be
/// easily and safely created at runtime.
#[derive(Clone, Debug)]
pub struct Arguments<'a, F: AsRef<str>, T: Copy + Display + ?Sized + 'a> {
    fmt: F,
    args: IndexMap<&'a str, T>,
}

impl<'a, F: AsRef<str>, T: Copy + Display + ?Sized + 'a> Arguments<'a, F, T> {
    #[doc(hidden)]
    pub fn new<I>(fmt: F, args: I) -> Self
    where
        I: IntoIterator<Item = &'a (&'a str, T)> + Copy,
    {
        Arguments { fmt, args: args.into_iter().copied().collect() }
    }
}

impl<'a, F: AsRef<str>, T: Copy + Display + ?Sized + 'a> Display for Arguments<'a, F, T> {
    fn fmt(&self, std_fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[derive(Debug, Eq, PartialEq)]
        enum State {
            /// Everything before and after a `{` or `}`.
            Piece,
            /// The argument passed to be formatted.
            Arg,
        }
        #[derive(Debug, Eq, PartialEq)]
        enum Brace {
            Left(usize),
            Right(usize),
        }
        impl Brace {
            fn index(&self) -> usize {
                match self {
                    Self::Left(idx) => *idx,
                    Self::Right(idx) => *idx,
                }
            }
        }

        let mut args = self.args.clone().into_iter();
        let fmt_str = self.fmt.as_ref();
        let mut braces = self
            .fmt
            .as_ref()
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| match c {
                '{' => Some(Brace::Left(idx)),
                '}' => Some(Brace::Right(idx)),
                _ => None,
            })
            .peekable();

        let mut state = State::Piece;
        let mut start = 0;
        let mut next_arg = None;

        while let Some(brace) = braces.next() {
            match state {
                State::Piece => {
                    let to = match braces.peek() {
                        Some(Brace::Left(_)) => return Err(fmt::Error),
                        Some(Brace::Right(_)) => brace.index(),
                        None => {
                            // TODO: this is an error
                            panic!("{:?} {} {:?}", state, start, brace)
                        }
                    };

                    fmt_iter(fmt_str.chars().skip(start).take(to - start), std_fmt)?;
                    state = State::Arg;

                    let arg_name = fmt_str
                        .chars()
                        .skip(to + 1)
                        .take_while(|c| *c != '}')
                        .collect::<String>();
                    if !arg_name.is_empty() {
                        next_arg = Some(arg_name);
                    }
                }
                State::Arg => {
                    if let Some(arg) =
                        next_arg.as_ref().and_then(|a| self.args.get(a.as_str()))
                    {
                        arg.fmt(std_fmt)?;
                        start = brace.index() + 1;
                        state = State::Piece;
                    } else {
                        match args.next() {
                            Some((_, arg)) => {
                                arg.fmt(std_fmt)?;

                                start = brace.index() + 1;
                                state = State::Piece;
                            }
                            None => return Err(fmt::Error),
                        }
                    }
                }
            }
        }

        fmt_iter(fmt_str.chars().skip(start), std_fmt)?;

        Ok(())
    }
}

fn fmt_iter<'a>(
    iter: impl Iterator<Item = char> + 'a,
    fmt: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    for item in iter {
        item.fmt(fmt)?
    }
    Ok(())
}
