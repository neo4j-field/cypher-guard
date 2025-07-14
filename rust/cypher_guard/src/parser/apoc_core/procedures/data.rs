// APOC data procedures
// Handles apoc.data.* procedures for data operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC data procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static DATA_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.data.url(url STRING)
        ("apoc.data.url", vec![
            ("url", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.email(email STRING)
        ("apoc.data.email", vec![
            ("email", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.domain(domain STRING)
        ("apoc.data.domain", vec![
            ("domain", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.subdomain(domain STRING)
        ("apoc.data.subdomain", vec![
            ("domain", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.phone(phone STRING)
        ("apoc.data.phone", vec![
            ("phone", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.creditCard(creditCard STRING)
        ("apoc.data.creditCard", vec![
            ("creditCard", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.iban(iban STRING)
        ("apoc.data.iban", vec![
            ("iban", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.isbn(isbn STRING)
        ("apoc.data.isbn", vec![
            ("isbn", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.isin(isin STRING)
        ("apoc.data.isin", vec![
            ("isin", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.cusip(cusip STRING)
        ("apoc.data.cusip", vec![
            ("cusip", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.sedol(sedol STRING)
        ("apoc.data.sedol", vec![
            ("sedol", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.figi(figi STRING)
        ("apoc.data.figi", vec![
            ("figi", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.lei(lei STRING)
        ("apoc.data.lei", vec![
            ("lei", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.swift(swift STRING)
        ("apoc.data.swift", vec![
            ("swift", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.bic(bic STRING)
        ("apoc.data.bic", vec![
            ("bic", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.routingNumber(routingNumber STRING)
        ("apoc.data.routingNumber", vec![
            ("routingNumber", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.accountNumber(accountNumber STRING)
        ("apoc.data.accountNumber", vec![
            ("accountNumber", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.ssn(ssn STRING)
        ("apoc.data.ssn", vec![
            ("ssn", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.ein(ein STRING)
        ("apoc.data.ein", vec![
            ("ein", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.itin(itin STRING)
        ("apoc.data.itin", vec![
            ("itin", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.passport(passport STRING)
        ("apoc.data.passport", vec![
            ("passport", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.driversLicense(driversLicense STRING)
        ("apoc.data.driversLicense", vec![
            ("driversLicense", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.vin(vin STRING)
        ("apoc.data.vin", vec![
            ("vin", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.licensePlate(licensePlate STRING)
        ("apoc.data.licensePlate", vec![
            ("licensePlate", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.ipv4(ipv4 STRING)
        ("apoc.data.ipv4", vec![
            ("ipv4", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.ipv6(ipv6 STRING)
        ("apoc.data.ipv6", vec![
            ("ipv6", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.mac(mac STRING)
        ("apoc.data.mac", vec![
            ("mac", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.uuid(uuid STRING)
        ("apoc.data.uuid", vec![
            ("uuid", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.hash(hash STRING)
        ("apoc.data.hash", vec![
            ("hash", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.base64(base64 STRING)
        ("apoc.data.base64", vec![
            ("base64", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.hex(hex STRING)
        ("apoc.data.hex", vec![
            ("hex", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.binary(binary STRING)
        ("apoc.data.binary", vec![
            ("binary", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.json(json STRING)
        ("apoc.data.json", vec![
            ("json", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.xml(xml STRING)
        ("apoc.data.xml", vec![
            ("xml", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.csv(csv STRING)
        ("apoc.data.csv", vec![
            ("csv", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.tsv(tsv STRING)
        ("apoc.data.tsv", vec![
            ("tsv", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.yaml(yaml STRING)
        ("apoc.data.yaml", vec![
            ("yaml", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.toml(toml STRING)
        ("apoc.data.toml", vec![
            ("toml", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.data.ini(ini STRING)
        ("apoc.data.ini", vec![
            ("ini", ApocType::String)
        ], vec![("result", ApocType::Any)]),
    ]
});

pub fn get_all_data_procedures() -> &'static [ProcedureSignature] {
    &DATA_PROCEDURES
}

// TODO: Implement data procedure validation
pub fn validate_data_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement data procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_url_signature() {
        let procedures = get_all_data_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.data.url")
            .expect("apoc.data.url should be defined");
        
        assert_eq!(procedure.1.len(), 1); // 1 parameter
        assert_eq!(procedure.1[0].0, "url");
        assert_eq!(procedure.1[0].1, ApocType::String);
    }

    #[test]
    fn test_data_email_signature() {
        let procedures = get_all_data_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.data.email")
            .expect("apoc.data.email should be defined");
        
        assert_eq!(procedure.1.len(), 1); // 1 parameter
        assert_eq!(procedure.1[0].0, "email");
        assert_eq!(procedure.1[0].1, ApocType::String);
    }

    #[test]
    fn test_data_credit_card_signature() {
        let procedures = get_all_data_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.data.creditCard")
            .expect("apoc.data.creditCard should be defined");
        
        assert_eq!(procedure.1.len(), 1); // 1 parameter
        assert_eq!(procedure.1[0].0, "creditCard");
        assert_eq!(procedure.1[0].1, ApocType::String);
    }

    #[test]
    fn test_all_data_procedures_have_signatures() {
        let procedures = get_all_data_procedures();
        assert!(!procedures.is_empty(), "Should have at least one data procedure");
        
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
            assert_eq!(yields[0].0, "result", "First yield field should be 'result'");
        }
    }
}


