mod gl_history;
mod model;

use dotenv::dotenv;

use gl_history::fetch_gl_history;
use crate::model::GitLabContribution;
use std::fs;

#[macro_use]
extern crate dotenv_codegen;

fn create_commit(commit_date: &str, commit_message: &str) -> String {
    format!(
"GIT_AUTHOR_DATE=\"{date}\" GIT_COMMITTER_DATE=\"{date}\" git commit --allow-empty -m \"{commit_message}\" > /dev/null\n
",
        date=commit_date,
        commit_message=commit_message
    )
}
fn main() {
    dotenv().ok();
    let contributions = fetch_gl_history("denis114", "2022-01-01", "2022-04-03");
    //println!("{}", contributions.len());

    let mut commit_string_builder = String::new();
    for c in contributions.iter() {
        commit_string_builder.push_str(&create_commit(&c.created_at, &c.action_name));
    }


    let template_bash = format!(
"#!/usr/bin/env bash\n
REPO={repo_name}\n
git clone git@github.com:{user}/$REPO.git\n
cd $REPO\n
git checkout main\n
{fake_commits}
git branch -M main\n
git remote add origin git@github.com:Hyde46/gitlab_history.git\n
git pull origin main\n
git push -u origin main\n"
    , repo_name=dotenv!("GH_REPO"), fake_commits=commit_string_builder, user="Hyde46");

    fs::write("clone.sh", &template_bash).expect("Unable to write file");
    //println!("{}", template_bash);
}
