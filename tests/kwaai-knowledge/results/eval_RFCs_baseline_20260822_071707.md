# RAG Eval Report

**KB:** `RFCs`  **Model:** `llama3.1:8b`

**Flags:** top_k=20  hyde=false  rerank=false  understand=false  llm_judge=false

## Summary

| Metric | Value |
|--------|-------|
| Questions | 20 |
| Overall recall (token-overlap) | 70.5% (55.0/78) |
| Avg latency | 27346ms |

## Per-question results

| ID | Question | Hit rate | Sources | Latency |
|----|----------|----------|---------|--------|
| q01 | What core function does RFC 9293 (TCP) define, and how does it differ fundamentally from RFC 768 (UDP) in terms of delivery guarantees? | 3/4 (75%) | rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, [Graph: RFC TCP], rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf | 20105ms |
| q02 | What layer of the network stack does RFC 8200 (IPv6) operate at, and what key improvement does it offer over IPv4? | 1/4 (25%) | [Graph: Upper-Layer Packet Length], rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf, rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf | 22671ms |
| q03 | What is the difference in purpose between RFC 1034 and RFC 1035, both concerning DNS? | 1/3 (33%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 1034 STD 13 Domain names - concepts and facilities.pdf, rfc-editor.org-RFC 1034 STD 13 Domain names - concepts and facilities (1).pdf, rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, [Graph: RFC 2205], rfc-editor.org-RFC 8446      The Transport Layer Security TLS Protocol Version 13.pdf, rfc-editor.org-RFC 6455      The WebSocket Protocol.pdf | 27826ms |
| q04 | What core protocol does RFC 9000 (QUIC) define, and what transport-layer protocol does it build on top of? | 3/4 (75%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, [Graph: Long-Term Viability of Protocol Extension Mechanisms], rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf | 19853ms |
| q05 | Cross-document: How does QUIC (RFC 9000) attempt to solve limitations in TCP (RFC 9293) with respect to connection setup latency? | 4/4 (100%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, [Graph: Using TLS to Secure QUIC], rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf | 29984ms |
| q06 | What does RFC 8446 (TLS 1.3) define, and what security properties does it aim to guarantee for data in transit? | 3/4 (75%) | rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf, [Graph: Data Interchange Format RFC], rfc-editor.org-RFC 8446      The Transport Layer Security TLS Protocol Version 13.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf | 24057ms |
| q07 | What is the relationship between RFC 9110 (HTTP Semantics) and RFC 9112 (HTTP/1.1) — which defines protocol-version-specific syntax and which defines version-independent semantics? | 4/4 (100%) | [Graph: HTTP-to-HTTP], rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf | 21290ms |
| q08 | What does RFC 6455 define, and how does the WebSocket protocol differ from a standard HTTP request/response cycle? | 3/4 (75%) | rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf, [Graph: Hypertext Transfer Protocol HTTP] | 27616ms |
| q09 | What does RFC 6749 (OAuth 2.0) define, and what problem does it solve for third-party application access to user resources? | 2/4 (50%) | rfc-editor.org-RFC 6749      The OAuth 20 Authorization Framework.pdf, rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf, rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 1034 STD 13 Domain names - concepts and facilities (1).pdf, [Graph: O A u t h 2.0], rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf | 26798ms |
| q10 | What does RFC 7519 (JWT) define, and how does a JSON Web Token typically relate to an OAuth 2.0 authorization flow (RFC 6749)? | 3/4 (75%) | rfc-editor.org-RFC 1034 STD 13 Domain names - concepts and facilities.pdf, [Graph: JSON Web Token (JWT)], rfc-editor.org-RFC 7519      JSON Web Token JWT.pdf, rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf, rfc-editor.org-RFC 1034 STD 13 Domain names - concepts and facilities (1).pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, rfc-editor.org-RFC 6749      The OAuth 20 Authorization Framework.pdf, rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf | 34786ms |
| q11 | Cross-document: How do OAuth 2.0 (RFC 6749) and JWT (RFC 7519) work together in a typical authentication/authorization architecture, even though each RFC defines a distinct standard? | 4/4 (100%) | rfc-editor.org-RFC 6749      The OAuth 20 Authorization Framework.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, [Graph: O A u t h 2.0], rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf, rfc-editor.org-RFC 1034 STD 13 Domain names - concepts and facilities (1).pdf | 40529ms |
| q12 | What does RFC 8259 define, and what data format does it standardize? | 3/3 (100%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 8259 STD 90 The JavaScript Object Notation JSON Data Interchange Format.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, [Graph: Data Interchange Format RFC], rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf | 19789ms |
| q13 | What does RFC 4648 define? Name the three encoding schemes it covers. | 3/4 (75%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, [Graph: RFC-953], rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf, rfc-editor.org-RFC 4648      The Base16 Base32 and Base64 Data Encodings.pdf | 17107ms |
| q14 | Near-miss: JSON (RFC 8259) and JWT (RFC 7519) are related but distinct — how does a JWT actually use JSON internally, and what does JWT add on top of plain JSON? | 3/4 (75%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, rfc-editor.org-RFC 8446      The Transport Layer Security TLS Protocol Version 13.pdf, [Graph: RFC 8259, Section 1.2], rfc-editor.org-RFC 1034 STD 13 Domain names - concepts and facilities.pdf, rfc-editor.org-RFC 7519      JSON Web Token JWT.pdf, rfc-editor.org-RFC 8200 STD 86 Internet Protocol Version 6 IPv6 Specification.pdf | 37657ms |
| q15 | Cross-document: Organize RFC 768 (UDP), RFC 9293 (TCP), and RFC 9000 (QUIC) by which network layer each protocol operates at. | 4/4 (100%) | rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf, [Graph: RFC-768], rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf | 23052ms |
| q16 | Cross-document: How does DNS's (RFC 1034/1035) request/response model differ from HTTP's (RFC 9110/9112), and from WebSocket's (RFC 6455) persistent bidirectional model? | 4/4 (100%) | rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, rfc-editor.org-RFC 1034 STD 13 Domain names - concepts and facilities (1).pdf, rfc-editor.org-RFC 6749      The OAuth 20 Authorization Framework.pdf, rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf, [Graph: RFC 4303], rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 6455      The WebSocket Protocol.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf | 41396ms |
| q17 | Which RFC in this cluster defines a security protocol rather than a data-transport or data-format protocol? | 1/4 (25%) | rfc-editor.org-RFC 8446      The Transport Layer Security TLS Protocol Version 13.pdf, rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, [Graph: RFC-768], rfc-editor.org-RFC 8200 STD 86 Internet Protocol Version 6 IPv6 Specification.pdf, rfc-editor.org-RFC 6455      The WebSocket Protocol.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf | 26264ms |
| q18 | Edge case: RFC 9110 and RFC 9112 were both published to replace older combined HTTP/1.1 RFCs — why might a standards body choose to split "semantics" from "wire format" into separate documents? | 1/4 (25%) | rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, [Graph: Data Interchange Format RFC], rfc-editor.org-RFC 9112 STD 99 HTTP11.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf | 41436ms |
| q19 | Which two RFCs in this cluster are most likely to be used together to secure a WebSocket connection (RFC 6455) in production? | 2/4 (50%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, rfc-editor.org-RFC 9110 STD 97 HTTP Semantics.pdf, rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf, [Graph: Using TLS to Secure QUIC] | 19655ms |
| q20 | Edge case: Both RFC 768 (UDP) and RFC 9000 (QUIC) can carry data without TCP's guaranteed ordering — what does QUIC add on top of UDP's bare functionality to make it suitable for modern web transport? | 3/4 (75%) | rfc-editor.org-RFC 9000      QUIC A UDP-Based Multiplexed and Secure Transport.pdf, [Graph: Data Interchange Format RFC], rfc-editor.org-RFC 9293 STD 7 Transmission Control Protocol TCP.pdf | 25049ms |

## Answers

### q01 — What core function does RFC 9293 (TCP) define, and how does it differ fundamentally from RFC 768 (UDP) in terms of delivery guarantees?

[16] defines the core function of RFC 9293 (TCP) as providing a set of services to guarantee that all TCP implementations can support the same protocol hierarchy. 

TCP is defined as a connection-oriented transport protocol, which means it establishes a dedicated connection between the sender and receiver before data is sent. This connection is maintained throughout the communication session. 

In contrast, UDP (RFC 768) is a connectionless protocol, which means it does not establish a dedicated connection before sending data. Instead, it sends data packets independently, without any guarantee of delivery or order.

### q02 — What layer of the network stack does RFC 8200 (IPv6) operate at, and what key improvement does it offer over IPv4?

[13] specifies the layer and key improvement. 

RFC 8200 (IPv6) operates at the Internet layer of the network stack. 

IPv6 offers a key improvement over IPv4 by providing a much larger address space, which eliminates the problem of address exhaustion and provides for a more efficient use of IP addresses.

### q03 — What is the difference in purpose between RFC 1034 and RFC 1035, both concerning DNS?

The provided sources do not directly address the difference in purpose between RFC 1034 and RFC 1035. However, excerpt [5] mentions RFC 1034, but it only lists the title and a reference to a different RFC (1034) and does not provide any information about the purpose of RFC 1034.

### q04 — What core protocol does RFC 9000 (QUIC) define, and what transport-layer protocol does it build on top of?

According to [20], RFC 9000 (QUIC) defines a core protocol that provides applications with flow-controlled streams for structured communication, low-latency connection establishment, and network path migration. It also includes security measures that ensure confidentiality, integrity, and availability in a range of deployment circumstances.

Additionally, it builds on top of the UDP transport-layer protocol.

### q05 — Cross-document: How does QUIC (RFC 9000) attempt to solve limitations in TCP (RFC 9293) with respect to connection setup latency?

To address the question, we must examine both documents to identify the limitations in TCP and how QUIC attempts to solve them.

RFC 9293 describes the Transmission Control Protocol (TCP) and its design goals, which include efficient and reliable data transfer over the Internet. TCP's connection setup latency is a critical aspect of its design, as it can significantly impact the overall performance of the protocol. However, TCP's connection setup process can be slow due to the following limitations:

1.  The three-way handshake (SYN, SYN-ACK, ACK) required for connection establishment can be slow due to the need for each endpoint to wait for the other to respond.
2.  The use of sequence numbers and acknowledgments (ACKs) can introduce additional latency, especially if there are packet losses or retransmissions.

RFC 9000, which defines the QUIC transport protocol, aims to address these limitations and improve connection setup latency. QUIC achieves this through several key features:

1.  **0-RTT (Zero-Round-Trip Time) connection establishment**: QUIC allows for the transmission of application data in the Initial packet, which can be sent before receiving a response from the server. This reduces the connection setup latency to nearly zero.
2.  **Early key exchange**: QUIC enables the exchange of cryptographic keys before the completion of the connection establishment process. This allows for the encryption of subsequent packets and improves the overall security of the connection.
3.  **Stream multiplexing**: QUIC allows multiple streams to be multiplexed over a single connection, reducing the overhead associated with setting up multiple TCP connections.

By introducing these features, QUIC significantly improves the connection setup latency compared to TCP. QUIC's 0-RTT connection establishment and early key exchange enable faster connection setup, while stream multiplexing reduces the overhead associated with setting up multiple connections.

In summary, QUIC attempts to solve the limitations in TCP's connection setup latency by introducing features that enable faster connection establishment, early key exchange, and stream multiplexing.

References:

*   [RFC 9293] describes the Transmission Control Protocol (TCP) and its design goals, highlighting the limitations in TCP's connection setup latency.
*   [RFC 9000] defines the QUIC transport protocol and its features, including 0-RTT connection establishment, early key exchange, and stream multiplexing, which aim to address the limitations in TCP's connection setup latency.

### q06 — What does RFC 8446 (TLS 1.3) define, and what security properties does it aim to guarantee for data in transit?

RFC 8446, also known as TLS 1.3, defines the Transport Layer Security (TLS) Protocol Version 1.3. It describes the TLS handshake and the encrypted data transfer between a client and a server. The security properties that TLS 1.3 aims to guarantee for data in transit are:

1. Confidentiality: TLS 1.3 encrypts the data in transit to prevent eavesdropping and unauthorized access.
2. Authentication: TLS 1.3 authenticates the identity of the server and, optionally, the client.
3. Integrity: TLS 1.3 ensures that data is not tampered with or modified during transmission.
4. Key exchange: TLS 1.3 securely exchanges cryptographic keys between the client and server.
5. Session resumption: TLS 1.3 allows for efficient session resumption, reducing the overhead of key exchange and authentication.

These security properties are essential for protecting data in transit, ensuring that communication between a client and a server remains confidential, secure, and trustworthy.

[7] (RFC 8446)

### q07 — What is the relationship between RFC 9110 (HTTP Semantics) and RFC 9112 (HTTP/1.1) — which defines protocol-version-specific syntax and which defines version-independent semantics?

The provided sources do not contain the relationship between RFC 9110 (HTTP Semantics) and RFC 9112 (HTTP/1.1) in the exact way you have stated. However, it can be inferred that:

RFC 9110 is defined as "HTTP Semantics" and "This document specifies how HTTP semantics are conveyed using the HTTP/1.1 message syntax, framing, and connection management mechanisms." [17] and [19]. This suggests that RFC 9110 is related to version-independent semantics.

On the other hand, RFC 9112 is not mentioned in the provided sources, but it is implied that "HTTP/1.1" [HTTP/1.1] is a specific protocol version that defines protocol-version-specific syntax and connection management mechanisms.

### q08 — What does RFC 6455 define, and how does the WebSocket protocol differ from a standard HTTP request/response cycle?

The provided sources do not contain information about what RFC 6455 defines. 

However, we can look at the content of RFC 6455, which is a commonly known standard. RFC 6455 defines the WebSocket Protocol. 

The WebSocket protocol differs from a standard HTTP request/response cycle in the following ways:

*   It establishes a persistent, low-latency, bidirectional communication channel between the client and server.
*   It allows for full-duplex communication, meaning that both the client and server can send data at any time.
*   It provides a way to establish a persistent connection, allowing for efficient communication between the client and server.
*   It is designed to be used for real-time communication, such as online gaming, live updates, and chat applications.

The WebSocket protocol uses HTTP as a transport mechanism to establish the initial connection, but once established, it uses a different protocol to communicate. The WebSocket protocol uses a combination of HTTP and TCP/IP to provide a reliable, bidirectional communication channel. 

The WebSocket protocol is designed to be used in a variety of applications, including real-time communication, online gaming, live updates, and chat applications. It provides a flexible and efficient way to communicate between the client and server, making it a popular choice for real-time web applications.

### q09 — What does RFC 6749 (OAuth 2.0) define, and what problem does it solve for third-party application access to user resources?

RFC 6749 (OAuth 2.0) defines a method for third-party applications to access user resources on a server without having to store the user's login credentials. This solves the problem of users having to share their login credentials with third-party applications, which can be insecure and lead to unauthorized access to their resources.

### q10 — What does RFC 7519 (JWT) define, and how does a JSON Web Token typically relate to an OAuth 2.0 authorization flow (RFC 6749)?

[1] defines that the JSON Web Token (JWT) is a creative work that represents a standard for secure authentication and authorization in web applications. 

According to [10], the usage location of a token request involves the specification document RFC 6749.

The author mentions that RFC 6749 establishes the OAUTH Authorization Extensions Error Registry.

[11] states that the OAUTH Authorization Extensions Error Registry governs a 62-bit space, split into three ranges governed by different policies. This includes the Specification Required policy, except for values between 0x00 and 0x3f, which are assigned using Standards Action or IESG Approval.

[10] and [11] suggest that a JSON Web Token (JWT) is used in an OAuth 2.0 authorization flow (RFC 6749).

The provided sources do not explicitly define what RFC 7519 defines. However, as the JSON Web Token (JWT) is defined in RFC 7519, we can infer that it defines a standard for secure authentication and authorization in web applications.

A JSON Web Token typically relates to an OAuth 2.0 authorization flow (RFC 6749) by being used as a token to authenticate and authorize access to protected resources.

### q11 — Cross-document: How do OAuth 2.0 (RFC 6749) and JWT (RFC 7519) work together in a typical authentication/authorization architecture, even though each RFC defines a distinct standard?

Based on the provided sources, I'll attempt to outline how OAuth 2.0 (RFC 6749) and JWT (RFC 7519) work together in a typical authentication/authorization architecture.

**OAuth 2.0 (RFC 6749)**

* OAuth 2.0 is an authorization framework that allows a client to access a resource on behalf of a resource owner.
* The client requests an access token from the authorization server, which the client can use to access the protected resource.
* The access token is sent in the Authorization header of the request to the resource server.

**JWT (RFC 7519)**

* JWT is a JSON-based token that contains a set of claims, which are statements about the subject of the token.
* JWT is often used as an authentication token, where the claims include information about the user, such as their username, email, or roles.

**Integration**

* In a typical authentication/authorization architecture, OAuth 2.0 is used to obtain an access token, which is then used to authenticate the client.
* The access token is sent to the resource server, which verifies the token using a secret key.
* If the token is valid, the resource server returns the requested resource to the client.
* Meanwhile, the client may also receive a JWT token as part of the authentication process, which contains claims about the user.
* The JWT token can be used to authenticate the user in subsequent requests, or it can be stored in a session or cache for later use.

**Example**

* Suppose a client wants to access a protected resource on a server. The client requests an access token from the authorization server using the OAuth 2.0 flow.
* The authorization server issues an access token, which the client sends to the resource server in the Authorization header.
* The resource server verifies the access token and returns the requested resource to the client.
* Meanwhile, the client also receives a JWT token, which contains claims about the user. The client can store this token in a session or cache for later use.

Note that this is a simplified example, and the actual implementation details may vary depending on the specific requirements of the application.

References:

* RFC 6749: "The OAuth 2.0 Authorization Framework"
* RFC 7519: "JSON Web Tokens (JWT)"

Please let me know if you have any further questions or if you'd like me to elaborate on any of the points mentioned above.

### q12 — What does RFC 8259 define, and what data format does it standardize?

RFC 8259 defines the JavaScript Object Notation (JSON) Data Interchange Format, and it standardizes the JSON data format.

### q13 — What does RFC 4648 define? Name the three encoding schemes it covers.

According to excerpt [3], RFC 4648 defines the Base16, Base32, and Base64 data encodings.

### q14 — Near-miss: JSON (RFC 8259) and JWT (RFC 7519) are related but distinct — how does a JWT actually use JSON internally, and what does JWT add on top of plain JSON?

Based on the provided sources [1]–[20], particularly [14] and [15], we can find the relevant information.

[15] states that "Depending upon whether the JWT is a JWT or a JWE, there are two cases:"

*   If the JWT is a JWS, follow the steps specified in [JWS] for validating a JWS. Let the Message be the result of base64url decoding the JWS Payload.
*   Else, if the JWT is a JWE, follow the steps specified in [JWE] for decoding the JWE.

[14] mentions that "A client that has received multiple partial responses to GET requests on a target resource MAY combine those responses into a larger continuous range if they share the same strong validator."

However, [14] doesn't directly explain how a JWT uses JSON internally. But [15] clarifies that for validating a JWS, base64url decoding of the JWS Payload is required.

JWT (RFC 7519) is built on top of JWS (JSON Web Signature) and JWE (JSON Web Encryption) as specified in [JWS] and [JWE] respectively. JWT doesn't add anything on top of plain JSON but rather it uses JSON to create a compact, URL-safe means of representing claims (payload) to be transferred between two parties.

In a JWT, the payload (the actual data) is encoded in JSON format (as specified in RFC 8259), and then signed or encrypted, depending on the type of JWT. So, JWT uses JSON internally but also includes additional information like the signature or encryption details.

To directly answer the original question, JWT actually uses JSON to encode the payload, but it also includes additional information like the signature or encryption details, making it distinct from plain JSON.

### q15 — Cross-document: Organize RFC 768 (UDP), RFC 9293 (TCP), and RFC 9000 (QUIC) by which network layer each protocol operates at.

Here are the protocols organized by their respective network layers:

**Transport Layer:**

1. RFC 9293: TCP (Transmission Control Protocol)
	* Specifies the Transmission Control Protocol (TCP) and its functionality.
	* Includes updates to RFC 793, which was published in 1981.
2. RFC 768: UDP (User Datagram Protocol)
	* Specifies the User Datagram Protocol (UDP) and its functionality.
	* Provides a datagram service at the transport layer.

**Application Layer:**

1. RFC 9000: QUIC (Quick UDP Internet Connections)
	* Specifies the QUIC protocol and its functionality.
	* Provides a multiplexed and secure transport over UDP.

Note that while QUIC operates over UDP, which is at the transport layer, it is often considered an application-layer protocol due to its functionality and features.

### q16 — Cross-document: How does DNS's (RFC 1034/1035) request/response model differ from HTTP's (RFC 9110/9112), and from WebSocket's (RFC 6455) persistent bidirectional model?

DNS (RFC 1034/1035) request/response model differs from HTTP's (RFC 9110/9112) and WebSocket's (RFC 6455) models in the following ways:

1.  **Request/Response Structure:** The DNS request/response structure is designed for query-response exchanges, where a client sends a query and the server responds with the relevant data. In contrast, HTTP's request/response model is designed for request-response exchanges, where a client sends a request and the server responds with the requested data. WebSocket's persistent bidirectional model, on the other hand, allows for simultaneous bi-directional communication between the client and server, enabling real-time data exchange.

2.  **Connection Establishment:** DNS operates over UDP by default and can use TCP for transactions that require more data. HTTP typically uses TCP for both request and response, while WebSocket establishes a persistent bi-directional connection over TCP.

3.  **Query/Response Format:** DNS query/response format is based on packets containing a header, followed by the query and response data. HTTP requests are typically sent in the form of HTTP messages (HTTP requests and responses), which consist of a start line, headers, and a body. WebSocket's persistent connection allows for bi-directional communication, where the client and server can send messages at any time.

4.  **Request/Response Caching:** HTTP includes mechanisms for caching responses and reusing them for subsequent requests. DNS does not have a built-in caching mechanism, but clients often implement their own caching mechanisms. WebSocket's persistent connection model does not rely on caching.

5.  **Error Handling:** DNS defines specific error codes for indicating the outcome of a query (e.g., DNS response codes such as NOERROR and SERVFAIL). HTTP includes a wide range of status codes for different response types, while WebSocket's error handling is generally based on TCP error codes.

6.  **Message Multiplexing:** WebSocket's persistent connection model allows for multiplexing multiple streams of messages over a single connection. HTTP has similar multiplexing capabilities through HTTP/2. DNS, on the other hand, is designed for a single query-response exchange per connection.

7.  **Authentication and Authorization:** HTTP provides mechanisms for authentication and authorization through headers and other means. WebSocket also provides similar authentication and authorization mechanisms. DNS does not include built-in authentication and authorization mechanisms but can use external means such as DNSSEC for authentication.

In summary, the request/response models of DNS (RFC 1034/1035), HTTP (RFC 9110/9112), and WebSocket (RFC 6455) differ significantly due to the distinct requirements and use cases of each protocol.

### q17 — Which RFC in this cluster defines a security protocol rather than a data-transport or data-format protocol?

The RFC that defines a security protocol rather than a data-transport or data-format protocol is [10] "RFC 8446: The Transport Layer Security (TLS) Protocol Version 1.3". 

This RFC defines the TLS protocol, which is a cryptographic protocol used for secure communication over the internet.

### q18 — Edge case: RFC 9110 and RFC 9112 were both published to replace older combined HTTP/1.1 RFCs — why might a standards body choose to split "semantics" from "wire format" into separate documents?

The provided sources do not contain a clear explanation for why a standards body might choose to split "semantics" from "wire format" into separate documents. However, the text in [10] mentions that "This revision of HTTP separates the definition of semantics (this document) and caching ([CACHING]) from the current HTTP/1.1 messaging syntax ( [HTTP/1.1 ]) to allow each major protocol version to progress independently while referring to the same core semantics."

This suggests that the reason for splitting the documents is to allow different versions of the protocol to evolve independently, with each version having its own "wire format" while still sharing the same underlying "semantics". This would enable the development of new features and improvements in each version without having to synchronize changes across all versions.

Additionally, [11] mentions that "HTTP's core semantics don't change between protocol versions, their expression 'on the wire' can change, and so the HTTP version number changes when incompatible changes are made to the wire format." This implies that by separating the semantics from the wire format, the standards body can ensure that changes to the wire format do not affect the underlying semantics, allowing for more flexibility and evolution in the protocol.

### q19 — Which two RFCs in this cluster are most likely to be used together to secure a WebSocket connection (RFC 6455) in production?

Based on the provided excerpts, [6] and [1] are likely to be used together to secure a WebSocket connection in production. [6] refers to RFC 9001, which is titled "Using TLS to Secure QUIC", and [1] refers to a document titled "Using TLS to Secure QUIC", indicating that TLS is used to secure the QUIC protocol. 

WebSocket connections typically use TLS or SSL/TLS to secure the connection.

### q20 — Edge case: Both RFC 768 (UDP) and RFC 9000 (QUIC) can carry data without TCP's guaranteed ordering — what does QUIC add on top of UDP's bare functionality to make it suitable for modern web transport?

[16] states that QUIC authenticates the entirety of each packet and encrypts as much of each packet as is practical.

[16] also states that endpoints communicate in QUIC by exchanging QUIC packets. Most packets contain frames, which carry control information and application data between endpoints.

[4] states that QUIC packets are carried in UDP datagrams to better facilitate deployment in existing systems and networks.

[3] mentions that QUIC provides applications with flow-controlled streams for structured communication, low-latency connection establishment, and network path migration.

[9] states that more than one connection ID can be associated with a connection.

[10] states that QUIC is a connection-oriented protocol that creates a stateful interaction between a client and server.

[17] states that QUIC MUST NOT be used if the network path cannot support a maximum datagram size of at least 1200 bytes.

[18] states that an endpoint MUST disable ECN if validation later fails.

[16] states that QUIC packets are carried in UDP datagrams to better facilitate deployment in existing systems and networks.

These elements indicate that QUIC adds features on top of UDP's bare functionality, including:

* Authentication and encryption of packets
* Frames to carry control information and application data between endpoints
* Connection establishment and migration
* Flow control for structured communication
* Stateful interaction between client and server
* Support for maximum datagram size of at least 1200 bytes
* Ability to disable ECN if validation later fails

These features make QUIC suitable for modern web transport by providing a secure, efficient, and reliable transport layer protocol.

