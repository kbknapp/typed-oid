# Container Image Reference

![Rust Version][rustc-image]
[![crates.io][crate-image]][crate-link]
[![Dependency Status][deps-image]][deps-link]

A library for using and handling Typed Object IDs.

<!-- vim-markdown-toc GFM -->

* [About](#about)
* [The Pitch](#the-pitch)
* [The Anti-Pitch](#the-anti-pitch)
* [Example](#example)
* [License](#license)

<!-- vim-markdown-toc -->

## About

An Object ID (OID) is a base32hex (which is base32 with the extended hex
alphabet; see [RFC4648] for details) encoded UUID. The UUID is either v4
(random) or v7 (based on UNIX Epoch; see [draft RFC4122v17] for details) - this
library further qualifies the OID with a "type" which is a short alphanumeric
prefix separated from the OID by a `-`. This library refers to Typed OIDs as
TOIDs which are distinct from the OID counterparts which lack the "type"
prefix.

For example `EXA-4GKFGPRVND4QT3PDR90PDKF66O`, by convention the prefix is three
ASCII characters, however that is not a hard constraint of TOIDs in general.

## The Pitch

TOIDs allow a "human readable subject line" in the form of the prefix, where
actual data is a UUIDv7. This means while debugging or reviewing a system it's
trivial to determine if an incorrect TOID was passed in a particular location by
looking at the prefix. This isn't achievable with bare UUIDs or GUIDs due to
their lacking of any typed identifiers.

In other words, without using TOIDs it's far too easy to incorrectly swap say a
`UserID` with an `OrderID` if they are both just simple GUIDs. Where as a TOID
would make the mistake more easily identifiable or even programmatically
impossible.

Base32hex encoding the UUID also allows compressing the data into a smaller and
more familiar format for humans, akin to a commit hash. Using the "extended hex
encoding" adds the additional property that the encodings do not lose their
sort order when compared bitwise.

Finally, using a UUIDv7 enables index locality when used as database entries.

## The Anti-Pitch

The downside to TOIDs is a layer of indirection when handling IDs and values,
it's not immediately obvious that the TOIDs are a prefixed UUIDv7.
Additionally, the prefixes themselves must be controlled in some manner
including migrated on changes which adds a layer of complexity at the
application layer.

There is also additional processing overhead compared to a bare UUID in order
to encode/decode as well as handling the appending and removing the prefixes.

However, we believe the drawbacks pale in comparison to the benefits derived
from the format.

## Example

```rust,no_run
use typed_oid::{error::*, Oid, OidStr, OidPrefix};
use uuid::Uuid;
use anyhow::Result;

fn main() -> Result<()> {
    // TOIDs come in two flavors, Oid<T> and `OidStr`.

    // A Oid<T> is a TOID using a Rust types as the type, e.g. a true Typed OID
    // These are less ergonomic, but more type-safe.
    run_oid()?;

    // A `OidStr` is a TOID using a bare string as the type, e.g. "Stringly Typed OID"
    // These easier to use, but less type-safe.
    run_oidstr()?;

    Ok(())
}

fn run_oidstr() -> Result<()> {
    // OIDs can be created with a given prefix alone
    #[cfg(feature = "uuid_v4")]
    {
        let oid = OidStr::new_v4("EXA")?;
        println!("OidStr from UUIDv4: {oid}");
    }
    #[cfg(feature = "uuid_v7")]
    {
        let oid = OidStr::new_v7_now("EXA")?;
        println!("OidStr from UUIDv7: {oid}");
    }
    // OIDs can also be created from the raw parts
    let oid = OidStr::try_with_uuid("EXA", "b3cfdafa-3fec-41e2-82bf-ff881131abf1")?;
    println!("OidStr from UUID: {oid}");

    // OIDs can be parsed from strings, however the "value" must be a valid
    // base32hex (no pad) encoded UUID
    let oid: OidStr = "EXA-4GKFGPRVND4QT3PDR90PDKF66O".parse()?;
    println!("OidStr from string: {oid}");

    // One can retrieve the various parts of the OID if needed
    println!("Components of {oid}:");
    println!("\tPrefix: {}", oid.prefix());
    println!("\tValue: {}", oid.value());
    println!("\tUUID: {}", oid.uuid());

    Ok(())
}

fn run_oid() -> Result<()> {
    // In order for a struct to be used as a type it must implement typed_oid::OidPrefix
    #[derive(Debug)]
    struct EXA;
    impl OidPrefix for EXA {}

    // We can create a new OID by generating a random UUID
    #[cfg(feature = "uuid_v4")]
    {
        let oid: Oid<EXA> = Oid::new_v4();
        println!("Oid<EXA> with new UUIDv4: {oid}");
    }
    #[cfg(feature = "uuid_v7")]
    {
        let oid: Oid<EXA> = Oid::new_v7_now();
        println!("Oid<EXA> with new UUIDv7: {oid}");
    }
    // Or by giving a UUID
    let oid: Oid<EXA> = Oid::try_with_uuid("b3cfdafa-3fec-41e2-82bf-ff881131abf1")?;
    println!("Oid<EXA> with new UUID: {oid}");

    // We can go the other direction and parse a string to a Oid<EXA>
    let oid: Oid<EXA> = "EXA-4GKFGPRVND4QT3PDR90PDKF66O".parse()?;
    println!("Oid<EXA> with from string: {oid}");

    // One can retrieve the various parts of the OID if needed
    println!("Components of {oid}:");
    println!("\tPrefix: {}", oid.prefix());
    println!("\tValue: {}", oid.value());
    println!("\tUUID: {}", oid.uuid());

    // However, if we change the prefix to something that doesn't match our EXA type
    // we get an error even if the UUID is valid
    let res = "FAIL-4GKFGPRVND4QT3PDR90PDKF66O".parse::<Oid<EXA>>();
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), Error::InvalidPrefix { valid_until: 0 });

    Ok(())
}
```

## License

This project is dual licensed under the terms of either the Apache License,
Version 2.0, [LICENSE-APACHE] or MIT [LICENSE-MIT] at your option.

`typed-oid` was originally forked from [`seaplane-oid`][seaplane_oid] v0.4.0
which was licensed under the Apache License, Version 2.0.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/typed-oid.svg
[crate-link]: https://crates.io/crates/typed-oid
[deps-image]: https://deps.rs/repo/github/kbknapp/typed-oid/status.svg
[deps-link]: https://deps.rs/crate/typed-oid
[rustc-image]: https://img.shields.io/badge/rustc-1.60+-blue.svg

[//]: # (Links)

[LICENSE-MIT]: https://github.com/kbknapp/typed-oid/blob/main/LICENSE-MIT
[LICENSE-APACHE]: https://github.com/kbknapp/typed-oid/blob/main/LICENSE-APACHE
[RFC4648]: https://datatracker.ietf.org/doc/html/rfc4648.html#section-7
[draft RFC4122v14]: https://datatracker.ietf.org/doc/html/draft-ietf-uuidrev-rfc4122bis#name-uuid-version-7
[seaplane_oid]: https://crates.io/crates/seaplane-oid
