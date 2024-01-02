// use reqwest::blocking::{Client, RequestBuilder};
// use serde_derive::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// struct UpdateIssueRequest {
//     title: Option<String>,
//     body: Option<String>,
// }

// fn main() {
//     let owner = "Amitpatel7090";
//     let repo = "Repo-with-Graphql";
//     let token = "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 ";
//     let issue_number_to_update = 2; 

//     let update_request = UpdateIssueRequest {
//         title: Some(" updates_issue2".to_string()),
//         body: Some("hiii there this issues description is updated by  code hihihihihihihi".to_string()),
//     };

//     if let Err(err) = update_github_issue(owner, repo, token, issue_number_to_update, update_request) {
//         eprintln!("Error updating GitHub issue: {:?}", err);
//     }
// }

// fn update_github_issue(
//     owner: &str,
//     repo: &str,
//     token: &str,
//     issue_number: u32,
//     update_request: UpdateIssueRequest,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let url = format!("https://api.github.com/repos/{}/{}/issues/{}", owner, repo, issue_number);

//     let mut headers = reqwest::header::HeaderMap::new();
//     headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", token).parse()?);

//     let client = Client::builder()
//         .user_agent("My Rust GitHub API Client")
//         .default_headers(headers)
//         .build()?;

//     let body = serde_json::to_string(&update_request)?;

//     let request_builder: RequestBuilder = client.patch(&url).body(body);

//     let response = request_builder.send()?;

//     if response.status().is_success() {
//         println!("GitHub issue updated successfully!");
//         Ok(())
//     } else {
//         let err = std::io::Error::new(
//             std::io::ErrorKind::Other,
//             "GitHub API error",
//         );
//         return Err(Box::new(err));
//     }
// }





use reqwest::blocking::{Client, RequestBuilder};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct UpdateIssueRequest {
    title: Option<String>,
    body: Option<String>,
}

fn main() {
    let owner = "Amitpatel7090";
    let repo = "Repo-with-Graphql";
    let token = "YOUR_GITHUB_TOKEN";
    let issue_number_to_update = 2; // Replace with the actual issue number you want to update

    let update_request = UpdateIssueRequest {
        title: Some(" updated_by_code".to_string()),
        body: Some("hiii there this issues description is updated by  code hihihihihihihi".to_string()),
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
    // Construct the GraphQL mutation
    let mutation = format!(
        r#"
        mutation {{
            updateIssue(input: {{
                repositoryOwner: "{owner}",
                repositoryName: "{repo}",
                issueNumber: {issue_number},
                title: "{title}",
                body: "{body}"
            }}) {{
                issue {{
                    id
                }}
            }}
        }}
    "#,
        owner = owner,
        repo = repo,
        issue_number = issue_number,
        title = update_request.title.unwrap_or_default(),
        body = update_request.body.unwrap_or_default(),
    );

    // Construct the GraphQL API endpoint URL
    let url = "https://api.github.com/graphql";

    // Create a new HTTP client with authentication
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", token).parse()?);

    let client = Client::builder()
        .user_agent("My Rust GitHub API Client")
        .default_headers(headers)
        .build()?;

    // Prepare the request body as JSON
    let request_builder: RequestBuilder = client.post(url).json(&json!({ "query": mutation }));

    // Send the request
    let response = request_builder.send()?;

    // Check if the request was successful (status code 2xx)
    if response.status().is_success() {
        println!("GitHub issue updated successfully!");
        Ok(())
    } else {
        let err = std::io::Error::new(
            std::io::ErrorKind::Other,
            "GitHub API error",
        );
        Err(Box::new(err))
    }
}



// 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  --> second  tocken


