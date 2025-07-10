import unittest

import chat_proto

suite "Type Encoding Tests":
  test "HistoryEntry roundtrip": 
    let x = chat_proto.HistoryEntry(message_id: "12345", retrieval_hint: @[1'u8, 2, 3, 255])
    let encoded = Protobuf.encode(x)
    let decoded = Protobuf.decode(encoded, HistoryEntry)

    check x.message_id == decoded.message_id
    check x.retrieval_hint == decoded.retrieval_hint
    check x == decoded
