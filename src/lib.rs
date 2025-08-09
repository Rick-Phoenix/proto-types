mod buf;

pub mod protovalidate_impls;
pub use buf::validate as protovalidate;

pub mod cel;

pub use protobuf::*;
mod protobuf;
mod protobuf_impls;

pub mod duration;
pub use duration::*;

mod timestamp;
pub use timestamp::*;

mod any;
mod any_impls;

mod field_mask_impls;

mod field_type;
pub use field_type::FieldType;

mod empty;

mod conversions;
mod datetime;
mod type_url;

use core::{convert::TryFrom, fmt, time};
use std::str::FromStr;

pub(crate) use proc_macro2::TokenStream as TokenStream2;
use prost::{
  alloc::{format, string::String, vec::Vec},
  DecodeError, EncodeError, Message, Name,
};
pub(crate) use type_url::{type_url_for, TypeUrl};

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
