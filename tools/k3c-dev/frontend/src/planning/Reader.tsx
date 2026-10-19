import { Callout, Dialog, Flex, IconButton, Progress, Text, TextField } from '@radix-ui/themes';
import { useEffect, useMemo, useRef, useState, type KeyboardEvent, type ReactNode, type RefObject } from 'react';
import { backend } from '../api';
import { errorText } from '@webkit/errors';
import { MarkdownView } from '../ui/MarkdownView';
import { CopyPrompt } from './PromptParts';
import { parseDoc, type ReaderSection } from './readerDoc';

// Lesemodus (B-439, wie der Ticket-Lesemodus der erp-qs-Workbench): fast fensterfüllender Dialog mit Inhaltsverzeichnis,
// Scroll-Spy, Suche (Strg+F, Enter/Umschalt+Enter), j/k für die nächste/vorige Rubrik, Fortschritt der abgehakten
// Kriterien und Esc. Der Text kommt aus planningGet (wie plan_get) und wird mit ui/MarkdownView gezeichnet.

interface Props {
  /** Ticket, Sprint, Session oder Projekt. */
  id: string;
  /** Titel im Detailkopf; Doppelklick, Enter oder Leertaste darauf öffnet den Lesemodus. */
  title: string;
  prompt: string;
  pill: ReactNode;
}

type Load = { state: 'loading' } | { state: 'ok'; text: string; at: Date } | { state: 'error'; message: string };

const SPY_OFFSET = 24;
const reduced = () => window.matchMedia?.('(prefers-reduced-motion: reduce)').matches ?? false;

// CSS Custom Highlight API (Chromium/WebView2): markiert Treffer, ohne den von React verwalteten DOM anzufassen.
type Registry = Map<string, unknown>;
const registry = () => (globalThis as unknown as { CSS?: { highlights?: Registry } }).CSS?.highlights;
const HighlightCtor = () => (globalThis as unknown as { Highlight?: new (...r: Range[]) => unknown }).Highlight;

/** Alle Fundstellen von q in den Textknoten unter root, ohne Groß-/Kleinschreibung. */
function findRanges(root: HTMLElement, q: string): Range[] {
  const out: Range[] = [];
  const walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT);
  for (let n = walker.nextNode(); n; n = walker.nextNode()) {
    const text = (n.textContent ?? '').toLowerCase();
    for (let at = text.indexOf(q); at >= 0; at = text.indexOf(q, at + q.length)) {
      const r = document.createRange();
      r.setStart(n, at);
      r.setEnd(n, at + q.length);
      out.push(r);
    }
  }
  return out;
}

function useDoc(id: string, open: boolean): Load {
  const [load, setLoad] = useState<Load>({ state: 'loading' });
  useEffect(() => {
    if (!open) return;
    let alive = true;
    setLoad({ state: 'loading' });
    backend.planningGet(id).then((text) => alive && setLoad({ state: 'ok', text, at: new Date() }),
      (e) => alive && setLoad({ state: 'error', message: errorText(e) }));
    return () => { alive = false; };
  }, [id, open]);
  return load;
}

/** Aktive Rubrik: die letzte, deren Überschrift oben vorbei ist; nach einem Sprung dessen Ziel, bis von Hand gescrollt wird. */
function useSpy(box: RefObject<HTMLDivElement | null>, toc: ReaderSection[], open: boolean) {
  const [active, setActive] = useState('');
  const pinned = useRef('');
  useEffect(() => {
    const el = box.current;
    if (!open || !el || toc.length === 0) return;
    let frame = 0;
    const update = () => {
      frame = 0;
      if (pinned.current) return setActive(pinned.current);
      const top = el.getBoundingClientRect().top + SPY_OFFSET;
      let cur = toc[0].id;
      for (const { id } of toc) if ((document.getElementById(id)?.getBoundingClientRect().top ?? Infinity) <= top) cur = id;
      if (el.scrollTop + el.clientHeight >= el.scrollHeight - 2) cur = toc[toc.length - 1].id;
      setActive(cur);
    };
    const onScroll = () => { if (!frame) frame = requestAnimationFrame(update); };
    const release = () => { pinned.current = ''; };
    update();
    el.addEventListener('scroll', onScroll, { passive: true });
    for (const ev of ['wheel', 'pointerdown', 'keydown']) el.addEventListener(ev, release, { passive: true });
    return () => {
      el.removeEventListener('scroll', onScroll);
      for (const ev of ['wheel', 'pointerdown', 'keydown']) el.removeEventListener(ev, release);
      if (frame) cancelAnimationFrame(frame);
    };
  }, [box, toc, open]);
  const jump = (id: string) => {
    pinned.current = id;
    setActive(id);
    document.getElementById(id)?.scrollIntoView({ block: 'start', behavior: reduced() ? 'auto' : 'smooth' });
  };
  return { active, jump, unpin: () => { pinned.current = ''; } };
}

