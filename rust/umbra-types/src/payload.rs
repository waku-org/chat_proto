pub mod types {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

use prost::Message;
pub use types::umbra::*;

use crate::{
    base::{EncryptedBytes, InboxV1Frame, UmbraEnvelopeV1, inbox_v1_frame},
    convos::private_v1::{PrivateV1Frame, private_v1_frame},
};

impl PrivateV1Frame {
    pub fn new(conversation_id: String, frame: private_v1_frame::FrameType) -> Self {
        PrivateV1Frame {
            conversation_id,
            frame_type: Some(frame),
        }
    }
}

impl InboxV1Frame {
    pub fn new(recipient: String, frame: inbox_v1_frame::FrameType) -> Self {
        InboxV1Frame {
            recipient: recipient,
            // conversation_id,
            frame_type: Some(frame),
        }
    }
}

pub trait ToEnvelope {
    fn to_envelope(self, conversation_id: String, salt: u64) -> UmbraEnvelopeV1;
}

impl ToEnvelope for EncryptedBytes {
    fn to_envelope(self, conversation_id: String, salt: u64) -> UmbraEnvelopeV1 {
        UmbraEnvelopeV1 {
            conversation_hint: conversation_id, // TODO
            salt,
            payload: self.encode_to_vec(), // Avoid allocation here?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    #[test]
    fn test_private_v1_roundtrip() {
        let text = "Hello, World!".to_string();

        let msg = PrivateV1Frame {
            conversation_id: "conversationId".to_string(),
            frame_type: Some(convos::private_v1::private_v1_frame::FrameType::Content(
                common_frames::ContentFrame {
                    domain: 0,
                    tag: 0,
                    bytes: text.encode_to_vec(),
                },
            )),
        };

        let reliable = base::ReliableBytes {
            message_id: "msg_id".into(),
            channel_id: msg.conversation_id.clone(),
            lamport_timestamp: 0,
            causal_history: vec![],
            bloom_filter: vec![1, 2, 3, 4],
            content: Some(msg.encode_to_vec()),
        };

        let buf = reliable.encode_to_vec();

        let reliable_msg = base::ReliableBytes::decode(&*buf).unwrap();

        let msg_from_bytes =
            convos::private_v1::PrivateV1Frame::decode(&*reliable_msg.content.unwrap())
                .expect("Failed to decode message");

        assert_eq!(
            msg, msg_from_bytes,
            "Encoded and decoded messages should match"
        );
    }
}
