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
        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.value, f)
            }
        }

        impl std::fmt::LowerHex for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::LowerHex::fmt(&self.value, f)
            }
        }

        impl std::fmt::UpperHex for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::UpperHex::fmt(&self.value, f)
            }
        }

        impl std::fmt::Binary for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Binary::fmt(&self.value, f)
            }
        }
    };
}
