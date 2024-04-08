pub mod csv;

pub trait DatabaseManager {
    fn add(description: String) -> Result<(), String>;
    fn list() -> Result<(), String>;
    fn update_description(id: u64, description: String) -> Result<(), String>;
    fn update_status(id: u64, status: String) -> Result<(), String>;
}
