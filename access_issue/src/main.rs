use reqwest::blocking::{Client, RequestBuilder};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CreateIssueRequest {
    title: String,
    body: String,
}

fn main() {
    let owner = "Amitpatel7090";
    let repo = "Repo-with-Graphql";
    let token = "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";

    let issue_request = CreateIssueRequest {
        title: "issue_from-code ".to_string(),
        body: "This issue is create dy local code.".to_string(),
    };

    if let Err(err) = create_github_issue(owner, repo, token, issue_request) {
        eprintln!("Error creating GitHub issue: {:?}", err);
    }
}

fn create_github_issue(
    owner: &str,
    repo: &str,
    token: &str,
    issue_request: CreateIssueRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", token).parse()?);

    let client = Client::builder()
        .user_agent("My Rust GitHub API Client")
        .default_headers(headers)
        .build()?;

    let body = serde_json::to_string(&issue_request)?;

    let request_builder: RequestBuilder = client.post(&url).body(body);

    let response = request_builder.send()?;

    if response.status().is_success() {
        println!("GitHub issue created successfully!");
        Ok(())
    } else {
        let err = std::io::Error::new(
            std::io::ErrorKind::Other,
            "GitHub API error",
        );
        return Err(Box::new(err));
    }
}



//   tocken --->  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
//----------------------------------
// let owner = "Amitpatel7090";
// let repo = "Repo-with-Graphql ";
// let token = "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";
