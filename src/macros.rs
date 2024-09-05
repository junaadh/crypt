/// # impl_pretty_print
///
/// * macro for implementing pretty print
/// * implements the needed traits for printing
/// * binary and hex printing traits are also implemented
/// * leverages the underlying primitives implementation
///
#[macro_export]
macro_rules! impl_pretty_print {
    ($ty: ty) => {
        use std::fmt;

        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.value, f)
            }
        }

        impl fmt::LowerHex for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::LowerHex::fmt(&self.value, f)
            }
        }

        impl fmt::UpperHex for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::UpperHex::fmt(&self.value, f)
            }
        }

        impl fmt::Binary for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Binary::fmt(&self.value, f)
            }
        }
    };
}
