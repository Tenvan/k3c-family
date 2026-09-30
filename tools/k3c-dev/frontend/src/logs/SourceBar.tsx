import type { Source } from '../api';
import { Tip } from '../ui/parts';
import { dotTone } from './lines';

const KINDS: Record<string, string> = { service: 'Dienst', run: 'Lauf', log: 'Log-Datei', console: 'Konsole' };
// Zustände von Läufen und Log-Dateien auf Deutsch; Dienste tragen schon deutsche Zustände.
const STATES: Record<string, string> = {
  running: 'läuft', ok: 'grün', failed: 'rot', timeout: 'Zeitlimit', entries: 'Einträge', empty: 'leer',
};

interface Props {
  sources: Source[];
  selected: string;
  open: boolean;
  onSelect: (name: string) => void;
  onToggle: () => void;
}

/** Quellenleiste (B-064): alle Quellen mit Zustands-Punkt, Details im Tooltip; eingeklappt nur die Punkte. */
export function SourceBar({ sources, selected, open, onSelect, onToggle }: Props) {
  return (
    <aside className={open ? 'src-bar' : 'src-bar src-bar-closed'}>
      <button className="src-toggle" onClick={onToggle} title={open ? 'Leiste einklappen' : 'Leiste ausklappen'}>
        {open ? '‹ Quellen' : '›'}
      </button>
      {sources.map((s) => (
        <Tip key={s.name} content={[KINDS[s.kind] ?? s.kind, STATES[s.state] ?? s.state, s.detail].filter(Boolean).join(' · ')}>
          <button
            className={s.name === selected ? 'src-item src-item-on' : 'src-item'}
            onClick={() => onSelect(s.name)}
            aria-current={s.name === selected}
          >
            <span className={`badge-dot tone-${dotTone(s)}`} aria-hidden />
            {open && <span className="src-name">{s.name}</span>}
          </button>
        </Tip>
      ))}
    </aside>
  );
}
