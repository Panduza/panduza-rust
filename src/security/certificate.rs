use rcgen::{
    BasicConstraints, Certificate, CertificateParams, CertificateSigningRequest,
    CertificateSigningRequestParams, DnType, DnValue::PrintableString, ExtendedKeyUsagePurpose,
    IsCa, KeyPair, KeyUsagePurpose,
};
use std::fs;
use time::{Duration, OffsetDateTime};

fn get_ca_params() -> CertificateParams {
    let mut params =
        CertificateParams::new(Vec::default()).expect("empty subject alt name can't produce error");
    let (yesterday, tomorrow) = validity_period(365000);
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.distinguished_name.push(
        DnType::CountryName,
        PrintableString("FR".try_into().unwrap()),
    );
    params
        .distinguished_name
        .push(DnType::OrganizationName, "Panduza");
    params
        .distinguished_name
        .push(DnType::CommonName, "Panduza Root CA");
    params.key_usages.push(KeyUsagePurpose::DigitalSignature);
    params.key_usages.push(KeyUsagePurpose::KeyCertSign);
    params.key_usages.push(KeyUsagePurpose::CrlSign);

    params.not_before = yesterday;
    params.not_after = tomorrow;
    return params;
}

pub fn generate_root_ca() -> (Certificate, KeyPair) {
    let params = get_ca_params();

    let key_pair = KeyPair::generate().unwrap();
    let cert = params.clone().self_signed(&key_pair).unwrap();
    (cert, key_pair)
}

pub fn get_root_ca(key_path: &str) -> (Certificate, KeyPair) {
    let key_pair = load_key_from_pem(key_path).unwrap();
    let params = get_ca_params();
    let cert = params.clone().self_signed(&key_pair).unwrap();
    (cert, key_pair)
}

pub fn generate_root_ca_with_key(key: &KeyPair) -> Certificate {
    let params = get_ca_params();
    let cert = params.clone().self_signed(&key).unwrap();
    cert
}

pub fn generate_cert_server(ca: &Certificate, ca_key: &KeyPair) -> (Certificate, KeyPair) {
    let names = vec!["localhost".into(), "127.0.0.1".into()];
    let mut params = CertificateParams::new(names).unwrap();
    let (yesterday, tomorrow) = validity_period(365000);
    params
        .distinguished_name
        .push(DnType::CommonName, "Panduza");
    params.use_authority_key_identifier_extension = true;
    params.key_usages.push(KeyUsagePurpose::DigitalSignature);
    params.key_usages.push(KeyUsagePurpose::KeyEncipherment);
    params
        .extended_key_usages
        .push(ExtendedKeyUsagePurpose::ServerAuth);
    params.not_before = yesterday;
    params.not_after = tomorrow;

    let key_pair = KeyPair::generate().unwrap();
    let cert = params.signed_by(&key_pair, ca, ca_key).unwrap();

    (cert, key_pair)
}

pub fn generate_cert_client(ca: &Certificate, ca_key: &KeyPair) -> (Certificate, KeyPair) {
    let names = vec!["localhost".into(), "127.0.0.1".into()];
    let mut params = CertificateParams::new(names).unwrap();
    let (yesterday, tomorrow) = validity_period(365000);
    params
        .distinguished_name
        .push(DnType::CommonName, "Panduza");
    params.use_authority_key_identifier_extension = true;
    params.key_usages.push(KeyUsagePurpose::DigitalSignature);
    params
        .extended_key_usages
        .push(ExtendedKeyUsagePurpose::ClientAuth);
    params.not_before = yesterday;
    params.not_after = tomorrow;

    let key_pair = KeyPair::generate().unwrap();
    let cert = params.signed_by(&key_pair, ca, ca_key).unwrap();
    (cert, key_pair)
}

pub fn generate_cert_server_with_san(
    ca: &Certificate,
    ca_key: &KeyPair,
    san: &str,
) -> (Certificate, KeyPair) {
    let names = vec!["localhost".into(), "127.0.0.1".into(), san.into()];
    let mut params = CertificateParams::new(names).unwrap();
    let (yesterday, tomorrow) = validity_period(365000);
    params.distinguished_name.push(DnType::CommonName, san);
    params.use_authority_key_identifier_extension = true;
    params.key_usages.push(KeyUsagePurpose::DigitalSignature);
    params.key_usages.push(KeyUsagePurpose::KeyEncipherment);
    params
        .extended_key_usages
        .push(ExtendedKeyUsagePurpose::ServerAuth);
    params.not_before = yesterday;
    params.not_after = tomorrow;

    let key_pair = KeyPair::generate().unwrap();
    let cert = params.signed_by(&key_pair, ca, ca_key).unwrap();
    (cert, key_pair)
}

