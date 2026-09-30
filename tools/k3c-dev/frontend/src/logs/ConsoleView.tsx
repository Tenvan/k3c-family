import { Button, Flex } from '@radix-ui/themes';
import { memo, useLayoutEffect, useRef, useState } from 'react';
import type { ConsoleLine } from '../api';
import { formatNumber } from '../lib/format';
import { NoticeCard } from '../ui/parts';
import { parseAnsi } from './ansi';
import { markOf, visibleLines } from './lines';
import { useConsole } from './useConsole';

const BOTTOM_PX = 24; // so nah am Ende gilt die Ansicht als „unten“

/** Reiter `Konsole` (B-064 › Konsole): Puffer der Quelle live, mit Nummern, Farben und Randmarken. */
export function ConsoleView({ name }: { name: string }) {
  const { lines, loaded, error } = useConsole(name);
  const [clearedAt, setClearedAt] = useState(0);
  const [follow, setFollow] = useState(true);
  const [atBottom, setAtBottom] = useState(true);
  const box = useRef<HTMLDivElement>(null);
  const bottom = useRef(true);
  const shown = visibleLines(lines, clearedAt);

  const toEnd = () => {
    const el = box.current;
    if (el) el.scrollTop = el.scrollHeight;
  };
  useLayoutEffect(() => {
    if (follow && bottom.current) toEnd();
  }, [shown.length, lines, follow]);
  const onScroll = () => {
    const el = box.current;
    if (!el) return;
    bottom.current = el.scrollHeight - el.scrollTop - el.clientHeight < BOTTOM_PX;
    setAtBottom(bottom.current);
  };

  return (
    <section className="console">
      <Flex className="console-head" align="center" gap="3">
        <span className="console-name">{name}</span>
        <span className="console-count">{formatNumber(shown.length)} Zeilen</span>
        <Flex gap="2" ml="auto">
          <Button size="1" variant="soft" color={follow ? undefined : 'gray'} onClick={() => setFollow(!follow)}>
            Mitlaufen: {follow ? 'an' : 'aus'}
          </Button>
          <Button size="1" variant="soft" color="gray" onClick={() => setClearedAt(lines.at(-1)?.seq ?? 0)}>
            Leeren
          </Button>
        </Flex>
      </Flex>
      {error && <NoticeCard title="Konsole nicht geladen" tone="error">{error}</NoticeCard>}
      {loaded && !error && shown.length === 0 && (
        <NoticeCard title="Keine Zeilen" tone="neutral">
          {clearedAt > 0 ? 'Anzeige geleert' : 'Die Quelle hat noch nichts geschrieben'}; neue Zeilen erscheinen hier live.
        </NoticeCard>
      )}
      <div className="console-body" ref={box} onScroll={onScroll}>
        {shown.map((l) => (
          <Row key={l.seq} line={l} />
        ))}
      </div>
      {!atBottom && (
        <Button className="console-end" size="1" onClick={toEnd}>
          Zum Ende ↓
        </Button>
      )}
    </section>
  );
}

/** Eine Zeile; memo, damit neue Zeilen die alten nicht neu zerlegen. */
const Row = memo(function Row({ line }: { line: ConsoleLine }) {
  const mark = markOf(line.text);
  return (
    <div className={mark ? `cline mark-${mark}` : 'cline'}>
      <span className="cline-no">{line.seq}</span>
      <span className="cline-text">
        {parseAnsi(line.text).map((s, i) => (
          <span key={i} className={s.className || undefined} style={s.style}>
            {s.text}
          </span>
        ))}
      </span>
    </div>
  );
});
