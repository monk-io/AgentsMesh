package blockstore

import (
	"database/sql/driver"
	"encoding/json"
	"errors"
)

// JSONMap is a generic JSONB field used by Block Store entities for
// free-form payload (block.data / block.meta / ref.meta / op.payload ...).
// Block-type-specific shape is validated at the service layer against a
// registered JSON schema, not at the GORM layer.
type JSONMap map[string]any

func (m *JSONMap) Scan(value interface{}) error {
	if value == nil {
		*m = nil
		return nil
	}
	var raw []byte
	switch v := value.(type) {
	case []byte:
		raw = v
	case string:
		raw = []byte(v)
	default:
		return errors.New("unsupported type for blockstore.JSONMap Scan")
	}
	return json.Unmarshal(raw, m)
}

func (m JSONMap) Value() (driver.Value, error) {
	if m == nil {
		return []byte("{}"), nil
	}
	return json.Marshal(m)
}

// Clone returns a deep copy via JSON round-trip; safe for storing in op diffs.
func (m JSONMap) Clone() JSONMap {
	if m == nil {
		return nil
	}
	raw, err := json.Marshal(m)
	if err != nil {
		return nil
	}
	var out JSONMap
	_ = json.Unmarshal(raw, &out)
	return out
}
