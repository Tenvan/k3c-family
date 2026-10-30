import { describe, expect, it, vi } from 'vitest';
import type { GitBranch, GitPR } from '../api';
import { ago } from './format';
import { cleanupMarkedPrompt, cleanupStalePrompt, fixPrPrompt, reviewAllPrsPrompt, staleBranches } from './prompts';
import { badges } from './StatusCard';
import { filterBranches, sortBranches } from './BranchesCard';

vi.mock('../api', () => ({ backend: {} }));

// B-580/AC-03, B-639: Prompts nennen in einer Zeile die betroffenen Branches bzw. PRs.

const br = (b: Partial<GitBranch> & Pick<GitBranch, 'name' | 'state'>): GitBranch => ({
  remote: false, upstream: '', sha: '', checked: true, pr: 0, track: 'fehlt', ahead: 0, behind: 0, baseAhead: 0, baseBehind: 0, date: 0, subject: '',
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

describe('Prompts in Kurzform (B-580/AC-03, B-639)', () => {
  it('Markierte: eine Zeile mit jedem Branch', () => {
    expect(cleanupMarkedPrompt(['sprint/bd2', 'sprint/gr8'])).toBe('Branches aufräumen: sprint/bd2, sprint/gr8');
  });

  it('Veraltete: gemergte und ungemergte mit gelöschtem Remote getrennt', () => {
    expect(staleBranches(branches).map((b) => b.name)).toEqual(['sprint/bd2', 'alt/arbeit']);
    expect(cleanupStalePrompt(branches)).toBe('Veraltete Branches aufräumen: sprint/bd2; ungemergt: alt/arbeit');
    expect(cleanupStalePrompt([])).toBe('Veraltete Branches aufräumen');
  });

  it('Beheben nennt PR, Branch und beide Probleme', () => {
    expect(fixPrPrompt(pr)).toBe('PR #447 beheben (sprint/w11): CI rot, Konflikt');
  });

  it('Alle prüfen nennt jeden PR', () => {
    expect(reviewAllPrsPrompt([pr, { ...pr, number: 452, head: 'sprint/k5' }])).toBe('Offene PRs prüfen und mergen: #447, #452');
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
    expect(badges(s).map((b) => b[1])).toEqual(['2 nicht gepusht', 'develop +2 / −0', 'main 3 hinter origin']);
    const changes = [{ path: 'a', code: 'M', staged: true }, { path: 'a', code: 'M', staged: false }, { path: 'b', code: '?', staged: false }];
    expect(badges({ ...s, changes }).map((b) => b[1])).toContain('2 Dateien nicht committet');
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
