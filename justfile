build-docs:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --open --no-deps

test:
    cargo test --all-features -- -q --nocapture

release version exec="": test
    ../pre_release.sh {{ version }} {{ exec }}
    cargo release {{ version }} {{ exec }}
