import { useCallback, useEffect, useRef, useState, type Dispatch, type SetStateAction } from 'react';
import { ActionButton, Flex, StatusBadge, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { errorText } from '@webkit/lib/errors';
import { backend, type GitAction, type GitPRList, type GitView } from '../api';
import { BranchesCard } from './BranchesCard';
import { Confirm, type ConfirmRequest } from './Confirm';
import { FilesCard } from './FilesCard';
import { ago } from './format';
import { usePrPoll } from './prPoll';
import { useRepoStamp } from './repoStamp';
import { PrsCard } from './PrsCard';
import { StatusCard } from './StatusCard';

interface Result {
  ok: boolean;
  text: string;
}

/** Reiter `Git` (B-580) in der Reihenfolge des Ablaufs: Dateien, Commit-Status, PRs, Branches. Die Seite bleibt
 *  eingehängt und liest das Repo nur beim ersten Öffnen, mit „Laden“ und nach einer eigenen Aktion; nur die PRs folgen
 *  GitHub im Takt, solange der Reiter offen ist (prPoll.ts). */
export function GitPage({ active }: { active: boolean }) {
  const [view, setView] = useState<GitView | null>(null);
  const [prs, setPrs] = useState<GitPRList | null>(null);
  const [error, setError] = useState('');
  const [loadedAt, setLoadedAt] = useState(0);
  const [result, setResult] = useState<Result | null>(null);
  const [ask, setAsk] = useState<ConfirmRequest | null>(null);
  const started = useRef(false);

  // Ein Fehler beim Nachladen lässt die zuletzt gesehenen PRs stehen und zeigt nur den Hinweis.
  const refreshPrs = useCallback(() => backend.gitPRs().then(setPrs,
    (e: unknown) => setPrs((cur) => ({ prs: cur?.prs ?? [], error: errorText(e), fetched: cur?.fetched }))), []);
  usePrPoll(active, prs, refreshPrs);
  const { stale, mark } = useRepoStamp(active, refreshPrs); // Repo von außen geändert: PRs sofort, Rest per Knopf

  const load = useCallback(async () => {
    const settled = await mark();
    const prsTask = refreshPrs();
    await backend.gitOverview().then((v) => {
      setView(v);
      setError('');
    }, (e: unknown) => setError(errorText(e)));
    await prsTask;
    settled();
    setLoadedAt(Date.now());
  }, [mark, refreshPrs]);
  useEffect(() => {
    if (!active || started.current) return;
    started.current = true;
    void load();
  }, [active, load]);

  const act = async (task: Promise<string>) => {
    try {
      setResult({ ok: true, text: await task });
    } catch (e) {
      setResult({ ok: false, text: errorText(e) });
    }
    await load();
  };
  const check = (names: string[]) => checkBranches(names, setView, setResult);
  return (
    <div className="git-page">
      <LoadBar loaded={view !== null} loadedAt={loadedAt} stale={stale} reading={!view && !error} result={result} load={load} />
      {error && <NoticeCard title="Repo nicht gelesen" tone="error">{error}</NoticeCard>}
      {view && (
        <>
          <FilesCard changes={view.status.changes} />
          <StatusCard status={view.status} files={view.files} run={(a: GitAction) => act(backend.gitAction(a))} confirm={setAsk} />
          <PrsCard branch={view.status.branch} list={prs} act={act} confirm={setAsk} />
          <BranchesCard branches={view.branches} remove={(names, remote) => act(backend.gitDeleteBranches(names, remote))} check={check} confirm={setAsk} />
        </>
      )}
      <Confirm request={ask} onClose={() => setAsk(null)} />
    </div>
  );
}

/** Die Squash-Prüfung liefert nur die Branch-Liste neu; der übrige Stand bleibt, bis jemand „Neu laden“ drückt. */
async function checkBranches(names: string[], setView: Dispatch<SetStateAction<GitView | null>>, setResult: (r: Result) => void) {
  try {
    const branches = await backend.gitCheckMerged(names);
    setView((v) => (v ? { ...v, branches } : v));
    setResult({ ok: true, text: names.length === 1 ? `${names[0]} geprüft` : 'Branches geprüft' });
  } catch (e) {
    setResult({ ok: false, text: errorText(e) });
  }
}

interface LoadBarProps {
  loaded: boolean;
  loadedAt: number;
  /** Das Repo hat sich seit dem Laden geändert (Fetch, Commit, Checkout … auch von außen). */
  stale: boolean;
  reading: boolean;
  result: Result | null;
  load: () => Promise<void>;
}

/** Kopfleiste: der eine Knopf zum Laden, Alter des Stands und Ergebnis der letzten Aktion. */
function LoadBar({ loaded, loadedAt, stale, reading, result, load }: LoadBarProps) {
  return (
    <Flex gap="3" align="center">
      <ActionButton size="1" color={stale ? 'amber' : undefined} onClick={load}>{loaded ? 'Neu laden' : 'Laden'}</ActionButton>
      {stale && <StatusBadge tone="warn">Repo geändert – PRs aktualisiert, Rest mit „Neu laden“</StatusBadge>}
      {loadedAt > 0 && !stale && <Text size="1" color="gray">Stand {ago(loadedAt)}</Text>}
      {reading && <Text size="2" color="gray">Lese das Repo …</Text>}
      {result && <Text size="2" color={result.ok ? 'green' : 'red'} role="status">{result.text}</Text>}
    </Flex>
  );
}
