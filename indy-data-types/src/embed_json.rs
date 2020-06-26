use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::marker::PhantomData;
use std::ops;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "serde")]
use serde_json::value::RawValue;

#[cfg(feature = "serde")]
pub trait EmbedExtract {
    type Inner: Serialize + for<'de> Deserialize<'de>;
}

pub struct EmbedJson<T> {
    _pd: PhantomData<T>,
    pub inner: String,
}

impl<T> EmbedJson<T> {
    pub fn from_string(inner: String) -> Self {
        Self {
            _pd: PhantomData,
            inner,
        }
    }
}

impl<T> Clone for EmbedJson<T> {
    fn clone(&self) -> Self {
        Self::from_string(self.inner.clone())
    }
}

impl<T> ops::Deref for EmbedJson<T> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> fmt::Debug for EmbedJson<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("EmbedJson")
            .field(&format_args!("{}", &self.inner))
            .finish()
    }
}

impl<T> fmt::Display for EmbedJson<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.inner)
    }
}

impl<T> PartialEq<EmbedJson<T>> for EmbedJson<T> {
    fn eq(&self, other: &EmbedJson<T>) -> bool {
        self.inner == other.inner
    }
}

impl<T> Eq for EmbedJson<T> {}

impl<T> PartialOrd for EmbedJson<T> {
    fn partial_cmp(&self, other: &EmbedJson<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for EmbedJson<T> {
    fn cmp(&self, other: &EmbedJson<T>) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

#[cfg(feature = "serde")]
impl<T: EmbedExtract> EmbedJson<T> {
    pub fn embed(value: &T::Inner) -> Result<Self, crate::ConversionError> {
        Ok(serde_json::to_string(value).map(Self::from_string)?)
    }

    pub fn extract(&self) -> Result<T::Inner, crate::ConversionError> {
        Ok(serde_json::from_str(&self.inner)?)
    }
}

#[cfg(feature = "serde")]
pub fn embed_json<T: EmbedExtract>(
    value: &T::Inner,
) -> Result<EmbedJson<T>, crate::ConversionError> {
    EmbedJson::embed(value)
}

#[cfg(feature = "serde")]
impl<T> Serialize for EmbedJson<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let raw = unsafe { std::mem::transmute::<&str, &RawValue>(&self.inner) };
        raw.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for EmbedJson<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let rawval = <&RawValue>::deserialize(deserializer)?;
        Ok(EmbedJson::from_string(rawval.get().to_string()))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "serde")]
    use super::*;

    #[cfg(feature = "serde")]
    #[test]
    fn embed_round_trip() {
        #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct V {
            val: i32,
        }

        impl EmbedExtract for V {
            type Inner = Self;
        }

        #[derive(Serialize, Deserialize)]
        struct TestEmbed {
            a: i32,
            v: EmbedJson<V>,
        }

        let json = r#"{"a": 10, "v": {"val":  5}}"#;
        let test = serde_json::from_str::<TestEmbed>(json).unwrap();
        assert_eq!(test.v.inner, r#"{"val":  5}"#);

        let inner = test.v.extract().unwrap();
        let cmp = V { val: 5 };
        assert_eq!(cmp, inner);

        let embed = embed_json::<V>(&inner).unwrap();
        assert_eq!(embed.inner, r#"{"val":5}"#);

        let reenc = serde_json::to_string(&test).unwrap();
        assert_eq!(reenc, r#"{"a":10,"v":{"val":  5}}"#);
    }
}