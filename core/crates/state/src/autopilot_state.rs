use std::collections::HashMap;

use agentsmesh_types::{AutopilotController, AutopilotIteration};
use serde_json::Value;

const MAX_ITERATIONS: usize = 200;
const MAX_THINKING_HISTORY: usize = 100;

const ACTIVE_PHASES: &[&str] = &["initializing", "running", "paused", "user_takeover", "waiting_approval"];

#[derive(Debug, Default)]
pub struct AutopilotState {
    controllers: Vec<AutopilotController>,
    current_controller: Option<AutopilotController>,
    iterations: HashMap<String, Vec<AutopilotIteration>>,
    thinking: HashMap<String, Option<Value>>,
    thinking_history: HashMap<String, Vec<Value>>,
}

impl AutopilotState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn controllers(&self) -> &[AutopilotController] {
        &self.controllers
    }

    pub fn current_controller(&self) -> Option<&AutopilotController> {
        self.current_controller.as_ref()
    }

    pub fn set_controllers(&mut self, controllers: Vec<AutopilotController>) {
        self.controllers = controllers;
    }

    pub fn set_current_controller(&mut self, controller: Option<AutopilotController>) {
        self.current_controller = controller;
    }

    pub fn add_controller(&mut self, controller: AutopilotController) {
        self.controllers.push(controller);
    }

    pub fn update_controller(&mut self, key: &str, controller: AutopilotController) {
        for c in &mut self.controllers {
            if c.autopilot_controller_key == key {
                *c = controller.clone();
            }
        }
        if let Some(ref mut cur) = self.current_controller {
            if cur.autopilot_controller_key == key {
                *cur = controller;
            }
        }
    }

    pub fn remove_controller(&mut self, key: &str) {
        self.controllers.retain(|c| c.autopilot_controller_key != key);
        if self.current_controller
            .as_ref()
            .is_some_and(|c| c.autopilot_controller_key == key)
        {
            self.current_controller = None;
        }
    }

    pub fn get_iterations(&self, key: &str) -> Option<&[AutopilotIteration]> {
        self.iterations.get(key).map(|v| v.as_slice())
    }

    pub fn set_iterations(&mut self, key: String, iters: Vec<AutopilotIteration>) {
        self.iterations.insert(key, iters);
    }

    pub fn add_iteration(&mut self, key: String, iteration: AutopilotIteration) {
        let iters = self.iterations.entry(key).or_default();
        iters.push(iteration);
        if iters.len() > MAX_ITERATIONS {
            let drain = iters.len() - MAX_ITERATIONS;
            iters.drain(..drain);
        }
    }

    pub fn get_thinking(&self, key: &str) -> Option<&Value> {
        self.thinking.get(key).and_then(|v| v.as_ref())
    }

    pub fn get_thinking_history(&self, key: &str) -> Option<&[Value]> {
        self.thinking_history.get(key).map(|v| v.as_slice())
    }

    pub fn update_thinking(&mut self, key: String, thinking: Value) {
        self.thinking.insert(key.clone(), Some(thinking.clone()));
        let history = self.thinking_history.entry(key).or_default();
        history.push(thinking);
        if history.len() > MAX_THINKING_HISTORY {
            let drain = history.len() - MAX_THINKING_HISTORY;
            history.drain(..drain);
        }
    }

    pub fn get_controller_by_pod_key(&self, pod_key: &str) -> Option<&AutopilotController> {
        self.controllers.iter().find(|c| {
            c.pod_key == pod_key
                && c.phase.as_deref().is_some_and(|p| ACTIVE_PHASES.contains(&p))
        })
    }
}
