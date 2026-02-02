## [Unreleased]

### 💥 BREAKING CHANGES

- ([107cfb2](https://github.com/Rick-Phoenix/proto-types/commit/107cfb26cec31309fbcd4fca7bbfe9906f07ed3f)) [**BREAKING**] Moved wrongly positioned protocheck internals to the proc macro crate
- ([1fc24ab](https://github.com/Rick-Phoenix/proto-types/commit/1fc24ab78c468d0a803d7758d583ade6294a09ee)) [**BREAKING**] Make FieldType non-exhaustive
- ([2e7cb46](https://github.com/Rick-Phoenix/proto-types/commit/2e7cb465380f6974ff949b70d46a02215a898379)) [**BREAKING**] Add FieldMask to FieldType
- ([981be57](https://github.com/Rick-Phoenix/proto-types/commit/981be579561c976e07f91c5056021e37314e71fe)) [**BREAKING**] Allow open ended Intervals
- ([a7f90ea](https://github.com/Rick-Phoenix/proto-types/commit/a7f90eaf6ee14a20f617750ed8ae1b8b2dac503e)) [**BREAKING**] Simplify google.type.Money errors
- ([a241b77](https://github.com/Rick-Phoenix/proto-types/commit/a241b7711fbe1f1a0a8da11d66b697c881acfd7f)) [**BREAKING**] Display nanos in TimeOfDay if they are more than 0
- ([0ada71e](https://github.com/Rick-Phoenix/proto-types/commit/0ada71e820f7680066712d5e80773adcbe062b91)) [**BREAKING**] Place constant for TimeOfDay as associated constants
- ([d869ab3](https://github.com/Rick-Phoenix/proto-types/commit/d869ab39fbdc00383c250fef4fe7bb8e60159bcf)) [**BREAKING**] Use idiomatic Display impl for Duration units
- ([cad4883](https://github.com/Rick-Phoenix/proto-types/commit/cad488371b821b5a0ba2e34ef1612ab57d574ec7)) [**BREAKING**] Use protobuf-compliant Duration Display impl, use separate helper for human-readable version
- ([4ad6670](https://github.com/Rick-Phoenix/proto-types/commit/4ad6670022c48cf97f8e4aa8a8c9172cd30d49ab)) [**BREAKING**] Use &str in http header helpers to avoid needless allocations
- ([71932b4](https://github.com/Rick-Phoenix/proto-types/commit/71932b4b6eb73f591ae29bd10ff307ebb1115d94)) [**BREAKING**] Make all error enums non-exhaustive
- ([a237b59](https://github.com/Rick-Phoenix/proto-types/commit/a237b59c84e2fed70037647ad3edea7011c888a1)) [**BREAKING**] Transform Empty into a unit struct

### ⛰️  Features

- ([28f4b1c](https://github.com/Rick-Phoenix/proto-types/commit/28f4b1cd9076d6c1784ce03aa34416da59b1d7da)) Cel conversions
- ([9b4b643](https://github.com/Rick-Phoenix/proto-types/commit/9b4b643c375d8852cbdeb87caaf750b2fcdc85d1)) Added Into<Status> for violations
- ([45d2fe3](https://github.com/Rick-Phoenix/proto-types/commit/45d2fe3c0b873ef7e8fd014416ce6b6fd0b83a3f)) Added conversions to/from NaiveDateTime and google.protobuf.Timestamp
- ([bef195d](https://github.com/Rick-Phoenix/proto-types/commit/bef195d5bf719e151cc2a13678da1c36dda06d48)) Diesel implementations for Timestamp
- ([bef8b3d](https://github.com/Rick-Phoenix/proto-types/commit/bef8b3d9d2e92b290bba74ca53ceedc6408788e8)) Diesel impls for DateTime
- ([b48baa7](https://github.com/Rick-Phoenix/proto-types/commit/b48baa7404fd47547158558bc3ffc660ccb63150)) Diesel impls for Date
- ([d86896a](https://github.com/Rick-Phoenix/proto-types/commit/d86896aadae3fed986f263d1ba30e39b4ccd8bfa)) Diesel impls for TimeOfDay
- ([bab56ec](https://github.com/Rick-Phoenix/proto-types/commit/bab56ecf4297aad6ddc48f12de0c51e495ce609e)) Diesel impls for Duration
- ([1ccb7f9](https://github.com/Rick-Phoenix/proto-types/commit/1ccb7f9d8f0e292f9a8bc742f02c4bb61447213a)) All diesel impls for mysql
- ([af2ede1](https://github.com/Rick-Phoenix/proto-types/commit/af2ede19461c5c540d5b517263e49da4a97838fc)) Implement IntoIterator, Deref and DerefMut for Violations
- ([03b80c4](https://github.com/Rick-Phoenix/proto-types/commit/03b80c4802cfb9a90a86c9bdd4e24298c64573ee)) Implement Extend for Violations
- ([f3c0fcf](https://github.com/Rick-Phoenix/proto-types/commit/f3c0fcf06e80789b41ff04c41eef85899db17711)) Vec-like initialization helpers for Violations
- ([c94bc58](https://github.com/Rick-Phoenix/proto-types/commit/c94bc5843a49cf8c2bbd381db95571f1c106d44f)) Impl IntoIterator, Deref, DerefMut and Extend for FieldPath
- ([1a5334e](https://github.com/Rick-Phoenix/proto-types/commit/1a5334e3a8e1a5408f8a494630bb93a0c8af1339)) Add FieldMask violations
- ([bc87751](https://github.com/Rick-Phoenix/proto-types/commit/bc877511d427206f3d1159bc5e97b2f18fda117e)) Added int wrappers
- ([03dde69](https://github.com/Rick-Phoenix/proto-types/commit/03dde6964d9a749d0a39bf53c71769cc98b68730)) types: Added to/from impls for Subscript
- ([d3b3c62](https://github.com/Rick-Phoenix/proto-types/commit/d3b3c6274f133fdb305952bb0965cb1edc038fd3)) No_std support
- ([48d2c97](https://github.com/Rick-Phoenix/proto-types/commit/48d2c9778d7a39e4861922e5933ca578fc3c7138)) Added violations enums
- ([aa478b2](https://github.com/Rick-Phoenix/proto-types/commit/aa478b2043dfa0bdd3a920a1bb6c33abd461cc6e)) Implemented Deref, DerefMut and iterator traits for FieldMask
- ([36b1846](https://github.com/Rick-Phoenix/proto-types/commit/36b184630144673839ca6d46a594f7bc6a058d77)) Added all operators for int wrappers
- ([c2bc292](https://github.com/Rick-Phoenix/proto-types/commit/c2bc292d338db6e5eb42381eb3fa1c57c1439910)) Ord impl for Any
- ([a5c66e2](https://github.com/Rick-Phoenix/proto-types/commit/a5c66e26c5c2e1c6d3b2f98b0ede10a801bec32a)) Added name method for int wrappers
- ([276a28e](https://github.com/Rick-Phoenix/proto-types/commit/276a28e8eac407543c761526511341613bf9a581)) Implement Hash for Violation, Violations, FieldPath and FieldPathElement
- ([cc28646](https://github.com/Rick-Phoenix/proto-types/commit/cc2864679cfd21146c6da432ccd36acf04f04100)) Implemented Ord for HttpHeader

### 🐛 Bug Fixes

- ([b693385](https://github.com/Rick-Phoenix/proto-types/commit/b69338533d8f43090e1099bd48aa6522d59ddc76)) Dereferencing to Vec rather than to a slice for Violations
- ([ce97600](https://github.com/Rick-Phoenix/proto-types/commit/ce97600ef3635f14dae61490bf0fcd82e3de2451)) Corrected rule type id for repeated.unique
- ([d59bd52](https://github.com/Rick-Phoenix/proto-types/commit/d59bd525822782e4206ff106326e9d74ca5d323a)) Use saturating operations to prevent panics with timestamps
- ([6f85771](https://github.com/Rick-Phoenix/proto-types/commit/6f85771fd3b7ad66e9081642672e58cdd676ce88)) Fixed past/future timestamp helpers
- ([ee78d82](https://github.com/Rick-Phoenix/proto-types/commit/ee78d82158528b2958b41aba048974278bf951c1)) Adjusted .now() timestamp method to support wasmbind
- ([f057078](https://github.com/Rick-Phoenix/proto-types/commit/f05707896569a355505fabf132f5f529f287028f)) Strenghten validation in validated Date constructor
- ([aff2565](https://github.com/Rick-Phoenix/proto-types/commit/aff2565eafde1c3ecae286984a444022b53f049e)) Corrected faulty logic for DateTime -> FixedOffset conversions
- ([a039c34](https://github.com/Rick-Phoenix/proto-types/commit/a039c34393ec14a55114aa1831ef8f526e414305)) Corrected Fraction methods to account for i64::MIN case
- ([8605991](https://github.com/Rick-Phoenix/proto-types/commit/8605991d8d2e3dbb9bf70c0b4965fbbb87cac317)) Correctly guarded `now()` methods with the std or chrono-wasm features
- ([6960ddd](https://github.com/Rick-Phoenix/proto-types/commit/6960dddc9b9c95f01a5ce4cb627f07e78bf17870)) Handling corner cases in Duration multiplications
- ([5693a81](https://github.com/Rick-Phoenix/proto-types/commit/5693a818a61864309b88f30f8a4a416efe4e4cfe)) Lacking Eq from Protovalidate structs
- ([47b3437](https://github.com/Rick-Phoenix/proto-types/commit/47b34376dd01b1552a01262ff0dae468a13c9a56)) Corrected is_scalar method for FieldType
- ([7cca7ef](https://github.com/Rick-Phoenix/proto-types/commit/7cca7ef7e6187e33621d94a2642c1f3a81fe9518)) Corrected diesel impls for feature-gated items
- ([0caddbd](https://github.com/Rick-Phoenix/proto-types/commit/0caddbd16bbdfa7cde63212618b83314430c63a1)) Wrong type url for Violation

### 🚜 Refactor

- ([8901caa](https://github.com/Rick-Phoenix/proto-types/commit/8901caa2c346ac56b1c9eddee403ca4c26354feb)) Moved google.rpc.Status outside of rpc flag
- ([efa5de8](https://github.com/Rick-Phoenix/proto-types/commit/efa5de8b1dd7489d576b8a230d452e34607f0292)) Make google.rpc.Code available by default
- ([53ecdea](https://github.com/Rick-Phoenix/proto-types/commit/53ecdea85df21ded7a3da516f135351291346ac8)) Made castings more explicit in timestamp parsing
- ([88a5092](https://github.com/Rick-Phoenix/proto-types/commit/88a50926b436ba2a2cea91b43d0f229ef70c3ac8)) Updated Violation impls
- ([02b7a6c](https://github.com/Rick-Phoenix/proto-types/commit/02b7a6c65a236188884f913286d40a0378579db3)) Remove trailing zeroes for nanos in Timestamp Display impl

### 📚 Documentation

- ([6fd263e](https://github.com/Rick-Phoenix/proto-types/commit/6fd263edbb71b358e0c334ff50e7522d2765ce56)) Changed doc_auto_cfg to doc_cfg
- ([6d98971](https://github.com/Rick-Phoenix/proto-types/commit/6d989715ebb818809ebfcf8a389af336f96370fa)) Updated readme with new diesel impls
- ([0b6a136](https://github.com/Rick-Phoenix/proto-types/commit/0b6a1360965e7353ff7dfc6644f1d15dd0264dbd)) Documentation for FieldType enum

### 🧪 Testing

- ([12afdd1](https://github.com/Rick-Phoenix/proto-types/commit/12afdd10a5c358a1cb618e3e46772b9d85d7ef31)) Added tests for timestamp operations
- ([f8b9ecb](https://github.com/Rick-Phoenix/proto-types/commit/f8b9ecb8a1b68f40e10735ed9f469b3b7e6bcda5)) Added tests for timestamp range helpers
- ([d11fb1c](https://github.com/Rick-Phoenix/proto-types/commit/d11fb1ce4d6861e85b8903bfee9c3b0b25b8dcb5)) Add tests for TimeOfDay

