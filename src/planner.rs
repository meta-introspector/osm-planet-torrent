// Rust: Task Planner Module
// Load MiniZinc-optimized task schedule

use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub task: String,
    pub start: usize,
    pub end: usize,
    pub duration: usize,
    pub priority: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Schedule {
    pub makespan: usize,
    pub total_weighted_time: usize,
    pub schedule: Vec<Task>,
}

impl Schedule {
    /// Load optimized schedule from MiniZinc
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Run MiniZinc solver
        let output = Command::new("minizinc")
            .args(&["proofs/task_planner.mzn", "--output-mode", "json"])
            .output()?;
        
        if !output.status.success() {
            return Err(format!("MiniZinc failed: {}", 
                String::from_utf8_lossy(&output.stderr)).into());
        }
        
        let json = String::from_utf8(output.stdout)?;
        let schedule: Schedule = serde_json::from_str(&json)?;
        
        Ok(schedule)
    }
    
    /// Get tasks in execution order
    pub fn execution_order(&self) -> Vec<Task> {
        let mut tasks = self.schedule.clone();
        tasks.sort_by_key(|t| t.start);
        tasks
    }
    
    /// Get next task to execute
    pub fn next_task(&self, completed: &[String]) -> Option<Task> {
        self.execution_order()
            .into_iter()
            .find(|t| !completed.contains(&t.task))
    }
    
    /// Get critical path tasks (priority 10)
    pub fn critical_tasks(&self) -> Vec<Task> {
        self.schedule.iter()
            .filter(|t| t.priority == 10)
            .cloned()
            .collect()
    }
    
    /// Print execution plan
    pub fn print_plan(&self) {
        println!("📋 Optimized Task Schedule");
        println!("Makespan: {} minutes", self.makespan);
        println!("Total weighted time: {}", self.total_weighted_time);
        println!();
        
        for task in self.execution_order() {
            let priority_icon = match task.priority {
                10 => "🔴",
                8..=9 => "🟡",
                _ => "🟢",
            };
            
            println!("{} [{}..{}] {} ({}m, priority {})",
                priority_icon,
                task.start,
                task.end,
                task.task,
                task.duration,
                task.priority
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule_loads() {
        let schedule = Schedule::load();
        assert!(schedule.is_ok());
    }

    #[test]
    fn test_execution_order() {
        let schedule = Schedule::load().unwrap();
        let tasks = schedule.execution_order();
        
        // Verify sorted by start time
        for i in 1..tasks.len() {
            assert!(tasks[i].start >= tasks[i-1].start);
        }
    }

    #[test]
    fn test_critical_tasks() {
        let schedule = Schedule::load().unwrap();
        let critical = schedule.critical_tasks();
        
        // All critical tasks have priority 10
        for task in critical {
            assert_eq!(task.priority, 10);
        }
    }
}
