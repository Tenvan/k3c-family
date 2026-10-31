import { licenseMark } from '../lib/license';
import type { AssetCandidate, CandidateApi } from './candidateTypes';
import type { SourceApi, SourceProposal } from './sourceTypes';
import type { AssetChoice, AssetHit, AssetSlot, Backend, CatalogAsset } from './types';

// Kandidaten und Zuordnen im Mock (B-677): setzt die Liste aus Zuordnungen, Katalog, Vorschlägen und Recherche-Treffern der
// anderen Mocks zusammen und führt „Zuordnen“ als einen Aufruf aus (Laden, Übernehmen, Zuordnen) wie das Go-Backend.

type Assets = Pick<Backend, 'assetsList' | 'assetsSet' | 'assetsHits' | 'assetsCatalog' | 'assetsDownload'>;

const STEP_LIMIT = 20;
const blockedFlag = (flag?: string): boolean => flag === 'ND' || flag === 'NC';
const fileName = (datei: string): string => datei.split('/').pop() ?? datei;

function fromProposal(p: SourceProposal): AssetCandidate {
  const marke = !p.lizenzErlaubt || p.status === 'gesperrt' ? 'gesperrt' : licenseMark(p.lizenz);
  const loaded = p.status === 'geladen';
  return {
    id: `sug:${p.id}`, stufe: loaded ? 'vorrat' : 'netz', praef: 0, titel: p.titel, datei: p.dateiname, quelle: p.quelle, urheber: p.urheber, lizenz: p.lizenz, marke,
    ...(marke === 'gesperrt' && { grund: p.sperrgrund ?? `Lizenz ${p.lizenz} nicht erlaubt (Q70)` }), groesse: p.groesse, vorschau: loaded ? (p.vorschau ?? '') : '',
    link: p.seite, herkunft: loaded ? ['vorrat', 'vorschlag'] : ['vorschlag'], zuordenbar: marke !== 'gesperrt' && p.status !== 'verworfen',
  };
}

function fromHit(h: AssetHit): AssetCandidate | undefined {
  if (h.lizenzStatus === 'ausgeschlossen' || h.datei) return undefined;
  const check = h.lizenzStatus === 'pruefen';
  const direct = !!h.download;
  const marke = check ? 'pruefen' : licenseMark(h.lizenz ?? '', h.kennzeichen);
  return {
    id: `hit:${h.id}`, stufe: 'netz', praef: 0, titel: h.titel, datei: h.titel, quelle: h.quelle, urheber: h.urheber, lizenz: h.lizenz ?? '', marke,
    ...(!direct || check ? { grund: check ? 'Lizenz prüfen' : 'kein direkter Download, Seite öffnen' } : {}), groesse: 0, vorschau: h.vorschau ?? '', link: h.link,
    herkunft: ['recherche'], zuordenbar: direct && !check && marke !== 'gesperrt', ...(check && { lizenzBestaetigen: true }),
  };
}

/** Katalog-Datei als Kandidat; Zuordnungen des Slots tragen ihre Präferenz und sind nicht mehr zuordenbar. */
function fromCatalog(x: CatalogAsset, praef = 0): AssetCandidate {
  const blocked = blockedFlag(x.kennzeichen);
  return {
    id: praef > 0 ? `wahl:${x.datei}` : `cat:${x.datei}`, stufe: 'spiel', praef, titel: x.titel ?? fileName(x.datei), datei: x.datei, quelle: x.pack, urheber: x.urheber, lizenz: x.lizenz,
    marke: licenseMark(x.lizenz, x.kennzeichen), ...(blocked && { grund: `Lizenz ${x.kennzeichen} nicht erlaubt (Q70)` }), groesse: 0, vorschau: x.datei, herkunft: ['katalog'],
    zuordenbar: praef === 0 && !blocked,
  };
}

/** Zuordnungen des Slots in Präferenz-Reihenfolge; fehlt die Datei im Katalog, bleibt eine Zeile ohne Lizenz. */
function choiceCandidates(slot: AssetSlot, catalog: readonly CatalogAsset[]): AssetCandidate[] {
  return slot.wahl.map((c, i) => fromCatalog(catalog.find((x) => x.datei === c.datei) ?? { datei: c.datei, art: slot.art, pack: '', lizenz: '' }, i + 1));
}

/** Recherche-Treffer und Vorschläge: derselbe Download erscheint einmal mit beiden Herkünften. */
function netCandidates(proposals: readonly SourceProposal[], hits: readonly AssetHit[]): AssetCandidate[] {
  const out = [...proposals.filter((p) => p.status === 'geladen'), ...proposals.filter((p) => p.status !== 'geladen')].map(fromProposal);
  for (const h of hits) {
    const c = fromHit(h);
    const same = h.download ? proposals.find((p) => p.dateiUrl === h.download) : undefined;
    const dup = same ? out.find((o) => o.id === `sug:${same.id}`) : undefined;
    if (dup) dup.herkunft = [...dup.herkunft, 'recherche'];
    else if (c) out.push(c);
  }
  return out;
}

