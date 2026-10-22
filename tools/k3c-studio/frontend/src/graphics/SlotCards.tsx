import { CREDITS } from '@game/tools/credits';
import { renderReference, type Card } from '@game/tools/spriteReference';
import { useEffect, useRef } from 'react';
import { fileUrl } from '../assets/assets';
import { Pixel, ladeBild, pixelContext } from './pixel';
import { figureCard, GRAFIK_SCALE, licenseFlag, type Resolved } from './scenesLogic';

// Karten der Slot-Szenen (umgezogen aus der früheren Asset-Vorschau): Bildkarte im Spielmaßstab mit Ausschnitt,
// NC-Kennzeichen, Lücke und fehlende Datei markiert; Figuren über renderReference aus dem Spiel-Code.

/** Beschriftung: Slot-ID, Beschreibung, Datei, Präferenz (Vergleich) und NC-Kennzeichen (Q70) */
function Caption({ r }: { r: Resolved }) {
  const nc = r.choice && licenseFlag(r.choice.datei, CREDITS);
  return (
    <figcaption>
      <b>{r.slot.id} · {r.slot.beschreibung}</b> {nc && <span className="sz-nc">NC: nur Lückenfüller</span>}
      <br />
      <span className="sz-dim">{r.choice ? `${r.choice.datei}${r.pref > 1 ? ` · Präferenz ${r.pref}` : ''}` : `Lücke: ${r.slot.id} hat keine Zuordnung`}</span>
    </figcaption>
  );
}

/** Bild oder Ausschnitt einer Zuordnung im Spielmaßstab */
export function ImageCard({ r }: { r: Resolved }) {
  const c = r.choice;
  const draw = async (canvas: HTMLCanvasElement) => {
    if (!c) return;
    const img = await ladeBild(fileUrl(c.datei));
    const cut = c.ausschnitt ?? { x: 0, y: 0, w: img.width, h: img.height };
    pixelContext(canvas, cut.w * GRAFIK_SCALE, cut.h * GRAFIK_SCALE)?.drawImage(img, cut.x, cut.y, cut.w, cut.h, 0, 0, canvas.width, canvas.height);
  };
  return (
    <figure className="sz-fig">
      {c ? <Pixel draw={draw} drawKey={JSON.stringify(c)} /> : <div className="sz-gap">Lücke: {r.slot.id}</div>}
      <Caption r={r} />
    </figure>
  );
}

/** Figuren im selben Maßstab wie im Spiel (renderReference, Karten aus data/sprites.json). */
// renderReference startet je Aufruf eine eigene requestAnimationFrame-Schleife; die Abmeldung beendet sie beim Neuzeichnen
// und Abbau der Szene (B-471).
export function FigurenCards({ title, items }: { title: string; items: Resolved[] }) {
  const ref = useRef<HTMLDivElement>(null);
  const cards: Card[] = items.flatMap((r) => (r.choice?.sheet ? [figureCard(r.slot, { ...r.choice, sheet: r.choice.sheet })] : []));
  const key = JSON.stringify(cards);
  useEffect(() => {
    const root = ref.current;
    if (!root) return;
    root.replaceChildren();
    const stop = cards.length > 0 ? renderReference(root, [{ title, cards }], true) : undefined;
    return stop;
    // cards ändern sich mit jedem Rendern; maßgeblich ist key
  }, [key, title]);
  return <div ref={ref} className="sz-figuren" />;
}
