use typed_oid::{error::*, OidPrefix};

#[cfg(not(feature = "uuid_v4"))]
fn oid_v4() -> Result<()> {
    printl!("Feature uuid_v4 is not enabled");
    Ok(())
}
#[cfg(not(feature = "uuid_v7"))]
fn oid_v7() -> Result<()> {
    println!("Feature uuid_v7 is not enabled");
    Ok(())
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
struct EXA;
impl OidPrefix for EXA {}

fn main() -> Result<()> {
    oid_v4()?;
    println!("---------------------");
    oid_v7()?;
    Ok(())
}

#[cfg(feature = "uuid_v4")]
fn oid_v4() -> Result<()> {
    use typed_oid::Oid;
    use uuid::Uuid;

    // We can create a new OID by generating a random UUID
    let oid: Oid<EXA> = Oid::new_v4();
    println!("Oid<EXA> with new UUIDv4: {oid}");

    // We can go the other direction and parse a string to a Oid<EXA>
    let oid: Oid<EXA> = "EXA-4GKFGPRVND4QT3PDR90PDKF66O".parse().unwrap();
    println!("Oid<EXA> with from string: {oid}");

    // TOIDs can also be created from the raw parts
    let oid: Oid<EXA> = Oid::with_uuid("b3cfdafa-3fec-41e2-82bf-ff881131abf1".parse::<Uuid>()?);
    println!("Oid from raw parts: {oid}");

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

#[cfg(feature = "uuid_v7")]
fn oid_v7() -> Result<()> {
    use typed_oid::Oid;
    use uuid::Uuid;

    // We can create a new OID by generating a random UUID
    let oid: Oid<EXA> = Oid::new_v7_now();
    println!("Oid<EXA> with new UUIDv7: {oid}");

    // We can go the other direction and parse a string to a Oid<EXA>
    let oid: Oid<EXA> = "EXA-066F28J3RDQ33EB4QM8LVP0TGK".parse().unwrap();
    println!("Oid<EXA> with from string: {oid}");

    // TOIDs can also be created from the raw parts
    let oid: Oid<EXA> = Oid::with_uuid("018cf159-8368-711a-915a-7dbc2b9fc70f".parse::<Uuid>()?);
    println!("Oid from raw parts: {oid}");

    // One can retrieve the various parts of the OID if needed
    println!("Components of {oid}:");
    println!("\tPreifx: {}", oid.prefix());
    println!("\tValue: {}", oid.value());
    println!("\tUUID: {}", oid.uuid());

    // However, if we change the prefix to something that doesn't match our EXA type
    // we get an error even if the UUID is valid
    let res = "FAIL-066F28J3RDQ33EB4QM8LVP0TGK".parse::<Oid<EXA>>();
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), Error::InvalidPrefix { valid_until: 0 });

    Ok(())
}
