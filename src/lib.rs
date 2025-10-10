#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

//! # Feature flags
#![doc = document_features::document_features!()]

/// Generated rust code from buf.validate protobuf package, with some added methods and structs.
#[cfg(feature = "protovalidate")]
pub mod protovalidate;

/// Implementations to allow conversion from well known types to [`cel::Value`](::cel::Value)
#[cfg(feature = "cel")]
pub mod cel;

#[cfg(feature = "rpc")]
mod rpc;
#[cfg(feature = "rpc")]
pub use rpc::*;

mod common;
pub use common::*;
pub use protobuf::*;
mod protobuf;
mod protobuf_impls;

/// Implementations and units for Duration structs.
pub mod duration;

pub mod timestamp;

mod any;
mod any_impls;

mod field_mask;

mod field_type;
#[doc(inline)]
pub use field_type::FieldType;

mod empty;

mod constants;
mod conversions;
mod datetime_internal;
mod type_url;

use core::{convert::TryFrom, fmt, time};
use std::str::FromStr;

use prost::{
  alloc::{format, string::String, vec::Vec},
  DecodeError, EncodeError, Message, Name,
};
pub(crate) use type_url::{type_url_for, TypeUrl};
<<<<<<< HEAD
<<<<<<< HEAD

<<<<<<< HEAD
impl ToTokens for FieldPathElement {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let field_number = &self.field_number;
    let field_name = &self.field_name;
    let field_type = &self.field_type;
    let key_type = &self.key_type;
    let value_type = &self.value_type;
    let subscript = &self.subscript;
||||||| parent of 716b263 (Defining wkt locally)
use crate::protobuf::{Duration, Timestamp};

impl ToTokens for FieldPathElement {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let field_number = &self.field_number;
    let field_name = &self.field_name;
    let field_type = &self.field_type;
    let key_type = &self.key_type;
    let value_type = &self.value_type;
    let subscript = &self.subscript;
=======
const NANOS_PER_SECOND: i32 = 1_000_000_000;

const NANOS_MAX: i32 = NANOS_PER_SECOND - 1;
>>>>>>> 716b263 (Defining wkt locally)

const PACKAGE_PREFIX: &str = "google.protobuf";
<<<<<<< HEAD

pub mod duration;
pub use duration::*;

<<<<<<< HEAD
impl ToTokens for ProtoType {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let path = quote! { protocheck::types::protobuf::field_descriptor_proto::Type };

