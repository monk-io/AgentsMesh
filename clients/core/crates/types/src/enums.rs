use serde::{Deserialize, Serialize};

macro_rules! status_enum {
    ($(#[$meta:meta])* $vis:vis enum $Name:ident { $($Variant:ident => $str:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        $vis enum $Name {
            $($Variant,)+
            #[serde(other)]
            Unknown,
        }

        impl std::fmt::Display for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self { $(Self::$Variant => $str,)+ Self::Unknown => "unknown" })
            }
        }
    };
}

status_enum! {
    pub enum PodStatus {
        Pending => "pending",
        Creating => "creating",
        Initializing => "initializing",
        Running => "running",
        Paused => "paused",
        Stopping => "stopping",
        Disconnected => "disconnected",
        Orphaned => "orphaned",
        Completed => "completed",
        Terminated => "terminated",
        Error => "error",
        Failed => "failed",
    }
}
impl Default for PodStatus { fn default() -> Self { Self::Pending } }

status_enum! {
    pub enum TicketStatus {
        Backlog => "backlog",
        Todo => "todo",
        InProgress => "in_progress",
        InReview => "in_review",
        Done => "done",
    }
}
impl Default for TicketStatus { fn default() -> Self { Self::Backlog } }

status_enum! {
    pub enum TicketPriority {
        None => "none",
        Low => "low",
        Medium => "medium",
        High => "high",
        Urgent => "urgent",
    }
}
impl Default for TicketPriority { fn default() -> Self { Self::None } }

status_enum! {
    pub enum RunnerStatus {
        Online => "online",
        Offline => "offline",
        Maintenance => "maintenance",
    }
}
impl Default for RunnerStatus { fn default() -> Self { Self::Offline } }

status_enum! {
    pub enum LoopRunStatus {
        Pending => "pending",
        Running => "running",
        Completed => "completed",
        Failed => "failed",
        Cancelled => "cancelled",
    }
}
impl Default for LoopRunStatus { fn default() -> Self { Self::Pending } }

status_enum! {
    pub enum AutopilotStatus {
        Idle => "idle",
        Running => "running",
        Paused => "paused",
        WaitingApproval => "waiting_approval",
        Completed => "completed",
        Failed => "failed",
        Terminated => "terminated",
    }
}
impl Default for AutopilotStatus { fn default() -> Self { Self::Idle } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pod_status_serde_roundtrip() {
        assert_eq!(serde_json::to_string(&PodStatus::Running).unwrap(), "\"running\"");
        assert_eq!(serde_json::from_str::<PodStatus>("\"running\"").unwrap(), PodStatus::Running);
        assert_eq!(serde_json::from_str::<PodStatus>("\"creating\"").unwrap(), PodStatus::Creating);
    }

    #[test]
    fn pod_status_unknown_fallback() {
        assert_eq!(serde_json::from_str::<PodStatus>("\"something_new\"").unwrap(), PodStatus::Unknown);
    }

    #[test]
    fn ticket_status_serde() {
        assert_eq!(serde_json::from_str::<TicketStatus>("\"in_progress\"").unwrap(), TicketStatus::InProgress);
        assert_eq!(serde_json::to_string(&TicketStatus::InReview).unwrap(), "\"in_review\"");
    }

    #[test]
    fn ticket_priority_serde() {
        assert_eq!(serde_json::from_str::<TicketPriority>("\"urgent\"").unwrap(), TicketPriority::Urgent);
        assert_eq!(serde_json::to_string(&TicketPriority::High).unwrap(), "\"high\"");
    }

    #[test]
    fn runner_status_serde() {
        assert_eq!(serde_json::from_str::<RunnerStatus>("\"online\"").unwrap(), RunnerStatus::Online);
        assert_eq!(serde_json::from_str::<RunnerStatus>("\"bogus\"").unwrap(), RunnerStatus::Unknown);
    }

    #[test]
    fn loop_run_status_serde() {
        assert_eq!(serde_json::from_str::<LoopRunStatus>("\"completed\"").unwrap(), LoopRunStatus::Completed);
        assert_eq!(serde_json::from_str::<LoopRunStatus>("\"failed\"").unwrap(), LoopRunStatus::Failed);
        assert_eq!(serde_json::to_string(&LoopRunStatus::Running).unwrap(), "\"running\"");
    }

    #[test]
    fn autopilot_status_serde() {
        assert_eq!(serde_json::from_str::<AutopilotStatus>("\"running\"").unwrap(), AutopilotStatus::Running);
        assert_eq!(serde_json::from_str::<AutopilotStatus>("\"waiting_approval\"").unwrap(), AutopilotStatus::WaitingApproval);
        assert_eq!(serde_json::to_string(&AutopilotStatus::Terminated).unwrap(), "\"terminated\"");
    }

    #[test]
    fn display_matches_serde() {
        assert_eq!(PodStatus::Running.to_string(), "running");
        assert_eq!(TicketStatus::InProgress.to_string(), "in_progress");
        assert_eq!(TicketPriority::Urgent.to_string(), "urgent");
        assert_eq!(RunnerStatus::Online.to_string(), "online");
        assert_eq!(LoopRunStatus::Completed.to_string(), "completed");
        assert_eq!(AutopilotStatus::WaitingApproval.to_string(), "waiting_approval");
    }

    #[test]
    fn default_values() {
        assert_eq!(PodStatus::default(), PodStatus::Pending);
        assert_eq!(TicketStatus::default(), TicketStatus::Backlog);
        assert_eq!(TicketPriority::default(), TicketPriority::None);
        assert_eq!(RunnerStatus::default(), RunnerStatus::Offline);
        assert_eq!(LoopRunStatus::default(), LoopRunStatus::Pending);
        assert_eq!(AutopilotStatus::default(), AutopilotStatus::Idle);
    }
}
