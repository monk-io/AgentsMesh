package eventbus

import (
	"encoding/json"
	"fmt"
	"time"

	"google.golang.org/protobuf/proto"
)

func NewEntityEvent(eventType EventType, orgID int64, entityType, entityID string, data proto.Message) (*Event, error) {
	var jsonData json.RawMessage
	if data != nil {
		b, err := MarshalEventData(data)
		if err != nil {
			return nil, fmt.Errorf("failed to marshal event data: %w", err)
		}
		jsonData = b
	}
	return &Event{
		Type:           eventType,
		Category:       CategoryEntity,
		OrganizationID: orgID,
		EntityType:     entityType,
		EntityID:       entityID,
		Data:           jsonData,
		Timestamp:      time.Now().UnixMilli(),
	}, nil
}
