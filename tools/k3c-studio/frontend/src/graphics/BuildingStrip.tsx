import { UNIT_PX as UNIT } from '@game/core/constants';
import { Pixel, bildZeichnen, ladeBild, pixelContext } from './pixel';
import { baumDatei, baumReihe, gebaeudeKarten, kachelAnzahl, stufenKarten, vorlageZeilen, type GebaeudeEintrag } from './scenesLogic';

// Streifen „Neue Gebäude (GEB)“ aus grafik/gebaeude/index.json (umgezogen aus der früheren Asset-Vorschau): Baum-Reihe,
// je Gebäude 32 px ×1 neben 16 px ×2, kachelbare Grafik dreifach, Stufenreihe, Gebäude-Vorlage; alles auf der
// Bodenlinie des höchsten Gebäudes (docs/standards/gebaeude-grafik.md § 9 Punkte 3–4).

/** Bodenlinie: Höhe des höchsten Gebäudes. Hintergrund-Schichten (15 Kacheln, eigener Streifen in der Umgebung) zählen
 *  nicht mit; in der früheren Asset-Vorschau taten sie es und schoben seit GEB7 alles weit nach unten. */
const bodenOf = (index: readonly GebaeudeEintrag[]) => Math.max(...index.filter((e) => e.klasse !== 'hintergrund').map((e) => e.kacheln.h)) * UNIT;

export function BuildingStrip({ index }: { index: readonly GebaeudeEintrag[] }) {
  const byId = new Map(index.map((e) => [e.id, e]));
  const boden = bodenOf(index);
  return (
    <div className="sz-strip sz-top">
      {baumReihe(index).length > 0 && <BaumFigur index={index} d={32} />}
      {baumReihe(index).length > 0 && <BaumFigur index={index} d={16} />}
      {baumReihe(index, undefined, 2).length > 0 && <BaumFigur index={index} d={32} stufe={2} />}
      {gebaeudeKarten(index).map((k) => {
        const e = byId.get(k.id)!;
        const n = kachelAnzahl(e);
        return [
          <figure key={`${k.id}:${k.dichte}`} className="sz-fig">
            <Pixel draw={(c) => bildZeichnen(c, k.datei, k.scale, n, boden)} drawKey={`${k.datei}:${n}:${boden}`} />
            <figcaption>{k.id} · {k.dichte} px ×{k.scale}</figcaption>
            {k.dichte === 32 && <Vorlage e={e} />}
          </figure>,
          ...(k.dichte === 32 ? stufenKarten(e).map((s) => (
            <figure key={`${s.id}:s${s.stufe}`} className="sz-fig">
              <Pixel draw={(c) => bildZeichnen(c, s.datei, 1, n, boden)} drawKey={`${s.datei}:${n}:${boden}`} />
              <figcaption>{s.id} · Stufe {s.stufe} · 32 px ×1</figcaption>
              <p className="sz-change">{s.aenderung}</p>
            </figure>
          )) : []),
        ];
      })}
    </div>
  );
}

/** Gebäude-Vorlage unter der Master-Karte: Motiv als Titel, Teile und Zusatzfelder als Liste (GEB8/AC-03). */
function Vorlage({ e }: { e: GebaeudeEintrag }) {
  const v = vorlageZeilen(e);
  if (!v) return null;
  return (
    <details className="sz-vorlage" open>
      <summary>{e.id}: {v.titel}</summary>
      <ul>{v.zeilen.map((z) => <li key={z}>{z}</li>)}</ul>
    </details>
  );
}

/** Überlappende Baum-Reihe in Dichte d, auf 32 px je Kachel skaliert (GEB1/AC-05); Stufe 2 = Look Höhle (GEB6.1). */
function BaumFigur({ index, d, stufe = 1 }: { index: readonly GebaeudeEintrag[]; d: number; stufe?: number }) {
  const reihe = baumReihe(index, undefined, stufe);
  const byId = new Map(index.map((e) => [e.id, e]));
  const boden = bodenOf(index);
  const draw = async (c: HTMLCanvasElement) => {
    const hoehe = Math.max(...reihe.map((b) => byId.get(b.id)!.kacheln.h));
    const imgs = await Promise.all(reihe.map((b) => ladeBild(baumDatei(byId.get(b.id)!, stufe, d)!)));
    const g = pixelContext(c, (reihe.at(-1)!.x + 1) * UNIT, hoehe * UNIT);
    imgs.forEach((img, i) => {
      const e = byId.get(reihe[i]!.id)!;
      g?.drawImage(img, reihe[i]!.x * UNIT, (hoehe - e.kacheln.h) * UNIT, e.kacheln.b * UNIT, e.kacheln.h * UNIT);
    });
    c.style.marginTop = `${Math.max(0, boden - c.height)}px`;
  };
  return (
    <figure className="sz-fig">
      <Pixel draw={draw} drawKey={`baum:${d}:${stufe}:${reihe.map((b) => b.id).join(',')}`} />
      <figcaption>{reihe.map((b) => b.id).join(', ')} · {stufe === 2 ? 'Look Höhle · ' : ''}{d} px ×{UNIT / d}</figcaption>
    </figure>
  );
}
