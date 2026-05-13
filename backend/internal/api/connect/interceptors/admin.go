package interceptors

import (
	"context"
	"errors"

	"connectrpc.com/connect"

	"github.com/anthropics/agentsmesh/backend/internal/domain/user"
	"github.com/anthropics/agentsmesh/backend/internal/infra/database"
	"github.com/anthropics/agentsmesh/backend/internal/middleware"
)

// AdminContext exposes the system-admin identity to RPC handlers without
// re-fetching the user record from DB. Lives in ctx for the duration of
// the request — set by NewAdminInterceptor after it validates the JWT
// claims AND the `is_system_admin = true` flag.
//
// Handlers that need the admin user id for audit logging pull it from
// here (see backend/internal/api/connect/admin/audit.go) — the standard
// auth interceptor populates TenantContext.UserID with the same id, but
// keeping a dedicated AdminContext makes the privilege explicit at every
// call-site.
type AdminContext struct {
	UserID    int64
	IPAddress string
	UserAgent string
}

type adminCtxKey struct{}

func withAdmin(ctx context.Context, a *AdminContext) context.Context {
	return context.WithValue(ctx, adminCtxKey{}, a)
}

// AdminFromContext returns the admin context attached by
// NewAdminInterceptor, or nil if the request is not on an admin route.
func AdminFromContext(ctx context.Context) *AdminContext {
	a, _ := ctx.Value(adminCtxKey{}).(*AdminContext)
	return a
}

// NewAdminInterceptor returns a Connect unary interceptor that mirrors
// REST's `middleware.AdminMiddleware` (backend/internal/middleware/admin.go):
//
//   1. Reads UserID from the TenantContext populated by NewAuthInterceptor
//      (the admin interceptor MUST be wired after auth).
//   2. Fetches the user record from `db`.
//   3. Rejects requests when `is_system_admin = false` (CodePermissionDenied)
//      or `is_active = false` (CodePermissionDenied with a distinct error).
//   4. Stores an AdminContext (UserID + IP + UA) for audit logging.
//
// Returns CodeUnauthenticated when no TenantContext is found — that means
// the auth interceptor wasn't wired correctly (programming error, never
// reachable in production).
func NewAdminInterceptor(db database.DB) connect.UnaryInterceptorFunc {
	return func(next connect.UnaryFunc) connect.UnaryFunc {
		return func(ctx context.Context, req connect.AnyRequest) (connect.AnyResponse, error) {
			if req.Spec().IsClient {
				return next(ctx, req)
			}

			tenant := middleware.GetTenant(ctx)
			if tenant == nil || tenant.UserID == 0 {
				return nil, connect.NewError(
					connect.CodeUnauthenticated,
					errors.New("authentication required"),
				)
			}

			var u user.User
			if err := db.First(&u, tenant.UserID); err != nil {
				return nil, connect.NewError(
					connect.CodeUnauthenticated,
					errors.New("user not found"),
				)
			}
			if !u.IsSystemAdmin {
				return nil, connect.NewError(
					connect.CodePermissionDenied,
					errors.New("system administrator privileges required"),
				)
			}
			if !u.IsActive {
				return nil, connect.NewError(
					connect.CodePermissionDenied,
					errors.New("account is disabled"),
				)
			}

			admin := &AdminContext{
				UserID:    u.ID,
				IPAddress: req.Peer().Addr,
				UserAgent: req.Header().Get("User-Agent"),
			}
			ctx = withAdmin(ctx, admin)
			return next(ctx, req)
		}
	}
}
