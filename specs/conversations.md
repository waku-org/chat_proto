---
title: CONVERSATIONS
name: Conversation based communications 
category: Standards Track
status: raw
tags: chat
editor: Jazz Alyxzander<jazz@status.im>
contributors:
---
# Abstract

This specification outlines the base message structure for the Conversations protocol. 

# Background / Rationale / Motivation

Modern communication systems require a reliable and flexible way to establish and manage conversations between participants. 
While simple message-passing protocols exist, they lack the ability to manage state across a stream of messages.

The Conversations Protocol addresses this gap by defining a mechanism to establish long term communication sessions in decentralized environments.


# Theory / Semantics

[TODO: remove this link, by summarizing and embedding in the spec so its standalone]
This is a lightweight framework for defining Conversation micro-protocols.
The high level Conversations approach outlined [here](https://forum.vac.dev/t/chatsdk-conversations/509). 
The intention is to provide the minimal building blocks for a wide range of micro-protocols, while allowing ConversationTypes to remain as flexible as possible. 

At a high level this Protocol defines what a ConversationType is, it's requirements, and a mechanism for initializing Conversations with others. 

**Frame**: 

## Conversations
A ConversationType is a specification which defines a method for clients to communicate.
Each ConversationType defines its own encryption, encoding, and message types which are necessary for operation.   

A ConversationType MUST define which content topics are valid places to receive messages.
A ConversationType MUST define which encryption scheme is used
A ConversationType MUST define which Frames are valid. 
- Clients MUST be able to decode frames deterministically. 
A ConversationType SHOULD define membership requirements and limitations.

A Conversation is an instance of a particular ConversationType which contains the associated state such as membership, encryption parameters, names etc.  

### ConversationIdentifiers

A Conversation instance MUST a conversation_id and the `conversation_id` MUST uniquely identify the conversation.
[TODO: Should more guidance be added later? e,g /<convo_type>/<version>/<ident>]


## Default Inbox
The default inbox allows clients to discover new conversations asynchronously without prior coordination. By listening in a static location


To achieve this all clients MUST implement a default Inbox with `inbox_address = HASH(client_address)`. [TODO: Define hash here, or embed as part of the Inbox spec]
See [Inbox](./inbox.md) for more information.


As the clients address is directly linked to the content_topic there is some metadata leakage, and this pathway SHOULD only be used as a last resort.    

## Framing
To disambiguate between different logical layers, payload types sent by a Conversation are referred to as `Frames`.

Conversations are free to determine which frames are needed for their specific use cases, with these caveats:
- The outer most frame MUST be wrapped in an UmbraEnvelope.
- All frames MUST be able to be decoded deterministically. [TODO: Unambiguously a better choice?]

Deterministic decoding means that clients can always classify a envelope as 1 of 3 states: Readable, BadlyFormed, Addressed to someone else.
 

### Conversation Hinting
[TODO: Needs lots of work]

UmbraEnvelopes enable deterministic decoding by containing a reference to the conversation which this message belongs. Clients only accepts envelopes with known `conversation_hints's`. All others can be discarded as there is insufficient information to properly decrypt/decode the messages.
 

Conversation identifiers (conversation_id) have the potential to leak sensitive metadata if exposed in cleartext. Frames sharing the same conversation_id could allow observers to infer social graph relationships, user activity patterns, or conversation linkage, depending on how conversation_id values are used by the specific ConversationType.

To mitigate this risk and provide a safer default for Conversation implementors, conversation_id values SHOULD be obscured in a way that prevents observers from linking frames belonging to the same conversation.

[TODO: Ratcheting Private Identifiers]



## Wire Format Specification / Syntax

The wire format is specified using protocol buffers v3.

```protobuf

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

Messages sent to the default inbox are linkable to an client (as it is derived from the clients address). This means that if a target client address is known to an observer, they can determine if any messages were sent to the target using the default inbox.  In this case the Envelopes contain no sender information, so this does not leak social graph information.

Messages sent via different pathways would have their own privacy guarantees.


## Copyright

Copyright and related rights waived via [CC0](https://creativecommons.org/publicdomain/zero/1.0/).

## References

A list of references.
