// Hand-maintained `prost::Message` mirrors of `proto/support_ticket/v1/support_ticket.proto`.
// Tag numbers match the .proto byte-for-byte; `tools/validate_prost_tags`
// runs at build time to catch drift (watch list §8). NO `Serialize` /
// `Deserialize` derives — binary wire only (conventions §2.5, §3).
//
// User-scoped service (conventions §3.5 exception #1): requests carry no
// `org_slug`. Tickets are scoped by the authenticated UserID server-side.

// ----- Entities -----

#[derive(Clone, PartialEq, prost::Message)]
pub struct SupportTicket {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(string, tag = "3")]
    pub title: String,
    #[prost(string, tag = "4")]
    pub category: String,
    #[prost(string, tag = "5")]
    pub status: String,
    #[prost(string, tag = "6")]
    pub priority: String,
    #[prost(string, tag = "7")]
    pub created_at: String,
    #[prost(string, tag = "8")]
    pub updated_at: String,
    #[prost(string, optional, tag = "9")]
    pub resolved_at: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SupportTicketUser {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, optional, tag = "2")]
    pub name: Option<String>,
    #[prost(string, tag = "3")]
    pub email: String,
    #[prost(string, optional, tag = "4")]
    pub avatar_url: Option<String>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SupportTicketAttachment {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub ticket_id: i64,
    #[prost(int64, optional, tag = "3")]
    pub message_id: Option<i64>,
    #[prost(int64, tag = "4")]
    pub uploader_id: i64,
    #[prost(string, tag = "5")]
    pub original_name: String,
    #[prost(string, tag = "6")]
    pub mime_type: String,
    #[prost(int64, tag = "7")]
    pub size: i64,
    #[prost(string, tag = "8")]
    pub created_at: String,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SupportTicketMessage {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub ticket_id: i64,
    #[prost(int64, tag = "3")]
    pub user_id: i64,
    #[prost(string, tag = "4")]
    pub content: String,
    #[prost(bool, tag = "5")]
    pub is_admin_reply: bool,
    #[prost(string, tag = "6")]
    pub created_at: String,
    #[prost(message, optional, tag = "7")]
    pub user: Option<SupportTicketUser>,
    #[prost(message, repeated, tag = "8")]
    pub attachments: Vec<SupportTicketAttachment>,
}

// ----- Requests / Responses -----

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListSupportTicketsRequest {
    #[prost(string, tag = "1")]
    pub status: String,
    #[prost(int32, optional, tag = "2")]
    pub offset: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub limit: Option<i32>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct ListSupportTicketsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: Vec<SupportTicket>,
    #[prost(int64, tag = "2")]
    pub total: i64,
    #[prost(int32, tag = "3")]
    pub limit: i32,
    #[prost(int32, tag = "4")]
    pub offset: i32,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetSupportTicketRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct SupportTicketDetail {
    #[prost(message, optional, tag = "1")]
    pub ticket: Option<SupportTicket>,
    #[prost(message, repeated, tag = "2")]
    pub messages: Vec<SupportTicketMessage>,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetAttachmentUrlRequest {
    #[prost(int64, tag = "1")]
    pub attachment_id: i64,
}

#[derive(Clone, PartialEq, prost::Message)]
pub struct GetAttachmentUrlResponse {
    #[prost(string, tag = "1")]
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    fn sample_ticket() -> SupportTicket {
        SupportTicket {
            id: 42,
            user_id: 7,
            title: "Login failure on 2FA".into(),
            category: "bug".into(),
            status: "open".into(),
            priority: "high".into(),
            created_at: "2026-05-12T13:16:10Z".into(),
            updated_at: "2026-05-12T13:16:10Z".into(),
            resolved_at: None,
        }
    }

    fn sample_attachment() -> SupportTicketAttachment {
        SupportTicketAttachment {
            id: 11,
            ticket_id: 42,
            message_id: Some(99),
            uploader_id: 7,
            original_name: "screenshot.png".into(),
            mime_type: "image/png".into(),
            size: 12_345,
            created_at: "2026-05-12T13:16:10Z".into(),
        }
    }

    fn sample_message() -> SupportTicketMessage {
        SupportTicketMessage {
            id: 99,
            ticket_id: 42,
            user_id: 7,
            content: "Steps to reproduce: ...".into(),
            is_admin_reply: false,
            created_at: "2026-05-12T13:16:10Z".into(),
            user: Some(SupportTicketUser {
                id: 7,
                name: Some("Alice".into()),
                email: "alice@example.com".into(),
                avatar_url: None,
            }),
            attachments: vec![sample_attachment()],
        }
    }

    #[test]
    fn support_ticket_round_trip_preserves_every_field() {
        let original = sample_ticket();
        let bytes = original.encode_to_vec();
        let decoded = SupportTicket::decode(&*bytes).unwrap();
        assert_eq!(original, decoded,
            "tag swap or transcription mistake would surface as field-value swap here");
    }

    #[test]
    fn support_ticket_with_resolved_at_round_trips() {
        let original = SupportTicket {
            resolved_at: Some("2026-05-13T00:00:00Z".into()),
            ..sample_ticket()
        };
        let bytes = original.encode_to_vec();
        let decoded = SupportTicket::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.resolved_at.as_deref(), Some("2026-05-13T00:00:00Z"));
    }

    #[test]
    fn support_ticket_attachment_round_trip() {
        let original = sample_attachment();
        let bytes = original.encode_to_vec();
        let decoded = SupportTicketAttachment::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.message_id, Some(99));
    }

    #[test]
    fn support_ticket_attachment_without_message_id_round_trips() {
        let no_msg = SupportTicketAttachment { message_id: None, ..sample_attachment() };
        let bytes = no_msg.encode_to_vec();
        let decoded = SupportTicketAttachment::decode(&*bytes).unwrap();
        assert_eq!(no_msg, decoded);
        assert!(decoded.message_id.is_none());
    }

    #[test]
    fn support_ticket_user_round_trip() {
        let original = SupportTicketUser {
            id: 7,
            name: Some("Alice".into()),
            email: "alice@example.com".into(),
            avatar_url: Some("https://example.com/avatar.png".into()),
        };
        let bytes = original.encode_to_vec();
        let decoded = SupportTicketUser::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn support_ticket_message_with_user_and_attachments_round_trips() {
        let original = sample_message();
        let bytes = original.encode_to_vec();
        let decoded = SupportTicketMessage::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.attachments.len(), 1);
        assert_eq!(decoded.user.as_ref().unwrap().email, "alice@example.com");
    }

    #[test]
    fn list_support_tickets_response_round_trip() {
        let original = ListSupportTicketsResponse {
            items: vec![sample_ticket()],
            total: 1,
            limit: 20,
            offset: 0,
        };
        let bytes = original.encode_to_vec();
        let decoded = ListSupportTicketsResponse::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
        assert_eq!(decoded.items.len(), 1);
    }

    #[test]
    fn list_support_tickets_optional_offset_zero_distinguishable_from_absent() {
        let with_zero = ListSupportTicketsRequest {
            status: String::new(),
            offset: Some(0),
            limit: None,
        };
        let absent = ListSupportTicketsRequest {
            status: String::new(),
            offset: None,
            limit: None,
        };
        assert_ne!(with_zero.encode_to_vec(), absent.encode_to_vec(),
            "explicit zero must encode different bytes from absent field");
        let r1 = ListSupportTicketsRequest::decode(&*with_zero.encode_to_vec()).unwrap();
        let r2 = ListSupportTicketsRequest::decode(&*absent.encode_to_vec()).unwrap();
        assert_eq!(r1.offset, Some(0));
        assert_eq!(r2.offset, None);
    }

    #[test]
    fn support_ticket_detail_round_trip() {
        let original = SupportTicketDetail {
            ticket: Some(sample_ticket()),
            messages: vec![sample_message()],
        };
        let bytes = original.encode_to_vec();
        let decoded = SupportTicketDetail::decode(&*bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn get_support_ticket_request_round_trip() {
        let req = GetSupportTicketRequest { id: 42 };
        let bytes = req.encode_to_vec();
        assert_eq!(req, GetSupportTicketRequest::decode(&*bytes).unwrap());
    }

    #[test]
    fn get_attachment_url_round_trip() {
        let req = GetAttachmentUrlRequest { attachment_id: 11 };
        let bytes = req.encode_to_vec();
        assert_eq!(req, GetAttachmentUrlRequest::decode(&*bytes).unwrap());

        let resp = GetAttachmentUrlResponse {
            url: "https://example.com/presigned?sig=abc".into(),
        };
        let resp_bytes = resp.encode_to_vec();
        assert_eq!(resp, GetAttachmentUrlResponse::decode(&*resp_bytes).unwrap());
    }
}
