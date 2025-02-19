use did_key::DIDKey;
use verifiable_credentials::VerifiableCredential;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("DID creation failed: {0}")]
    DIDCreationError(String),
    #[error("Credential verification failed: {0}")]
    CredentialVerificationError(String),
}

pub struct Identity {
    did_key: DIDKey,
}

impl Identity {
    pub fn new() -> Result<Self, IdentityError> {
        let did_key = DIDKey::generate().map_err(|e| IdentityError::DIDCreationError(e.to_string()))?;
        Ok(Self { did_key })
    }

    pub fn create_did(&self) -> Result<String, IdentityError> {
        Ok(self.did_key.to_did())
    }

    pub fn verify_credential(&self, credential: &str) -> Result<bool, IdentityError> {
        let vc = VerifiableCredential::from_json_str(credential)
            .map_err(|e| IdentityError::CredentialVerificationError(e.to_string()))?;
        vc.verify().map_err(|e| IdentityError::CredentialVerificationError(e.to_string()))
    }
}