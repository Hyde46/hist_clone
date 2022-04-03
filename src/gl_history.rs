use crate::model::GitLabContribution;

const CONTRIBUTIONS_PER_PAGE: i32 = 40;

fn fetch_page(username: &str, from: &str, to: &str, page: i32) -> Result<Vec<GitLabContribution>, Box<dyn std::error::Error>> {
    let url = format!("https://gitlab.com/api/v4/users/{username}/events?before={before}&after={after}&sort=desc&page={page}&per_page={per_page}", username=username, before=to, after=from, page=page, per_page=CONTRIBUTIONS_PER_PAGE);
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header("PRIVATE-TOKEN", dotenv!("GL_TOKEN"))
        .header("Content-Type", "application/json")
        .send()
        .unwrap()
        .text();
    let data: Vec<GitLabContribution> = serde_json::from_str(&res.unwrap())?;
    Ok(data)
}

pub fn fetch_gl_history(username: &str, from: &str, to: &str) -> Vec<GitLabContribution> {
    // Year format YYYY-MM-DD
    let mut contributions: Vec<GitLabContribution> = vec![];
    let mut page = 1;
    loop {
        let contribution_page = fetch_page(username, from, to, page);
        match contribution_page {
            Ok(mut data) => {
                if data.len() == 0 {
                    break;
                }
                contributions.append(&mut data);
            },
            Err(_e) => println!("Failed to fetch data")
        }
        page += 1;
    }
    contributions
}