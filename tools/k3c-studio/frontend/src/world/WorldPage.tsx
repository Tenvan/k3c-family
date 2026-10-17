import { useState } from 'react';
import { LivePreview } from '../stage/LivePreview';
import { ChunkPreview } from './ChunkPreview';

/** Seite Welt-Betrachter des Asset-Studios (B-460): oben die Chunk-Vorschau, beim Öffnen maximiert (AC-02) und per
 *  Knopf verkleinerbar; darunter die Live-Vorschau mit der Spielgrafik (AC-04) für dasselbe Level. Ist die
 *  Chunk-Vorschau maximiert, ist der Platz darunter eingeklappt. */
export function WorldPage() {
  const [maximized, setMaximized] = useState(true);
  const [source, setSource] = useState({ seed: 'k3c', biome: 'forest' });
  return (
    <div className="wd-page">
      <ChunkPreview maximized={maximized} onToggle={() => setMaximized((m) => !m)} onLoaded={(seed, biome) => setSource((s) => (s.seed === seed && s.biome === biome ? s : { seed, biome }))} />
      <section className="wd-live" hidden={maximized} aria-label="Live-Vorschau">
        <LivePreview seed={source.seed} biome={source.biome} active={!maximized} />
      </section>
    </div>
  );
}
