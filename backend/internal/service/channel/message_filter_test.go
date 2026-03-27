package channel

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/anthropics/agentsmesh/backend/internal/domain/channel"
	"gorm.io/gorm"
)

// setupMessagesWithSpacedTimes creates a channel with 5 messages whose created_at
// timestamps are explicitly spaced 1 second apart via GORM Update. This avoids SQLite
// timestamp precision issues that make sub-second comparisons unreliable.
// Returns messages in chronological order: [oldest ... newest].
func setupMessagesWithSpacedTimes(t *testing.T, db *gorm.DB, svc *Service, ctx context.Context) (*channel.Channel, []*channel.Message) {
	t.Helper()

	ch, err := svc.CreateChannel(ctx, &CreateChannelRequest{OrganizationID: 1, Name: "filter-" + t.Name()})
	if err != nil {
		t.Fatalf("CreateChannel failed: %v", err)
	}

	base := time.Date(2025, 1, 1, 12, 0, 0, 0, time.UTC)
	for i := 0; i < 5; i++ {
		msg, err := svc.SendMessage(ctx, ch.ID, nil, nil, channel.MessageTypeText,
			fmt.Sprintf("Msg%c", 'A'+i), channel.MessageMetadata{}, nil)
		if err != nil {
			t.Fatalf("SendMessage failed: %v", err)
		}
		ts := base.Add(time.Duration(i) * time.Second)
		// Use GORM Update (not raw SQL) to ensure consistent time serialization format
		db.Model(&channel.Message{}).Where("id = ?", msg.ID).Update("created_at", ts)
	}

	allMsgs, _, err := svc.GetMessages(ctx, ch.ID, nil, nil, 10)
	if err != nil || len(allMsgs) != 5 {
		t.Fatalf("Setup failed: err=%v, count=%d", err, len(allMsgs))
	}
	return ch, allMsgs
}

func TestGetMessages_AfterFilter(t *testing.T) {
	db := setupTestDB(t)
	svc := newTestService(db)
	ctx := context.Background()
	ch, allMsgs := setupMessagesWithSpacedTimes(t, db, svc, ctx)

	t.Run("after returns only newer messages", func(t *testing.T) {
		// allMsgs chronological: [A, B, C, D, E]
		// after C.CreatedAt should return [D, E]
		after := allMsgs[2].CreatedAt
		msgs, hasMore, err := svc.GetMessages(ctx, ch.ID, nil, &after, 10)
		if err != nil {
			t.Fatalf("GetMessages failed: %v", err)
		}
		if len(msgs) != 2 {
			t.Errorf("Expected 2 messages after index 2, got %d", len(msgs))
		}
		if hasMore {
			t.Error("Expected hasMore=false with limit=10")
		}
	})

	t.Run("after returns chronological order", func(t *testing.T) {
		after := allMsgs[0].CreatedAt
		msgs, _, _ := svc.GetMessages(ctx, ch.ID, nil, &after, 10)
		for i := 1; i < len(msgs); i++ {
			if msgs[i].CreatedAt.Before(msgs[i-1].CreatedAt) {
				t.Error("After-only messages should be in chronological order")
			}
		}
	})

	t.Run("after with hasMore", func(t *testing.T) {
		// after A.CreatedAt → 4 messages (B,C,D,E), limit=2 → hasMore=true
		after := allMsgs[0].CreatedAt
		msgs, hasMore, err := svc.GetMessages(ctx, ch.ID, nil, &after, 2)
		if err != nil {
			t.Fatalf("GetMessages failed: %v", err)
		}
		if len(msgs) != 2 {
			t.Errorf("Expected 2 messages, got %d", len(msgs))
		}
		if !hasMore {
			t.Error("Expected hasMore=true for 4 messages with limit=2")
		}
		// Should return the 2 oldest after A: [B, C]
		if len(msgs) >= 2 {
			if msgs[0].Content != "MsgB" {
				t.Errorf("Expected first message 'MsgB', got '%s'", msgs[0].Content)
			}
			if msgs[1].Content != "MsgC" {
				t.Errorf("Expected second message 'MsgC', got '%s'", msgs[1].Content)
			}
		}
	})

	t.Run("after last message returns empty", func(t *testing.T) {
		after := allMsgs[4].CreatedAt
		msgs, hasMore, err := svc.GetMessages(ctx, ch.ID, nil, &after, 10)
		if err != nil {
			t.Fatalf("GetMessages failed: %v", err)
		}
		if len(msgs) != 0 {
			t.Errorf("Expected 0 messages after last, got %d", len(msgs))
		}
		if hasMore {
			t.Error("Expected hasMore=false for empty result")
		}
	})

	t.Run("before and after combined (time window)", func(t *testing.T) {
		// allMsgs: [A, B, C, D, E]
		// after A, before E → [B, C, D]
		after := allMsgs[0].CreatedAt
		before := allMsgs[4].CreatedAt
		msgs, hasMore, err := svc.GetMessages(ctx, ch.ID, &before, &after, 10)
		if err != nil {
			t.Fatalf("GetMessages failed: %v", err)
		}
		if len(msgs) != 3 {
			t.Errorf("Expected 3 messages in window, got %d", len(msgs))
		}
		if hasMore {
			t.Error("Expected hasMore=false with limit=10")
		}
		// Combined queries use DESC+reverse → chronological order
		for i := 1; i < len(msgs); i++ {
			if msgs[i].CreatedAt.Before(msgs[i-1].CreatedAt) {
				t.Error("Combined filter messages should be in chronological order")
			}
		}
	})

	t.Run("before and after combined with hasMore", func(t *testing.T) {
		after := allMsgs[0].CreatedAt
		before := allMsgs[4].CreatedAt
		msgs, hasMore, err := svc.GetMessages(ctx, ch.ID, &before, &after, 1)
		if err != nil {
			t.Fatalf("GetMessages failed: %v", err)
		}
		if len(msgs) != 1 {
			t.Errorf("Expected 1 message, got %d", len(msgs))
		}
		if !hasMore {
			t.Error("Expected hasMore=true for 3 messages with limit=1")
		}
	})
}

