use reqwest::blocking::{Client, RequestBuilder};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UpdateIssueRequest {
    title: Option<String>,
    body: Option<String>,
}

fn main() {
    let owner = "Amitpatel7090";
    let repo = "Repo-with-Graphql";
    let token = "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";
    let issue_number_to_update = 1; // Replace with the actual issue number you want to update

    let update_request = UpdateIssueRequest {
        title: Some(" updated_by_code".to_string()),
        body: Some("hiii there this issues description is updated by  code".to_string()),
    };

    if let Err(err) = update_github_issue(owner, repo, token, issue_number_to_update, update_request) {
        eprintln!("Error updating GitHub issue: {:?}", err);
    }
}

fn update_github_issue(
    owner: &str,
    repo: &str,
    token: &str,
    issue_number: u32,
    update_request: UpdateIssueRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/issues/{}", owner, repo, issue_number);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", token).parse()?);

    let client = Client::builder()
        .user_agent("My Rust GitHub API Client")
        .default_headers(headers)
        .build()?;

    let body = serde_json::to_string(&update_request)?;

    let request_builder: RequestBuilder = client.patch(&url).body(body);

    let response = request_builder.send()?;

    if response.status().is_success() {
        println!("GitHub issue updated successfully!");
        Ok(())
    } else {
        let err = std::io::Error::new(
            std::io::ErrorKind::Other,
            "GitHub API error",
        );
        return Err(Box::new(err));
    }
}
