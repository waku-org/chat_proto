---
title: PROTOCOL_TAGS
name: Payload tags in Multiprotocol Environments
category: Standards Track
status: raw
tags: chat
editor: Jazz Alyxzander<jazz@status.im>
contributors:
---



## Abstract


## Background / Rationale / Motivation
In a multi-protocol environment, it is not always possible for clients to determine how to decode a received message.
This is especially true when deploying a new version of a protocol, as older clients do not have the logic to decode these new payloads. 

If clients cannot deterministically decode messages then they must attempt decoding the payload into each possible type. 
In this context failure to decode a message leads to ambiguity as to whether the payload is corrupted or simply an unsupported format.”

Decoding hints can be encoded into the transport medium, however this results in the data being tightly coupled to a specific implementation - which makes versioning more difficult.  
 Payloads should be self-describing, allowing decoding without depending on details within a specific version of the software.

Having a mechanism to identify which protocol generated a payload, makes decoding more efficient and straightforward. 


## Theory / Semantics
This specification introduces a lightweight tagging scheme for identifying payload types in a multi-protocol environment. It enables clients to determine how to decode payloads without trial-and-error, even when multiple protocol versions exist.

Each payload is tagged with a reference to the specification which defines its structure. 
Tagging acts as a reference to the protocol specification that defines the payload’s structure.
As each payload contains a link to the associated specification, clients can decode the message accordingly. 

To accomplish this the wire format includes two different Identifiers. 
1. domain: the governing body who oversees tag registration.
2. tag: which specification defines this payload type.

### Tags

Tags are a mapping of a numeric to a specification. Numeric values are used in this context to save space in the payload.

The mapping MUST contain at least the tag value and a link to the associated specification for the payload. It CAN include more data as necessary. 
How the tags are managed is outside of this specification and is up to the domain.

- Tags MUST NOT be reused.
- Tags SHOULD correspond to the specifications number if there is one.
- Tags SHOULD only be used for identifying the root data type. 
Its assumed that all types from the root can be decoded deterministically. 

### Registries
registries are a mapping of a numeric to a authority responsible for managing tag values. Numeric values are used in this context to save space in the payload.


`registry` mappings can be found in appendix A.

### Decoding

Clients can safely ignore payloads which they do not understand. 


## Wire Format Specification

These types are defined using Protobuf V3.

```
message TaggedPayload {
	uint32 registry = 1;            // Mapping to specification registry
	uint32 tag = 10;                // Mapping to type specification
	bytes payload_bytes = 20;       // Encoded object
}
```

## Security/Privacy Considerations

Explicitly tagging payloads with their type does not meaningfully leak metadata. 
To provide any privacy protections contents need to be encrypted. Occluding the payloads only slows an adversary from decoding that data. 

In cases where the payloads are unencrypted and the type secrecy is critical, payload tagging should not be used.


## Copyright

Copyright and related rights waived via [CC0](https://creativecommons.org/publicdomain/zero/1.0/).

## References

A list of references.

## Appendix A: Registries



| Month    | Name | Link                                                 |
| -------- | -----| ---------------------------------------------------- |
| 0        | Vac  | https://github.com/vacp2p/rfc-index                  |