func TestGetMessages_BeforeFilter(t *testing.T) {
	db := setupTestDB(t)
	svc := newTestService(db)
	ctx := context.Background()
	ch, allMsgs := setupMessagesWithSpacedTimes(t, db, svc, ctx)

	t.Run("before returns only older messages", func(t *testing.T) {
		// allMsgs: [A, B, C, D, E], before C → [A, B]
		before := allMsgs[2].CreatedAt
		msgs, hasMore, err := svc.GetMessages(ctx, ch.ID, &before, nil, 10)
		if err != nil {
			t.Fatalf("GetMessages failed: %v", err)
		}
		if len(msgs) != 2 {
			t.Errorf("Expected 2 messages before index 2, got %d", len(msgs))
		}
		if hasMore {
			t.Error("Expected hasMore=false with limit=10")
		}
	})

	t.Run("before with hasMore", func(t *testing.T) {
		// before E → 4 messages [A,B,C,D], limit=2 → hasMore=true
		before := allMsgs[4].CreatedAt
		msgs, hasMore, err := svc.GetMessages(ctx, ch.ID, &before, nil, 2)
		if err != nil {
			t.Fatalf("GetMessages failed: %v", err)
		}
		if len(msgs) != 2 {
			t.Errorf("Expected 2 messages, got %d", len(msgs))
		}
		if !hasMore {
			t.Error("Expected hasMore=true for 4 messages with limit=2")
		}
	})
}

func TestGetMessagesMentioning_HasMore(t *testing.T) {
	db := setupTestDB(t)
	svc := newTestService(db)
	ctx := context.Background()

	ch, _ := svc.CreateChannel(ctx, &CreateChannelRequest{OrganizationID: 1, Name: "mention-hasmore"})

	base := time.Date(2025, 1, 1, 12, 0, 0, 0, time.UTC)
	for i := 0; i < 3; i++ {
		msg, _ := svc.SendMessage(ctx, ch.ID, nil, nil, channel.MessageTypeText,
			fmt.Sprintf("msg%d", i), channel.MessageMetadata{},
			[]MentionInput{{Type: "pod", ID: "target-pod"}})
		ts := base.Add(time.Duration(i) * time.Second)
		db.Model(&channel.Message{}).Where("id = ?", msg.ID).Update("created_at", ts)
	}

	t.Run("hasMore=false when all fit", func(t *testing.T) {
		msgs, hasMore, err := svc.GetMessagesMentioning(ctx, ch.ID, "target-pod", 10)
		if err != nil {
			t.Fatalf("GetMessagesMentioning failed: %v", err)
		}
		if len(msgs) != 3 {
			t.Errorf("Expected 3 messages, got %d", len(msgs))
		}
		if hasMore {
			t.Error("Expected hasMore=false")
		}
	})

	t.Run("hasMore=true when limit exceeded", func(t *testing.T) {
		msgs, hasMore, err := svc.GetMessagesMentioning(ctx, ch.ID, "target-pod", 2)
		if err != nil {
			t.Fatalf("GetMessagesMentioning failed: %v", err)
		}
		if len(msgs) != 2 {
			t.Errorf("Expected 2 messages, got %d", len(msgs))
		}
		if !hasMore {
			t.Error("Expected hasMore=true for 3 mentions with limit=2")
		}
	})

	t.Run("chronological order", func(t *testing.T) {
		msgs, _, _ := svc.GetMessagesMentioning(ctx, ch.ID, "target-pod", 10)
		for i := 1; i < len(msgs); i++ {
			if msgs[i].CreatedAt.Before(msgs[i-1].CreatedAt) {
				t.Error("Mentioned messages should be in chronological order")
			}
		}
	})
}

func TestGetMessagesMentioning_FalsePositive(t *testing.T) {
	db := setupTestDB(t)
	svc := newTestService(db)
	ctx := context.Background()

	ch, _ := svc.CreateChannel(ctx, &CreateChannelRequest{OrganizationID: 1, Name: "mention-fp"})

	// Message with podKey in mentioned_pods (should match)
	svc.SendMessage(ctx, ch.ID, nil, nil, channel.MessageTypeText, "hello structured",
		channel.MessageMetadata{}, []MentionInput{{Type: "pod", ID: "target-pod"}})

	// Message with podKey in content only via @mention (legacy, should match)
	svc.SendMessage(ctx, ch.ID, nil, nil, channel.MessageTypeText, "@target-pod legacy",
		channel.MessageMetadata{}, nil)

	// Message with podKey in unrelated metadata field (should NOT match)
	svc.SendMessage(ctx, ch.ID, nil, nil, channel.MessageTypeText, "unrelated",
		channel.MessageMetadata{"reply_to": "target-pod"}, nil)

	msgs, _, err := svc.GetMessagesMentioning(ctx, ch.ID, "target-pod", 10)
	if err != nil {
		t.Fatalf("GetMessagesMentioning failed: %v", err)
	}
	// Should find 2 (structured mention + legacy @mention), not the reply_to one
	if len(msgs) != 2 {
		t.Errorf("Expected 2 matching messages (structured + legacy), got %d", len(msgs))
		for i, m := range msgs {
			t.Logf("  msg[%d]: content=%q, metadata=%v", i, m.Content, m.Metadata)
		}
	}
}
