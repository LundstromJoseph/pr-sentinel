export type AppConfig = {
  github_token: string;
  username: string;
  repo_config: RepoConfig[];
};

export type RepoConfig = {
  repo_name: string;
  needed_approvals: number;
};

export type AppData = {
  pull_requests: PullRequestsData;
};

export type PullRequestsData = {
  last_updated: number;
  pull_requests: PullRequestItem[];
};

export type PullRequestItem = {
  id: number;
  title: string;
  repository_url: string;
  login: string;
  avatar_url: string;
  url: string;
  created_at: string;
  updated_at: string;
  html_url: string;
  category: PullRequestCategory;
};

export type PullRequestCategory =
  | "MineApproved"
  | "MineChangesRequested"
  | "MinePending"
  | "ReviewRequested"
  | "Rereview"
  | "ReviewMissing";
