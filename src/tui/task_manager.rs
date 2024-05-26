use std::collections::HashMap;

use tokio_util::sync::CancellationToken;

/// TaskManager manages the control and cancellation of background tasks.
///
/// # Examples
///
/// ```
/// let task_manager = TaskManager::new();
/// let task = || { do_thing() };
/// task_manager.run_task(task, TaskKey::EventHandler);
/// task_manager.cancel_all().await;
/// ```
pub struct TaskManager {
    tasks: HashMap<TaskKey, CancellationToken>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
        }
    }
    pub fn run_task(&mut self, key: TaskKey, task: impl FnOnce(CancellationToken) + Send + 'static) {
        let cancellation_token = CancellationToken::new();

        self.tasks.insert(key, cancellation_token.clone());

        tokio::spawn(async { task(cancellation_token) });
    }

    pub async fn cancel_all(&mut self) {
        for task in &self.tasks {
            task.1.cancel()
        }
    }

    pub async fn cancel_task(&mut self, key: TaskKey) {
        self.tasks.get(&key).map(|x| x.cancel());
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum TaskKey {
    EventHandler,
    LoginCallback,
}
