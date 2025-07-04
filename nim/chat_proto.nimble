# Package
packageName   = "chat_proto"
version       = "0.1.0"
author        = "jazzz"
description   = "Type definitions for the chat protocol"
license       = "MIT"
bin = @["chat_proto"]

# Dependencies
requires "nim >= 2.0.14"
requires "chronicles"
requires "protobuf_serialization"
requires "protobuf"
