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

    pub async fn get_user(&self) -> Result<Author, String> {
        let user = self.client.current().user().await;

        match user {
            Ok(user) => Ok(user),
            Err(e) => {
                crate::log::error(&format!("Error getting user: {}", e));
                return Err(e.to_string());
            }
        }
    }

    pub async fn search_pull_requests(
        &self,
        query: String,
    ) -> Result<Vec<GithubPRWithReviews>, String> {
        crate::log::info("Fetching PRs");

        let per_page = if cfg!(dev) { 50 } else { 50 };

        let github_response = self
            .client
            .search()
            .issues_and_pull_requests(&query)
            .sort("updated")
            .order("desc")
            .per_page(per_page)
            .send()
            .await;

        let ok_response = match github_response {
            Ok(response) => response,
            Err(e) => {
                crate::log::error(&format!("Error searching pull requests: {}", e));
                return Err(e.to_string());
            }
        };

        use futures::future::join_all;

        let fetch_futures = ok_response.items.iter().map(|issue| {
            let repository_url = issue.repository_url.to_string();
            let (owner, repo) = get_owner_and_repo(repository_url);
            let pr_number = issue.number;
            let client = &self.client;
            let issue = issue.clone();

            async move {
                let reviewers_url =
                    format!("/repos/{owner}/{repo}/pulls/{pr_number}/requested_reviewers");
                let reviewers_future = client.get(&reviewers_url, None::<&()>);

                let owner = owner.to_string();
                let repo = repo.to_string();
                let pulls = client.pulls(owner, repo);
                let reviews_future = pulls.list_reviews(pr_number).send();

                let (reviewers, reviews) = tokio::join!(reviewers_future, reviews_future);

                let reviewers = reviewers.unwrap_or_else(|e| {
                    crate::log::error(&format!("Error listing reviewers, continuing...: {}", e));
                    GithubPRReviewResponse { users: Vec::new() }
                });
                let reviews = reviews.unwrap_or_else(|e| {
                    crate::log::error(&format!("Error listing reviews, continuing...: {}", e));
                    Page::default()
                });

                GithubPRWithReviews {
                    pr: issue,
                    reviews: reviews.items,
                    reviewers,
                }
            }
        });

        let github_with_reviews = join_all(fetch_futures).await;

        crate::log::info("Done fetching PRs");

        Ok(github_with_reviews)
    }
}

pub fn get_owner_and_repo(repository_url: String) -> (String, String) {
    let owner = repository_url.split("/").nth(4).unwrap();
    let repo = repository_url.split("/").nth(5).unwrap();
    (owner.to_string(), repo.to_string())
}
