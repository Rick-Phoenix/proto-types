use core::ops::{Deref, DerefMut};

use crate::*;

impl Deref for FieldMask {
  type Target = [String];

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.paths
  }
}

impl DerefMut for FieldMask {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.paths
  }
}

impl IntoIterator for FieldMask {
  type Item = String;
  type IntoIter = alloc::vec::IntoIter<String>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.paths.into_iter()
  }
}

impl<'a> IntoIterator for &'a FieldMask {
  type Item = &'a String;
  type IntoIter = core::slice::Iter<'a, String>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.paths.iter()
  }
}

impl FromIterator<String> for FieldMask {
  fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
    Self {
      paths: iter.into_iter().collect(),
    }
  }
}

impl FieldMask {
  /// Creates a new instance.
  #[must_use]
  #[inline]
  pub const fn new(paths: Vec<String>) -> Self {
    Self { paths }
  }

  /// Checks if a path is present in the list.
  #[must_use]
  #[inline]
  pub fn contains_path(&self, path: &str) -> bool {
    self.paths.iter().any(|p| p == path)
  }

  #[deprecated = "You can use .push() directly to leverage the DerefMut impl"]
  pub fn add_path(&mut self, path: &str) {
    self.paths.push(path.to_string());
  }
}

#[cfg(feature = "serde")]
mod serde_impls {
  use super::*;

  use core::fmt;

  use serde::{Deserialize, Serialize};

  use crate::FieldMask;
  impl Serialize for FieldMask {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer,
    {
      let joined_paths = self.paths.join(",");
      serializer.serialize_str(&joined_paths)
    }
  }

  impl<'de> Deserialize<'de> for FieldMask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'de>,
    {
      struct FieldMaskVisitor;

      impl serde::de::Visitor<'_> for FieldMaskVisitor {
        type Value = FieldMask;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
          formatter.write_str("a comma-separated string of field paths")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
          E: serde::de::Error,
        {
          if value.is_empty() {
            return Ok(FieldMask { paths: Vec::new() });
          }

          let paths: Vec<String> = value
            .split(",")
            .map(|s| s.trim().to_string())
            .collect();

          Ok(FieldMask { paths })
        }
      }

      deserializer.deserialize_str(FieldMaskVisitor)
    }
  }
}
