use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GitLabContribution {
    pub id: u32,
    pub action_name: String,
    pub created_at: String,
}