/** Katalog-Datei direkt über ihren Schlüssel (wie Go): unabhängig davon, mit welcher Suche sie gefunden wurde. */
async function catalogCandidate(a: Assets, art: string, datei: string): Promise<AssetCandidate | undefined> {
  const x = (await a.assetsCatalog(datei, art, 20)).find((c) => c.datei === datei);
  return x && fromCatalog(x);
}

export function mockCandidates(a: Assets, s: SourceApi): CandidateApi {
  const slotOf = async (slotId: string) => {
    const list = await a.assetsList();
    const slot = list.slots.find((x) => x.id === slotId);
    if (!slot) throw new Error(`unbekannter Asset-Slot "${slotId}"`);
    return { slot, stand: list.stand };
  };
  const view: CandidateApi['assetCandidates'] = async (slotId, query, limit) => {
    const { slot, stand } = await slotOf(slotId);
    const catalog = await a.assetsCatalog(query || slot.beschreibung.split(':').pop()?.trim() || slot.id, slot.art, 400);
    const mine = new Set(slot.wahl.map((c) => c.datei));
    const proposals = (await s.sources()).vorschlaege.filter((p) => p.slot === slotId && p.status !== 'verworfen' && !p.verwendet);
    const out = [
      ...choiceCandidates(slot, catalog),
      ...netCandidates(proposals, await a.assetsHits(slotId)),
      ...catalog.filter((x) => !mine.has(x.datei)).slice(0, limit > 0 ? limit : 60).map((x) => fromCatalog(x)),
    ];
    const offline = (await s.network()).offline;
    return { offline, stand, kandidaten: out.map((c) => (offline && c.stufe === 'netz' ? { ...c, zuordenbar: false, grund: c.grund ?? 'offline' } : c)) };
  };
  /** Netz-Vorschlag: laden (Fortschritt abfragen, bis fertig), übernehmen und als Datei zuordnen. */
  const load = async (p: SourceProposal): Promise<string> => {
    await s.sourceDownload([p.id], p.groesse);
    for (let i = 0; i < STEP_LIMIT; i += 1) {
      const job = (await s.sourceProgress()).find((j) => j.id === p.id);
      if (job?.status === 'fehler') throw new Error(job.fehler ?? 'Download fehlgeschlagen');
      if (job?.status === 'fertig') break;
    }
    await s.sourceAdopt(p.id);
    return ((await s.sources()).vorschlaege.find((x) => x.id === p.id)?.vorschau) ?? '';
  };
  /** Datei eines Vorschlags: im Vorrat vorhanden oder nach dem Laden; Netz-Vorschläge brauchen die bestätigte Größe. */
  const fileOf = async (id: string, now: AssetCandidate, confirmedBytes: number): Promise<string> => {
    const p = (await s.sources()).vorschlaege.find((x) => `sug:${x.id}` === id);
    if (!p) throw new Error(`Vorschlag ${id} unbekannt`);
    if (now.stufe === 'netz' && p.groesse !== confirmedBytes) throw new Error('bestätigt weicht von der Größe ab: bitte neu bestätigen');
    return (p.status === 'geladen' ? p.vorschau : undefined) ?? (await load(p));
  };
  const resolve = async (slot: AssetSlot, id: string): Promise<AssetCandidate> => {
    const now = id.startsWith('cat:') ? await catalogCandidate(a, slot.art, id.slice(4)) : (await view(slot.id, '', 400)).kandidaten.find((c) => c.id === id);
    if (!now) throw new Error(`Kandidat ${id} unbekannt`);
    if (!now.zuordenbar) throw new Error(now.grund ?? 'nicht zuordenbar');
    return now;
  };
  return {
    assetCandidates: view,
    assetCandidateAssign: async (slotId, id, stand, confirmedBytes) => {
      const { slot, stand: seen } = await slotOf(slotId);
      if (stand !== seen) throw new Error('Register geändert, bitte erneut speichern');
      const now = await resolve(slot, id);
      if (id.startsWith('hit:')) {
        await a.assetsDownload(slotId, id.slice(4)); // hängt die Datei im Mock selbst an
        const after = await slotOf(slotId);
        return { slot: after.slot, stand: after.stand };
      }
      const datei = id.startsWith('sug:') ? await fileOf(id, now, confirmedBytes) : id.slice(4);
      const wahl: AssetChoice[] = [...slot.wahl, { datei }];
      return a.assetsSet(slotId, wahl, (await slotOf(slotId)).stand);
    },
  };
}
