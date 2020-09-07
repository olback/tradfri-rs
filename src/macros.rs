#[macro_export]
macro_rules! impl_from {
    ($t:ty) => {
        impl From<$t> for crate::Error {
            fn from(err: $t) -> crate::Error {
                crate::Error {
                    cause: format!("{}", err)
                }
            }
        }
    };
}
