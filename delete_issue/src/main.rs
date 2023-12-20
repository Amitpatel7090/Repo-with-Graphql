use reqwest::blocking::{Client, RequestBuilder};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CloseIssueRequest {
    state: String,
}

fn main() {
    let owner = "Amitpatel7090";
    let repo = "Repo-with-Graphql";
    let token = "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";
    let issue_number_to_close = 1; // Replace with the actual issue number you want to close

    if let Err(err) = close_github_issue(owner, repo, token, issue_number_to_close) {
        eprintln!("Error closing GitHub issue: {:?}", err);
    }
}

fn close_github_issue(
    owner: &str,
    repo: &str,
    token: &str,
    issue_number: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/issues/{}", owner, repo, issue_number);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", token).parse()?);

    let client = Client::builder()
        .user_agent("My Rust GitHub API Client")
        .default_headers(headers)
        .build()?;

    let close_request = CloseIssueRequest { state: "closed".to_string() };
    let body = serde_json::to_string(&close_request)?;

    let request_builder: RequestBuilder = client.patch(&url).body(body);

    let response = request_builder.send()?;

    if response.status().is_success() {
        println!("GitHub issue closed successfully!");
        Ok(())
    } else {
        let err = std::io::Error::new(
            std::io::ErrorKind::Other,
            "GitHub API error",
        );
        return Err(Box::new(err));
    }
}
