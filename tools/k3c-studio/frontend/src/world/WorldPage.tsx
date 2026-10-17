import { useState } from 'react';
import { ChunkPreview } from './ChunkPreview';

/** Seite Welt-Betrachter des Asset-Studios (B-460): oben die Chunk-Vorschau, beim Öffnen maximiert (AC-02) und per
 *  Knopf verkleinerbar; darunter folgt die Live-Vorschau mit der Spielgrafik (WZ7.3). Ist die Chunk-Vorschau maximiert,
 *  ist der Platz darunter eingeklappt. */
export function WorldPage() {
  const [maximized, setMaximized] = useState(true);
  return (
    <div className="wd-page">
      <ChunkPreview maximized={maximized} onToggle={() => setMaximized((m) => !m)} />
      <section className="wd-live" hidden={maximized} aria-label="Live-Vorschau">
        <p className="wd-note">Die Live-Vorschau mit der Spielgrafik folgt (WZ7.3).</p>
      </section>
    </div>
  );
}
