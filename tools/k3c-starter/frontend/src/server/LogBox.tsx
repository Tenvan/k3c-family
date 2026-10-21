import { useLayoutEffect, useRef } from 'react';

/** Wie nah am Ende (px) die Box noch als „am Ende“ gilt. */
const END_SLACK = 4;

/** Meldungen des Servers; rutscht mit neuen Zeilen mit, solange sie am Ende stand, sonst bleibt die Lesestelle. */
export function LogBox({ lines }: { lines: string[] }) {
  const box = useRef<HTMLPreElement>(null);
  const atEnd = useRef(true);
  const track = () => {
    const el = box.current;
    if (el) atEnd.current = el.scrollTop + el.clientHeight >= el.scrollHeight - END_SLACK;
  };
  useLayoutEffect(() => {
    const el = box.current;
    if (el && atEnd.current) el.scrollTop = el.scrollHeight;
  }, [lines]);
  return (
    <pre ref={box} onScroll={track} className="srv-log" aria-label="Meldungen des Servers">
      {lines.join('\n')}
    </pre>
  );
}
