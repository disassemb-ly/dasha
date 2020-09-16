use std::fmt;

#[derive(Clone, Copy)]
pub enum Format {
    Att,
}

pub trait DisplayFormat: Sized {
    fn fmt(&self, fmt: Format, f: &mut fmt::Formatter) -> fmt::Result;

    fn display(self, fmt: Format) -> Display<Self> {
        Display(self, fmt)
    }
}

pub struct Display<T>(T, Format)
where
    T: DisplayFormat;

impl<T> fmt::Display for Display<T>
where
    T: DisplayFormat,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(self.1, f)
    }
}