/** Treffer der Suche (ab 2 Zeichen) mit Markierung; step springt zum nächsten oder vorigen. */
function useSearch(content: RefObject<HTMLElement | null>, box: RefObject<HTMLDivElement | null>, query: string, body: unknown, unpin: () => void) {
  const [hits, setHits] = useState<Range[]>([]);
  const [hit, setHit] = useState(0);
  useEffect(() => {
    const q = query.trim().toLowerCase();
    setHit(0);
    setHits(content.current && q.length >= 2 ? findRanges(content.current, q) : []);
  }, [query, body, content]);
  useEffect(() => {
    const reg = registry();
    const H = HighlightCtor();
    if (!reg || !H) return;
    reg.set('pl-search', new H(...hits));
    reg.set('pl-search-current', new H(...(hits[hit] ? [hits[hit]] : [])));
    return () => { reg.delete('pl-search'); reg.delete('pl-search-current'); };
  }, [hits, hit]);
  const step = (dir: 1 | -1) => {
    const el = box.current;
    if (!hits.length || !el) return;
    const next = (hit + dir + hits.length) % hits.length;
    setHit(next);
    unpin();
    const delta = hits[next].getBoundingClientRect().top - el.getBoundingClientRect().top - el.clientHeight / 3;
    el.scrollBy({ top: delta, behavior: reduced() ? 'auto' : 'smooth' });
  };
  return { hits, hit, step };
}

/** Titel im Detailkopf mit Lese-Symbol; beide öffnen den Lesemodus. */
export function Reader({ id, title, prompt, pill }: Props) {
  const [open, setOpen] = useState(false);
  return (
    <>
      <span className="pl-reader-title" role="button" tabIndex={0} title="Doppelklick, Enter oder Leertaste: Volltext lesen"
        onDoubleClick={() => setOpen(true)} onKeyDown={(e) => (e.key === 'Enter' || e.key === ' ') && (e.preventDefault(), setOpen(true))}>
        {title}
      </span>
      <IconButton size="1" variant="ghost" color="gray" aria-label="Volltext lesen" title="Volltext lesen" onClick={() => setOpen(true)}>📖</IconButton>
      <Dialog.Root open={open} onOpenChange={setOpen}>
        {open && <ReaderDialog id={id} title={title} prompt={prompt} pill={pill} />}
      </Dialog.Root>
    </>
  );
}

function ReaderDialog({ id, title, prompt, pill }: Props) {
  const [full, setFull] = useState(false);
  const [query, setQuery] = useState('');
  const load = useDoc(id, true);
  const box = useRef<HTMLDivElement>(null);
  const content = useRef<HTMLElement>(null);
  const search = useRef<HTMLInputElement>(null);
  const doc = useMemo(() => (load.state === 'ok' ? parseDoc(load.text) : null), [load]);
  const toc = useMemo(() => doc?.sections.filter((s) => s.title) ?? [], [doc]);
  const spy = useSpy(box, toc, true);
  // Gerendertes Markdown nur neu, wenn sich der Text ändert: sonst zeigen die Such-Ranges auf verworfene Knoten.
  const body = useMemo(() => doc?.sections.map((s, i) => (
    s.title ? <section key={s.id} id={s.id}><h2>{s.title}</h2><MarkdownView source={s.text} headingOffset={1} /></section>
      : <MarkdownView key={i} source={s.text} headingOffset={1} />)), [doc]);
  const found = useSearch(content, box, query, body, spy.unpin);
  const onKeyDown = (e: KeyboardEvent<HTMLDivElement>) => {
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'f') {
      e.preventDefault();
      search.current?.select();
      return;
    }
    if (e.target instanceof HTMLInputElement || e.ctrlKey || e.metaKey || e.altKey || (e.key !== 'j' && e.key !== 'k')) return;
    const at = toc.findIndex((s) => s.id === spy.active);
    const next = toc[Math.min(Math.max(at + (e.key === 'j' ? 1 : -1), 0), toc.length - 1)];
    if (next) spy.jump(next.id);
  };
  return (
    <Dialog.Content aria-describedby={undefined} className={`pl-reader${full ? ' is-full' : ''}`} maxWidth="none" onKeyDown={onKeyDown}
      onOpenAutoFocus={(e) => { e.preventDefault(); box.current?.focus(); }}>
      <header className="pl-reader-head">
        {pill}
        <Dialog.Title size="4" className="pl-reader-heading">{title}</Dialog.Title>
        <Flex align="center" gap="2">
          {prompt && <CopyPrompt prompt={prompt} what={id} />}
          <IconButton size="1" variant="ghost" color="gray" aria-label={full ? 'Vollbild verlassen' : 'Vollbild'} title={full ? 'Vollbild verlassen' : 'Vollbild'}
            onClick={() => setFull((f) => !f)}>{full ? '⤡' : '⤢'}</IconButton>
          <Dialog.Close><IconButton size="1" variant="ghost" color="gray" aria-label="Schließen" title="Schließen (Esc)">×</IconButton></Dialog.Close>
        </Flex>
        <ReaderMeta done={doc?.done ?? 0} total={doc?.total ?? 0} query={query} setQuery={setQuery} search={search} found={found} />
      </header>
      <div className="pl-reader-body">
        {toc.length > 0 && <ReaderToc toc={toc} active={spy.active} jump={spy.jump} />}
        <div className="pl-reader-scroll" ref={box} tabIndex={-1}>
          <article className="pl-reader-content pl-md" ref={content}>
            {load.state === 'loading' && <Text color="gray" size="2">Lade {id} …</Text>}
            {load.state === 'error' && <Callout.Root color="red" role="alert" size="1"><Callout.Text>{id} nicht lesbar: {load.message}</Callout.Text></Callout.Root>}
            {body}
          </article>
        </div>
      </div>
      <footer className="pl-reader-foot">
        <code>{doc?.path}</code>
        <Text size="1" color="gray">
          {load.state === 'ok' ? `geladen ${load.at.toLocaleTimeString('de-DE')} · ` : ''}j/k: Rubrik · Strg+F: Suche · Esc: schließen
        </Text>
      </footer>
    </Dialog.Content>
  );
}

