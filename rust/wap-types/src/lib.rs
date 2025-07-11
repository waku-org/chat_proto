pub mod payload;

pub use payload::types::wap::*;

#[cfg(test)]

mod tests {
    use super::*;
    use prost::Message;

    #[test]
    fn test_private_v1_roundtrip() {
        convos::private_v1::PrivateV1Frame {
            conversation_id: "conversationId".to_string(),
            frame_type: Some(convos::private_v1::private_v1_frame::FrameType::Content(
                common_frames::ContentFrame {
                    domain: 0,
                    tag: 0,
                    bytes: "Hello, World!".to_string().encode_to_vec(),
                },
            )),
        };
    }
}
