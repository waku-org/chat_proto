---
title: INBOX
name: Inbound messages  
category: Standards Track
status: raw
tags: chat
editor: Jazz Alyxzander<jazz@status.im>
contributors:
---
## Abstract


## Background / Rationale / Motivation



## Theory / Semantics

Inboxes are inbound only conversation types, which allow a client to recieve messages from contacts. 
An inbox does not have a defined set of participants, and is used to receive messages when there does not exist and established channel between contacts. 
All messages to an inbox MUST be encrypted. [TODO: Reason]

### Accepted types
Inboxes SHOULD attempt to handle all envelopes sent with the conversation_id `/inbox/v1/<inbox_address>`.

### Default Inbox

[Differentiate between Inbox the place you recieve and the  Protocol -- Is htere a difference]
An inbox is the default conversation type. 
It is a catch-all for all messages which 



[TODO: is domain separation needed here or should we save some bytes?]


**Encryption**

All Frames sent to the INBOX must be encrypted.

Frames CAN be encrypted

**Content Topic**

// TODO: Inbox Topics will be defined in ContactBundles, allowing for dynamic topic usage

All clients must listen for messages posted with the content topic `/inbox/<inbox_address>`

###  Invites 
Conversations are required to define their own Invites, which contain the required information to bootstrap new participants.


[TODO: InviteV1]

### EncryptedBytes

The EncryptedBytes message is a self-describing wrapper for all encrypted payloads. As the protocol grows it will include potentially different encryption mechanisms. This message type makes no assumptions about the encryption used an allows new conversation types to use the same messaging framework.


[TODO: Why isn't this defined within the  conversation frames like SDS?]



### Invites

Individual Conversations are responsible for defining a payload which will be used to initialize remote participants.

#### Invite Encryption 

Invite




## Wire Format Specification / Syntax
The wire format is specified using protocol buffers v3.

```mermaid

message EncryptedBytes {

    oneof encryption {
        bytes encrypted_bytes=1;
        Plaintext plaintext = 2;
		Ecies ecies = 3;
    }
   
    
    message Ecies {
        bytes encrypted_bytes=1;
        bytes ephemeral_pubkey = 2;
        bytes tag = 3;
        
    }

    message Plaintext {
        bytes payload=1;
    }
}

```

## Implementation Suggestions (optional)
An optional *implementation suggestions* section may provide suggestions on how to approach implementation details, and, 
if available, point to existing implementations for reference.


## (Further Optional Sections)


## Security/Privacy Considerations


## Copyright

Copyright and related rights waived via [CC0](https://creativecommons.org/publicdomain/zero/1.0/).

## References

A list of references.
