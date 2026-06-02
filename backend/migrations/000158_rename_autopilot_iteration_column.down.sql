ALTER INDEX IF EXISTS idx_autopilot_iterations_autopilot_controller_id
    RENAME TO idx_autopilot_iterations_autopilot_id;

ALTER TABLE autopilot_iterations RENAME COLUMN autopilot_controller_id TO autopilot_id;
