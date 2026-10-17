import { useState } from 'react';
import { LivePreview } from '../stage/LivePreview';
import { ChunkPreview } from './ChunkPreview';
import { GameLauncher } from './GameLauncher';
import { PlanEditor } from './PlanEditor';
import { usePlan } from './usePlan';

/** Seite Welt-Betrachter des Asset-Studios (B-460): oben die Chunk-Vorschau, beim Öffnen maximiert (AC-02) und per
 *  Knopf verkleinerbar; darunter der Editor für geplante Chunks (B-461) und die Live-Vorschau mit der Spielgrafik
 *  (AC-04). Beide Vorschauen lesen dieselbe Quelle: Insel, Stufe und der Entwurf der Planung (B-461/AC-11). Ist die
 *  Chunk-Vorschau maximiert, ist der Platz für die Live-Vorschau eingeklappt. */
export function WorldPage() {
  const [maximized, setMaximized] = useState(true);
  const [source, setSource] = useState({ seed: 'k3c', biome: 'forest' });
  const plan = usePlan();
  return (
    <div className="wd-page">
      <ChunkPreview maximized={maximized} onToggle={() => setMaximized((m) => !m)} source={plan.source}
        onLoaded={(seed, biome) => setSource((s) => (s.seed === seed && s.biome === biome ? s : { seed, biome }))} />
      <PlanEditor plan={plan} />
      <GameLauncher seed={source.seed} biome={source.biome} />
      <section className="wd-live" hidden={maximized} aria-label="Live-Vorschau">
        <LivePreview seed={source.seed} biome={source.biome} active={!maximized} plan={plan.source} />
      </section>
    </div>
  );
}
