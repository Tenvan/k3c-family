import { Text } from '@radix-ui/themes';
import { useEffect, useState } from 'react';
import { backend, type GitHubData } from '../api';
import { ActionButton } from '@webkit/parts';
import { Pill } from './Pill';

/** GitHub-Stand: beim Öffnen und bei `planning:changed` aus dem Zwischenspeicher, per Knopf frisch. */
export function useGitHub(): [GitHubData | null, () => Promise<void>] {
  const [gh, setGh] = useState<GitHubData | null>(null);
  const load = (force: boolean) => backend.githubStatus(force).then(setGh, () => setGh(null));
  useEffect(() => {
    void load(false);
    return backend.on('planning:changed', () => void load(false));
  }, []);
  return [gh, () => load(true)];
}

const TONE = { grün: 'ok', rot: 'error', läuft: 'warn', '–': 'neutral' } as const;

/** Kopfzeile über der Filterleiste: letzter CI-Lauf auf develop, Hinweis von gh, Neu laden (B-212). */
export function GitHubBar({ gh, reload }: { gh: GitHubData | null; reload: () => Promise<void> }) {
  return (
    <div className="pl-ghbar">
      <Text size="1" color="gray">GitHub</Text>
      {gh?.develop && (
        <button type="button" className="pl-link" onClick={() => backend.openUrl(gh.develop!.url)} title={gh.develop.created}>
          <Pill tone={TONE[gh.develop.ci]}>develop · CI {gh.develop.ci}</Pill>
        </button>
      )}
      {gh?.develop && <Text size="1" color="gray" className="pl-ellipsis">{gh.develop.title}</Text>}
      {gh?.error && <Pill tone="warn">{gh.error}</Pill>}
      {gh === null && <Text size="1" color="gray">lädt …</Text>}
      <ActionButton size="1" variant="ghost" color="gray" className="pl-push" onClick={reload}>Neu laden</ActionButton>
    </div>
  );
}
