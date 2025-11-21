# Data Flows & Authentication Architecture
## Progressive Authentication, Personal Data Integration, and Token Economics

**Version**: 1.0
**Date**: November 20, 2025
**Status**: Technical Specification
**Related Documents**: [ARCHITECTURE.md](../ARCHITECTURE.md), [VERIDA_INTEGRATION.md](../VERIDA_INTEGRATION.md), [CHALLENGE_ARCHITECTURES.md](./CHALLENGE_ARCHITECTURES.md)

---

## Table of Contents

1. [Progressive Authentication](#progressive-authentication)
2. [Personal Data Integration](#personal-data-integration)
3. [VDA Token Economics](#vda-token-economics)
4. [Privacy-Preserving AI Inference](#privacy-preserving-ai-inference)
5. [Multi-Chain Identity](#multi-chain-identity)

---

## Progressive Authentication

### 1.1 Authentication Levels Overview

```mermaid
graph LR
    A[Anonymous] -->|Email Verification| B[Email Verified]
    B -->|Biometric Setup| C[Biometric Verified]
    C -->|Create Verida DID| D[Full Sovereign]
    D -->|Connect Wallet| E[Wallet Connected]

    style A fill:#E0E0E0,color:#000
    style B fill:#B3E5FC,color:#000
    style C fill:#81C784,color:#000
    style D fill:#FFD54F,color:#000
    style E fill:#FFD700,color:#000
```

**Access Levels by Authentication State:**

| Level | Data Access | AI Capabilities | Rewards | Identity |
|-------|------------|----------------|---------|----------|
| **Anonymous** | None | Basic inference | None | Session only |
| **Email Verified** | Basic profile | Enhanced inference | 10% rewards | Email-based |
| **Biometric Verified** | Device storage | Personal AI | 50% rewards | Device-bound |
| **Full Sovereign** | Private datastore | Fully personal AI | 100% rewards | Self-sovereign DID |
| **Wallet Connected** | Multi-chain data | Premium AI | 100% + bonuses | Multi-chain verified |

### 1.2 Complete Authentication Flow

```mermaid
sequenceDiagram
    autonumber
    participant User
    participant App as KwaaiNet App
    participant Auth as Auth Controller
    participant Verida
    participant Email as Email Service
    participant Biometric as Device Biometrics
    participant Wallet as VDA Wallet

    Note over User,Wallet: Level 1: Anonymous → Email Verified

    User->>App: First Launch
    App->>Auth: initialize(anonymous)
    Auth-->>App: anonymous_session
    App-->>User: Basic AI Available

    User->>App: Sign Up with Email
    App->>Auth: upgrade_to_email(email)
    Auth->>Email: send_verification_code(email)
    Email-->>User: Verification Email

    User->>App: Enter Verification Code
    App->>Auth: verify_code(code)
    Auth->>Auth: create_basic_profile()
    Auth-->>App: email_verified_session
    App-->>User: Enhanced AI + 10% Rewards

    Note over User,Wallet: Level 2: Email → Biometric

    User->>App: Enable Biometric Auth
    App->>Biometric: request_biometric_setup()
    Biometric-->>User: Face ID / Fingerprint Prompt
    User->>Biometric: Authenticate
    Biometric-->>App: biometric_token

    App->>Auth: upgrade_to_biometric(token)
    Auth->>Auth: create_device_bound_keys()
    Auth-->>App: biometric_session
    App-->>User: Personal AI + 50% Rewards

    Note over User,Wallet: Level 3: Biometric → Sovereign

    User->>App: Create Sovereign Identity
    App->>Auth: upgrade_to_sovereign()
    Auth->>Verida: create_did(user_info)

    Verida->>Verida: generate_did()
    Verida->>Verida: register_on_chain()
    Verida-->>Auth: did_created(did:vda:polpos:0x...)

    Auth->>Verida: initialize_storage_endpoints()
    Verida-->>Auth: endpoints_configured

    Auth-->>App: sovereign_session(did)
    App-->>User: Full Sovereign AI + 100% Rewards

    Note over User,Wallet: Level 4: Sovereign → Wallet Connected

    User->>App: Connect VDA Wallet
    App->>Wallet: request_connection(did)
    Wallet-->>User: Wallet Connect QR
    User->>Wallet: Approve Connection
    Wallet-->>App: wallet_connected(address)

    App->>Auth: upgrade_to_wallet(wallet_address)
    Auth->>Verida: link_wallet_to_did(did, address)
    Verida-->>Auth: wallet_linked

    Auth-->>App: wallet_session(did, wallet)
    App-->>User: Premium AI + Staking + Multi-Chain + Bonuses
```

### 1.3 Authentication State Machine

```mermaid
stateDiagram-v2
    [*] --> Anonymous

    Anonymous --> EmailVerified: Email Verification
    Anonymous --> EmailVerified: Social Login

    EmailVerified --> BiometricVerified: Biometric Setup
    EmailVerified --> FullSovereign: Skip Biometric

    BiometricVerified --> FullSovereign: Create DID

    FullSovereign --> WalletConnected: Connect Wallet

    state Anonymous {
        [*] --> SessionOnly
        SessionOnly --> BasicAI
    }

    state EmailVerified {
        [*] --> EmailProfile
        EmailProfile --> BasicStorage
        BasicStorage --> LimitedRewards
    }

    state BiometricVerified {
        [*] --> DeviceKeys
        DeviceKeys --> LocalStorage
        LocalStorage --> MediumRewards
    }

    state FullSovereign {
        [*] --> VeridaDID
        VeridaDID --> PrivateDatastore
        PrivateDatastore --> E2EEncryption
        E2EEncryption --> FullRewards
    }

    state WalletConnected {
        [*] --> VDAWallet
        VDAWallet --> Staking
        Staking --> MultiChain
        MultiChain --> PremiumFeatures
        PremiumFeatures --> MaxRewards
    }

    WalletConnected --> [*]

    note right of Anonymous
        AI Access: Basic
        Data: None
        Rewards: 0%
    end note

    note right of EmailVerified
        AI Access: Enhanced
        Data: Basic Profile
        Rewards: 10%
    end note

    note right of BiometricVerified
        AI Access: Personal
        Data: Device Storage
        Rewards: 50%
    end note

    note right of FullSovereign
        AI Access: Fully Personal
        Data: Private Datastore
        Rewards: 100%
    end note

    note right of WalletConnected
        AI Access: Premium
        Data: Multi-Chain
        Rewards: 100% + Bonuses
    end note
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

## VDA Token Economics

### 3.1 Triple Service Token Economy

```mermaid
graph TB
    subgraph "Earning VDA Tokens"
        Compute[Contribute Compute<br/>100 VDA/hour]
        Storage[Host Private Storage<br/>50 VDA/GB/month]
        Identity[Identity Verification<br/>25 VDA/verification]
        MultiChain[Multi-Chain Verification<br/>2 VDA/verification]
    end

    subgraph "Token Modifiers"
        Base[Base Rewards]
        Green[Green Energy Bonus<br/>+30-70%]
        Privacy[Privacy Certification<br/>+20%]
        Community[Community Challenges<br/>+10-20%]
        Staking[Staking Multiplier<br/>+5-15%]
    end

    subgraph "Spending VDA Tokens"
        InferenceUse[AI Inference<br/>10 VDA/minute]
        StorageAccess[Storage Access<br/>5 VDA/GB/month]
        PremiumAI[Premium AI Features<br/>Variable pricing]
        DataVerify[Data Verification<br/>1-10 VDA/tx]
    end

    subgraph "VDA Wallet & Staking"
        Wallet[VDA Wallet<br/>Polygon POS]
        StakingPool[Staking Pool<br/>Lock for benefits]
        Governance[Governance<br/>Vote on proposals]
    end

    subgraph "Network Effects"
        Liquidity[Increased Liquidity]
        Utility[Higher Utility Value]
        Adoption[Mass Adoption]
    end

    Compute --> Base
    Storage --> Base
    Identity --> Base
    MultiChain --> Base

    Base --> Green
    Base --> Privacy
    Base --> Community
    Base --> Staking

    Green --> Wallet
    Privacy --> Wallet
    Community --> Wallet
    Staking --> Wallet

    Wallet --> InferenceUse
    Wallet --> StorageAccess
    Wallet --> PremiumAI
    Wallet --> DataVerify

    Wallet --> StakingPool
    StakingPool --> Governance
    StakingPool -.Multiplier.-> Staking

    InferenceUse --> Liquidity
    StorageAccess --> Utility
    PremiumAI --> Adoption

    style Wallet fill:#8B4789,color:#fff
    style Green fill:#50C878,color:#fff
    style Utility fill:#FFD700,color:#000
```

### 3.2 VDA Token Lifecycle Flow

```mermaid
sequenceDiagram
    autonumber
    participant User
    participant Node as KwaaiNet Node
    participant Rewards as Reward Calculator
    participant Wallet as VDA Wallet
    participant Contract as VDA Contract<br/>(Polygon)
    participant Staking as Staking Pool
    participant AI as AI Services

    Note over User,AI: Earning Phase

    User->>Node: Contribute Compute (1 hour)
    Node->>Node: execute_inference_tasks()
    Node->>Rewards: calculate_rewards(compute_contribution)

    Rewards->>Rewards: base_reward = 100 VDA

    alt Green Energy Detected
        Rewards->>Rewards: apply_green_bonus(+50%)
        Rewards->>Rewards: total = 150 VDA
    end

    alt Privacy Certified
        Rewards->>Rewards: apply_privacy_bonus(+20%)
        Rewards->>Rewards: total = 180 VDA
    end

    Rewards->>Wallet: credit_rewards(180 VDA)
    Wallet->>Contract: transfer(user_address, 180 VDA)
    Contract-->>Wallet: transaction_confirmed
    Wallet-->>User: Balance Updated: +180 VDA

    Note over User,AI: Staking Phase

    User->>Wallet: Stake 500 VDA for 30 days
    Wallet->>Staking: stake(500 VDA, 30 days)
    Staking->>Contract: lock_tokens(user, 500, duration)
    Contract-->>Staking: staked_confirmed

    Staking-->>User: Staking Active<br/>+10% Reward Multiplier<br/>Governance Rights

    Note over User,AI: Spending Phase

    User->>AI: Request Premium AI Inference
    AI->>Wallet: check_balance()
    Wallet-->>AI: balance = 180 VDA (available)

    AI->>Wallet: deduct_payment(10 VDA)
    Wallet->>Contract: transfer(ai_service, 10 VDA)
    Contract-->>Wallet: payment_confirmed

    AI-->>User: Premium AI Response
    Wallet-->>User: Balance: 170 VDA available<br/>500 VDA staked

    Note over User,AI: Reward Multiplier from Staking

    Node->>Rewards: calculate_next_reward()
    Rewards->>Staking: get_multiplier(user)
    Staking-->>Rewards: multiplier = 1.10 (10%)
    Rewards->>Rewards: 100 VDA * 1.10 = 110 VDA
    Rewards->>Wallet: credit_rewards(110 VDA)
```

### 3.3 Economic Incentive Alignment

```mermaid
graph TB
    subgraph "Individual Contributors"
        Smartphone[Smartphone User<br/>$4-12/month]
        PC[Gaming PC User<br/>$30-200/month]
        Server[Server Operator<br/>$500-5K/month]
    end

    subgraph "Platform Integrators"
        Website[Website Owner<br/>$200-1.2K/month<br/>per 1M visitors]
        App[Mobile App<br/>$3K-25K/month]
        Enterprise[Enterprise<br/>$10K-100K/month]
    end

    subgraph "Network Economics"
        Supply[Compute Supply]
        Demand[Inference Demand]
        Pricing[Dynamic Pricing]
    end

    subgraph "Value Drivers"
        Utility[Token Utility]
        Scarcity[Token Scarcity<br/>via staking]
        Growth[Network Growth]
    end

    Smartphone --> Supply
    PC --> Supply
    Server --> Supply

    Website --> Demand
    App --> Demand
    Enterprise --> Demand

    Supply --> Pricing
    Demand --> Pricing

    Pricing --> Utility
    Utility --> Growth

    Smartphone -.Stakes.-> Scarcity
    PC -.Stakes.-> Scarcity
    Server -.Stakes.-> Scarcity

    Scarcity --> Utility
    Growth --> Utility

    style Pricing fill:#4A90E2,color:#fff
    style Utility fill:#FFD700,color:#000
    style Growth fill:#50C878,color:#fff
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
        PolygonProof[Polygon Verification<br/>VDA Token Balance]
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
        VDARewards[VDA Rewards]
        Staking[Staking Services]
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
    TokenBalances --> VDARewards
    PolygonProof --> Staking

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
