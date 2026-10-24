import type { GitBranch, GitPR } from '../api';

// Aufträge an Claude von der Seite Git (B-580/AC-03): kopieren oder als neue Code-Session öffnen (CopyPrompt). Sie
// nennen die Regeln des Repos, damit Claude nichts erzwingt, was die Seite selbst ablehnt.

const RULES = [
  'Regeln: nie Force-Push, nichts an main, develop nur über PRs.',
  'Vor dem Löschen prüfen, ob develop den Inhalt hat (auch per Squash-Merge: Diff gegen origin/develop),',
  'sonst den Branch stehen lassen und melden. Remote-Branches nur löschen, wenn sie noch existieren.',
].join(' ');

/** Prompt für die markierten Branches. */
export function cleanupMarkedPrompt(names: string[]): string {
  return [
    `Räume im k3c-Repo diese ${names.length} Branches auf: ${names.join(', ')}.`,
    'Erst `git fetch --prune`, dann je Branch prüfen und lokal löschen, den Remote-Branch nur nach Rückfrage.',
    RULES,
    'Am Ende eine Tabelle: Branch, Befund, gelöscht ja/nein.',
  ].join('\n');
}

/** Kandidaten für „Veraltete aufräumen“: gemergt oder Remote gone, nicht ausgecheckt, nicht develop/main. */
export function staleBranches(branches: GitBranch[]): GitBranch[] {
  return branches.filter((b) => !b.remote && (b.deletable || (b.track === 'gone' && b.state === 'ungemergt')));
}

/** Prompt für alle veralteten Branches; ungemergte mit gelöschtem Remote sind ausdrücklich zu prüfen. */
export function cleanupStalePrompt(branches: GitBranch[]): string {
  const stale = staleBranches(branches);
  const merged = stale.filter((b) => b.deletable).map((b) => b.name);
  const unclear = stale.filter((b) => !b.deletable).map((b) => b.name);
  const lines = ['Räume im k3c-Repo die veralteten lokalen Branches auf. Erst `git fetch --prune`.'];
  if (merged.length > 0) lines.push(`Laut Workbench in develop enthalten (löschen): ${merged.join(', ')}.`);
  if (unclear.length > 0) lines.push(`Remote gelöscht, aber ungemergt (prüfen, ob die Arbeit woanders steckt, sonst melden): ${unclear.join(', ')}.`);
  if (stale.length === 0) lines.push('Die Workbench sieht keine; prüfe trotzdem `git branch -vv` auf [gone].');
  lines.push('Auch Worktrees unter .claude/worktrees ansehen, aber keinen ohne Rückfrage entfernen.', RULES);
  lines.push('Am Ende eine Tabelle: Branch, Befund, gelöscht ja/nein.');
  return lines.join('\n');
}

/** Prompt zum Beheben eines PRs mit roter CI oder Konflikt. */
export function fixPrPrompt(pr: GitPR): string {
  const problems = [pr.ci === 'rot' && 'die CI ist rot', pr.merge === 'Konflikt' && 'er hat einen Konflikt mit develop'].filter(Boolean);
  return [
    `PR #${pr.number} „${pr.title}“ (Branch ${pr.head}) im k3c-Repo: ${problems.join(' und ') || 'bitte prüfen'}.`,
    `Arbeite im Branch ${pr.head}: develop hineinmergen und Konflikte lösen, CI-Fehler mit \`gh pr checks ${pr.number}\` und den Logs finden und beheben.`,
    'Vor dem Push `task check:diff` grün. Pushen ohne Force, nicht mergen. Melde, was du geändert hast.',
  ].join('\n');
}

/** Prompt, der alle offenen PRs prüft und mergebare per Squash mergt. */
export function reviewAllPrsPrompt(prs: GitPR[]): string {
  return [
    `Prüfe die ${prs.length} offenen PRs gegen develop im k3c-Repo: ${prs.map((p) => `#${p.number} (${p.head})`).join(', ')}.`,
    'Je PR: Entwürfe überspringen; Konflikte mit develop lösen, rote CI beheben, dann bei grüner CI per Squash mergen.',
    'Nach jedem Merge die übrigen neu prüfen (develop hat sich geändert). Nie Force-Push, nichts an main.',
    'Stopp und Rückfrage bei inhaltlichen Entscheidungen. Am Ende eine Tabelle: PR, Ergebnis.',
  ].join('\n');
}
