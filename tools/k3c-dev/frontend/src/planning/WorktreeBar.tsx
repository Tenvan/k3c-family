import { Text } from '@radix-ui/themes';
import { useState } from 'react';
import { backend, type WorktreeChange, type WorktreeDiff } from '../api';
import { ActionButton } from '../ui/parts';
import { Pill } from './Pill';

const TONE = { neu: 'ok', geändert: 'warn', entfernt: 'error' } as const;
const MARK = { neu: '+ ', geändert: '~ ', entfernt: '− ' } as const;

/** Marke einer Änderung: „neu“ grün, „geändert“ amber; Titel nennt Objekt und Text. */
function ChangePill({ c }: { c: WorktreeChange }) {
  return (
    <Pill tone={TONE[c.kind]} title={`${c.item} ${c.id}: ${c.text}`}>
      {MARK[c.kind]}{c.id} · {c.text}
    </Pill>
  );
}

/**
 * Workflow-Check: gleicht auf Knopfdruck die Planung der Worktrees mit der Wurzel ab und zeigt, was dort neu oder
 * anders ist. Nur Anzeige; die Planung selbst (Boards, Zähler, Filter) bleibt die der Wurzel.
 */
export function WorktreeBar() {
  const [diffs, setDiffs] = useState<WorktreeDiff[] | null>(null);
  const [err, setErr] = useState<string | null>(null);
  const check = async () => {
    try {
      setDiffs(await backend.worktreeCheck());
      setErr(null);
    } catch (e) {
      setDiffs(null);
      setErr(e instanceof Error ? e.message : String(e)); // Wails reicht Go-Fehler als Text durch
    }
  };
  return (
    <div className="pl-wtbar">
      <div className="pl-ghbar">
        <Text size="1" color="gray">Worktrees</Text>
        {err && <Pill tone="error">{err}</Pill>}
        {diffs?.length === 0 && <Text size="1" color="gray">keine weiteren Worktrees</Text>}
        <ActionButton size="1" variant="ghost" color="gray" className="pl-push" onClick={check}>Workflow-Check</ActionButton>
      </div>
      {diffs?.map((d) => (
        <div key={d.path} className="pl-wtrow" title={d.path}>
          <Pill tone="violet">{d.branch || d.path}</Pill>
          {d.error && <Pill tone="error">{d.error}</Pill>}
          {!d.error && d.changes.length === 0 && <Text size="1" color="gray">keine Planungsunterschiede</Text>}
          {d.changes.map((c) => <ChangePill key={`${c.item}-${c.id}-${c.text}`} c={c} />)}
        </div>
      ))}
    </div>
  );
}
