package eventbus

import (
	"encoding/json"

	"google.golang.org/protobuf/encoding/protojson"
	"google.golang.org/protobuf/proto"
)

var eventDataMarshalOptions = protojson.MarshalOptions{
	UseProtoNames:   true,
	EmitUnpopulated: false,
}

// MarshalEventData encodes a typed proto event-data message as the wire
// JSON that lands in Event.Data. UseProtoNames keeps the snake_case wire
// stable; EmitUnpopulated=false matches `omitempty` semantics.
func MarshalEventData(msg proto.Message) (json.RawMessage, error) {
	b, err := eventDataMarshalOptions.Marshal(msg)
	if err != nil {
		return nil, err
	}
	return json.RawMessage(b), nil
}