/** Inhaltsverzeichnis links; die aktive Rubrik ist markiert, ein Klick springt hin. */
function ReaderToc({ toc, active, jump }: { toc: ReaderSection[]; active: string; jump: (id: string) => void }) {
  return (
    <nav aria-label="Inhalt" className="pl-reader-toc">
      <Text as="div" size="1" color="gray" weight="medium" className="pl-reader-toc-label">Inhalt</Text>
      {toc.map((s) => (
        <button key={s.id} type="button" className={s.id === active ? 'is-active' : undefined}
          aria-current={s.id === active ? 'location' : undefined} onClick={() => jump(s.id)}>{s.title}</button>
      ))}
    </nav>
  );
}

interface MetaProps {
  done: number;
  total: number;
  query: string;
  setQuery: (q: string) => void;
  search: RefObject<HTMLInputElement | null>;
  found: ReturnType<typeof useSearch>;
}

/** Zweite Kopfzeile: Fortschritt der Kriterien und Suche mit Trefferzahl und Pfeilen. */
function ReaderMeta({ done, total, query, setQuery, search, found }: MetaProps) {
  const { hits, hit, step } = found;
  return (
    <Flex align="center" gap="3" wrap="wrap" className="pl-reader-meta">
      {total > 0 && (
        <Flex align="center" gap="2" className="pl-reader-ac" title={`${done} von ${total} Kriterien abgehakt`}>
          <Progress size="1" value={Math.round((done / total) * 100)} color={done === total ? 'green' : undefined} aria-label="Abgehakte Kriterien" />
          <Text size="1">{done} von {total} abgehakt</Text>
        </Flex>
      )}
      <TextField.Root ref={search} size="1" className="pl-reader-search" aria-label="Im Text suchen" placeholder="Suchen (Strg+F)" value={query}
        onChange={(e) => setQuery(e.target.value)} onKeyDown={(e) => e.key === 'Enter' && (e.preventDefault(), step(e.shiftKey ? -1 : 1))}>
        <TextField.Slot>⌕</TextField.Slot>
        {query.trim().length >= 2 && (
          <TextField.Slot>
            <Text size="1" color="gray">{hits.length ? `${hit + 1}/${hits.length}` : '0'}</Text>
            <IconButton size="1" variant="ghost" color="gray" disabled={!hits.length} aria-label="Vorheriger Treffer" title="Vorheriger Treffer (Umschalt+Enter)" onClick={() => step(-1)}>↑</IconButton>
            <IconButton size="1" variant="ghost" color="gray" disabled={!hits.length} aria-label="Nächster Treffer" title="Nächster Treffer (Enter)" onClick={() => step(1)}>↓</IconButton>
          </TextField.Slot>
        )}
      </TextField.Root>
    </Flex>
  );
}
