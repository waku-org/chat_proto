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

This specification outlines the requirements for defining a ConversationType

# Background / Rationale / Motivation

Modern communication systems require a reliable and flexible way to establish and manage conversations between participants. 
While simple message-passing protocols exist, they lack the ability to manage state across a stream of messages.

The Conversations Protocol addresses this gap by defining a mechanism to establish long term communication sessions in decentralized environments.


# Theory / Semantics

A ConversationType is a specification which defines a method for clients to communicate.
Each ConversationType defines its own encryption, encoding, and message types which are necessary for operation.   

A Conversation is an instance of a particular ConversationType which contains the associated state such as membership, encryption parameters, names etc.  

## Requirements
To be considered valid, every ConversationType specification is required to define the operation completely. 

A ConversationType MUST define which encryption scheme is used
A ConversationType MUST define which Frames are used.
A ConversationType MUST define which encoding is used.
A ConversationType MUST define which content topics are valid places to receive messages.
A ConversationType MUST define how to generate conversation_ids

A ConversationType SHOULD define membership requirements and limitations.
A ConversationType SHOULD define privacy and security guarantees.


## ConversationType Identifiers
ConversationTypes are identified by the title of the specification. This allows developers to lookup the associated specification.

[TODO: This doesn't make any sense, as its been mentioned that new versions of a conversationType are distinct types. Which is it? ]

E.g. inbox.md -> InboxV1

## Conversation Identifiers

conversation_ids allow for the efficient lookup of encryption state. 

Care should be taken to ensure that conversation_ids do not conflict.
[TODO: Should more guidance be added later? e,g  mandating a format to ensure uniqueness between conversationTypes -- /<convo_name>/<version>/<ident>]

[TODO: touch on the nuance of generating conversation_ids from participant lists?]


## Framing
To disambiguate between different logical layers, payload types sent by a Conversation are referred to as `Frames`.
Conversations are free to determine which frames are needed for their specific use cases.

ConversationTypes MUST define a section which defines all possible frames 

ConversationTypes SHOULD maintain a deterministic decoding tree.


## Encryption
Conversation types are free to choose which ever encryption mechanism works best for their application. 
[TODO: Expand on recomendations]


## Content Topics
Content topics are how ConversationTypes define where an how messages are discovered by participants. 

When developing new ConversationTypes contributors should consider:
- Privacy impacts of the chosen topic policy.
- Channel binding and the impacts on message security.



## Implementation Suggestions (optional)
An optional *implementation suggestions* section may provide suggestions on how to approach implementation details, and, 
if available, point to existing implementations for reference.


## (Further Optional Sections)


## Security/Privacy Considerations

This approach puts heavy requirements on ConversationTypes to build their own cryptography without providing much guidance. Finding mechanisms that provide safety while maintaining the flexibility should be prioritized in follow up work.

## Copyright

Copyright and related rights waived via [CC0](https://creativecommons.org/publicdomain/zero/1.0/).

## References

A list of references.
