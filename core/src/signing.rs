use crate::types::*;
use crate::Result;
use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Verifier};
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::Sha256;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::Serialize;

/// Keypair for signing provenance nodes
#[derive(Debug, Clone)]
pub struct SigningKeypair {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl SigningKeypair {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let mut secret_key = [0u8; 32];
        csprng.fill_bytes(&mut secret_key);
        let signing_key = SigningKey::from_bytes(&secret_key);
        let verifying_key = signing_key.verifying_key();
        Self { signing_key, verifying_key }
    }

    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self> {
        let signing_key = SigningKey::from_bytes(bytes);
        let verifying_key = signing_key.verifying_key();
        Ok(Self { signing_key, verifying_key })
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(self.verifying_key.to_bytes())
    }

    pub fn private_key_hex(&self) -> String {
        hex::encode(self.signing_key.to_bytes())
    }
}

/// Sign a provenance node (canonicalized, excluding signature field)
pub fn sign_node(node: &ProvenanceNode, keypair: &SigningKeypair) -> Result<Signature> {
    let canonical = canonicalize_node(node)?;
    let signature = keypair.signing_key.sign(&canonical);
    Ok(Signature {
        algorithm: "ed25519".to_string(),
        public_key: keypair.public_key_hex(),
        signature: hex::encode(signature.to_bytes()),
    })
}

/// Verify a node's signature
pub fn verify_node(node: &ProvenanceNode) -> Result<bool> {
    let Some(sig) = &node.signature else {
        return Ok(false);
    };

    if sig.algorithm != "ed25519" {
        return Err(crate::WitnessError::Crypto("Unsupported algorithm".to_string()));
    }

    let canonical = canonicalize_node(node)?;
    let public_key_bytes = hex::decode(&sig.public_key)
        .map_err(|e| crate::WitnessError::Crypto(format!("Invalid public key hex: {}", e)))?;
    let signature_bytes = hex::decode(&sig.signature)
        .map_err(|e| crate::WitnessError::Crypto(format!("Invalid signature hex: {}", e)))?;

    let verifying_key = VerifyingKey::from_bytes(&public_key_bytes.try_into()
        .map_err(|_| crate::WitnessError::Crypto("Invalid public key length".to_string()))?)
        .map_err(|e| crate::WitnessError::Crypto(format!("Invalid public key: {}", e)))?;

    let signature = ed25519_dalek::Signature::from_bytes(&signature_bytes.try_into()
        .map_err(|_| crate::WitnessError::Crypto("Invalid signature length".to_string()))?);

    Ok(verifying_key.verify(&canonical, &signature).is_ok())
}

/// Create canonical byte representation of a node for signing (excludes signature field)
fn canonicalize_node(node: &ProvenanceNode) -> Result<Vec<u8>> {
    #[derive(Serialize)]
    struct CanonicalNode {
        id: Uuid,
        cid: CID,
        epistemic_type: EpistemicType,
        payload: serde_json::Value,
        author: Author,
        timestamp: DateTime<Utc>,
        parents: Vec<Uuid>,
        labels: Vec<String>,
        domain: Option<String>,
        source_uri: Option<String>,
    }

    let canonical = CanonicalNode {
        id: node.id,
        cid: node.cid.clone(),
        epistemic_type: node.epistemic_type,
        payload: node.payload.clone(),
        author: node.author.clone(),
        timestamp: node.timestamp,
        parents: node.parents.clone(),
        labels: node.labels.clone(),
        domain: node.domain.clone(),
        source_uri: node.source_uri.clone(),
    };

    // Use deterministic JSON serialization
    let json = serde_json::to_vec(&canonical)?;
    Ok(json)
}

/// Compute CID for any serializable value
pub fn compute_cid<T: Serialize>(value: &T) -> Result<CID> {
    let json = serde_json::to_vec(value)?;
    Ok(CID::new(&json))
}

/// Verify CID matches content
pub fn verify_cid<T: Serialize>(value: &T, expected_cid: &CID) -> Result<bool> {
    let computed = compute_cid(value)?;
    Ok(computed == *expected_cid)
}