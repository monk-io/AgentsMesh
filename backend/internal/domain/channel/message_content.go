package channel

import (
	"database/sql/driver"
	"encoding/json"
	"errors"
)

// InlineElement types
const (
	InlineText      = "text"
	InlineMention   = "mention"
	InlineLink      = "link"
	InlineLinebreak = "linebreak"
)

// Mention entity types
const (
	EntityPod     = "pod"
	EntityUser    = "user"
	EntityTicket  = "ticket"
	EntityChannel = "channel"
)

type InlineElement struct {
	Type       string `json:"type"`
	Text       string `json:"text,omitempty"`
	Bold       bool   `json:"bold,omitempty"`
	Italic     bool   `json:"italic,omitempty"`
	Strike     bool   `json:"strike,omitempty"`
	Code       bool   `json:"code,omitempty"`
	EntityType string `json:"entity_type,omitempty"`
	EntityKey  string `json:"entity_key,omitempty"`
	Display    string `json:"display,omitempty"`
	URL        string `json:"url,omitempty"`
}

type Block struct {
	Type     string            `json:"type"`
	Elements []InlineElement   `json:"elements,omitempty"`
	Level    int               `json:"level,omitempty"`
	Language string            `json:"language,omitempty"`
	Text     string            `json:"text,omitempty"`
	Ordered  bool              `json:"ordered,omitempty"`
	Items    [][]InlineElement `json:"items,omitempty"`
}

type MessageContent struct {
	Kind          string  `json:"kind"`
	Blocks        []Block `json:"blocks,omitempty"`
	AttachmentKey string  `json:"attachment_key,omitempty"`
}

func (mc *MessageContent) Scan(value interface{}) error {
	if value == nil {
		*mc = MessageContent{}
		return nil
	}
	var bytes []byte
	switch v := value.(type) {
	case []byte:
		bytes = v
	case string:
		bytes = []byte(v)
	default:
		return errors.New("unsupported type for MessageContent.Scan")
	}
	return json.Unmarshal(bytes, mc)
}

func (mc MessageContent) Value() (driver.Value, error) {
	return json.Marshal(mc)
}

type MessageMentions struct {
	Pods    []string `json:"pods,omitempty"`
	Users   []int64  `json:"users,omitempty"`
	Channel bool     `json:"channel,omitempty"`
}

func (mm *MessageMentions) Scan(value interface{}) error {
	if value == nil {
		*mm = MessageMentions{}
		return nil
	}
	var bytes []byte
	switch v := value.(type) {
	case []byte:
		bytes = v
	case string:
		bytes = []byte(v)
	default:
		return errors.New("unsupported type for MessageMentions.Scan")
	}
	return json.Unmarshal(bytes, mm)
}

func (mm MessageMentions) Value() (driver.Value, error) {
	return json.Marshal(mm)
}
