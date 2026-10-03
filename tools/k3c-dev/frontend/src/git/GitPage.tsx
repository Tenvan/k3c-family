import { Button, Checkbox, Flex, Select, Text, TextArea, TextField } from '@radix-ui/themes';
import { useEffect, useState } from 'react';
import { backend, type GitFile, type GitView } from '../api';
import { errorText } from '../lib/errors';
import { ActionButton, NoticeCard, StatusBadge } from '../ui/parts';
import { formatMessage, MAX_SUBJECT, parseFieldError, pruneSelection, splitPath, statusLabel, statusTone } from './git';

/** Reiter `Git`: Dateien stagen und committen. Verändert nur den Index und die Historie (kein Discard, Push oder Stash). */
export function GitPage() {
  const [view, setView] = useState<GitView | null>(null);
  const [error, setError] = useState('');
  const [notice, setNotice] = useState('');
  const load = () => backend.git().then((v) => { setView(v); setError(''); }, (e) => setError(errorText(e)));
  useEffect(() => void load(), []);
  if (error) return <NoticeCard title="Git nicht lesbar" tone="error">{error}</NoticeCard>;
  if (!view) return <Text color="gray">Lade Git-Stand …</Text>;
  return (
    <div className="gt-page">
      <div className="gt-files">
        <FileList title="Nicht gestaged" files={view.unstaged} action="Stagen" onApply={async (paths) => setView(await backend.gitStage(paths))} />
        <FileList title="Gestaged" files={view.staged} action="Unstagen" onApply={async (paths) => setView(await backend.gitUnstage(paths))} />
      </div>
      <div className="gt-side">
        <Flex align="center" gap="2">
          <Text weight="bold">Commit</Text>
          <StatusBadge tone="neutral">{view.branch || 'kein Branch'}</StatusBadge>
          <Button size="1" variant="soft" color="gray" onClick={() => void load()}>Neu laden</Button>
        </Flex>
        <CommitForm view={view} onDone={(v, hash) => { setView(v); setNotice(`Commit ${hash} angelegt`); }} />
        {notice && <NoticeCard title="Fertig" tone="ok">{notice}</NoticeCard>}
        <Text size="2" weight="bold">Letzte Commits</Text>
        <ul className="gt-recent">{view.recent.map((l) => <li key={l}><code>{l.slice(0, 7)}</code> {l.slice(8)}</li>)}</ul>
      </div>
    </div>
  );
}

function FileList({ title, files, action, onApply }: {
  title: string; files: GitFile[]; action: string; onApply: (paths: string[]) => Promise<unknown>;
}) {
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [error, setError] = useState('');
  useEffect(() => setSelected((s) => pruneSelection(s, files)), [files]);
  const toggle = (p: string) => setSelected((s) => { const n = new Set(s); if (!n.delete(p)) n.add(p); return n; });
  const apply = (paths: string[]) => async () => {
    setError('');
    try {
      await onApply(paths);
    } catch (e) {
      setError(errorText(e));
    }
  };
  const all = files.map((f) => f.path);
  return (
    <section className="gt-list">
      <Flex align="center" gap="2">
        <Text weight="bold">{title}</Text>
        <Text size="1" color="gray">{files.length}</Text>
        <Flex gap="2" ml="auto">
          <ActionButton size="1" variant="soft" disabled={selected.size === 0} onClick={apply([...selected])}>{action} ({selected.size})</ActionButton>
          <ActionButton size="1" variant="soft" color="gray" disabled={files.length === 0} onClick={apply(all)}>Alle</ActionButton>
        </Flex>
      </Flex>
      {error && <p className="svc-error">{error}</p>}
      <div className="gt-rows">
        {files.length === 0 && <Text size="2" color="gray">Keine Dateien.</Text>}
        {files.map((f) => {
          const { dir, name } = splitPath(f.path);
          return (
            <label key={f.path} className="gt-row" title={f.path}>
              <Checkbox checked={selected.has(f.path)} onCheckedChange={() => toggle(f.path)} />
              <StatusBadge tone={statusTone(f.status)}>{statusLabel(f.status)}</StatusBadge>
              <span className="gt-path"><span className="gt-dir">{dir}</span>{name}</span>
            </label>
          );
        })}
      </div>
    </section>
  );
}

function CommitForm({ view, onDone }: { view: GitView; onDone: (v: GitView, hash: string) => void }) {
  const [m, setM] = useState({ type: 'feat', scope: '', subject: '', body: '' });
  const [err, setErr] = useState({ field: '', text: '' });
  const set = (patch: Partial<typeof m>) => { setM({ ...m, ...patch }); setErr({ field: '', text: '' }); };
  const commit = async () => {
    try {
      const r = await backend.gitCommit(m);
      setM({ ...m, subject: '', body: '' });
      onDone(r.view, r.hash);
    } catch (e) {
      setErr(parseFieldError(errorText(e)));
    }
  };
  const bad = (f: string) => (err.field === f ? { color: 'red' as const } : {});
  return (
    <div className="gt-form">
      <Flex gap="2">
        <Select.Root size="2" value={m.type} onValueChange={(type) => set({ type })}>
          <Select.Trigger {...bad('type')} />
          <Select.Content>{view.types.map((t) => <Select.Item key={t} value={t}>{t}</Select.Item>)}</Select.Content>
        </Select.Root>
        <TextField.Root className="gt-scope" size="2" list="gt-domains" placeholder="Domäne (srv, cli …)" value={m.scope}
          {...bad('scope')} onChange={(e) => set({ scope: e.target.value })} />
        <datalist id="gt-domains">{view.domains.map((d) => <option key={d} value={d} />)}</datalist>
      </Flex>
      <TextField.Root size="2" placeholder="Betreff" value={m.subject} {...bad('subject')} onChange={(e) => set({ subject: e.target.value })}>
        <TextField.Slot side="right"><Text size="1" color={m.subject.length > MAX_SUBJECT ? 'red' : 'gray'}>{m.subject.length}/{MAX_SUBJECT}</Text></TextField.Slot>
      </TextField.Root>
      <TextArea size="2" rows={5} placeholder="Rumpf (optional)" value={m.body} onChange={(e) => set({ body: e.target.value })} />
      {err.text && <p className="svc-error">{err.text}</p>}
      <pre className="gt-preview">{formatMessage(m)}</pre>
      <ActionButton disabled={view.staged.length === 0} onClick={commit}>
        {view.staged.length === 0 ? 'Nichts gestaged' : `Committen (${view.staged.length} Dateien)`}
      </ActionButton>
    </div>
  );
}
