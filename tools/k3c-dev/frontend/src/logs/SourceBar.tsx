import type { Source } from '../api';
import { dotTone } from './lines';
import { RoleTags } from './RoleTags';
import { describe, tagsOf, type Group } from './sources';

interface Props {
  groups: Group[];
  selected: string;
  onSelect: (name: string) => void;
}

/** Quellen ohne Dienst unter den Dienst-Karten: gruppiert, je Quelle Zustands-Punkt, Zweck und Detail. */
export function SourceBar({ groups, selected, onSelect }: Props) {
  return (
    <>
      {groups.map((g) => (
        <section key={g.title} className="src-group">
          <h3 className="svc-label">{g.title.toUpperCase()}</h3>
          {g.items.map((s) => (
            <Item key={s.name} s={s} on={s.name === selected} onSelect={onSelect} />
          ))}
        </section>
      ))}
    </>
  );
}

function Item({ s, on, onSelect }: { s: Source; on: boolean; onSelect: (name: string) => void }) {
  return (
    <button className={on ? 'src-item src-item-on' : 'src-item'} onClick={() => onSelect(s.name)} aria-current={on}>
      <span className={`badge-dot tone-${dotTone(s)}`} aria-hidden />
      <span className="src-text">
        <span className="src-name">{s.name}</span>
        <RoleTags tags={tagsOf(s)} />
        <span className="src-desc">{describe(s)}</span>
        {s.detail && <span className="src-desc">{s.detail}</span>}
      </span>
    </button>
  );
}
