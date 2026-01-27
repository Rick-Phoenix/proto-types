use alloc::{vec, vec::IntoIter};

use prost::Message;

use crate::{
  Any, Code, Status, String, ToString, Vec,
  protovalidate::{FieldPath, FieldPathElement, Violation, Violations},
};

impl FromIterator<Violation> for Violations {
  fn from_iter<T: IntoIterator<Item = Violation>>(iter: T) -> Self {
    Self {
      violations: iter.into_iter().collect(),
    }
  }
}

impl IntoIterator for Violations {
  type Item = Violation;
  type IntoIter = IntoIter<Violation>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.violations.into_iter()
  }
}

impl core::ops::Deref for Violations {
  type Target = Vec<Violation>;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.violations
  }
}

impl core::ops::DerefMut for Violations {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.violations
  }
}

impl Extend<Violation> for Violations {
  #[inline]
  fn extend<T: IntoIterator<Item = Violation>>(&mut self, iter: T) {
    self.violations.extend(iter)
  }
}

impl IntoIterator for FieldPath {
  type Item = FieldPathElement;
  type IntoIter = IntoIter<FieldPathElement>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.elements.into_iter()
  }
}

impl core::ops::Deref for FieldPath {
  type Target = Vec<FieldPathElement>;
  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.elements
  }
}

impl core::ops::DerefMut for FieldPath {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.elements
  }
}

impl FromIterator<FieldPathElement> for FieldPath {
  fn from_iter<T: IntoIterator<Item = FieldPathElement>>(iter: T) -> Self {
    Self {
      elements: iter.into_iter().collect(),
    }
  }
}

impl Extend<FieldPathElement> for FieldPath {
  #[inline]
  fn extend<T: IntoIterator<Item = FieldPathElement>>(&mut self, iter: T) {
    self.elements.extend(iter)
  }
}

impl FieldPath {
  #[deprecated = "With the Deref impl, you can just use .last()"]
  /// Returns the last member in the elements list, if the list is not empty.
  #[must_use]
  #[inline]
  pub fn last_field(&self) -> Option<&FieldPathElement> {
    self.elements.last()
  }

  /// Returns the second last member in the elements list, if the list is not empty.
  #[must_use]
  #[inline]
  pub fn parent_field(&self) -> Option<&FieldPathElement> {
    let second_last = self
      .elements
      .get(self.elements.len().wrapping_sub(2));

    match second_last {
      Some(el) => Some(el),
      None => None,
    }
  }

  /// Checks if the elements list is empty or not.
  #[must_use]
  #[inline]
  #[deprecated = "With the Deref impl, you can just use !.is_empty()"]
  pub const fn has_fields(&self) -> bool {
    !self.elements.is_empty()
  }

  /// Returns the name of the last member in the elements list, if there is one.
  #[must_use]
  #[inline]
  pub fn last_field_name(&self) -> Option<&str> {
    self.elements.last().map(|f| f.field_name())
  }

  /// Searches for a FieldPathElement by name in the elements list.
  #[must_use]
  #[inline]
  pub fn get_field(&self, name: &str) -> Option<&FieldPathElement> {
    self
      .elements
      .iter()
      .find(|&field| field.field_name() == name)
  }

  /// Returns a vector with the names from each path element (including any eventual Subscript like a vector index or map key)
  /// (e.g. `["person", "friends", "0", "address","street_name"]`)
  #[must_use]
  pub fn field_path(&self) -> Vec<String> {
    let mut path: Vec<String> = Vec::new();

    for field in self.elements.iter() {
      if let Some(field_name) = field.field_name.as_ref() {
        path.push(field_name.clone());
      }

      if let Some(key) = &field.subscript {
        path.push(key.to_string());
      }
    }

    path
  }

  /// Returns all of the names from each path element (including any eventual Subscript like a vector index or map key), joined by a dot (e.g. `person.friends.0.address.street_name`)
  #[must_use]
  pub fn field_path_str(&self) -> String {
    self.field_path().join(".")
  }
}

impl Violations {
  /// Creates a new collection of Violations with the specified initial capacity.
  #[must_use]
  #[inline]
  pub fn with_capacity(capacity: usize) -> Self {
    let violations = Vec::with_capacity(capacity);
    Self { violations }
  }

  /// Creates a new empty collection of Violations.
  #[must_use]
  #[inline]
  pub const fn new() -> Self {
    Self { violations: vec![] }
  }

  /// Searches for a violation with a specific rule id.
  #[must_use]
  #[inline]
  pub fn violation_by_rule_id(&self, rule_id: &str) -> Option<&Violation> {
    self
      .violations
      .iter()
      .find(|v| v.rule_id() == rule_id)
  }

