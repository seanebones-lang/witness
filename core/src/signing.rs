use crate::Result;
use crate::types::*;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rand::RngCore;
use rand::rngs::OsRng;
use serde::Serialize;
use std::collections::BTreeMap;
use uuid::Uuid;

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
        Self {
            signing_key,
            verifying_key,
        }
    }

    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self> {
        let signing_key = SigningKey::from_bytes(bytes);
        let verifying_key = signing_key.verifying_key();
        Ok(Self {
            signing_key,
            verifying_key,
        })
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
        return Err(crate::WitnessError::Crypto(
            "Unsupported algorithm".to_string(),
        ));
    }

    let canonical = canonicalize_node(node)?;
    let public_key_bytes = hex::decode(&sig.public_key)
        .map_err(|e| crate::WitnessError::Crypto(format!("Invalid public key hex: {}", e)))?;
    let signature_bytes = hex::decode(&sig.signature)
        .map_err(|e| crate::WitnessError::Crypto(format!("Invalid signature hex: {}", e)))?;

    let verifying_key = VerifyingKey::from_bytes(
        &public_key_bytes
            .try_into()
            .map_err(|_| crate::WitnessError::Crypto("Invalid public key length".to_string()))?,
    )
    .map_err(|e| crate::WitnessError::Crypto(format!("Invalid public key: {}", e)))?;

    let signature = ed25519_dalek::Signature::from_bytes(
        &signature_bytes
            .try_into()
            .map_err(|_| crate::WitnessError::Crypto("Invalid signature length".to_string()))?,
    );

    Ok(verifying_key.verify(&canonical, &signature).is_ok())
}

/// Create canonical byte representation of a node for signing (excludes signature field)
fn canonicalize_node(node: &ProvenanceNode) -> Result<Vec<u8>> {
    // Keep the established Author field order, but never sign HashMap iteration
    // order: a deserialized map has a new random seed and may reorder its keys.
    #[derive(Serialize)]
    struct CanonicalAuthor<'a> {
        id: &'a str,
        name: &'a Option<String>,
        author_type: AuthorType,
        metadata: BTreeMap<&'a str, &'a serde_json::Value>,
    }
    #[derive(Serialize)]
    struct CanonicalNode<'a> {
        id: Uuid,
        cid: CID,
        epistemic_type: EpistemicType,
        payload: serde_json::Value,
        author: CanonicalAuthor<'a>,
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
        author: CanonicalAuthor {
            id: &node.author.id,
            name: &node.author.name,
            author_type: node.author.author_type,
            metadata: node
                .author
                .metadata
                .iter()
                .map(|(key, value)| (key.as_str(), value))
                .collect(),
        },
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn node() -> ProvenanceNode {
        let payload = serde_json::json!({"reading": 18.4, "unit": "degC"});
        ProvenanceNode {
            id: Uuid::new_v4(),
            cid: compute_cid(&payload).unwrap(),
            epistemic_type: EpistemicType::Observed,
            payload,
            author: Author {
                id: "instrument:test".to_string(),
                name: Some("Test instrument".to_string()),
                author_type: AuthorType::Instrument,
                metadata: HashMap::new(),
            },
            timestamp: Utc::now(),
            parents: Vec::new(),
            signature: None,
            labels: vec!["test".to_string()],
            domain: Some("climate".to_string()),
            source_uri: None,
        }
    }

    #[test]
    fn signature_verifies_unchanged_node() {
        let keypair = SigningKeypair::generate();
        let mut node = node();
        node.signature = Some(sign_node(&node, &keypair).unwrap());

        assert!(verify_node(&node).unwrap());
        assert!(verify_cid(&node.payload, &node.cid).unwrap());
    }

    #[test]
    fn signature_rejects_tampered_payload() {
        let keypair = SigningKeypair::generate();
        let mut node = node();
        node.signature = Some(sign_node(&node, &keypair).unwrap());
        node.payload["reading"] = serde_json::json!(99.9);

        assert!(!verify_node(&node).unwrap());
        assert!(!verify_cid(&node.payload, &node.cid).unwrap());
    }

    #[test]
    fn signature_survives_metadata_serialization_round_trips() {
        let keypair = SigningKeypair::generate();
        let mut node = node();
        for index in 0..32 {
            node.author.metadata.insert(
                format!("field-{index:02}"),
                serde_json::json!({"value": index}),
            );
        }
        node.signature = Some(sign_node(&node, &keypair).unwrap());
        let serialized = serde_json::to_string(&node).unwrap();
        for _ in 0..20 {
            let restored: ProvenanceNode = serde_json::from_str(&serialized).unwrap();
            assert!(
                verify_node(&restored).unwrap(),
                "signature changed after deserializing author metadata"
            );
        }
    }
}
