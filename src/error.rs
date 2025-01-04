#[derive(thiserror::Error, Debug)]
#[error("Uh oh: {0}")]
pub struct Error(String);

macro_rules! convert_ssr_error {
    ($($ty:ty $(,)?)*) => {
        $(
            #[cfg(feature = "ssr")]
            impl From<$ty> for Error {
                fn from(value: $ty) -> Self {
                    Self(value.to_string())
                }
            }
        )*
    };
}
