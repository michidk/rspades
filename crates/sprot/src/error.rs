use std::fmt;

#[derive(Debug)]
pub struct VariantError<T> {
    kind: &'static str,
    value: T,
}

impl<T> VariantError<T>
where
    T: fmt::Debug + fmt::Display,
{
    pub const fn new(kind: &'static str, value: T) -> Self {
        Self { kind, value }
    }
}

impl<T> fmt::Display for VariantError<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "No variant for enum {} and value {}",
            self.kind, self.value
        )
    }
}

impl<T> std::error::Error for VariantError<T> where Self: fmt::Debug + fmt::Display {}
