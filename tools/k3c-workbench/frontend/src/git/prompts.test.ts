import { describe, expect, it, vi } from 'vitest';
import type { GitBranch, GitPR } from '../api';
import { ago } from './format';
import { cleanupMarkedPrompt, cleanupStalePrompt, fixPrPrompt, reviewAllPrsPrompt, staleBranches } from './prompts';
import { badges } from './StatusCard';
import { filterBranches, sortBranches } from './BranchesCard';

vi.mock('../api', () => ({ backend: {} }));

// B-580/AC-03: Prompts nennen die betroffenen Branches bzw. PRs und die Regeln des Repos.

const br = (b: Partial<GitBranch> & Pick<GitBranch, 'name' | 'state'>): GitBranch => ({
  remote: false, upstream: '', sha: '', track: 'fehlt', ahead: 0, behind: 0, baseAhead: 0, baseBehind: 0, date: 0, subject: '',
  deletable: b.state === 'gemergt' || b.state === 'squash', reason: '', ...b,
});

const branches = [
  br({ name: 'develop', state: 'basis', date: 5 }),
  br({ name: 'sprint/bd2', state: 'squash', track: 'gone', date: 3 }),
  br({ name: 'alt/arbeit', state: 'ungemergt', track: 'gone', date: 2 }),
  br({ name: 'feat/x', state: 'aktuell', date: 9 }),
  br({ name: 'neu', state: 'ungemergt', date: 4 }),
  br({ name: 'sprint/k5', state: 'ungemergt', remote: true, track: 'nur remote', date: 8 }),
];

const pr: GitPR = { number: 447, title: 'W11', url: '', head: 'sprint/w11', author: '', updated: '', state: 'offen', ci: 'rot', merge: 'Konflikt' };

describe('Prompts (B-580/AC-03)', () => {
  it('Markierte: nennt jeden Branch und verbietet Force', () => {
    const p = cleanupMarkedPrompt(['sprint/bd2', 'sprint/gr8']);
    expect(p).toContain('diese 2 Branches auf: sprint/bd2, sprint/gr8.');
    expect(p).toContain('nie Force-Push');
  });

  it('Veraltete: gemergte zum Löschen, ungemergte mit gelöschtem Remote zum Prüfen', () => {
    expect(staleBranches(branches).map((b) => b.name)).toEqual(['sprint/bd2', 'alt/arbeit']);
    const p = cleanupStalePrompt(branches);
    expect(p).toContain('(löschen): sprint/bd2.');
    expect(p).toContain('sonst melden): alt/arbeit.');
    expect(p).not.toContain('feat/x');
    expect(cleanupStalePrompt([])).toContain('keine');
  });

  it('Beheben nennt PR, Branch und beide Probleme', () => {
    const p = fixPrPrompt(pr);
    expect(p).toContain('PR #447 „W11“ (Branch sprint/w11)');
    expect(p).toContain('die CI ist rot und er hat einen Konflikt mit develop');
    expect(p).toContain('gh pr checks 447');
  });

  it('Alle prüfen nennt jeden PR', () => {
    expect(reviewAllPrsPrompt([pr, { ...pr, number: 452, head: 'sprint/k5' }])).toContain('#447 (sprint/w11), #452 (sprint/k5)');
  });
});

describe('Branch-Liste', () => {
  it('sortiert lokal vor remote, dann nach Zustand und Alter', () => {
    expect(sortBranches(branches).map((b) => b.name)).toEqual(['feat/x', 'develop', 'neu', 'alt/arbeit', 'sprint/bd2', 'sprint/k5']);
  });

  it('filtert', () => {
    expect(filterBranches(branches, 'remote').map((b) => b.name)).toEqual(['sprint/k5']);
    expect(filterBranches(branches, 'ungemergt').map((b) => b.name)).toEqual(['alt/arbeit', 'neu', 'sprint/k5']);
  });
});

describe('Statuskarte und Zeit', () => {
  it('Pillen sagen, was zu tun ist', () => {
    const s = { branch: 'f', upstream: 'origin/f', upstreamGone: false, ahead: 2, behind: 0, baseAhead: 2, baseBehind: 0, mainBehind: 3,
      changes: [], stashes: [], worktrees: [], fetchedAt: 0, advice: [] };
    expect(badges(s).map((b) => b[1])).toEqual(['2 nicht gepusht', 'develop +2 / −0', 'Working Tree sauber', 'main 3 hinter origin']);
  });

  it('ago', () => {
    const now = 1_000_000_000_000;
    expect(ago(0, now)).toBe('nie');
    expect(ago(now - 30_000, now)).toBe('gerade');
    expect(ago(now - 5 * 60_000, now)).toBe('vor 5 min');
    expect(ago(now - 3 * 3_600_000, now)).toBe('vor 3 h');
    expect(ago(now - 3 * 86_400_000, now)).toBe('vor 3 Tagen');
  });
});
