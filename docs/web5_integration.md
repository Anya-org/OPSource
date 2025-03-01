# Web5 Integration in OPSource

This document provides an overview of the Web5 integration in the OPSource project, including the implementation of decentralized identifiers (DIDs), verifiable credentials, and decentralized web nodes (DWNs) with Bitcoin anchoring.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Components](#components)
  - [Decentralized Identifiers (DIDs)](#decentralized-identifiers-dids)
  - [Verifiable Credentials](#verifiable-credentials)
  - [Decentralized Web Nodes (DWNs)](#decentralized-web-nodes-dwns)
  - [Bitcoin Anchoring](#bitcoin-anchoring)
- [API Reference](#api-reference)
- [Usage Examples](#usage-examples)
- [Security Considerations](#security-considerations)
- [Future Enhancements](#future-enhancements)

## Overview

The OPSource project integrates the Web5 stack to provide a comprehensive solution for decentralized identity and data storage. The Web5 implementation includes the following key features:

- **Decentralized Identifiers (DIDs)**: Self-sovereign identities that are cryptographically verifiable and controlled by their owners.
- **Verifiable Credentials**: Attestations or claims about an entity that can be cryptographically verified and are tamper-evident.
- **Decentralized Web Nodes (DWNs)**: Personal data stores that allow users to store and manage their data securely.
- **Bitcoin Anchoring**: Enhanced security and integrity by anchoring data to the Bitcoin blockchain.

## Architecture

The Web5 integration in OPSource follows a modular architecture:

```
                                  +-------------------+
                                  |     API Server    |
                                  +-------------------+
                                           |
                +------------------------------------------------------+
                |                          |                           |
    +-----------------------+  +------------------------+  +------------------------+
    |   DID Management     |  | Credential Management  |  |    DWN Management     |
    +-----------------------+  +------------------------+  +------------------------+
                |                          |                           |
                +------------------------------------------------------+
                                           |
                                  +-------------------+
                                  | Bitcoin Integration|
                                  +-------------------+
```

## Components

### Decentralized Identifiers (DIDs)

DIDs in OPSource are implemented based on the W3C DID specification. The implementation supports:

- Creation of new DIDs with cryptographic key pairs
- Resolution of DIDs to their corresponding DID documents
- Management of DID documents, including key rotation and service endpoints
- Support for multiple DID methods, with a focus on `did:key` and `did:web`

Example DID document:

```json
{
  "@context": "https://www.w3.org/ns/did/v1",
  "id": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
  "verificationMethod": [{
    "id": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK#z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
    "type": "Ed25519VerificationKey2020",
    "controller": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
    "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
  }],
  "authentication": [
    "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK#z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
  ],
  "assertionMethod": [
    "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK#z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
  ]
}
```

### Verifiable Credentials

The OPSource implementation of verifiable credentials follows the W3C Verifiable Credentials Data Model. Key features include:

- Issuance of credentials with customizable claims
- Verification of credentials, including signature verification and expiration checks
- Revocation of credentials through a revocation registry
- Bitcoin anchoring of credentials for enhanced security and auditability
- Support for selective disclosure and zero-knowledge proofs

Example verifiable credential:

```json
{
  "@context": [
    "https://www.w3.org/2018/credentials/v1",
    "https://www.w3.org/2018/credentials/examples/v1"
  ],
  "id": "http://example.edu/credentials/1872",
  "type": ["VerifiableCredential", "AlumniCredential"],
  "issuer": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
  "issuanceDate": "2023-06-23T19:23:24Z",
  "credentialSubject": {
    "id": "did:key:z6MkrJVnaZkeFzdQcWZJPEBV6CvgBz9XLLBxyve8jvMa9LwF",
    "alumniOf": "Example University"
  },
  "bitcoinAnchoring": {
    "txid": "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b",
    "blockHeight": 680000,
    "confirmations": 5,
    "timestamp": "2021-05-01T12:00:00Z"
  },
  "proof": {
    "type": "Ed25519Signature2020",
    "created": "2023-06-23T19:23:24Z",
    "verificationMethod": "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK#z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK",
    "proofPurpose": "assertionMethod",
    "proofValue": "z43gF7qTTRM4m9wK2ZbmPmN8x4NX3jSADYQbwH8J9yHQbTMc6QNVoXnVPrcYfFWqjq8QZvz68C3dRUAWV5FSYGwvY"
  }
}
```

### Decentralized Web Nodes (DWNs)

DWNs provide personal data storage for users. The OPSource implementation includes:

- Creation and management of DWNs for DIDs
- Support for different data types and schemas
- CRUD operations for data records
- Query capabilities for retrieving and filtering data
- Access control based on DIDs and permissions
- Bitcoin anchoring for data integrity and provenance

### Bitcoin Anchoring

Bitcoin anchoring enhances the security and integrity of Web5 components by:

- Recording cryptographic commitments to data on the Bitcoin blockchain
- Leveraging Bitcoin's immutability and security for data provenance
- Providing verifiable timestamps for data
- Supporting verification of data integrity through blockchain confirmations

## API Reference

The OPSource Web5 API provides the following endpoints:

### DID Endpoints

- `POST /api/web5/did`: Create a new DID
- `GET /api/web5/did/{did}`: Resolve a DID

### Credential Endpoints

- `POST /api/web5/credential/issue`: Issue a verifiable credential
- `POST /api/web5/credential/verify`: Verify a credential
- `POST /api/web5/credential/revoke`: Revoke a credential
- `GET /api/web5/credential/status/{id}`: Check the status of a credential

### DWN Endpoints

- `POST /api/web5/dwn/create`: Create a new DWN for a DID
- `POST /api/web5/dwn/process`: Process a standard DWN message
- `POST /api/web5/dwn/process/enhanced`: Process a DWN message with enhanced features like Bitcoin anchoring
- `POST /api/web5/dwn/query`: Query a DWN
- `POST /api/web5/dwn/query/anchored`: Query a DWN with anchoring verification
- `POST /api/web5/dwn/anchor/{did}/{message_id}`: Anchor a DWN message to Bitcoin
- `GET /api/web5/dwn/status/{did}/{message_id}`: Get anchoring status for a DWN message
- `GET /api/web5/dwn/verify/{did}/{message_id}`: Verify Bitcoin anchoring for a DWN message

## Usage Examples

### Creating a DID and DWN

```python
import requests

# Create a DID
response = requests.post("http://localhost:8000/api/web5/did")
did_data = response.json()
did = did_data["id"]

# Create a DWN for the DID
response = requests.post("http://localhost:8000/api/web5/dwn/create", json={"did": did})
```

### Storing Data in a DWN with Bitcoin Anchoring

```python
import requests
import uuid
import json
import time

# Create a message
message = {
    "id": str(uuid.uuid4()),
    "type": "RecordsWrite",
    "from": did,
    "to": did,
    "recordId": str(uuid.uuid4()),
    "data": json.dumps({
        "title": "Test Document",
        "content": "This is a test document",
        "tags": ["test", "web5"]
    }),
    "dataFormat": "application/json",
    "published": True,
    "dateCreated": int(time.time() * 1000)
}

# Enhanced options with Bitcoin anchoring
options = {
    "anchor_to_bitcoin": True,
    "wait_for_broadcast": True,
    "include_anchoring_status": True
}

# Store data with anchoring
response = requests.post(
    "http://localhost:8000/api/web5/dwn/process/enhanced",
    json={"did": did, "message": message, "options": options}
)
```

### Querying a DWN

```python
import requests
import uuid

# Create a query message
query = {
    "id": str(uuid.uuid4()),
    "type": "RecordsQuery",
    "from": did,
    "to": did,
    "dataFormat": "application/json"
}

# Query the DWN
response = requests.post(
    "http://localhost:8000/api/web5/dwn/query",
    json={"did": did, "query": query}
)
```

## Security Considerations

When using the Web5 functionality in OPSource, consider the following security aspects:

1. **Private Key Management**: DIDs and their private keys must be securely stored and managed.
2. **Credential Validation**: Always verify credentials, including their signatures, expiration, and revocation status.
3. **Bitcoin Transaction Fees**: Consider the costs associated with Bitcoin anchoring, especially for frequent operations.
4. **Confirmation Thresholds**: Define appropriate confirmation thresholds for Bitcoin-anchored data based on security requirements.
5. **Access Control**: Configure proper access controls for DWNs to protect sensitive data.

## Future Enhancements

The Web5 integration in OPSource will be enhanced with the following features:

1. **DID Rotation and Recovery**: Improved mechanisms for key rotation and recovery.
2. **Selective Disclosure**: Enhanced support for selective disclosure in verifiable credentials.
3. **DWN Federation**: Support for federated DWNs for improved scalability and reliability.
4. **Advanced Query Capabilities**: More powerful query options for DWNs.
5. **Lightning Network Integration**: Faster and cheaper anchoring using the Lightning Network.
6. **Cross-Chain Anchoring**: Support for anchoring to multiple blockchains.
7. **Standard Interoperability**: Enhanced alignment with emerging Web5 standards.
