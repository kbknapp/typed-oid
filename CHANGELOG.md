<a name="0.4.2"></a>
## 0.4.2 Typed-OID Release (2024-06-24)

#### Fixes

*   Impl `Debug` manually for `Oid<T>` for better display ([8d384ce](https://github.com/kbknapp/typed-oid/commit/8d384ce6e0f6714b6e53928483a693b67aa46b9a))

<a name="0.4.1"></a>
## 0.4.1 Typed-OID Release (2024-06-24)

#### Fixes

*   `Oid<T>` used `derive(Copy, Clone)` which sometimes does not properly check the bounds (See rust#26925); the traits are now implemented manually ([f9cdb2a](https://github.com/kbknapp/typed-oid/commit/f9cdb2af57bd541fc4156f691159d73b253bf737))

<a name="0.4.0"></a>
## 0.4.0 Typed-OID Release (2024-06-21)

#### Features

*   `Hash` is implemented for `Oid` and `OidStr` ([be16d32](https://github.com/kbknapp/typed-oid/commit/be16d322bca90d5f27f719dfa815b6a9fe65e93b))

<a name="0.3.2"></a>
## 0.3.2 Typed-OID Release (2024-04-13)

#### Features

*   surrealdb conversions can also be base32hex encoded IDs ([237b6b73](https://github.com/kbknapp/typed-oid/commit/237b6b7368c8c54ad9d786dab5f4f131839f05bb))

<a name="0.3.1"></a>
## 0.3.1 Typed-OID Release (2024-04-13)

#### Features

*   adds convenience methods for creating Oids from base32hex encoded UUID strings ([3ad9a649](https://github.com/kbknapp/typed-oid/commit/3ad9a6491522ef5aad53227ddfa0785719c21016))

<a name="0.3.0"></a>
## 0.3.0 Typed-OID Release (2024-04-12)

#### Features

*   allow UUID to come from string-ish conversion ([d6e00940](https://github.com/kbknapp/typed-oid/commit/d6e009407fb74238e46682c6d03c0a4244cd54ab))
*   add support for converting to/from [SurrealDB](https://surrealdb.com) `Thing` types ([98956986](https://github.com/kbknapp/typed-oid/commit/989569865fcc23772226195f03cb62f170676e94))
  * Use crate feature `surrealdb` (**NOTE** using this feature bumps the MSRV to 1.75.0)
*   allow typed-oid's string prefix to differ from type name ([f2f891e9](https://github.com/kbknapp/typed-oid/commit/f2f891e93a61ca6ef075974318fb7c3a746a51d2))

#### Tests

*   remove unused imports ([eb12f4bb](https://github.com/kbknapp/typed-oid/commit/eb12f4bbeb0533da4694dd68f0554e6fcc4384ee))
*   fix `compile_fail` tests to fail for the correct reason ([84c9e03e](https://github.com/kbknapp/typed-oid/commit/84c9e03efd738a9515c98473d941bf7cc3698609))
*   adds long OID tests ([62cad9af](https://github.com/kbknapp/typed-oid/commit/62cad9afeac07ce98b3eaf075ff5613a825ff769))

#### Documentation

*   add MSRV notes to README.md ([58f0661d](https://github.com/kbknapp/typed-oid/commit/58f0661de594815c64fcebb6bf8c23b9d6294b4d))
*   fix examples when no crate features are used ([7fc10c98](https://github.com/kbknapp/typed-oid/commit/7fc10c98ce724345427cc5baa62125ef9766521c))
*   removed no longer correct constraints ([0ca311ba](https://github.com/kbknapp/typed-oid/commit/0ca311ba1ff52691066dcdb63b2c392c56a1b28a))
*   enable feature flags for docsrs ([d2be13e5](https://github.com/kbknapp/typed-oid/commit/d2be13e5c5766356da4a8a72685d6e8b6f051760))
*   fix docs.rs crate-features ([489f8770](https://github.com/kbknapp/typed-oid/commit/489f8770a73476540dc9d43d96db42424e888043))

#### Meta

*   use nightly in rust-toolchain ([01c9e169](https://github.com/kbknapp/typed-oid/commit/01c9e169ab3eaba310d6513e11ac7fc2f72abb2a))

<a name="0.2.1"></a>
## 0.2.1 (2024-04-09)

### Documentation

- Fix compilation on docs.rs for crate feature flags

<a name="0.2.0"></a>
## 0.2.0 (2024-04-04)

### Improvements

- Allows `Send` and `Sync` to be auto implented for `Oid<T>`

### Maintanance

- Update dependencies

### Documentation

- Fix cargo feature flags in docs.rs

