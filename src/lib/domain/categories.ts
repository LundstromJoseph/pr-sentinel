import type { PullRequestCategory } from "$lib/types";

type Category = {
  name: string;
  level: number;
  key: string;
  prCategories: PullRequestCategory[];
};

export const categories: Category[] = [
  {
    name: "Author",
    level: 0,
    key: "mine",
    prCategories: [
      "MineApproved",
      "MineChangesRequested",
      "MinePending",
    ] as const,
  },
  {
    name: "Approved",
    level: 1,
    key: "mine_approved",
    prCategories: ["MineApproved"] as const,
  },
  {
    name: "Changes requested",
    level: 1,
    key: "mine_changes_requested",
    prCategories: ["MineChangesRequested"] as const,
  },

  {
    name: "Reviewer",
    level: 0,
    key: "review_requested",
    prCategories: ["ReviewRequested", "Rereview", "ReviewMissing"] as const,
  },
  {
    name: "Re-review",
    level: 1,
    key: "re-review",
    prCategories: ["Rereview"] as const,
  },
  {
    name: "Review missing",
    level: 1,
    key: "review_missing",
    prCategories: ["ReviewMissing"] as const,
  },
] as const;
