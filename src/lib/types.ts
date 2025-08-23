export type GithubFilter = {
  id: string;
  name: string;
  query: string;
  notify: boolean;
};

export type AppConfig = {
  github_token: string;
  filters: GithubFilter[];
};

export type AppData = {
  pull_requests: {
    [key: string]: PullRequestsData;
  };
};

export type PullRequestsData = {
  last_updated: number;
  pull_requests: PullRequestItem[];
};

export type PullRequestItem = {
  id: number;
  title: string;
  repository_url: string;
  user: {
    login: string;
    avatar_url: string;
  };
  url: string;
  created_at: string;
  updated_at: string;
  pull_request: {
    html_url: string;
  };
};
