# Data Flows & Authentication Architecture
## Progressive Authentication, Personal Data Integration, and Privacy Patterns

**Version**: 2.0
**Date**: February 4, 2026
**Status**: Technical Specification
**Related Documents**: [ARCHITECTURE.md](../ARCHITECTURE.md), [VERIDA_INTEGRATION.md](../VERIDA_INTEGRATION.md), [CHALLENGE_ARCHITECTURES.md](./CHALLENGE_ARCHITECTURES.md)

---

## Table of Contents

1. [Progressive Authentication](#progressive-authentication)
2. [Personal Data Integration](#personal-data-integration)
3. [Privacy-Preserving AI Inference](#privacy-preserving-ai-inference)
4. [Multi-Chain Identity (Optional)](#multi-chain-identity)

---

## Progressive Authentication

### 1.1 Authentication Levels Overview

```mermaid
graph LR
    A[Anonymous] -->|PassKey Registration| P[PassKey Verified]
    A -->|Email Verification| B[Email Verified]
    P -->|Create Verida DID| D[Full Sovereign]
    B -.->|Direct path| D
    D -->|Connect Wallet| E[Wallet Connected]

    style A fill:#E0E0E0,color:#000
    style P fill:#81C784,color:#000
    style B fill:#B3E5FC,color:#000
    style D fill:#FFD54F,color:#000
    style E fill:#FFD700,color:#000
```

**Primary Path (Recommended):** Anonymous → PassKey → Sovereign → Wallet
**Legacy Path:** Anonymous → Email → Sovereign → Wallet

**Access Levels by Authentication State:**

| Level | Data Access | AI Capabilities | Identity | Security |
|-------|-------------|-----------------|----------|----------|
| **Anonymous** | None | Basic inference | Session only | None |
| **PassKey Verified** | Device + cloud sync | Personal AI | FIDO2 credential | Phishing-resistant |
| **Email Verified** *(legacy)* | Basic profile | Enhanced inference | Email-based | Medium |
| **Full Identity** | Private datastore | Fully personal AI | Self-sovereign DID | High |
| **Optional Wallet** | Multi-chain data (optional) | Full features | Multi-chain verified | Maximum |

### 1.2 PassKey Technology Overview

```mermaid
graph TB
    subgraph "PassKey Architecture (FIDO2/WebAuthn)"
        User[User Device]
        Authenticator[Platform Authenticator<br/>Face ID / Touch ID / Windows Hello]
        KeyPair[Asymmetric Key Pair<br/>Private key never leaves device]
    end

    subgraph "PassKey Sync Ecosystems"
        Apple[Apple Keychain<br/>iCloud sync]
        Google[Google Password Manager<br/>Chrome sync]
        Microsoft[Microsoft Authenticator<br/>Windows sync]
    end

    subgraph "Security Properties"
        Phishing[Phishing Resistant<br/>Origin-bound credentials]
        Replay[Replay Resistant<br/>Challenge-response]
        Privacy[Privacy Preserving<br/>No shared secrets]
    end

    User --> Authenticator
    Authenticator --> KeyPair

    KeyPair -.Synced via.-> Apple
    KeyPair -.Synced via.-> Google
    KeyPair -.Synced via.-> Microsoft

    KeyPair --> Phishing
    KeyPair --> Replay
    KeyPair --> Privacy

    style Authenticator fill:#4A90E2,color:#fff
    style Phishing fill:#50C878,color:#fff
```

### 1.3 Complete Authentication Flow (Primary Path with PassKeys)

```mermaid
sequenceDiagram
    autonumber
    participant User
    participant App as KwaaiNet App
    participant Auth as Auth Controller
    participant WebAuthn as WebAuthn API
    participant Verida
    participant Wallet as VDA Wallet

    Note over User,Wallet: Level 1: Anonymous → PassKey Verified

    User->>App: First Launch
    App->>Auth: initialize(anonymous)
    Auth-->>App: anonymous_session
    App-->>User: Basic AI Available (0% rewards)

    User->>App: Create PassKey
    App->>Auth: request_passkey_registration()
    Auth->>Auth: generate_challenge()
    Auth->>WebAuthn: navigator.credentials.create()

    WebAuthn-->>User: Biometric Prompt<br/>(Face ID / Touch ID / Windows Hello)
    User->>WebAuthn: Authenticate with biometric
    WebAuthn->>WebAuthn: Generate key pair<br/>(private key stays on device)
    WebAuthn-->>Auth: PublicKeyCredential<br/>{id, publicKey, attestation}

    Auth->>Auth: store_public_key(credential)
    Auth->>Auth: create_passkey_profile()
    Auth-->>App: passkey_verified_session
    App-->>User: Personal AI Enabled

    Note over User,Wallet: Level 2: PassKey → Full Identity

    User->>App: Create Sovereign Identity
    App->>Auth: upgrade_to_sovereign()

    Auth->>WebAuthn: navigator.credentials.get()
    WebAuthn-->>User: Biometric confirmation
    User->>WebAuthn: Confirm identity
    WebAuthn-->>Auth: Signed assertion

    Auth->>Verida: create_did(passkey_credential)
    Verida->>Verida: generate_did()
    Verida->>Verida: link_passkey_to_did()
    Verida->>Verida: register_on_chain()
    Verida-->>Auth: did_created(did:vda:polpos:0x...)

    Auth->>Verida: initialize_storage_endpoints()
    Verida-->>Auth: endpoints_configured

    Auth-->>App: full_identity_session(did)
    App-->>User: Full Identity AI Enabled

    Note over User,Wallet: Level 3: Optional Wallet Connection

    User->>App: Connect VDA Wallet
    App->>Wallet: request_connection(did)
    Wallet-->>User: Wallet Connect QR / Deep Link
    User->>Wallet: Approve Connection
    Wallet-->>App: wallet_connected(address)

    App->>Auth: upgrade_to_wallet(wallet_address)
    Auth->>Verida: link_wallet_to_did(did, address)
    Verida-->>Auth: wallet_linked

    Auth-->>App: wallet_session(did, wallet)
    App-->>User: Premium AI + Staking + Multi-Chain + Bonuses
```

### 1.4 Legacy Authentication Flow (Email Fallback)

```mermaid
sequenceDiagram
    autonumber
    participant User
    participant App as KwaaiNet App
    participant Auth as Auth Controller
    participant Email as Email Service
    participant Verida

    Note over User,Verida: For devices without PassKey support

    User->>App: First Launch (no WebAuthn)
    App->>App: detect_webauthn_support()
    App-->>User: Offer Email Registration

    User->>App: Sign Up with Email
    App->>Auth: upgrade_to_email(email)
    Auth->>Email: send_verification_code(email)
    Email-->>User: Verification Email

    User->>App: Enter Verification Code
    App->>Auth: verify_code(code)
    Auth->>Auth: create_basic_profile()
    Auth-->>App: email_verified_session
    App-->>User: Enhanced AI Enabled

    Note over User,Verida: Upgrade to Full Identity (skip biometric)

    User->>App: Create Full Identity
    App->>Auth: upgrade_to_full_identity()
    Auth->>Verida: create_did(email_credential)
    Verida->>Verida: generate_did()
    Verida-->>Auth: did_created(did:vda:polpos:0x...)
    Auth-->>App: full_identity_session(did)
    App-->>User: Full Identity AI Enabled
```

### 1.5 Authentication State Machine

```mermaid
stateDiagram-v2
    [*] --> Anonymous

    Anonymous --> PassKeyVerified: PassKey Registration<br/>(recommended)
    Anonymous --> EmailVerified: Email Verification<br/>(legacy fallback)

    PassKeyVerified --> FullIdentity: Create DID
    EmailVerified --> FullIdentity: Create DID

    FullIdentity --> OptionalWallet: Optional Wallet

    state Anonymous {
        [*] --> SessionOnly
        SessionOnly --> BasicAI
    }

    state PassKeyVerified {
        [*] --> FIDO2Credential
        FIDO2Credential --> CrossDeviceSync
        CrossDeviceSync --> PersonalAI
    }

    state EmailVerified {
        [*] --> EmailProfile
        EmailProfile --> BasicStorage
    }

    state FullIdentity {
        [*] --> VeridaDID
        VeridaDID --> PrivateDatastore
        PrivateDatastore --> E2EEncryption
    }

    state OptionalWallet {
        [*] --> WalletIntegration
        WalletIntegration --> MultiChain
        MultiChain --> PremiumFeatures
    }

    OptionalWallet --> [*]

    note right of Anonymous
        AI Access: Basic
        Data: None
        Security: None
    end note

    note right of PassKeyVerified
        AI Access: Personal
        Data: Device + Cloud Sync
        Security: Phishing-resistant
    end note

    note right of EmailVerified
        AI Access: Enhanced
        Data: Basic Profile
        Security: Medium
    end note

    note right of FullIdentity
        AI Access: Fully Personal
        Data: Private Datastore
        Security: High (DID)
    end note

    note right of OptionalWallet
        AI Access: Full Features
        Data: Multi-Chain (Optional)
        Security: Maximum
    end note
```

### 1.6 PassKey Registration & Authentication Detail

```mermaid
sequenceDiagram
    autonumber
    participant User
    participant Browser
    participant App as KwaaiNet App
    participant WebAuthn as WebAuthn API
    participant Platform as Platform Authenticator<br/>(Secure Enclave / TPM)
    participant Server as Auth Server

    Note over User,Server: PassKey Registration (One-time setup)

    User->>App: Click "Create PassKey"
    App->>Server: POST /auth/passkey/register/begin
    Server->>Server: Generate challenge & user ID
    Server-->>App: {challenge, rpId, user, pubKeyCredParams}

    App->>WebAuthn: navigator.credentials.create(options)
    WebAuthn->>Platform: Request key generation
    Platform-->>User: Biometric prompt (Face ID / Touch ID)
    User->>Platform: Authenticate
    Platform->>Platform: Generate P-256 key pair in secure hardware
    Platform-->>WebAuthn: PublicKeyCredential

    WebAuthn-->>App: {id, rawId, response: {clientDataJSON, attestationObject}}
    App->>Server: POST /auth/passkey/register/complete
    Server->>Server: Verify attestation & store public key
    Server-->>App: {success: true, credentialId}
    App-->>User: PassKey created! Personal AI enabled

    Note over User,Server: PassKey Authentication (Subsequent logins)

    User->>App: Click "Sign in with PassKey"
    App->>Server: POST /auth/passkey/login/begin
    Server->>Server: Generate challenge
    Server-->>App: {challenge, allowCredentials, rpId}

    App->>WebAuthn: navigator.credentials.get(options)
    WebAuthn->>Platform: Request signature
    Platform-->>User: Biometric prompt
    User->>Platform: Authenticate
    Platform->>Platform: Sign challenge with private key
    Platform-->>WebAuthn: AuthenticatorAssertionResponse

    WebAuthn-->>App: {id, response: {authenticatorData, signature, clientDataJSON}}
    App->>Server: POST /auth/passkey/login/complete
    Server->>Server: Verify signature with stored public key
    Server-->>App: {success: true, sessionToken}
    App-->>User: Signed in! Welcome back
```

---

## Personal Data Integration

### 2.1 Multi-Source Personal Data Flow

```mermaid
graph TB
    subgraph "User Data Sources"
        Gmail[Gmail API<br/>Email & Contacts]
        Calendar[Calendar API<br/>Events & Meetings]
        Drive[Google Drive<br/>Documents]
        Telegram[Telegram API<br/>Messages]
        Custom[Custom Connectors<br/>Extensible]
    end

    subgraph "Verida Private Storage"
        OAuth[OAuth Manager]
        Sync[Data Sync Engine]
        Store[Encrypted Datastores]
        Schema[Schema Validator]
    end

    subgraph "Privacy Controls"
        Filter[Privacy Filter]
        ACL[Access Control]
        Encryption[E2E Encryption]
        Audit[Audit Logger]
    end

    subgraph "AI Context Layer"
        Analyzer[Relevance Analyzer]
        Context[Context Builder]
        Enhance[Prompt Enhancer]
    end

    subgraph "KwaaiNet Inference"
        Inference[Inference Engine]
        PersonalAI[Personalized AI Response]
    end

    Gmail --> OAuth
    Calendar --> OAuth
    Drive --> OAuth
    Telegram --> OAuth
    Custom --> OAuth

    OAuth --> Sync
    Sync --> Schema
    Schema --> Store

    Store --> Filter
    Filter --> ACL
    ACL --> Encryption
    Encryption --> Audit

    Encryption --> Analyzer
    Analyzer --> Context
    Context --> Enhance

    Enhance --> Inference
    Inference --> PersonalAI

    style Store fill:#50C878,color:#fff
    style Encryption fill:#FF6B6B,color:#fff
    style PersonalAI fill:#FFD700,color:#000
```

### 2.2 Personal Data Access Flow with Privacy Levels

```mermaid
sequenceDiagram
    autonumber
    participant User
    participant KwaaiNet
    participant Privacy as Privacy Controller
    participant Verida as Verida Storage
    participant Sources as Data Sources

    User->>KwaaiNet: AI Query: "Summarize my emails from today"
    KwaaiNet->>Privacy: check_privacy_level(user)

    Privacy-->>KwaaiNet: privacy_level = "personalized"

    KwaaiNet->>Privacy: request_data_access("email", query_context)
    Privacy->>Privacy: check_user_consent("email")
    Privacy->>Privacy: validate_query_legitimacy()

    alt Consent Granted & Legitimate
        Privacy->>Verida: gather_personal_context(query)
        Verida->>Sources: fetch_relevant_emails(today)
        Sources-->>Verida: email_data[]

        Verida->>Verida: apply_data_minimization()
        Note over Verida: Only include relevant fields<br/>Remove sensitive metadata

        Verida->>Privacy: filtered_data
        Privacy->>Privacy: apply_privacy_filters()
        Privacy->>Privacy: encrypt_sensitive_fields()
        Privacy->>Audit: log_data_access(user, sources, purpose)

        Privacy-->>KwaaiNet: personal_context
        KwaaiNet->>KwaaiNet: enhance_prompt_with_context()
        KwaaiNet->>KwaaiNet: run_inference()
        KwaaiNet-->>User: Personalized AI Response

        opt User Grants Learning Permission
            KwaaiNet->>Verida: store_interaction_for_learning(query, response)
        end

    else Consent Not Granted
        Privacy-->>KwaaiNet: data_access_denied
        KwaaiNet->>KwaaiNet: run_inference_without_context()
        KwaaiNet-->>User: Generic AI Response + Prompt for Consent
    end
```

### 2.3 Data Minimization & Privacy Filtering

```mermaid
graph TB
    subgraph "Raw Data from Sources"
        Email[Email Content:<br/>- Subject<br/>- Body<br/>- Sender<br/>- Recipients<br/>- Timestamp<br/>- Attachments<br/>- Metadata]
        Calendar[Calendar Events:<br/>- Title<br/>- Description<br/>- Attendees<br/>- Location<br/>- Time<br/>- Meeting Link<br/>- Metadata]
    end

    subgraph "Privacy Filters"
        Relevance[Relevance Filter<br/>Query-specific]
        Sensitive[Sensitive Data Filter<br/>PII, Credentials]
        Temporal[Temporal Filter<br/>Time-based scope]
        Minimize[Data Minimization<br/>Essential fields only]
    end

    subgraph "Privacy Levels"
        P1[Level 1: Anonymous<br/>No personal data]
        P2[Level 2: Basic<br/>Preferences only]
        P3[Level 3: Personalized<br/>Full context]
    end

    subgraph "Processed Data"
        Safe[Safe Context:<br/>- Relevant summary<br/>- Sanitized content<br/>- Temporal scope<br/>- No sensitive data]
    end

    Email --> Relevance
    Calendar --> Relevance

    Relevance --> Sensitive
    Sensitive --> Temporal
    Temporal --> Minimize

    P1 -.Configures.-> Relevance
    P2 -.Configures.-> Sensitive
    P3 -.Configures.-> Temporal

    Minimize --> Safe

    style Sensitive fill:#FF6B6B,color:#fff
    style Safe fill:#50C878,color:#fff
```


---

## Privacy-Preserving AI Inference

### 4.1 Privacy-Preserving Inference Architecture

```mermaid
graph TB
    subgraph "User Environment"
        Query[User Query]
        PersonalData[Personal Data<br/>Local Access]
    end

    subgraph "Privacy Layer"
        Encrypt[Client-Side Encryption]
        Anonymize[Data Anonymization]
        Differential[Differential Privacy]
        ZK[Zero-Knowledge Proofs]
    end

    subgraph "Secure Computation"
        Local[Local Pre-Processing]
        Federated[Federated Learning]
        SecureAgg[Secure Aggregation]
    end

    subgraph "Distributed Inference"
        Shard1[Inference Shard 1]
        Shard2[Inference Shard 2]
        Shard3[Inference Shard 3]
    end

    subgraph "Privacy Verification"
        Audit[Privacy Audit Log]
        Verify[Third-Party Verification]
        Cert[Privacy Certificate]
    end

    Query --> Local
    PersonalData --> Local

    Local --> Encrypt
    Encrypt --> Anonymize
    Anonymize --> Differential
    Differential --> ZK

    ZK --> Federated
    Federated --> SecureAgg

    SecureAgg --> Shard1
    SecureAgg --> Shard2
    SecureAgg --> Shard3

    Shard1 --> Audit
    Shard2 --> Audit
    Shard3 --> Audit

    Audit --> Verify
    Verify --> Cert

    style Encrypt fill:#FF6B6B,color:#fff
    style ZK fill:#7B68EE,color:#fff
    style Cert fill:#50C878,color:#fff
```

### 4.2 End-to-End Encrypted Inference Flow

```mermaid
sequenceDiagram
    autonumber
    participant User
    participant Device as User Device
    participant Privacy as Privacy Layer
    participant Node1 as Node 1
    participant Node2 as Node 2
    participant Node3 as Node 3
    participant Audit as Audit System

    Note over User,Audit: User Query with Personal Context

    User->>Device: Query: "Schedule meeting based on my calendar"
    Device->>Device: fetch_personal_calendar_data()
    Device->>Device: extract_relevant_context()

    Device->>Privacy: encrypt_with_user_key(context)
    Privacy->>Privacy: generate_ephemeral_keys()
    Privacy->>Privacy: apply_differential_privacy()
    Privacy->>Privacy: create_zero_knowledge_proof()

    Note over Privacy: Context encrypted locally<br/>No plaintext leaves device

    Privacy->>Privacy: shard_encrypted_context()

    par Distribute Encrypted Shards
        Privacy->>Node1: encrypted_shard_1
        Privacy->>Node2: encrypted_shard_2
        Privacy->>Node3: encrypted_shard_3
    end

    Note over Node1,Node3: Nodes process encrypted data<br/>Cannot decrypt individual shards

    Node1->>Node1: process_encrypted_shard()
    Node2->>Node2: process_encrypted_shard()
    Node3->>Node3: process_encrypted_shard()

    par Return Encrypted Results
        Node1-->>Privacy: encrypted_result_1
        Node2-->>Privacy: encrypted_result_2
        Node3-->>Privacy: encrypted_result_3
    end

    Privacy->>Privacy: combine_encrypted_results()
    Privacy->>Privacy: verify_zero_knowledge_proofs()

    Privacy-->>Device: encrypted_combined_result
    Device->>Device: decrypt_with_user_key()
    Device-->>User: Plaintext AI Response

    Note over Audit: Privacy Audit Trail

    Privacy->>Audit: log_privacy_preserving_inference(user, timestamp)
    Note over Audit: Audit log contains:<br/>- No personal data<br/>- Only metadata<br/>- ZK proof verification
```

### 4.3 Privacy Level Configuration

```mermaid
graph TB
    subgraph "Privacy Levels"
        L1[Level 1: Anonymous<br/>No personal data]
        L2[Level 2: Basic<br/>Aggregated preferences]
        L3[Level 3: Personalized<br/>Full context with encryption]
    end

    subgraph "Data Access by Level"
        L1Data[Level 1 Access:<br/>- Public models only<br/>- No user data<br/>- Generic responses]
        L2Data[Level 2 Access:<br/>- User preferences<br/>- Anonymized history<br/>- Basic personalization]
        L3Data[Level 3 Access:<br/>- Private datastores<br/>- Full email/calendar/docs<br/>- Highly personalized]
    end

    subgraph "Encryption Methods"
        L1Enc[Level 1:<br/>- TLS in transit<br/>- No storage encryption]
        L2Enc[Level 2:<br/>- TLS in transit<br/>- Basic storage encryption]
        L3Enc[Level 3:<br/>- E2E encryption<br/>- User-controlled keys<br/>- Zero-knowledge]
    end

    subgraph "User Control"
        Settings[Privacy Settings]
        Granular[Granular Permissions]
        Revoke[Revoke Access Anytime]
    end

    L1 --> L1Data
    L2 --> L2Data
    L3 --> L3Data

    L1 --> L1Enc
    L2 --> L2Enc
    L3 --> L3Enc

    L1Data --> Settings
    L2Data --> Settings
    L3Data --> Settings

    Settings --> Granular
    Granular --> Revoke

    style L3 fill:#50C878,color:#fff
    style L3Enc fill:#FF6B6B,color:#fff
    style Revoke fill:#FFD700,color:#000
```

---

## Multi-Chain Identity

### 5.1 Multi-Chain Identity Architecture

```mermaid
graph TB
    subgraph "Verida DID Foundation"
        DID[Verida DID<br/>did:vda:polpos:0x...]
        DidDoc[DID Document]
        Keys[Cryptographic Keys]
    end

    subgraph "Blockchain Networks"
        Polygon[Polygon POS<br/>Primary Network]
        Ethereum[Ethereum<br/>Layer 1]
        Arbitrum[Arbitrum<br/>Layer 2]
        Other[Other Chains<br/>Extensible]
    end

    subgraph "Identity Proofs"
        PolygonProof[Polygon Verification<br/>On-Chain Data]
        EthProof[Ethereum Verification<br/>ENS Domain]
        ArbProof[Arbitrum Verification<br/>Transaction History]
    end

    subgraph "Cross-Chain Data"
        TokenBalances[Token Balances<br/>All Chains]
        NFTs[NFT Ownership<br/>Multi-Chain]
        Transactions[Transaction History<br/>Aggregated]
        Reputation[Cross-Chain Reputation]
    end

    subgraph "KwaaiNet Integration"
        PersonalAI[Personalized AI<br/>with Web3 context]
        OptionalPayments[Optional Payment Systems]
    end

    DID --> DidDoc
    DidDoc --> Keys

    DID --> Polygon
    DID --> Ethereum
    DID --> Arbitrum
    DID --> Other

    Polygon --> PolygonProof
    Ethereum --> EthProof
    Arbitrum --> ArbProof

    PolygonProof --> TokenBalances
    EthProof --> NFTs
    ArbProof --> Transactions
    TokenBalances --> Reputation

    Reputation --> PersonalAI
    TokenBalances --> OptionalPayments

    style DID fill:#50C878,color:#fff
    style Reputation fill:#FFD700,color:#000
    style PersonalAI fill:#4A90E2,color:#fff
```

### 5.2 Cross-Chain Verification Flow

```mermaid
sequenceDiagram
    autonumber
    participant User
    participant KwaaiNet
    participant Verida
    participant Polygon
    participant Ethereum
    participant Registry as Cross-Chain Registry

    Note over User,Registry: Initial Identity Setup

    User->>KwaaiNet: Create Multi-Chain Identity
    KwaaiNet->>Verida: create_did()
    Verida->>Polygon: register_did_on_chain()
    Polygon-->>Verida: did_registered

    Note over User,Registry: Link Additional Chains

    User->>KwaaiNet: Link Ethereum Wallet
    KwaaiNet->>Verida: request_cross_chain_link(ethereum_address)
    Verida->>Ethereum: verify_ownership_challenge()
    Ethereum-->>User: Sign Challenge Message
    User->>Ethereum: Sign with Private Key
    Ethereum-->>Verida: signature_verified

    Verida->>Registry: register_cross_chain_link(did, eth_address)
    Registry-->>Verida: link_registered

    Note over User,Registry: Aggregate Cross-Chain Data

    User->>KwaaiNet: Request Personalized AI with Web3 Context
    KwaaiNet->>Verida: gather_multi_chain_context(did)

    par Fetch from Multiple Chains
        Verida->>Polygon: get_vda_balance(did)
        Verida->>Ethereum: get_nft_collection(eth_address)
        Verida->>Ethereum: get_ens_domain(eth_address)
    end

    Polygon-->>Verida: vda_balance
    Ethereum-->>Verida: nft_collection
    Ethereum-->>Verida: ens_domain

    Verida->>Registry: aggregate_cross_chain_data()
    Registry-->>Verida: unified_profile

    Verida-->>KwaaiNet: multi_chain_context
    KwaaiNet->>KwaaiNet: enhance_ai_with_web3_context()
    KwaaiNet-->>User: Personalized AI Response (with Web3 insights)
```

### 5.3 Identity Data Portability

```mermaid
graph TB
    subgraph "User's Sovereign Identity"
        Core[Verida DID Core]
        Keys[Self-Controlled Keys]
    end

    subgraph "Blockchain Attestations"
        B1[Polygon: VDA Balance & Staking]
        B2[Ethereum: ENS & NFTs]
        B3[Arbitrum: Activity History]
        B4[Other Chains: Extensible]
    end

    subgraph "Data Portability"
        Export[Export All Data<br/>JSON/CSV]
        Import[Import to New Platform]
        Transfer[Transfer Between Services]
    end

    subgraph "Interoperability"
        KwaaiNet[KwaaiNet]
        VeridaApps[Other Verida Apps]
        Web3Apps[Web3 Applications]
        Standard[W3C DID Standard]
    end

    Core --> Keys
    Keys --> B1
    Keys --> B2
    Keys --> B3
    Keys --> B4

    B1 --> Export
    B2 --> Export
    B3 --> Export
    B4 --> Export

    Export --> Import
    Import --> Transfer

    Core --> Standard
    Standard --> KwaaiNet
    Standard --> VeridaApps
    Standard --> Web3Apps

    Transfer --> KwaaiNet
    Transfer --> VeridaApps
    Transfer --> Web3Apps

    style Core fill:#50C878,color:#fff
    style Keys fill:#FF6B6B,color:#fff
    style Standard fill:#4A90E2,color:#fff
```

---

## Related Documentation

- [Main Architecture](../ARCHITECTURE.md) - High-level system architecture
- [Challenge Architectures](./CHALLENGE_ARCHITECTURES.md) - Detailed component diagrams
- [Verida Architecture](./VERIDA_ARCHITECTURE.md) - Deep dive into Verida integration
- [Deployment Architecture](./DEPLOYMENT_ARCHITECTURE.md) - Platform deployment patterns
- [Verida Integration Documentation](../docs/VERIDA_INTEGRATION.md) - Complete integration guide

---

**Document Status**: Draft - Technical Specification
**Next Review**: December 2025
**Maintainer**: KwaaiNet Architecture Team
