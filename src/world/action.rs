#[derive(Debug, Clone)]
pub struct Action {
    name: String,
    description: String,
}

impl Default for Action {
    fn default() -> Self {
        Self { 
            name: String::from("Nothing"), 
            description: String::from("I basicaly do nothing"),
        }
    }
}
