// use reqwest::RequestBuilder;
// use rocket::{routes, http::{Status,ContentType},Error as OtherError, State, response::content, };
// use reqwest::Client;
// use serde_json::{Value};
// use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};

use async_graphql::{context::Context, EmptySubscription, Object,InputObject,Error, FieldResult, Schema, SimpleObject};
use rocket_contrib::json::JsonValue;
use crate::json;

use serde::{Deserialize,Serialize}; 
use serde_json::Value;
// use reqwest::Error;

use reqwest::Response;
use  std::env;
use dotenv::dotenv;
#[derive(SimpleObject,Serialize,Debug,Deserialize)]
pub struct Issue {
    title: String,
    body: String,
}

impl Issue {
    pub fn new(title: String, body: String) -> Self {
        Self { title, body }
    }
}
#[derive(InputObject)]
pub struct DeleteIssue {
    issue_id: String,
}

#[derive(InputObject)]
pub struct CreateIssueRequest {
    title: String,
    body: String,
}
#[derive(Deserialize, Debug,Serialize,SimpleObject)]
pub struct Label {
    pub lable_id:String,
    pub name: String,
}
#[derive(SimpleObject,Serialize,Debug,Deserialize)]
pub struct IssueLable {
    data:Issue,
    lable:Vec<Label>,
}

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> String {
        "Hello, World!".to_string()
    }
    async fn get_labels(
        &self,
        owner:String,
         repo: String,
       
    ) -> Result<Vec<Value>,Error> {
     
        let query = r#"
        query GetRepositoryLabels($owner: String!, $repo: String!) {
            repository(owner: $owner, name: $repo) {
              labels(first: 100) {
                nodes {
                    id 
                  name
                  descritpion
                }
              }
            }
          }
          
        "#;
    
   
        let variables = json!({
            "owner": owner,
            "repoName": repo,
        }); 
    
        let request_body = json!({
            "query": query,
            "variables": variables,
        });
    
        let response = get_response(request_body).await;

        if response.status().is_success() {
            let response_json: serde_json::Value = response.json().await?;
            let data: Vec<Value> = serde_json::from_value(response_json["data"]["repository"]["labels"]["nodes"].clone()).unwrap();
        
            Ok(data)
        
        }
       else{ Err("Failed to get labels".into())
}
}
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_issue(
        &self,
        repository_id: String,
        title: String,
        body: String,
    ) -> FieldResult<Issue> {
  
        let mutation = r#"
            mutation CreateIssue($input: CreateIssueInput!) {
              createIssue(input: $input) {
                issue {
                  id
                  title
                  body
                  state
                }
              }
            }
        "#;

        let variables = json!({
            "input": {
                "repositoryId": repository_id,
                "title": title,
                "body": body,
            }
        });

        let request_body = json!({
            "query": mutation,
            "variables": variables,
        });

        let response = get_response(request_body).await;

        if response.status().is_success() {
            let response_json: serde_json::Value = response.json().await?;
            let new_issue: Issue = serde_json::from_value(response_json["data"]["createIssue"]["issue"].clone()).unwrap();
            Ok(new_issue)
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to create issue: {}", error_message).into())
        }
    }
    async fn update_issue(
        &self,
        issue_id: String,
        new_title: String,
        new_body: String,
    ) -> FieldResult<Issue> {
          
        let mutation = r#"
            mutation UpdateIssue($input: UpdateIssueInput!) {
              updateIssue(input: $input) {
                issue {
                  id
                  title
                  body
                  state
                }
              }
            }
        "#;

        let variables = json!({
            "input": {
                "id": issue_id,
                "title": new_title,
                "body": new_body,
            }
        });

        let request_body = json!({
            "query": mutation,
            "variables": variables,
        });

        let response = get_response(request_body).await;


        if response.status().is_success() {
            let response_json: serde_json::Value = response.json().await?;
            let updated_issue: Issue =
                serde_json::from_value(response_json["data"]["updateIssue"]["issue"].clone())
                    .unwrap();
            Ok(updated_issue)
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to update issue: {}", error_message).into())
        }
    }
    async fn delete_issue(&self, issue_id:String) -> FieldResult<String> {
    
        let mutation = r#"
            mutation DeleteIssue($input: DeleteIssueInput!) {
              deleteIssue(input: $input) {
                clientMutationId
              }
            }
        "#;

        let variables = json!({
            "input": {
                "issueId": issue_id,
            }
        });

        let request_body = json!({
            "query": mutation,
            "variables": variables,
        });

        let response = get_response(request_body).await;


        if response.status().is_success() {
            Ok(format!("Issue with ID {} deleted successfully.", issue_id))
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to delete issue: {}", error_message).into())
        }
    }
    async fn close_issue(&self, issue_id:String) -> FieldResult<String> {
        dotenv().ok();
        let token = env::var("TOCKEN").expect("GITHUB_TOKEN not found");
        println!("{}",token);

        let mutation = r#"
            mutation CloseIssue($input: CloseIssueInput!) {
              closeIssue(input: $input) {
                clientMutationId
              }
            }
        "#;

        let variables = json!({
            "input": {
                "issueId": issue_id,
            }
        });

        let request_body = json!({
            "query": mutation,
            "variables": variables,
        });
        let response = get_response(request_body).await;


        if response.status().is_success() {
            Ok(format!("Issue with ID {} close successfully.", issue_id))
        } else {
            let error_message = response.text().await?;
            Err(format!("Failed to close issue: {}", error_message).into())
        }
    }
    
}

pub async fn get_response(request_body:JsonValue)->reqwest::Response{
  
    dotenv().ok();
    let token = env::var("TOCKEN").expect("GITHUB_TOKEN not found");
    let response = reqwest::Client::new()
    .post("https://api.github.com/graphql")
    .header("Authorization", format!("Bearer {}", token))
    .header("User-Agent", "graphql-rust-client")
    .json(&request_body)
    .send().await.unwrap();
response
}
pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;
