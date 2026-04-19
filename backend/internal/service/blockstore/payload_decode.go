package blockstoreservice

import (
	"encoding/json"
	"fmt"
)

// payloadAs decodes a raw op payload into a concrete struct via a JSON round-trip.
// Using JSON rather than reflection keeps field-name mapping predictable and
// matches what REST clients already send.
func payloadAs[T any](raw map[string]any) (T, error) {
	var out T
	buf, err := json.Marshal(raw)
	if err != nil {
		return out, err
	}
	if err := json.Unmarshal(buf, &out); err != nil {
		return out, fmt.Errorf("decode payload: %w", err)
	}
	return out, nil
}
