---
title: CONVERSATIONS
name: Conversation based communications 
category: Standards Track
status: raw
tags: chat
editor: Jazz Alyxzander<jazz@status.im>
contributors:
---
## Abstract

This specification outlines the base message structure for the Conversations protocol. 

## Background / Rationale / Motivation

Modern communication systems require a reliable and flexible way to establish and manage conversations between participants. 
While simple message-passing protocols exist, they lack the ability to manage state across a stream of messages.

The Conversations Protocol addresses this gap by providing a clear and standardized way for participants to initiate, conversations in decentralized environments.



## Theory / Semantics

[TODO: remove this link, by sumamrizing and embedding in the spec so its standalone]
This is a lightweight framework for defining Conversation micro-protocols.
The high level Conversations approach outlined [here](https://forum.vac.dev/t/chatsdk-conversations/509). 
The intention is to provide the minimal building blocks for a wide range of micro-protocols, while allowing ConversationTypes to remain as flexible as possible. 


### Conversations
A conversation is responsible for defining its encryption, encoding, and message types which are necessary for operation. 

A ConversationType MUST define which content topics are valid places to receive messages.
A ConversationType MUST define which encryption scheme is used
A ConversationType MUST define which Frames are valid. 
- Clients MUST be able to decode frames deterministically. 

A ConversationType SHOULD define membership requirements and limitations.

Every Conversation instance MUST have a conversation_id.


**Decoding** 
Clients determine how to decode a byte sequence by the `conversation_id`. 

[TODO: Should we add more guidance later? e,g /<convo_type>/<version>/<ident>]
`conversation_id's` must be uniquely identify which encryption state to. As there may be many ConversationTypes defined, specifications should be mindful of conflicts. 

Clients only accepts envelopes with known `conversation_id's`. All others can be discarded as there is insufficient information to properly decrypt/decode the messages.


**Framing**
To maintain a deterministic decoding tree, all frames on the wire MUST be wrapped with a `UmbraEnvelope`. 
This provides a common type for all frames.

Underneath UmbraEnvelope, ConversationTypes are free to wrap frames as desired, however it is recommended to use `EncryptedFrame` to  ensure clients can decode messages regardless of software version.


**Conversation Hinting**
[TODO: Needs lots of work]
Conversation identifiers (conversation_id) have the potential to leak sensitive metadata if exposed in cleartext. Frames sharing the same conversation_id could allow observers to infer social graph relationships, user activity patterns, or conversation linkage, depending on how conversation_id values are used by the specific ConversationType.

To mitigate this risk and provide a safer default for Conversation implementors, conversation_id values SHOULD be obscured in a way that prevents observers from linking frames belonging to the same conversation.

[TODO: Ratcheting Private Identifiers]



### Default Inbox
To provide a baseline for interaction, clients need a method to receive initial frames/invites. To achieve this all clients MUST implement a default Inbox with `inbox_address = HASH(client_address)`. [TODO: Define hash here, or embed as part of the Inbox spec]

See [Inbox](./inbox.md) for more information.


## Wire Format Specification / Syntax

The wire format is specified using protocol buffers v3.

```mermaid

message UmbraEnvelopeV1 {
    
    string conversation_hint = 1;
    uint32 salt = 2;           
    
    EncryptedBytes encrypted_bytes = 10;
}


```

## Implementation Suggestions (optional)
An optional *implementation suggestions* section may provide suggestions on how to approach implementation details, and, 
if available, point to existing implementations for reference.


## (Further Optional Sections)


## Security/Privacy Considerations
[TODO: cover Conversation_id -> Message Types ]



## Copyright

Copyright and related rights waived via [CC0](https://creativecommons.org/publicdomain/zero/1.0/).

## References

A list of references.
