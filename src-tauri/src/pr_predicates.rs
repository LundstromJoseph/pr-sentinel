use std::collections::HashMap;

use octocrab::models::pulls::{Review, ReviewState};

use crate::{
    app_data::PullRequestCategory,
    github_service::{get_owner_and_repo, GithubPRWithReviews},
    AppConfig,
};

// Type alias for the function signature
pub struct CategoryPredicate {
    pub predicate: fn(&GithubPRWithReviews, &AppConfig) -> bool,
    pub category: PullRequestCategory,
}

fn mine_pending(pr_with_reviews: &GithubPRWithReviews, config: &AppConfig) -> bool {
    return is_mine(pr_with_reviews, config);
}

fn mine_approved(pr_with_reviews: &GithubPRWithReviews, config: &AppConfig) -> bool {
    if !is_mine(pr_with_reviews, config) {
        return false;
    }
    let needed_approvals = needed_approvals(pr_with_reviews, config);

    let all_latest_reviews = latest_reviews(pr_with_reviews);

    return all_latest_reviews
        .iter()
        .filter(|r| review_is(r, ReviewState::Approved))
        .count()
        >= needed_approvals;
}

fn mine_changes_requested(pr_with_reviews: &GithubPRWithReviews, config: &AppConfig) -> bool {
    if !is_mine(pr_with_reviews, config) {
        return false;
    }
    let all_latest_reviews = latest_reviews(pr_with_reviews);
    return all_latest_reviews
        .iter()
        .any(|r| review_is(r, ReviewState::ChangesRequested));
}

fn re_review(pr_with_reviews: &GithubPRWithReviews, config: &AppConfig) -> bool {
    if is_mine(pr_with_reviews, config) {
        return false;
    }
    let reviews_by_user = group_by_user(pr_with_reviews.reviews.clone());
    let username = config.username.clone().unwrap_or("".to_string());
    let has_reviews = reviews_by_user.contains_key(&username);
    if !has_reviews {
        return false;
    }
    let user_latest_review = get_latest_review(&reviews_by_user[&username]);
    return is_user_review_requested(pr_with_reviews, &username)
        || user_latest_review.state == Some(ReviewState::Dismissed);
}

fn review_missing(pr_with_reviews: &GithubPRWithReviews, config: &AppConfig) -> bool {
    if is_mine(pr_with_reviews, config) {
        return false;
    }
    let needed_approvals = needed_approvals(pr_with_reviews, config);
    let reviews_by_user = group_by_user(pr_with_reviews.reviews.clone());
    let unique_reviewers = reviews_by_user.keys().len();
    let username = config.username.clone().unwrap_or("".to_string());
    let current_user_has_reviewed = reviews_by_user.contains_key(&username);
    return unique_reviewers < needed_approvals && !current_user_has_reviewed;
}

fn review_requested(pr_with_reviews: &GithubPRWithReviews, config: &AppConfig) -> bool {
    return !is_mine(pr_with_reviews, config);
}

pub static PR_CATEGORIES: &[CategoryPredicate] = &[
    CategoryPredicate {
        predicate: mine_changes_requested,
        category: PullRequestCategory::MineChangesRequested,
    },
    CategoryPredicate {
        predicate: mine_approved,
        category: PullRequestCategory::MineApproved,
    },
    CategoryPredicate {
        predicate: mine_pending,
        category: PullRequestCategory::MinePending,
    },
    CategoryPredicate {
        predicate: re_review,
        category: PullRequestCategory::Rereview,
    },
    CategoryPredicate {
        predicate: review_missing,
        category: PullRequestCategory::ReviewMissing,
    },
    CategoryPredicate {
        predicate: review_requested,
        category: PullRequestCategory::ReviewRequested,
    },
];

fn group_by_user(reviews: Vec<Review>) -> HashMap<String, Vec<Review>> {
    let mut reviews_by_user = HashMap::new();
    for review in reviews {
        if review.user.is_none() {
            continue;
        }
        reviews_by_user
            .entry(review.user.clone().unwrap().login)
            .or_insert(Vec::new())
            .push(review);
    }
    reviews_by_user
}

fn get_latest_review(reviews: &Vec<Review>) -> Review {
    reviews
        .iter()
        .max_by_key(|r| r.submitted_at)
        .unwrap()
        .clone()
}

fn is_user_review_requested(pr_with_reviews: &GithubPRWithReviews, username: &String) -> bool {
    pr_with_reviews
        .reviewers
        .users
        .iter()
        .any(|r| r.login.clone() == username.clone())
}

fn latest_reviews(pr_with_reviews: &GithubPRWithReviews) -> Vec<(Review, bool)> {
    let reviews_by_user = group_by_user(pr_with_reviews.reviews.clone());
    let all_latest_reviews: Vec<(Review, bool)> = reviews_by_user
        .iter()
        .map(|(key, value)| {
            let is_review_requested = is_user_review_requested(pr_with_reviews, &key);

            let approved_and_changes_requested_reviews: Vec<Review> = value
                .iter()
                .filter(|r| {
                    r.state == Some(ReviewState::Approved)
                        || r.state == Some(ReviewState::ChangesRequested)
                })
                .cloned()
                .collect();

            if approved_and_changes_requested_reviews.len() == 0 {
                return (get_latest_review(value), is_review_requested);
            }
            return (
                get_latest_review(&approved_and_changes_requested_reviews),
                is_review_requested,
            );
        })
        .collect();
    return all_latest_reviews;
}

fn is_mine(pr_with_reviews: &GithubPRWithReviews, config: &AppConfig) -> bool {
    return pr_with_reviews.pr.user.login == config.username.clone().unwrap_or("".to_string());
}

fn pr_repo_name(pr_with_reviews: &GithubPRWithReviews) -> String {
    let (owner, repo) = get_owner_and_repo(&pr_with_reviews.pr.repository_url.to_string());

    return format!("{}/{}", owner, repo);
}

fn needed_approvals(pr_with_reviews: &GithubPRWithReviews, config: &AppConfig) -> usize {
    let pr_repo_name = pr_repo_name(pr_with_reviews);
    return config
        .repo_config
        .iter()
        .find(|r| r.repo_name == pr_repo_name)
        .map(|f| f.needed_approvals)
        .unwrap_or(1);
}

fn review_is(review: &(Review, bool), state: ReviewState) -> bool {
    if review.1 {
        return false;
    }
    return review.0.state == Some(state);
}
