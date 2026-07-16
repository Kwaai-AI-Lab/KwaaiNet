# RAG Eval Report

**KB:** `RFCs`  **Model:** `llama3.1:8b`

**Flags:** top_k=5  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Overall recall (token-overlap) | 66.7% (52.0/78) |
| Avg latency | 2892ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | What core function does RFC 9293 (TCP) define, and how does it differ fundamentally from RFC 768 (UDP) in terms of delivery guarantees? | 2/4 (50%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf | 2491ms |
| q02 | What layer of the network stack does RFC 8200 (IPv6) operate at, and what key improvement does it offer over IPv4? | 2/4 (50%) | rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf | 2277ms |
| q03 | What is the difference in purpose between RFC 1034 and RFC 1035, both concerning DNS? | 2/3 (67%) | rfc-editor.org-RFC 1034 STD 13 Domain names - concepts and facilities.pdf, rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf, rfc-editor.org-RFC 1034 STD 13 Domain names - concepts and facilities (1).pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf | 2247ms |
| q04 | What core protocol does RFC 9000 (QUIC) define, and what transport-layer protocol does it build on top of? | 3/4 (75%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf | 1209ms |
| q05 | Cross-document: How does QUIC (RFC 9000) attempt to solve limitations in TCP (RFC 9293) with respect to connection setup latency? | 4/4 (100%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf | 2922ms |
| q06 | What does RFC 8446 (TLS 1.3) define, and what security properties does it aim to guarantee for data in transit? | 2/4 (50%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf | 3058ms |
| q07 | What is the relationship between RFC 9110 (HTTP Semantics) and RFC 9112 (HTTP/1.1) — which defines protocol-version-specific syntax and which defines version-independent semantics? | 4/4 (100%) | rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf | 5799ms |
| q08 | What does RFC 6455 define, and how does the WebSocket protocol differ from a standard HTTP request/response cycle? | 2/4 (50%) | rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf | 1756ms |
| q09 | What does RFC 6749 (OAuth 2.0) define, and what problem does it solve for third-party application access to user resources? | 3/4 (75%) | rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf | 2666ms |
| q10 | What does RFC 7519 (JWT) define, and how does a JSON Web Token typically relate to an OAuth 2.0 authorization flow (RFC 6749)? | 2/4 (50%) | rfc-editor.org-RFC 7519      JSON Web Token JWT.pdf, rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf | 1613ms |
| q11 | Cross-document: How do OAuth 2.0 (RFC 6749) and JWT (RFC 7519) work together in a typical authentication/authorization architecture, even though each RFC defines a distinct standard? | 4/4 (100%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf | 4073ms |
| q12 | What does RFC 8259 define, and what data format does it standardize? | 3/3 (100%) | rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf, rfc-editor.org-RFC 8259 STD 90 The JavaScript Object Notation JSON Data Interchange Format.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf | 1353ms |
| q13 | What does RFC 4648 define? Name the three encoding schemes it covers. | 3/4 (75%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 4648      The Base16 Base32 and Base64 Data Encodings.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf | 1361ms |
| q14 | Near-miss: JSON (RFC 8259) and JWT (RFC 7519) are related but distinct — how does a JWT actually use JSON internally, and what does JWT add on top of plain JSON? | 3/4 (75%) | rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, rfc-editor.org-RFC 8259 STD 90 The JavaScript Object Notation JSON Data Interchange Format.pdf, rfc-editor.org-RFC 7519      JSON Web Token JWT.pdf | 4846ms |
| q15 | Cross-document: Organize RFC 768 (UDP), RFC 9293 (TCP), and RFC 9000 (QUIC) by which network layer each protocol operates at. | 3/4 (75%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf | 3553ms |
| q16 | Cross-document: How does DNS's (RFC 1034/1035) request/response model differ from HTTP's (RFC 9110/9112), and from WebSocket's (RFC 6455) persistent bidirectional model? | 4/4 (100%) | rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf | 6487ms |
| q17 | Which RFC in this cluster defines a security protocol rather than a data-transport or data-format protocol? | 0/4 (0%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf | 2447ms |
| q18 | Edge case: RFC 9110 and RFC 9112 were both published to replace older combined HTTP/1.1 RFCs — why might a standards body choose to split "semantics" from "wire format" into separate documents? | 2/4 (50%) | rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf | 3440ms |
| q19 | Which two RFCs in this cluster are most likely to be used together to secure a WebSocket connection (RFC 6455) in production? | 2/4 (50%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf | 2086ms |
| q20 | Edge case: Both RFC 768 (UDP) and RFC 9000 (QUIC) can carry data without TCP's guaranteed ordering — what does QUIC add on top of UDP's bare functionality to make it suitable for modern web transport? | 2/4 (50%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf | 2175ms |

## Answers

### q01 — What core function does RFC 9293 (TCP) define, and how does it differ fundamentally from RFC 768 (UDP) in terms of delivery guarantees?

The provided sources do not contain information about the core function defined by RFC 9293 or its differences with RFC 768 regarding delivery guarantees.

However, [5] mentions that RFC 9293 defines several flags for the TCP header, specifically:

1. URG (Urgent pointer field is significant)
2. ACK (Acknowledgment field is significant)
3. SYN (Synchronize sequence numbers)
4. FIN (No more data from sender)

This does not provide information about the core function of RFC 9293 or its differences with UDP in terms of delivery guarantees.

The provided sources also do not contain any information about RFC 768, which would be necessary to answer the second part of your question.

Therefore, I must say that "The provided sources do not contain that information."

### q02 — What layer of the network stack does RFC 8200 (IPv6) operate at, and what key improvement does it offer over IPv4?

The provided sources do not contain information about RFC 8200 (IPv6). They mention [1] and [13], which refer to IPv4 and IPv6 respectively, but there is no mention of RFC 8200. 

However, based on general knowledge that can be inferred from the given excerpts:

* The layer at which IPv6 operates in the network stack is mentioned as "below" TCP, so it is at the IP layer ([1], [3]).
* IPv4 is the older protocol version that offers a limited address space and is said to be used for backward compatibility.
* IPv6 is the newer protocol version that is layered below TCP.

### q03 — What is the difference in purpose between RFC 1034 and RFC 1035, both concerning DNS?

The provided sources do not contain information about a difference in purpose between RFC 1034 and RFC 1035. 

However, [2] mentions "RFCs 8 8 2 , 8 8 3 , 9 7 3" as obsoleted by November 1987, which suggests that these documents may include information about DNS. But there is no mention of RFC 1034 and RFC 1035 in this context.

It's worth noting that the Internet Society (ISI) Request for Comments 1034 lists RFCs 882, 883, and 973 as obsoleted by November 1987.

### q04 — What core protocol does RFC 9000 (QUIC) define, and what transport-layer protocol does it build on top of?

The core protocol defined by RFC 9000 (QUIC) is a UDP-based multiplexed and secure transport [1]. 

It builds on top of the UDP transport-layer protocol.

### q05 — Cross-document: How does QUIC (RFC 9000) attempt to solve limitations in TCP (RFC 9293) with respect to connection setup latency?

The provided sources do not explicitly state how QUIC attempts to solve limitations in TCP with respect to connection setup latency. However, [2] mentions that lessons learned from TCP have been applied in the design of newer transports like QUIC.

[1] does mention that QUIC provides applications with "low-latency connection establishment." But it does not specifically address what issues or limitations in TCP it is addressing.

A more detailed discussion about how QUIC attempts to solve limitations in TCP with respect to connection setup latency can be found in [5], which references RFC 793. However, the content of [5] does not explicitly describe the improvements of QUIC over TCP regarding this specific aspect. 

It is worth noting that a direct comparison between QUIC and TCP, specifically with regards to connection setup latency, is not provided by any of the sources.

A relevant excerpt from [2]: "Lessons learned in this respect from TCP have been applied in the design of newer transports like QUIC."

### q06 — What does RFC 8446 (TLS 1.3) define, and what security properties does it aim to guarantee for data in transit?

The provided sources do not mention RFC 8446 (TLS 1.3). However, [4] refers to Appendix E.1 of [TLS13], which implies that the source is related to TLS 1.3.

[TLS13] is mentioned as a document describing properties that depend on the TLS handshake providing certain properties, such as secrecy or uniqueness of session keys, and authentication of participating peers. The security guarantees provided by QUIC are stated to be affected if any attack on the TLS handshake compromises these properties.

Thus, based on [4], we can infer that TLS 1.3 defines some properties for data in transit, but the exact details are not specified in the given excerpts.

A more accurate answer would be:

The provided sources do not explicitly define what RFC 8446 (TLS 1.3) defines or guarantee for security properties of data in transit. However, [4] mentions that TLS 1.3 is related to properties described in Appendix E.1 and that any attack on the TLS handshake could affect QUIC's security guarantees.

### q07 — What is the relationship between RFC 9110 (HTTP Semantics) and RFC 9112 (HTTP/1.1) — which defines protocol-version-specific syntax and which defines version-independent semantics?

The provided sources do not contain information about the specific relationships between these two documents. However, based on a general understanding of HTTP protocol versions and their respective specifications:

* The relationship is that both are part of the HTTP/1.x series.
* RFC 9110 (HTTP Semantics) defines the semantics for multiple HTTP versions, including HTTP/1.1, while 
RFC 9112 (HTTP/1.1) defines specific syntax for HTTP/1.1 protocol.

To accurately answer this question based on the provided sources:

According to [3], "An abstract data type for HTTP messages has been introduced to define the components of a message and their semantics as an abstraction across multiple HTTP versions, rather than in terms of the specific syntax form of HTTP/1.1 in  [HTTP/1.1 ], and reflect the contents after the message is parsed."

This indicates that RFC 9110 (HTTP Semantics) focuses on version-independent semantics, while [HTTP/1.1] focuses on protocol-version-specific syntax.

However, as there is no information about a document called "RFC 9112", I will focus only on [4], where it states: "The version "HTTP/1.1" is defined by the combined specifications of this document, "HTTP Caching"  [CACHING], and "HTTP/1.1"  [HTTP/1.1 ]."

This suggests that both RFC 9110 (which is not explicitly mentioned) and HTTP/1.1 define aspects of the same version.

The information does not provide a clear distinction between which document defines protocol-version-specific syntax, and which defines version-independent semantics.

A more accurate statement would be: "RFC 9110 (HTTP Semantics) defines an abstract data type for HTTP messages with semantics across multiple HTTP versions, but there is no explicit information about the relationship between this document and RFC 9112 in these sources. However, it can be inferred that both documents are related to defining aspects of the 'HTTP/1.1' version."

In terms of who wrote the two documents:

* There is no direct information provided about the authors of "RFC 9110 (HTTP Semantics)" or "RFC 9112 (HTTP/1.1)".

### q08 — What does RFC 6455 define, and how does the WebSocket protocol differ from a standard HTTP request/response cycle?

The provided sources do not mention RFC 6455 or its definition. However, I found that in real-world contexts, RFC 6455 defines the WebSocket Protocol.

Since it's not mentioned in the provided excerpts, I'll state what the sources partially address:

The sources discuss HTTP and some of its properties (stateless request/response protocol, Upgrade header field), but they do not directly compare or describe the WebSocket protocol.

### q09 — What does RFC 6749 (OAuth 2.0) define, and what problem does it solve for third-party application access to user resources?

The provided sources do not contain information about RFC 6749 or OAuth 2.0. There is no mention of these terms in the given excerpts.

However, the first source excerpt mentions "additional mechanisms" for authentication that are not defined by this specification. It also notes that custom mechanisms use the Set-Cookie and Cookie header fields for passing tokens related to authentication, but it does not specify a particular standard or protocol like OAuth 2.0.

The second source excerpt discusses the "https" scheme and its use in identifying resources that require confidential connections, as well as registrations with IANA, but again, there is no mention of RFC 6749 or OAuth 2.0.

Therefore, based on the provided information, it is not possible to determine what problem OAuth 2.0 solves for third-party application access to user resources.

### q10 — What does RFC 7519 (JWT) define, and how does a JSON Web Token typically relate to an OAuth 2.0 authorization flow (RFC 6749)?

The provided sources do not contain information about RFC 7519 (JWT), or specifically what it defines, nor its relationship with an OAuth 2.0 authorization flow as defined in RFC 6749.

[1]–[5] are entirely unrelated to JWT and OAuth 2.0.

### q11 — Cross-document: How do OAuth 2.0 (RFC 6749) and JWT (RFC 7519) work together in a typical authentication/authorization architecture, even though each RFC defines a distinct standard?

The provided sources do not contain information about the working relationship between OAuth 2.0 (RFC 6749) and JWT (RFC 7519) in a typical authentication/authorization architecture. 

However, based on general knowledge of these standards, here is a synthesized answer:

OAuth 2.0 is an authorization framework that allows clients to access resources on behalf of a resource owner. It uses tokens to represent the client's permissions and access rights.

JSON Web Tokens (JWT) are a compact, URL-safe means of representing claims between two parties. They can be used for authentication and authorization in various contexts, including microservices architecture, API security, and single sign-on systems.

In many modern authentication/authorization architectures, OAuth 2.0 is used to obtain an access token from an Authorization Server (AS) after the user has authenticated with the AS. This access token contains information about the client's permissions and can be sent in the HTTP Authorization header or as a query parameter when making requests to protected resources.

JSON Web Tokens can then be used to represent claims that are verified by the resource server, such as information about the user, their role, or any additional attributes associated with the access token. This provides an additional layer of security and ensures that only authorized users have access to sensitive data or functionality.

However, it's essential to note that this synthesized answer is based on general knowledge and may not be entirely accurate without specific information from RFC 6749 and RFC 7519 sources.

### q12 — What does RFC 8259 define, and what data format does it standardize?

RFC 8259 defines a data interchange format called JavaScript Object Notation (JSON) [1]. 

The provided sources do not mention the specifics of the data that this format is used for.

### q13 — What does RFC 4648 define? Name the three encoding schemes it covers.

RFC 4648 defines the Base16, Base32, and Base64 data encodings.

[5] states this explicitly: "The Base16, Base32, and Base64 Data Encodings".

### q14 — Near-miss: JSON (RFC 8259) and JWT (RFC 7519) are related but distinct — how does a JWT actually use JSON internally, and what does JWT add on top of plain JSON?

According to the provided sources:

* RFC 8259: STD 90: The JavaScript Object Notation (JSON) Data Interchange Format [1] defines the JSON data interchange format.
* RFC 7519 [not present in the given excerpts], which is related to JWT, would be expected to use JSON internally, as JWT stands for "JSON Web Token". However, since only RFC 8259 is provided, we can infer that JWT likely uses JSON as its underlying format.

The excerpt from [4] discusses JWT but does not explicitly state how it uses JSON. It defines the structure of a JWT, which consists of:
* A JSON object (JWTC laims Set) that contains claims conveyed by the JWT.
* The JWTC laims Set is a JSON object that represents the claims.

Thus, we can conclude that JWT internally uses JSON to represent its claims and other data. However, JWT adds additional functionality on top of plain JSON:

1.  **Digital signatures**: JWTs are digitally signed using algorithms like HMAC or RSA to ensure authenticity and prevent tampering.
2.  **Compact serialization**: JWTs use a compact encoding scheme that allows them to be represented as a small string, often included in an HTTP header or query parameter.
3.  **Structured claims**: JWTs provide a standardized way of representing claims using JSON objects, which can include data such as user identity, permissions, and other application-specific information.
4.  **Secure encoding**: JWTs typically use base64url encoding for compact serialization, which provides an additional layer of security against tampering.

In summary, while JWT uses JSON internally to represent its claims and structure, it adds a range of features on top of plain JSON, including digital signatures, compact serialization, structured claims, and secure encoding.

### q15 — Cross-document: Organize RFC 768 (UDP), RFC 9293 (TCP), and RFC 9000 (QUIC) by which network layer each protocol operates at.

Based on the provided excerpts, here is the organization of the protocols by their corresponding network layers:

**Transport Layer:**

1. TCP (RFC 793, RFC 1011, RFC 1122, RFC 9293)
	* Mentioned in RFCs 1011 and 1122 as updating parts of TCP requirements
	* Described in detail in RFC 9293
2. UDP (RFC 768)

**Application Layer:**

1. QUIC (RFC 9000)
	* Defined as a multiplexed and secure transport protocol

Note that the network layers are not explicitly mentioned in the provided excerpts, but based on general knowledge of the protocols, here is their classification:

* Transport layer: TCP (RFCs 793, 1011, 1122, 9293) and UDP (RFC 768)
* Application layer: QUIC (RFC 9000)

TCP and UDP are considered transport-layer protocols because they provide services for reliable data transfer between end systems. On the other hand, QUIC is considered an application-layer protocol because it provides a multiplexed and secure transport service to applications, while being designed as a complete transport protocol itself.

### q16 — Cross-document: How does DNS's (RFC 1034/1035) request/response model differ from HTTP's (RFC 9110/9112), and from WebSocket's (RFC 6455) persistent bidirectional model?

To address the question, we'll compare the DNS request/response models to the HTTP and WebSocket models mentioned in the given excerpts.

**DNS Request/Response Model:**

Excerpts [1] and [4] are irrelevant for this comparison as they only discuss HTTP/1.0 and HTTP-related specifications without mentioning the DNS model. To understand the DNS request/response model, we need external knowledge of RFC 1034/1035.

The DNS (Domain Name System) protocol uses a client-server architecture where clients send requests to servers to resolve domain names into IP addresses. The DNS request typically involves a query sent by a client resolver to a recursive or authoritative name server for the zone containing the queried domain name.

**HTTP Request/Response Model:**

In contrast, HTTP (Hypertext Transfer Protocol) uses a client-server model where clients send requests to servers, which then respond with requested resources. According to [1], this document updates RFC 3864 and obsoletes some other RFCs including parts of RFC 7230. The main difference between the DNS and HTTP request/response models lies in their interaction patterns.

The HTTP protocol is based on a connection-oriented architecture where a client establishes a connection with an origin server, sends requests, receives responses, and then closes the connection.

**WebSocket Persistent Bidirectional Model:**

The WebSocket protocol differs from both DNS and HTTP in its persistent bidirectional communication model. According to [5], this specification defines how clients can establish a persistent connection to a server for exchanging messages bi-directionally.

In contrast to the request/response nature of HTTP, WebSockets provide real-time communication with low latency. The protocol is designed to allow servers and clients to push content to each other through bidirectional, full-duplex connections that remain open across requests.

**Comparison:**

To summarize:

- DNS uses a client-server model for name resolution where queries are sent by clients to servers.
- HTTP also employs a client-server architecture but is focused on delivering requested resources over established connections.
- WebSocket introduces a persistent bi-directional communication model, enabling real-time exchange of messages between clients and servers.

The main difference lies in their interaction patterns: DNS involves query-resolution pairs, while HTTP uses request-response cycles. In contrast, WebSockets facilitate continuous bidirectional data transfer between the client and server.

**Limitations of Comparison:**

Since excerpts [1]-[5] do not contain explicit descriptions of the WebSocket protocol's persistent bidirectional model or the full DNS request/response process, further external knowledge is necessary for a comprehensive comparison.

### q17 — Which RFC in this cluster defines a security protocol rather than a data-transport or data-format protocol?

The provided sources do not contain that information. However, based on general knowledge, it is known that TLS (Transport Layer Security) is a security protocol that is integrated with QUIC to ensure confidentiality, integrity, and availability in various deployment circumstances.

According to the excerpts [3] and [5], there are mentions of HTTP, QUIC, and TLS, but it doesn't explicitly state which one defines a security protocol. Nonetheless, based on general knowledge and typical network protocols, it is likely that [4] (TLS) is the correct answer as it defines the integration of TLS with QUIC for key negotiation, loss detection, etc.

However, this is not directly stated in any of the excerpts provided above.

### q18 — Edge case: RFC 9110 and RFC 9112 were both published to replace older combined HTTP/1.1 RFCs — why might a standards body choose to split "semantics" from "wire format" into separate documents?

This question requires synthesizing information from multiple sources.

According to [4], the document specifies how HTTP semantics are conveyed using the HTTP/1.1 message syntax, framing, and connection management mechanisms. 

However, we also know that in 2014, the RFCs related to HTTP/1.1 were revised again (see [1] for details on this revision). It's possible that a standards body would choose to split "semantics" from "wire format" into separate documents at this point because they are now separate and distinct entities with different purposes.

The provided sources do not explicitly state why the standards body might choose to split "semantics" from "wire format", but it is likely due to changes in the structure of HTTP/1.1 following its revisions in 1999 and again in 2014.

It's also worth noting that [3] discusses how proxies and gateways need to handle different formats for date comparisons, which implies a certain level of complexity in handling "semantics" versus "wire format". 

However, the precise reason why the standards body might choose to split these entities into separate documents is not explicitly stated in any of the provided sources.

### q19 — Which two RFCs in this cluster are most likely to be used together to secure a WebSocket connection (RFC 6455) in production?

Based on the provided excerpts, we can identify two relevant RFCs that could be used together to secure a WebSocket connection:

1. [3] mentions stateless reset tokens, which are discussed in more detail.
2. [4] discusses changes to the TCP specification, including the incorporation of a more secure initial sequence number generation algorithm from RFC 6528.

While there is no direct mention of WebSockets or their specific security requirements, these two RFCs suggest that securing a connection involves using stateless reset tokens and a more secure initial sequence number generation algorithm.

### q20 — Edge case: Both RFC 768 (UDP) and RFC 9000 (QUIC) can carry data without TCP's guaranteed ordering — what does QUIC add on top of UDP's bare functionality to make it suitable for modern web transport?

[1] states that QUIC provides applications with "flow-controlled streams for structured communication, low-latency connection establishment, and network path migration." [2] adds that the client updates the flow control limits on all sending streams using the updated values of initial max stream data bidi remote and initial max stream data uni. 

Additionally, [3] states that QUIC provides a credit-based scheme to limit stream creation and to bound the amount of data that can be sent.

It appears that QUIC adds features such as flow control, low-latency connection establishment, network path migration, and a credit-based scheme for limiting stream creation and data transmission.

