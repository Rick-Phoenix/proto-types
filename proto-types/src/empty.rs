use prost::Name;

use crate::{String, constants::PACKAGE_PREFIX, type_url_for};

/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs.
///
/// A typical example is to use it as the request
/// or the response type of an API method. For instance:
///
/// ```proto
/// service Foo {
///   rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
/// }
/// ```
///
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Empty;

impl ::prost::Message for Empty {
	fn encode_raw(&self, _: &mut impl ::prost::bytes::BufMut) {}
	fn merge_field(
		&mut self,
		tag: u32,
		wire_type: ::prost::encoding::wire_type::WireType,
		buf: &mut impl ::prost::bytes::Buf,
		ctx: ::prost::encoding::DecodeContext,
	) -> ::core::result::Result<(), ::prost::DecodeError> {
		::prost::encoding::skip_field(wire_type, tag, buf, ctx)
	}
	#[inline]
	fn encoded_len(&self) -> usize {
		0
	}
	fn clear(&mut self) {}
}

impl From<()> for Empty {
	fn from((): ()) -> Self {
		Self {}
	}
}

impl Name for Empty {
	const PACKAGE: &'static str = PACKAGE_PREFIX;

	const NAME: &'static str = "Empty";

	fn type_url() -> String {
		type_url_for::<Self>()
	}
}

#[cfg(feature = "serde")]
mod serde_impls {
	use super::*;
	use crate::format;
	use core::fmt;

	use serde::{Deserialize, Serialize, ser::SerializeStruct};

	use crate::Empty;
	impl Serialize for Empty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: serde::Serializer,
		{
			// Serialize as an empty struct (which maps to an empty JSON object `{}`)
			serializer.serialize_struct("Empty", 0)?.end()
		}
	}

	impl<'de> Deserialize<'de> for Empty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: serde::Deserializer<'de>,
		{
			struct EmptyVisitor;

			impl<'de> serde::de::Visitor<'de> for EmptyVisitor {
				type Value = Empty;

				fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
					formatter.write_str("an empty object `{}`")
				}

				fn visit_map<A>(self, mut _map: A) -> Result<Self::Value, A::Error>
				where
					A: serde::de::MapAccess<'de>,
				{
					// Ensure there are no unexpected fields in the map
					if let Some(key) = _map.next_key::<String>()? {
						return Err(serde::de::Error::custom(format!(
							"Unexpected field in Empty message: {key}"
						)));
					}
					Ok(Empty {})
				}

				// Also allow deserializing from unit (`()`) if needed, though `{}` is standard for JSON
				fn visit_unit<E>(self) -> Result<Self::Value, E>
				where
					E: serde::de::Error,
				{
					Ok(Empty {})
				}
			}

			deserializer.deserialize_unit_struct("Empty", EmptyVisitor) // Expect a struct with no fields
		}
	}
}