pub fn generate_cert_client_with_san(
    ca: &Certificate,
    ca_key: &KeyPair,
    san: &str,
    days: i32,
) -> (Certificate, KeyPair) {
    let names = vec!["localhost".into(), "127.0.0.1".into(), san.into()];
    let mut params = CertificateParams::new(names).unwrap();
    let (yesterday, tomorrow) = validity_period(days);
    params.distinguished_name.push(DnType::CommonName, san);
    params.use_authority_key_identifier_extension = true;
    params.key_usages.push(KeyUsagePurpose::DigitalSignature);
    params
        .extended_key_usages
        .push(ExtendedKeyUsagePurpose::ClientAuth);
    params.not_before = yesterday;
    params.not_after = tomorrow;

    let key_pair = KeyPair::generate().unwrap();
    let cert = params.signed_by(&key_pair, ca, ca_key).unwrap();
    (cert, key_pair)
}

pub fn generate_cert_server_from_pem(key_path: &str) -> (Certificate, KeyPair) {
    let (ca, key_pair) = get_root_ca(key_path);

    let (cert, key) = generate_cert_server(&ca, &key_pair);
    return (cert, key);
}

pub fn generate_cert_client_from_pem(key_path: &str) -> (Certificate, KeyPair) {
    let (ca, key_pair) = get_root_ca(key_path);

    let (cert, key) = generate_cert_client(&ca, &key_pair);
    return (cert, key);
}

pub fn generate_cert_server_from_pem_with_san(key_path: &str, san: &str) -> (Certificate, KeyPair) {
    let (ca, key_pair) = get_root_ca(key_path);

    let (cert, key) = generate_cert_server_with_san(&ca, &key_pair, san);
    return (cert, key);
}

pub fn generate_cert_client_from_pem_with_san(
    key_path: &str,
    san: &str,
    days: i32,
) -> (Certificate, KeyPair) {
    let (ca, key_pair) = get_root_ca(key_path);

    let (cert, key) = generate_cert_client_with_san(&ca, &key_pair, san, days);
    return (cert, key);
}

pub fn load_key_from_pem(key_path: &str) -> Result<KeyPair, Box<dyn std::error::Error>> {
    let ca_key_pem = fs::read_to_string(key_path)?;
    let ca_key = KeyPair::from_pem(&ca_key_pem)?;

    Ok(ca_key)
}

pub fn generate_key() -> KeyPair {
    KeyPair::generate().expect("Fail to generate key")
}

fn validity_period(days: i32) -> (OffsetDateTime, OffsetDateTime) {
    let day = Duration::new(86400, 0);
    let yesterday = OffsetDateTime::now_utc().checked_sub(day).unwrap();
    let after = OffsetDateTime::now_utc().checked_add(day * days).unwrap();
    (yesterday, after)
}

pub fn generate_csr_client(client_key: &KeyPair) -> CertificateSigningRequest {
    let names = vec!["localhost".into(), "127.0.0.1".into()];
    let mut params = CertificateParams::new(names).unwrap();
    let (yesterday, tomorrow) = validity_period(365);
    params
        .extended_key_usages
        .push(ExtendedKeyUsagePurpose::ClientAuth);
    params.not_before = yesterday;
    params.not_after = tomorrow;

    let csr = params.serialize_request(client_key).unwrap();

    return csr;
}

pub fn sign_csr_with_ca(
    csr_pem: &str,
    ca_cert: &Certificate,
    ca_key: &KeyPair,
    cn: &str,
) -> Result<Certificate, rcgen::Error> {
    let mut csr_params = CertificateSigningRequestParams::from_pem(csr_pem)?;

    let (yesterday, tomorrow) = validity_period(30);

    csr_params.params.not_before = yesterday;
    csr_params.params.not_after = tomorrow;

    csr_params
        .params
        .distinguished_name
        .push(DnType::CommonName, cn);

    csr_params.params.use_authority_key_identifier_extension = true;

    let cert = csr_params.signed_by(ca_cert, ca_key)?;

    Ok(cert)
}
