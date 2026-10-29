import { useTicketPage } from '@webkit/lib/ticketContext';
import { useState } from 'react';
import { loadPref, savePref } from '../lib/prefs';
import { StageMode } from './StageMode';
import { useGuard } from './WorldGuard';
import { MODES, WorldHeader, type Mode } from './WorldHeader';
import { WorldStructure } from './WorldStructure';
import { useChatDraft } from './useChatDraft';
import { usePlan } from './usePlan';
import { useWorldDraft } from './useWorldDraft';

/** Seite Welten Editor des Asset-Studios (B-460, B-461, B-477, B-488, B-664, B-665): zwei Modi unter einem gemeinsamen Kopf.
 *  „Welt-Struktur“ bearbeitet die Welt-Datei (Inseln, Stufen, Skalierung), „Stufe bearbeiten“ zeigt eine Stufe mit Chunk-Vorschau,
 *  Live-Vorschau und Inspektor. Der Kopf trägt die Auswahl (Welt, Insel, Stufe), Speichern und „Im Spiel öffnen“; ein Wechsel
 *  mit offenem Entwurf fragt zurück (`useGuard`). Der Modus bleibt gemerkt, eine Welt aus dem Chat öffnet Welt-Struktur. */
export function WorldPage() {
  const [mode, setModeState] = useState<Mode>(() => loadPref('world.mode', MODES, 'stufe'));
  const [maximized, setMaximized] = useState(true); // Bühne füllt die Fläche (M, ändert die Mindesthöhe der Live-Vorschau)
  const [source, setSource] = useState({ seed: 'k3c', biome: 'forest' });
  const plan = usePlan();
  const world = useWorldDraft(plan);
  const guard = useGuard(plan, world);
  const setMode = (next: Mode) => { setModeState(next); savePref('world.mode', next); };
  useChatDraft(world, () => setMode('welt'), () => (plan.dirty ? 'Antwort aus dem Chat nicht geladen: erst die Planung speichern oder verwerfen' : undefined));
  useTicketPage('welt', `${source.seed}/${source.biome}`);

  return (
    <div className="wd-root">
      <div className="wd-page" data-maximized={maximized} data-mode={mode}>
        <WorldHeader mode={mode} onMode={setMode} plan={plan} world={world} guard={guard} seed={source.seed} biome={source.biome} />
        {mode === 'welt'
          ? <WorldStructure plan={plan} world={world} guard={guard} onMode={setMode} />
          : <StageMode plan={plan} source={source} onSource={setSource} maximized={maximized} onToggleMax={() => setMaximized((m) => !m)} />}
      </div>
    </div>
  );
}
