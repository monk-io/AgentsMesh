use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampFields {
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmptyResponse {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn pagination_roundtrip() {
        let p = Pagination {
            limit: Some(20),
            offset: Some(0),
            total: Some(100),
        };
        let json = serde_json::to_string(&p).unwrap();
        let decoded: Pagination = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.limit, Some(20));
        assert_eq!(decoded.total, Some(100));
    }

    #[test]
    fn pagination_all_optional() {
        let json = "{}";
        let p: Pagination = serde_json::from_str(json).unwrap();
        assert!(p.limit.is_none());
        assert!(p.offset.is_none());
        assert!(p.total.is_none());
    }

    #[test]
    fn timestamp_fields_roundtrip() {
        let ts = TimestampFields {
            created_at: Some("2026-01-01T00:00:00Z".into()),
            updated_at: None,
        };
        let json = serde_json::to_string(&ts).unwrap();
        let decoded: TimestampFields = serde_json::from_str(&json).unwrap();
        assert!(decoded.created_at.is_some());
        assert!(decoded.updated_at.is_none());
    }

    #[test]
    fn timestamp_fields_all_optional() {
        let json = "{}";
        let ts: TimestampFields = serde_json::from_str(json).unwrap();
        assert!(ts.created_at.is_none());
        assert!(ts.updated_at.is_none());
    }
}
