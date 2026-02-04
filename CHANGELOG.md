## [0.2.0] - 2026-02-04

### 💥 BREAKING CHANGES

- [107cfb2](https://github.com/Rick-Phoenix/proto-types/commit/107cfb26cec31309fbcd4fca7bbfe9906f07ed3f) [**BREAKING**] Moved wrongly positioned protocheck internals to the proc macro crate
- [1fc24ab](https://github.com/Rick-Phoenix/proto-types/commit/1fc24ab78c468d0a803d7758d583ade6294a09ee) [**BREAKING**] Make FieldType non-exhaustive
- [73f6f4d](https://github.com/Rick-Phoenix/proto-types/commit/73f6f4d389e749c96ecc68261f290445a8e8c564) [**BREAKING**] Allow open ended Intervals
- [d6db45b](https://github.com/Rick-Phoenix/proto-types/commit/d6db45be79a7112d341f0652625426e8163f5ce6) [**BREAKING**] Simplify google.type.Money errors
- [fb34ab9](https://github.com/Rick-Phoenix/proto-types/commit/fb34ab911c83855ec12185e22b8e0dd157287d17) [**BREAKING**] Place constant for TimeOfDay as associated constants
- [0b0d1e7](https://github.com/Rick-Phoenix/proto-types/commit/0b0d1e7e1b9ab18fcdc90532537558127edb477f) [**BREAKING**] Use protobuf-compliant Duration Display impl, use separate helper for human-readable version
- [e843bce](https://github.com/Rick-Phoenix/proto-types/commit/e843bce916b9595ba3b6d8b23c55036a9dcfe054) [**BREAKING**] Use &str in http header helpers to avoid needless allocations
- [caf41d3](https://github.com/Rick-Phoenix/proto-types/commit/caf41d3dbd05ec960e871921e4098bc06c4aed6f) [**BREAKING**] Make all error enums non-exhaustive
- [72d8103](https://github.com/Rick-Phoenix/proto-types/commit/72d810389a947db5ad241b9f3530ed5e06db833d) [**BREAKING**] Transform Empty into a unit struct

### ⛰️  Features

- [28f4b1c](https://github.com/Rick-Phoenix/proto-types/commit/28f4b1cd9076d6c1784ce03aa34416da59b1d7da) Cel conversions
- [9b4b643](https://github.com/Rick-Phoenix/proto-types/commit/9b4b643c375d8852cbdeb87caaf750b2fcdc85d1) Added Into<Status> for violations
- [45d2fe3](https://github.com/Rick-Phoenix/proto-types/commit/45d2fe3c0b873ef7e8fd014416ce6b6fd0b83a3f) Added conversions to/from NaiveDateTime and google.protobuf.Timestamp
- [bef195d](https://github.com/Rick-Phoenix/proto-types/commit/bef195d5bf719e151cc2a13678da1c36dda06d48) Diesel implementations for Timestamp
- [bef8b3d](https://github.com/Rick-Phoenix/proto-types/commit/bef8b3d9d2e92b290bba74ca53ceedc6408788e8) Diesel impls for DateTime
- [b48baa7](https://github.com/Rick-Phoenix/proto-types/commit/b48baa7404fd47547158558bc3ffc660ccb63150) Diesel impls for Date
- [d86896a](https://github.com/Rick-Phoenix/proto-types/commit/d86896aadae3fed986f263d1ba30e39b4ccd8bfa) Diesel impls for TimeOfDay
- [bab56ec](https://github.com/Rick-Phoenix/proto-types/commit/bab56ecf4297aad6ddc48f12de0c51e495ce609e) Diesel impls for Duration
- [1ccb7f9](https://github.com/Rick-Phoenix/proto-types/commit/1ccb7f9d8f0e292f9a8bc742f02c4bb61447213a) All diesel impls for mysql
- [af2ede1](https://github.com/Rick-Phoenix/proto-types/commit/af2ede19461c5c540d5b517263e49da4a97838fc) Implement IntoIterator, Deref and DerefMut for Violations
- [03b80c4](https://github.com/Rick-Phoenix/proto-types/commit/03b80c4802cfb9a90a86c9bdd4e24298c64573ee) Implement Extend for Violations
- [f3c0fcf](https://github.com/Rick-Phoenix/proto-types/commit/f3c0fcf06e80789b41ff04c41eef85899db17711) Vec-like initialization helpers for Violations
- [c94bc58](https://github.com/Rick-Phoenix/proto-types/commit/c94bc5843a49cf8c2bbd381db95571f1c106d44f) Impl IntoIterator, Deref, DerefMut and Extend for FieldPath
- [2596d06](https://github.com/Rick-Phoenix/proto-types/commit/2596d06ceeda7f94dbbe6f49ecff92162d10624a) Add FieldMask violations
- [c59e22f](https://github.com/Rick-Phoenix/proto-types/commit/c59e22fa19e366bfe24c5649dd1129505a63c0dd) Added int wrappers
- [8064d29](https://github.com/Rick-Phoenix/proto-types/commit/8064d296736a7135e5b30f06b6dd4dc4b6e9a6a6) types: Added to/from impls for Subscript
- [9e62178](https://github.com/Rick-Phoenix/proto-types/commit/9e621787b40a6d63b74968c4298a183470669ab9) No_std support
- [d00e10e](https://github.com/Rick-Phoenix/proto-types/commit/d00e10e49c848253dd07f3c0986a59f03cad99af) Added violations enums
- [7a60931](https://github.com/Rick-Phoenix/proto-types/commit/7a609313f68feb9ddd45e85534cff32b2c269f91) Implemented Deref, DerefMut and iterator traits for FieldMask
- [7927dba](https://github.com/Rick-Phoenix/proto-types/commit/7927dbad089bcca5f2a98af79473f0015b82a6b4) Added all operators for int wrappers
- [69da585](https://github.com/Rick-Phoenix/proto-types/commit/69da585d8b76bcfacb872f3cedeae592791cc811) Ord impl for Any
- [3241848](https://github.com/Rick-Phoenix/proto-types/commit/3241848729dd319400f6719e9b3fed6106efa57e) Added name method for int wrappers
- [59ae35b](https://github.com/Rick-Phoenix/proto-types/commit/59ae35b6bb7dc16b2d17f5a1d7397de57ac9f1ff) Implement Hash for Violation, Violations, FieldPath and FieldPathElement
- [f58acd9](https://github.com/Rick-Phoenix/proto-types/commit/f58acd9d798c917efdc3fc62c49325f5c8821c09) Implemented Ord for HttpHeader
- [e01907b](https://github.com/Rick-Phoenix/proto-types/commit/e01907b05cc6a5125b1327724b2c49b7e4c40a1e) Add and Sub std::time::Duration for Timestamp
- [688c5fc](https://github.com/Rick-Phoenix/proto-types/commit/688c5fcc0800fdb85cde505a5d97d43860f06cf1) Sub and Add with core::time::Duration for Duration
- [0eb9246](https://github.com/Rick-Phoenix/proto-types/commit/0eb9246c9feec3b8bffd8076000df9a468c1a8a9) Sub and Add with chrono::TimeDelta for Duration
- [d31f789](https://github.com/Rick-Phoenix/proto-types/commit/d31f789189d5130c8a6e5a97cd221f227594d347) Sub and Add with chrono::TimeDelta for Timestamp
- [5c3a717](https://github.com/Rick-Phoenix/proto-types/commit/5c3a717e338345f315dba9ff2496353cc6e27db6) PartialOrd and PartialEq for core::time::Duration and chrono::TimeDelta for Duration

### 🐛 Bug Fixes

- [b693385](https://github.com/Rick-Phoenix/proto-types/commit/b69338533d8f43090e1099bd48aa6522d59ddc76) Dereferencing to Vec rather than to a slice for Violations
- [ce97600](https://github.com/Rick-Phoenix/proto-types/commit/ce97600ef3635f14dae61490bf0fcd82e3de2451) Corrected rule type id for repeated.unique
- [be33f5d](https://github.com/Rick-Phoenix/proto-types/commit/be33f5d6186770fe4e3c7e082d15c29370f66588) Use saturating operations to prevent panics with timestamps
- [120dc94](https://github.com/Rick-Phoenix/proto-types/commit/120dc946f2a2f6998999d1552386bfc61de29c54) Fixed past/future timestamp helpers
- [c3184ae](https://github.com/Rick-Phoenix/proto-types/commit/c3184ae12c8c2ebbbe62e82d8d06ef134fdd9d58) Adjusted .now() timestamp method to support wasmbind
- [04a0dff](https://github.com/Rick-Phoenix/proto-types/commit/04a0dffdcc7b9aec840b2ab3437576724b91de41) Strenghten validation in validated Date constructor
- [f73a1f9](https://github.com/Rick-Phoenix/proto-types/commit/f73a1f9a86012d94e7255c9dcbd726307f55b197) Corrected faulty logic for DateTime -> FixedOffset conversions
- [f0cadf2](https://github.com/Rick-Phoenix/proto-types/commit/f0cadf258d10c642ae28104bdf38752e524c67b8) Corrected Fraction methods to account for i64::MIN case
- [6761b43](https://github.com/Rick-Phoenix/proto-types/commit/6761b43ddcdddbf0113a59a458bdfda09161ff17) Correctly guarded `now()` methods with the std or chrono-wasm features
- [0c5097d](https://github.com/Rick-Phoenix/proto-types/commit/0c5097dd5fd5327328da81d64cba6e6ef76e63ac) Handling corner cases in Duration multiplications
- [d564a4f](https://github.com/Rick-Phoenix/proto-types/commit/d564a4f31cac3800d3bfef5b92e1d611f03e1b09) Lacking Eq from Protovalidate structs
- [0532c8e](https://github.com/Rick-Phoenix/proto-types/commit/0532c8ecfa41c13b1f974f0a0732d123b8ffd777) Corrected is_scalar method for FieldType
- [ee03fc4](https://github.com/Rick-Phoenix/proto-types/commit/ee03fc4899ff310d5544948fb53446ad9efca07e) Corrected diesel impls for feature-gated items
- [acfcb85](https://github.com/Rick-Phoenix/proto-types/commit/acfcb85f7edf2d4ac6c0a37412a557b01b5c7891) Wrong type url for Violation

### 🚜 Refactor

- [8901caa](https://github.com/Rick-Phoenix/proto-types/commit/8901caa2c346ac56b1c9eddee403ca4c26354feb) Moved google.rpc.Status outside of rpc flag
- [efa5de8](https://github.com/Rick-Phoenix/proto-types/commit/efa5de8b1dd7489d576b8a230d452e34607f0292) Make google.rpc.Code available by default
- [53ecdea](https://github.com/Rick-Phoenix/proto-types/commit/53ecdea85df21ded7a3da516f135351291346ac8) Made castings more explicit in timestamp parsing
- [4665604](https://github.com/Rick-Phoenix/proto-types/commit/466560468c09c450cfc76a79fc039ba60b74cce6) Add FieldMask to FieldType
- [90501aa](https://github.com/Rick-Phoenix/proto-types/commit/90501aaa085ce5f6b3f029fcdeb61dafb78da5b1) Display nanos in TimeOfDay if they are more than 0
- [6d69995](https://github.com/Rick-Phoenix/proto-types/commit/6d6999514c2f33a35539aaa7c4c9abc417c8b811) Use idiomatic Display impl for Duration units
- [8ed10ad](https://github.com/Rick-Phoenix/proto-types/commit/8ed10ad6dee93849d82c6c52e3cf9a32c5005989) Updated Violation impls
- [9b937a4](https://github.com/Rick-Phoenix/proto-types/commit/9b937a4f5e29726deaf3e00a7f47f27de1a8674e) Remove trailing zeroes for nanos in Timestamp Display impl

### 📚 Documentation

- [6fd263e](https://github.com/Rick-Phoenix/proto-types/commit/6fd263edbb71b358e0c334ff50e7522d2765ce56) Changed doc_auto_cfg to doc_cfg
- [6d98971](https://github.com/Rick-Phoenix/proto-types/commit/6d989715ebb818809ebfcf8a389af336f96370fa) Updated readme with new diesel impls
- [1802730](https://github.com/Rick-Phoenix/proto-types/commit/18027303c82f4f86ab8a03f9a8118f620049c903) Documentation for FieldType enum
- [b1b6c09](https://github.com/Rick-Phoenix/proto-types/commit/b1b6c09e61961f9298d6bed54490823e6b609371) Documentation for int wrappers
- [2d65fda](https://github.com/Rick-Phoenix/proto-types/commit/2d65fdab571cdfefab7dca4399af915e204d0039) Documenting violation enums

### 🧪 Testing

- [7dac981](https://github.com/Rick-Phoenix/proto-types/commit/7dac9810d594fee3716f68a18bcdc69b6510218c) Added tests for timestamp operations
- [b98cd72](https://github.com/Rick-Phoenix/proto-types/commit/b98cd72d15b4f56f584b11f44f4fda9d6eb292af) Added tests for timestamp range helpers
- [29a8b8d](https://github.com/Rick-Phoenix/proto-types/commit/29a8b8db7b0520c4a9c06d04c0752d61287eccae) Add tests for TimeOfDay
- [e52af5c](https://github.com/Rick-Phoenix/proto-types/commit/e52af5c29c54bd98c27f46c36531a4a0104a4351) Updated tests for timestamp operations

### ⚙️ Miscellaneous Tasks

- [57ca635](https://github.com/Rick-Phoenix/proto-types/commit/57ca6356d7807024eb505af1537c2089dc63e16e) Switched to hard tabs
- [fbf2112](https://github.com/Rick-Phoenix/proto-types/commit/fbf21121df327ccf6a6dfbe4b8b11c3a62164b45) Added blame ignore file

