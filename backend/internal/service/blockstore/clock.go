package blockstoreservice

import "time"

// timeNowUTC is a package-internal indirection so tests can freeze the clock.
var timeNowUTC = func() time.Time {
	return time.Now().UTC()
}
