use serde::de::DeserializeOwned;
use serde_json::from_slice;

pub trait Extract {
    fn extract(req: &[u8]) -> Self;
}

impl<T> Extract for T
where
    T: DeserializeOwned + Default,
{
    fn extract(req: &[u8]) -> T {
        match from_slice(req) {
            Ok(v) => v,
            _ => T::default(),
        }
    }
}
