package planning

import (
	"context"
	"regexp"
	"strings"
	"time"

	"k3c/tools/k3c-dev/internal/proc"
)

// Worktrees liefert die Branches aller Worktrees des Repos (`git worktree list`); ohne git oder Repo keine.
func Worktrees(root string) []string {
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()
	out, err := proc.Command(ctx, []string{"git", "-C", root, "worktree", "list", "--porcelain"}).Output()
	if err != nil {
		return nil
	}
	return parseWorktrees(string(out))
}

func parseWorktrees(porcelain string) []string {
	var branches []string
	for _, l := range strings.Split(strings.ReplaceAll(porcelain, "\r\n", "\n"), "\n") {
		if b, ok := strings.CutPrefix(l, "branch refs/heads/"); ok {
			branches = append(branches, b)
		}
	}
	return branches
}

// markWorktrees ordnet Branches den Sprints zu: `sprint/s4`, `s4-4-work`, `f4/4-review` gehören zu S4 bzw. F4.
// ponytail: nur über den Branch-Namen; ein Worktree auf einem fremd benannten Branch bleibt unerkannt.
func markWorktrees(sprints []Sprint, branches []string) {
	for i, s := range sprints {
		if s.ID == "" {
			continue
		}
		re := regexp.MustCompile(`(?i)^(sprint/)?` + regexp.QuoteMeta(s.ID) + `([-/.]|$)`)
		for _, b := range branches {
			if re.MatchString(b) {
				sprints[i].Worktree = b
				break
			}
		}
	}
}
