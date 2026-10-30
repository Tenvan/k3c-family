import { Button, Text } from '@webkit/atoms';
import { CheckField } from '@webkit/molecules';
import type { ReactNode } from 'react';

export type Tab = 'chunk' | 'gebaeude' | 'ansicht';
export const TABS: readonly Tab[] = ['chunk', 'gebaeude', 'ansicht'];
const LABELS: Record<Tab, string> = { chunk: 'Chunk', gebaeude: 'Gebäude', ansicht: 'Ansicht' };

/** Inspektor neben der Live-Vorschau (B-665, Glossar: Inspektor) mit den Reitern Chunk, Gebäude und Ansicht. Alle Reiter
 *  stehen im DOM (der inaktive ist versteckt), weil die Live-Vorschau ihre Bedienung in `treeHost` (Gebäude) und `layersHost`
 *  (Ansicht) einhängt. */
export function Inspector({ tab, onTab, chunk, setTreeHost, setLayersHost, band, onBand }: {
  tab: Tab; onTab: (tab: Tab) => void; chunk: ReactNode; setTreeHost: (el: HTMLElement | null) => void; setLayersHost: (el: HTMLElement | null) => void;
  band: boolean; onBand: (on: boolean) => void;
}) {
  return (
    <aside className="wd-inspector" aria-label="Inspektor">
      <div className="wd-tabs" role="tablist" aria-label="Inspektor">
        {TABS.map((t) => (
          <Button key={t} role="tab" variant="ghost" color="gray" highContrast className="wd-tab" aria-selected={t === tab} data-on={t === tab} onClick={() => onTab(t)}>{LABELS[t]}</Button>
        ))}
      </div>
      <div role="tabpanel" aria-label="Chunk" hidden={tab !== 'chunk'} className="wd-panel">{chunk}</div>
      <div role="tabpanel" aria-label="Gebäude" hidden={tab !== 'gebaeude'} className="wd-panel"><div ref={setTreeHost} /></div>
      <div role="tabpanel" aria-label="Ansicht" hidden={tab !== 'ansicht'} className="wd-panel">
        <section className="wd-card">
          <Text size="2" weight="bold">Hintergrund-Ebenen</Text>
          <div ref={setLayersHost} className="wd-layers" />
        </section>
        <section className="wd-card">
          <Text size="2" weight="bold">Anzeigen</Text>
          <CheckField label="Band mit Abschnitten über der Bühne" checked={band} onCheckedChange={(c) => onBand(c === true)} aria-label="Band über der Bühne" />
          <Text size="1" color="gray">Zoom: Strg + Rad beide Vorschauen, Alt + Rad nur die unter dem Zeiger; Pfeile scrollen, + und − zoomen.</Text>
        </section>
      </div>
    </aside>
  );
}
