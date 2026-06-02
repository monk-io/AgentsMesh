-- Fix schema drift: autopilot_iterations FK column was left as `autopilot_id`
-- by migration 000041, but the Go model (AutopilotIteration.AutopilotControllerID)
-- and infra queries expect `autopilot_controller_id`. Without this rename,
-- CreateIteration/ListIterations fail in production with "column does not exist".
ALTER TABLE autopilot_iterations RENAME COLUMN autopilot_id TO autopilot_controller_id;

ALTER INDEX IF EXISTS idx_autopilot_iterations_autopilot_id
    RENAME TO idx_autopilot_iterations_autopilot_controller_id;
