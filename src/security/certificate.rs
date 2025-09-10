use super::utils::{load_key_from_pem, validity_period};
use rcgen::{
    BasicConstraints, Certificate, CertificateParams, CertificateSigningRequest,
    CertificateSigningRequestParams, DnType, DnValue::PrintableString, ExtendedKeyUsagePurpose,
    IsCa, KeyPair, KeyUsagePurpose,
};

const CA_VALIDITY_DAYS: i32 = 365000;

/// Certificate parameters
///
/// # Fields
///
/// * `san` - Addresses and domains to connect to the router
/// * `validity_days` - Validity days
/// * `common_name` - Role of the user : writer.local, reader.local, logger.local, platform.local, admin.local
///
/// # Example
///
/// ```
/// let cert_params = CertParams {
///     san: vec!["localhost".into(), "127.0.0.1".into()],
///     validity_days: 365,
///     common_name: "writer.local".into(),
/// };
/// ```
///
pub struct CertParams {
    pub san: Vec<String>,
    pub validity_days: i32,
    pub common_name: String,
}

/// ------ ROOT CA ------

/// Set certificate authority parameters
fn get_ca_params() -> CertificateParams {
    let mut params =
        CertificateParams::new(Vec::default()).expect("empty subject alt name can't produce error");
    let (yesterday, tomorrow) = validity_period(CA_VALIDITY_DAYS);
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

/// Generate a root CA certificate and key
pub fn generate_root_ca() -> (Certificate, KeyPair) {
    let params = get_ca_params();

    let key_pair = KeyPair::generate().unwrap();
    let root_ca_cert = params.clone().self_signed(&key_pair).unwrap();
    (root_ca_cert, key_pair)
}

/// Generate a root CA certificate with a key
pub fn generate_root_ca_with_key(key: &KeyPair) -> Certificate {
    let params = get_ca_params();
    let cert = params.clone().self_signed(&key).unwrap();
    cert
}

/// Get a root CA certificate and key from a .pem key file
pub fn get_root_ca(key_path: &str) -> (Certificate, KeyPair) {
    let key_pair = load_key_from_pem(key_path).unwrap();
    let params = get_ca_params();
    let root_ca_cert = params.clone().self_signed(&key_pair).unwrap();
    (root_ca_cert, key_pair)
}

/// ------ SERVER CERTIFICATE ------

/// Generate a server certificate and key (for router) in the chain of trust
pub fn generate_cert_server_with_san(
    ca: &Certificate,
    ca_key: &KeyPair,
    cert_params: CertParams,
) -> (Certificate, KeyPair) {
    let mut params = CertificateParams::new(cert_params.san).unwrap();
    let (yesterday, tomorrow) = validity_period(cert_params.validity_days);
    params
        .distinguished_name
        .push(DnType::CommonName, cert_params.common_name);
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

/// Generate a server certificate and key (for router) in the chain of trust from the .pem root key file
pub fn generate_cert_server_from_pem_with_san(
    ca_key_path: &str,
    cert_params: CertParams,
) -> (Certificate, KeyPair) {
    let (ca, key_pair) = get_root_ca(ca_key_path);

    let (cert, key) = generate_cert_server_with_san(&ca, &key_pair, cert_params);
    return (cert, key);
}

/// ------ CLIENT CERTIFICATE ------

/// Generate a client certificate and key in the chain of trust
pub fn generate_cert_client_with_san(
    ca: &Certificate,
    ca_key: &KeyPair,
    cert_params: CertParams,
) -> (Certificate, KeyPair) {
    let mut params = CertificateParams::new(cert_params.san).unwrap();
    let (yesterday, tomorrow) = validity_period(cert_params.validity_days);
    params
        .distinguished_name
        .push(DnType::CommonName, cert_params.common_name);
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

/// Generate a client certificate and key in the chain of trust from the .pem root key file
pub fn generate_cert_client_from_pem_with_san(
    ca_key_path: &str,
    cert_params: CertParams,
) -> (Certificate, KeyPair) {
    let (ca, key_pair) = get_root_ca(ca_key_path);

    let (cert, key) = generate_cert_client_with_san(&ca, &key_pair, cert_params);
    return (cert, key);
}

/// ------ CERTIFICATE SIGNING REQUEST ------

/// Generate a client certificate signing request
pub fn generate_csr_client(
    client_key: &KeyPair,
    san: Vec<String>,
    validity_days: i32,
) -> CertificateSigningRequest {
    let mut params = CertificateParams::new(san).unwrap();
    let (yesterday, tomorrow) = validity_period(validity_days);
    params
        .extended_key_usages
        .push(ExtendedKeyUsagePurpose::ClientAuth);
    params.not_before = yesterday;
    params.not_after = tomorrow;

    let csr = params.serialize_request(client_key).unwrap();

    return csr;
}

/// Sign a client certificate signing request with a CA certificate and key
pub fn sign_csr_with_ca(
    csr_pem: &str,
    ca_cert: &Certificate,
    ca_key: &KeyPair,
    cn: &str,
    validity_days: i32,
) -> Result<Certificate, rcgen::Error> {
    let mut csr_params = CertificateSigningRequestParams::from_pem(csr_pem)?;

    let (yesterday, tomorrow) = validity_period(validity_days);

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