  /// Searches for a violation with a specific `field_path` string.
  ///
  /// Keep in mind the `field_path` will include Subscripts like vector indexes or map keys.
  ///
  /// # Examples
  /// ```rust
  /// use proto_types::protovalidate::{FieldPath, FieldPathElement, Violation, Violations};
  ///
  /// let violations = Violations {
  ///    violations: vec![Violation {
  ///      field: Some(FieldPath {
  ///        elements: vec![
  ///          FieldPathElement {
  ///            field_name: Some("person".to_string()),
  ///            ..Default::default()
  ///          },
  ///          FieldPathElement {
  ///            field_name: Some("name".to_string()),
  ///            ..Default::default()
  ///          },
  ///        ],
  ///      }),
  ///      ..Default::default()
  ///    }],
  ///  };
  ///  assert!(violations.violation_by_field_path("person.name").is_some());
  /// ```
  #[must_use]
  #[inline]
  pub fn violation_by_field_path(&self, path: &str) -> Option<&Violation> {
    self.violations.iter().find(|v| {
      v.field
        .as_ref()
        .is_some_and(|vi| vi.field_path_str() == path)
    })
  }
}

impl Violation {
  /// Returns the last member in the elements list, if there is one.
  #[must_use]
  #[inline]
  pub fn last_field(&self) -> Option<&FieldPathElement> {
    self.field.as_ref().and_then(|f| f.last())
  }

  /// Returns the second last member in the elements list, if there is one.
  #[must_use]
  #[inline]
  pub fn parent_field(&self) -> Option<&FieldPathElement> {
    if let Some(fields) = &self.field {
      return fields.parent_field();
    }

    None
  }

  /// Searches for a field in the FieldPath list with a specific name.
  #[must_use]
  #[inline]
  pub fn get_field(&self, name: &str) -> Option<&FieldPathElement> {
    if let Some(fields) = &self.field {
      return fields.get_field(name);
    }

    None
  }

  /// If the FieldPath is present, it will return the list of the names for each path element.
  #[must_use]
  pub fn field_path(&self) -> Option<Vec<String>> {
    if let Some(fields) = &self.field {
      return Some(fields.field_path());
    }

    None
  }

  /// Returns the element names composing the violation's rule, like ["string", "max_len"].
  #[must_use]
  pub fn rule_path(&self) -> Option<Vec<String>> {
    if let Some(rules) = &self.rule {
      return Some(rules.field_path());
    }

    None
  }

  /// If there is a FieldPath, it returns the path elements' names, joined by a dot (e.g. `person.friends.0.address.street_name`).
  #[must_use]
  pub fn field_path_str(&self) -> Option<String> {
    if let Some(fields) = &self.field {
      return Some(fields.field_path_str());
    }

    None
  }

  /// If a rule path is defined, it returns the rule path segments for this violation, joined by a dot (e.g. `map.keys.string.min_len`)
  #[must_use]
  pub fn rule_path_str(&self) -> Option<String> {
    if let Some(rules) = &self.rule {
      return Some(rules.field_path_str());
    }

    None
  }

  /// Checks whether this violation has a FieldPath or not. This may not be the case when a violation is triggered by a rule defined with (buf.validate.message).cel in a message
  #[must_use]
  #[inline]
  pub const fn has_fields(&self) -> bool {
    self.field.is_some()
  }

  /// Checks if the list of FieldPathElements contains a field with a particular name.
  #[must_use]
  #[inline]
  pub fn has_field_by_name(&self, name: &str) -> bool {
    self
      .field
      .as_ref()
      .map(|f| f.elements.iter().any(|e| e.field_name() == name))
      .unwrap_or_default()
  }

  /// If a list of path elements is defined, it returns the name of the invalid field (the last field in the list of path elements)
  #[must_use]
  #[inline]
  pub fn field_name(&self) -> Option<&str> {
    self.last_field().map(|f| f.field_name())
  }
}

impl From<Violations> for Status {
  fn from(value: Violations) -> Self {
    let message = if value.violations.len() == 1 && !value.violations[0].message().is_empty() {
      value.violations[0].message()
    } else {
      "Validation failure"
    };

    Self {
      code: Code::InvalidArgument.into(),
      message: message.to_string(),
      details: vec![Any {
        type_url: "type.googleapis.com/buf.validate.Violations".to_string(),
        value: value.encode_to_vec(),
      }],
    }
  }
}

impl From<Violation> for Status {
  fn from(value: Violation) -> Self {
    let message = if value.message().is_empty() {
      "Validation failure"
    } else {
      value.message()
    };

    Self {
      code: Code::InvalidArgument.into(),
      message: message.to_string(),
      details: vec![Any {
        type_url: "type.googleapis.com/buf.validate.Violation".to_string(),
        value: value.encode_to_vec(),
      }],
    }
  }
}
