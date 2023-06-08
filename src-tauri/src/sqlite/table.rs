
pub mod example;
pub mod config;
pub mod state;
pub mod todo;


pub use crate::sqlite::schema::example::table as TableExample;
pub use crate::sqlite::schema::example::dsl as ColExample;

pub use crate::sqlite::schema::config::table as TableConfig;
pub use crate::sqlite::schema::config::dsl as ColConfig;

pub use crate::sqlite::schema::state::table as TableState;
pub use crate::sqlite::schema::state::dsl as ColState;

pub use crate::sqlite::schema::todo::table as TableTodo;
pub use crate::sqlite::schema::todo::dsl as ColTodo;
