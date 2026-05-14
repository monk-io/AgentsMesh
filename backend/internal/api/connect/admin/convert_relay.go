package adminconnect

import (
	"time"

	"github.com/anthropics/agentsmesh/backend/internal/service/relay"
	adminv1 "github.com/anthropics/agentsmesh/proto/gen/go/admin/v1"
)

func toProtoRelay(r *relay.RelayInfo) *adminv1.AdminRelay {
	if r == nil {
		return nil
	}
	return &adminv1.AdminRelay{
		Id:            r.ID,
		Url:           r.URL,
		Region:        r.Region,
		Capacity:      int32(r.Capacity),
		Connections:   int32(r.CurrentConnections),
		CpuUsage:      r.CPUUsage,
		MemoryUsage:   r.MemoryUsage,
		LastHeartbeat: r.LastHeartbeat.UTC().Format(time.RFC3339),
		Healthy:       r.Healthy,
		AvgLatencyMs:  int32(r.AvgLatencyMs),
		Latitude:      r.Latitude,
		Longitude:     r.Longitude,
	}
}
