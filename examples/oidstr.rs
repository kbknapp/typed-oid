use typed_oid::error::*;

#[cfg(not(feature = "uuid_v4"))]
fn oid_v4() -> Result<()> {
    println!("Feature uuid_v4 is not enabled");
    Ok(())
}
#[cfg(not(feature = "uuid_v7"))]
fn oid_v7() -> Result<()> {
    println!("Feature uuid_v7 is not enabled");
    Ok(())
}

fn main() -> Result<()> {
    oid_v4()?;
    println!("---------------------");
    oid_v7()?;
    Ok(())
}

#[cfg(feature = "uuid_v4")]
fn oid_v4() -> Result<()> {
    use typed_oid::OidStr;
    use uuid::Uuid;

    // OIDs can be created with a given prefix alone, which generates a new
    // UUIDv4
    let oid = OidStr::new_v4("EXA")?;
    println!("OidStr from UUIDv4: {oid}");

    // OIDs can be parsed from strings, however the "value" must be a valid
    // base32hex (no pad) encoded UUID
    let oid: OidStr = "EXA-4GKFGPRVND4QT3PDR90PDKF66O".parse()?;
    println!("OidStr from string: {oid}");

    // OIDs can also be created from the raw parts
    let oid = OidStr::with_uuid(
        "EXA",
        "b3cfdafa-3fec-41e2-82bf-ff881131abf1"
            .parse::<Uuid>()
            .unwrap(),
    )?;
    println!("OidStr from raw parts: {oid}");

    // One can retrieve the various parts of the OID if needed
    println!("Components of {oid}:");
    println!("\tPrefix: {}", oid.prefix());
    println!("\tValue: {}", oid.value());
    println!("\tUUID: {}", oid.uuid());

    Ok(())
}

#[cfg(feature = "uuid_v7")]
fn oid_v7() -> Result<()> {
    use typed_oid::OidStr;
    use uuid::Uuid;

    // OIDs can be created with a given prefix alone, which generates a new
    // UUIDv7 using the current unix timestamp
    let oid = OidStr::new_v7_now("EXA")?;
    println!("OidStr from UUIDv7: {oid}");

    // OIDs can be parsed from strings, however the "value" must be a valid
    // base32hex (no pad) encoded UUID
    let oid: OidStr = "EXA-066F28J3RDQ33EB4QM8LVP0TGK".parse()?;
    println!("OidStr from string: {oid}");

    // OIDs can also be created from the raw parts
    let oid = OidStr::with_uuid(
        "EXA",
        "0185e030-ffcf-75fa-a12a-ae8549bd7600".parse::<Uuid>()?,
    )?;
    println!("OidStr from raw parts: {oid}");

    // One can retrieve the various parts of the OID if needed
    println!("Components of {oid}:");
    println!("\tPrefix: {}", oid.prefix());
    println!("\tValue: {}", oid.value());
    println!("\tUUID: {}", oid.uuid());

    Ok(())
}
