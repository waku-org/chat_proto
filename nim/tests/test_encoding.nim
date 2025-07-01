import unittest

import chat_types

suite "Type Encoding Tests":
  test "HistoryEntry roundtrip": 
    let x = chat_types.HistoryEntry(message_id: "12345", retrieval_hint: @[1'u8, 2, 3, 255])
    let encoded = Protobuf.encode(x)
    let decoded = Protobuf.decode(encoded, HistoryEntry)

    check x.message_id == decoded.message_id
    check x.retrieval_hint == decoded.retrieval_hint
    check x == decoded

  test "Multiplication works":
    check(2 * 3 == 6)