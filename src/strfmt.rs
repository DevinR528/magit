//! All credit goes to https://github.com/A1-Triard/dyn-fmt

use std::fmt::{self, Display};

/// This is the dynamic equivalent to the `format!` macro.
///
/// This macro takes a format string and any number of the same type arguments.
/// The format arguments must implement [`Display`](std::fmt::Arguments) and match the
/// number of positional arguments in the format string.
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
    ($fmt:expr, $( $args:expr ),*) => {
        format!("{}", $crate::strfmt::Arguments::new($fmt, &[ $( $args ),* ]))
    };
}

/// This structure represents a format string combined with its arguments.
/// In contrast with [`fmt::Arguments`](std::fmt::Arguments) this structure can be easily
/// and safely created at runtime.
#[derive(Clone, Debug)]
pub struct Arguments<
    'a,
    F: AsRef<str>,
    T: Display + ?Sized + 'a,
    I: IntoIterator<Item = &'a T>,
> {
    fmt: F,
    args: I,
}

impl<'a, F: AsRef<str>, T: Display + ?Sized + 'a, I: IntoIterator<Item = &'a T> + Copy>
    Arguments<'a, F, T, I>
{
    /// Creates a new instance of a [`Display`] able structure,
    /// representing formatted arguments. A runtime analog of
    /// [`format_args!`](std::format_args) macro. Extra arguments are ignored, missing
    /// arguments are replaced by empty string. # Examples:
    /// ```rust
    /// magit::strfmt::Arguments::new("{}a{}b{}c", &[1, 2, 3]); // "1a2b3c"
    /// magit::strfmt::Arguments::new("{}a{}b{}c", &[1, 2, 3, 4]); // "1a2b3c"
    /// magit::strfmt::Arguments::new("{}a{}b{}c", &[1, 2]); // "1a2bc"
    /// magit::strfmt::Arguments::new("{{}}{}", &[1, 2]); // Error! braces cannot be used at all
    /// ```
    pub fn new(fmt: F, args: I) -> Self { Arguments { fmt, args } }
}

impl<'a, F: AsRef<str>, T: Display + ?Sized + 'a, I: IntoIterator<Item = &'a T> + Copy>
    Display for Arguments<'a, F, T, I>
{
    fn fmt(&self, std_fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[derive(Debug, Eq, PartialEq)]
        enum State {
            Piece,
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

        let mut args = self.args.into_iter();
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
                }
                State::Arg => match args.next() {
                    Some(arg) => {
                        arg.fmt(std_fmt)?;

                        start = brace.index() + 1;
                        state = State::Piece;
                    }
                    None => return Err(fmt::Error),
                },
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
