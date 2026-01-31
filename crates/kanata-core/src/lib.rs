/// Agent orchestration core for Kanata.
pub mod agent;
pub mod context;
pub mod dispatcher;
pub mod memory;
pub mod mock;
pub mod prompt;
pub mod undo;

pub use agent::Agent;
pub use context::ContextAssembler;
pub use dispatcher::ToolDispatcher;
pub use memory::ProjectMemory;
pub use mock::MockAgentSession;
pub use undo::UndoManager;