    match self {
      ProtoType::Double => tokens.extend(quote! { #path::Double }),
      ProtoType::Float => tokens.extend(quote! { #path::Float }),
      ProtoType::Int64 => tokens.extend(quote! { #path::Int64 }),
      ProtoType::Uint64 => tokens.extend(quote! { #path::Uint64 }),
      ProtoType::Int32 => tokens.extend(quote! { #path::Int32 }),
      ProtoType::Fixed64 => tokens.extend(quote! { #path::Fixed64 }),
      ProtoType::Fixed32 => tokens.extend(quote! { #path::Fixed32 }),
      ProtoType::Bool => tokens.extend(quote! { #path::Bool }),
      ProtoType::String => tokens.extend(quote! { #path::String }),
      ProtoType::Group => tokens.extend(quote! { #path::Group }),
      ProtoType::Message => tokens.extend(quote! { #path::Message }),
      ProtoType::Bytes => tokens.extend(quote! { #path::Bytes }),
      ProtoType::Uint32 => tokens.extend(quote! { #path::Uint32 }),
      ProtoType::Enum => tokens.extend(quote! { #path::Enum }),
      ProtoType::Sfixed32 => tokens.extend(quote! { #path::Sfixed32 }),
      ProtoType::Sfixed64 => tokens.extend(quote! { #path::Sfixed64 }),
      ProtoType::Sint32 => tokens.extend(quote! { #path::Sint32 }),
      ProtoType::Sint64 => tokens.extend(quote! { #path::Sint64 }),
    }
  }
}

impl ToTokens for Ignore {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let path = quote! { protocheck::types::protovalidate::Ignore };

    match self {
      Ignore::Unspecified => tokens.extend(quote! { #path::Unspecified }),
      Ignore::IfZeroValue => tokens.extend(quote! { #path::IfZeroValue }),
      Ignore::Always => tokens.extend(quote! { #path::Always }),
    }
  }
}

impl ToTokens for Subscript {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    match self {
      Subscript::Index(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::Index(#value)
        });
      }
      Subscript::BoolKey(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::BoolKey(#value)
        });
      }
      Subscript::IntKey(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::IntKey(#value)
        });
      }
      Subscript::UintKey(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::UintKey(#value)
        });
      }
      Subscript::StringKey(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::StringKey(#value)
        });
      }
    }
  }
}
||||||| parent of 716b263 (Defining wkt locally)
impl ToTokens for ProtoType {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let path = quote! { protocheck::types::protobuf::field_descriptor_proto::Type };

    match self {
      ProtoType::Double => tokens.extend(quote! { #path::Double }),
      ProtoType::Float => tokens.extend(quote! { #path::Float }),
      ProtoType::Int64 => tokens.extend(quote! { #path::Int64 }),
      ProtoType::Uint64 => tokens.extend(quote! { #path::Uint64 }),
      ProtoType::Int32 => tokens.extend(quote! { #path::Int32 }),
      ProtoType::Fixed64 => tokens.extend(quote! { #path::Fixed64 }),
      ProtoType::Fixed32 => tokens.extend(quote! { #path::Fixed32 }),
      ProtoType::Bool => tokens.extend(quote! { #path::Bool }),
      ProtoType::String => tokens.extend(quote! { #path::String }),
      ProtoType::Group => tokens.extend(quote! { #path::Group }),
      ProtoType::Message => tokens.extend(quote! { #path::Message }),
      ProtoType::Bytes => tokens.extend(quote! { #path::Bytes }),
      ProtoType::Uint32 => tokens.extend(quote! { #path::Uint32 }),
      ProtoType::Enum => tokens.extend(quote! { #path::Enum }),
      ProtoType::Sfixed32 => tokens.extend(quote! { #path::Sfixed32 }),
      ProtoType::Sfixed64 => tokens.extend(quote! { #path::Sfixed64 }),
      ProtoType::Sint32 => tokens.extend(quote! { #path::Sint32 }),
      ProtoType::Sint64 => tokens.extend(quote! { #path::Sint64 }),
    }
  }
}

impl ToTokens for Ignore {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let path = quote! { protocheck::types::protovalidate::Ignore };

    match self {
      Ignore::Unspecified => tokens.extend(quote! { #path::Unspecified }),
      Ignore::IfZeroValue => tokens.extend(quote! { #path::IfZeroValue }),
      Ignore::Always => tokens.extend(quote! { #path::Always }),
    }
  }
}

impl ToTokens for Subscript {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    match self {
      Subscript::Index(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::Index(#value)
        });
      }
      Subscript::BoolKey(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::BoolKey(#value)
        });
      }
      Subscript::IntKey(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::IntKey(#value)
        });
      }
      Subscript::UintKey(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::UintKey(#value)
        });
      }
      Subscript::StringKey(value) => {
        tokens.extend(quote! {
            protocheck::types::protovalidate::Subscript::StringKey(#value)
        });
      }
    }
  }
}

impl ToTokens for Duration {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let seconds = self.seconds;
    let nanos = self.nanos;

    tokens.extend(quote! {
      protocheck::types::protobuf::Duration {
          seconds: #seconds,
          nanos: #nanos,
      }
    });
  }
}

impl ToTokens for Timestamp {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let seconds = self.seconds;
    let nanos = self.nanos;

    tokens.extend(quote! {
      protocheck::types::protobuf::Timestamp {
          seconds: #seconds,
          nanos: #nanos,
      }
    });
  }
}
=======
mod timestamp;
<<<<<<< HEAD
mod timestamp_impls;
pub use timestamp::TimestampError;
<<<<<<< HEAD
>>>>>>> 716b263 (Defining wkt locally)
||||||| parent of 9231a96 (Corrected types import)
=======
||||||| parent of d64de5b (Reorganized types and added duration helpers)
mod timestamp_impls;
pub use timestamp::TimestampError;
=======
pub use timestamp::*;
>>>>>>> d64de5b (Reorganized types and added duration helpers)

pub mod cel;

mod field_mask_impls;
>>>>>>> 9231a96 (Corrected types import)
||||||| parent of 3bb326c (File paths reorganization)

pub mod duration;
pub use duration::*;

mod timestamp;
pub use timestamp::*;

pub mod cel;

mod field_mask_impls;
=======
>>>>>>> 3bb326c (File paths reorganization)
||||||| parent of 0982079 (Moved constants to top of the crate)

const NANOS_PER_SECOND: i32 = 1_000_000_000;

const NANOS_MAX: i32 = NANOS_PER_SECOND - 1;

const PACKAGE_PREFIX: &str = "google.protobuf";
=======
>>>>>>> 0982079 (Moved constants to top of the crate)
||||||| parent of 8915c76 (refactor: moved google.rpc.Status outside of rpc flag)
=======

/// The `Status` type defines a logical error model that is suitable for
/// different programming environments, including REST APIs and RPC APIs. It is
/// used by [gRPC](<https://github.com/grpc>). Each `Status` message contains
/// three pieces of data: error code, error message, and error details.
///
/// You can find out more about this error model and how to work with it in the
/// [API Design Guide](<https://cloud.google.com/apis/design/errors>).
#[derive(Clone, PartialEq, ::prost::Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Status {
  /// The status code, which should be an enum value of
  /// [google.rpc.Code][google.rpc.Code].
  #[prost(int32, tag = "1")]
  pub code: i32,
  /// A developer-facing error message, which should be in English. Any
  /// user-facing error message should be localized and sent in the
  /// [google.rpc.Status.details][google.rpc.Status.details] field, or localized
  /// by the client.
  #[prost(string, tag = "2")]
  pub message: ::prost::alloc::string::String,
  /// A list of messages that carry the error details.  There is a common set of
  /// message types for APIs to use.
  #[prost(message, repeated, tag = "3")]
  pub details: ::prost::alloc::vec::Vec<crate::protobuf::Any>,
}
>>>>>>> 8915c76 (refactor: moved google.rpc.Status outside of rpc flag)
