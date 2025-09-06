use octocrab::{
    models::{issues::Issue, pulls::Review, Author, SimpleUser},
    Octocrab, Page,
};
use serde::Deserialize;

pub struct GithubClient {
    client: Octocrab,
}

pub struct GithubPRWithReviews {
    pub pr: Issue,
    pub reviews: Vec<Review>,
    pub reviewers: GithubPRReviewResponse,
}

#[derive(Deserialize)]
pub struct GithubPRReviewResponse {
    pub users: Vec<SimpleUser>,
    //pub teams: Vec<RequestedTeam>,
}

impl GithubClient {
    pub fn new(github_token: String) -> GithubClient {
        return GithubClient {
            client: octocrab::instance()
                .user_access_token(github_token)
                .unwrap(),
        };
    }

    pub async fn get_user(&self) -> Result<Author, Box<dyn std::error::Error + Send + Sync>> {
        let user = self.client.current().user().await?;

        Ok(user)
    }

    pub async fn search_pull_requests(
        &self,
        query: String,
    ) -> Result<Vec<GithubPRWithReviews>, Box<dyn std::error::Error + Send + Sync>> {
        let mut github_with_reviews = Vec::new();

        let github_response = self
            .client
            .search()
            .issues_and_pull_requests(&query)
            .sort("updated")
            .order("desc")
            .per_page(30)
            .send()
            .await
            .unwrap_or_else(|e| {
                eprintln!("Error searching pull requests: {}", e);
                Page::default()
            });

        for (index, issue) in github_response.items.iter().enumerate() {
            println!(
                "Fetching details for PR {} out of {}",
                index,
                github_response.items.len()
            );
            let repository_url = issue.repository_url.to_string();

            let owner = repository_url.split("/").nth(4).unwrap();
            let repo = repository_url.split("/").nth(5).unwrap();

            let pr_number = issue.number;

            let reviewers: GithubPRReviewResponse = self
                .client
                .get(
                    &format!("/repos/{owner}/{repo}/pulls/{pr_number}/requested_reviewers"),
                    None::<&()>,
                )
                .await
                .unwrap();

            let reviews = self
                .client
                .pulls(owner.to_string(), repo.to_string())
                .list_reviews(issue.number)
                .send()
                .await
                .unwrap_or_else(|e| {
                    eprintln!("Error listing reviews: {}", e);
                    Page::default()
                });

            github_with_reviews.push(GithubPRWithReviews {
                pr: issue.clone(),
                reviews: reviews.items.clone(),
                reviewers: reviewers,
            });
        }

        println!("Done fetching PRs");

        Ok(github_with_reviews)
    }
}
