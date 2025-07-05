# chat_proto
![State](https://img.shields.io/badge/State-WIP-red)


This repository contains the core implementation of the $CHAT_PROTO types. 

To make them easy to use, implementations in Rust and Nim are provided developers can focus on building. 


## Structure

- **proto:** Protobuf definitions for Core and Conversation types
- **nim:**  nimble package for generated types in nim
- **rust:** cargo crate for generated types in rust
- **specs:** current home of the specifications - these will likely be moved out of the repo. They are currently in lockstep with the type definitions which minimizes desync.   

## Related Repositories

- [Nim POC](https://github.com/waku-org/nim-chat-poc/tree/jazzz/inbox) - This is a demo of the types being consumed in`nim`
  - Importing packages from a monorepo appears to be broken in `nimble`, as a short term work around. The `.proto` files have been embedded in the POC, and will be removed once resolved

- [Rust POC](https://github.com/jazzz/umbra) - This is demo of the types being consumed in `rust`

