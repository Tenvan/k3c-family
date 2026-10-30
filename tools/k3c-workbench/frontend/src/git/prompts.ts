import type { GitBranch, GitPR } from '../api';

// Aufträge an Claude von der Seite Git (B-580/AC-03): kopieren oder als neue Code-Session öffnen (CopyPrompt). Kurzform
// ohne Anweisungen (B-639): Art und Namen; die Regeln des Repos setzt GitHub durch (develop und main geschützt).

/** Prompt für die markierten Branches. */
export const cleanupMarkedPrompt = (names: string[]): string => `Branches aufräumen: ${names.join(', ')}`;

/** Kandidaten für „Veraltete aufräumen“: gemergt oder Remote gone, nicht ausgecheckt, nicht develop/main. */
export function staleBranches(branches: GitBranch[]): GitBranch[] {
  return branches.filter((b) => !b.remote && (b.deletable || (b.track === 'gone' && b.state === 'ungemergt')));
}

/** Prompt für alle veralteten Branches; ungemergte mit gelöschtem Remote stehen getrennt. */
export function cleanupStalePrompt(branches: GitBranch[]): string {
  const stale = staleBranches(branches);
  const merged = stale.filter((b) => b.deletable).map((b) => b.name);
  const unclear = stale.filter((b) => !b.deletable).map((b) => b.name);
  const parts = [merged.join(', '), unclear.length > 0 ? `ungemergt: ${unclear.join(', ')}` : ''].filter(Boolean);
  return parts.length > 0 ? `Veraltete Branches aufräumen: ${parts.join('; ')}` : 'Veraltete Branches aufräumen';
}

/** Prompt zum Beheben eines PRs mit roter CI oder Konflikt. */
export function fixPrPrompt(pr: GitPR): string {
  const problems = [pr.ci === 'rot' && 'CI rot', pr.merge === 'Konflikt' && 'Konflikt'].filter(Boolean);
  return `PR #${pr.number} beheben (${pr.head})${problems.length > 0 ? `: ${problems.join(', ')}` : ''}`;
}

/** Prompt, der alle offenen PRs prüft und mergebare mergt. */
export const reviewAllPrsPrompt = (prs: GitPR[]): string => `Offene PRs prüfen und mergen: ${prs.map((p) => `#${p.number}`).join(', ')}`;
