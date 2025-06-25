pub mod types {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

pub use types::umbra::*;

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    #[test]
    fn test_private_v1_roundtrip() {
        let text = "Hello, World!".to_string();

        let msg = convos::private_v1::PrivateV1Frame {
            message_id: "messageId".to_string(),
            conversation_id: "conversationId".to_string(),
            reliability_info: Some(base::ReliabilityInfo {
                lamport_timestamp: 0,
                causal_history: vec![],
                bloom_filter: vec![1, 2, 3, 4],
            }),
            frame_type: Some(convos::private_v1::private_v1_frame::FrameType::Content(
                common_frames::ContentFrame {
                    domain: 0,
                    tag: 0,
                    bytes: text.encode_to_vec(),
                },
            )),
        };

        let bytes = msg.encode_to_vec();

        let msg_from_bytes =
            convos::private_v1::PrivateV1Frame::decode(&*bytes).expect("Failed to decode message");

        assert_eq!(
            msg, msg_from_bytes,
            "Encoded and decoded messages should match"
        );
    }
}
