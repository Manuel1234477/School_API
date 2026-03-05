pub mod auth;
pub mod auth_controllers;
pub mod admin;
pub mod student;
pub mod mentor;
pub mod school;

#[cfg(test)]
mod tests;

pub use auth::*;
pub use admin::*;
pub use student::*;
pub use mentor::*;
pub use school::*;
