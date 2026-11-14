import { Box, Button, Flex, Text, ValueSlider } from '@webkit/atoms';
import { useEffect, useState } from 'react';
import { type Animation, AnimationSync } from './AnimationSync';
import { useZoom } from '../zoom';
import { diffCache, frameCache, loadSpec } from './reviewStore';
import { Boxes, pngGroesse } from './Boxes';
import { text } from './fehlertext';
import { assetsFrame, assetsPixelDiff, type Box as Rahmen, type Frame, type PixelDiff, type ReviewAsset } from '../api';

interface Stand {
  frame?: Frame;
  fehler?: string;
}

/** Animationen der Spec im Arbeitsstand; gelöschte Assets gibt es dort nicht (leer = erste der Basis, RV1.2). */
function useAnimationen(asset: ReviewAsset): Animation[] | undefined {
  const [liste, setListe] = useState<Animation[]>();
  useEffect(() => {
    let aktiv = true;
    setListe(undefined);
    const leer = [{ name: '', frames: 1, fps: 8 }];
    if (asset.status === 'geloescht') setListe(leer);
    else
      loadSpec(asset.id).then(
        (s) => aktiv && setListe(Object.entries(s.animationen ?? {}).map(([name, a]) => ({ name, frames: a.frames, fps: a.fps ?? 8 }))),
        () => aktiv && setListe(leer),
      );
    return () => {
      aktiv = false;
    };
  }, [asset.id, asset.status]);
  return liste;
}

/** Lädt einen Frame (Basis oder Arbeitsstand) über den Cache der Ansicht; das alte Bild bleibt bis zum neuen stehen. */
function useFrame(cache: Map<string, Promise<Frame>>, id: string, anim: string | undefined, nr: number, basis: string | null): Stand {
  const [stand, setStand] = useState<Stand>({});
  useEffect(() => {
    if (anim === undefined || basis === null) return setStand({});
    const key = `${id}|${anim}|${nr}|${basis}`;
    let p = cache.get(key);
    if (!p) cache.set(key, (p = assetsFrame(id, anim, nr, '', basis)));
    let aktiv = true;
    p.then((frame) => aktiv && setStand({ frame }), (e) => aktiv && setStand({ fehler: text(e) }));
    return () => {
      aktiv = false;
    };
  }, [cache, id, anim, nr, basis]);
  return stand;
}

function Seite({ titel, stand, zoom }: { titel: 'Vorher' | 'Nachher'; stand: Stand; zoom: number }) {
  return (
    <Flex direction="column" gap="1" align="center">
      <Text size="1" color="gray">{titel}</Text>
      {stand.frame && <img alt={titel} src={stand.frame.bild} data-zoom={zoom} style={{ height: pngGroesse(stand.frame.bild).h * zoom, imageRendering: 'pixelated' }} />}
      {stand.fehler && <Text size="1" color="red">{titel === 'Vorher' ? `Frame fehlt in der Basis: ${stand.fehler}` : stand.fehler}</Text>}
    </Flex>
  );
}

/** Überblendung, Pixel-Diff-Overlay und Rahmen auf einem gemeinsamen Bild. */
interface StapelProps {
  vorher?: Frame;
  nachher?: Frame;
  blend: number;
  diff?: PixelDiff;
  teile: [string, Rahmen][];
  zoom: number;
}

function Stapel({ vorher, nachher, blend, diff, teile, zoom }: StapelProps) {
  const basis = nachher ?? vorher;
  if (!basis) return null;
  const groesse = pngGroesse(basis.bild);
  const ebene = { position: 'absolute', inset: 0, width: '100%', height: '100%', imageRendering: 'pixelated' } as const;
  return (
    <Box position="relative" style={{ width: 'fit-content', height: groesse.h * zoom, aspectRatio: `${groesse.b} / ${groesse.h}` }}>
      {vorher && <img alt="Vorher (Überblendung)" data-ebene="vorher" src={vorher.bild} style={ebene} />}
      {nachher && <img alt="Nachher (Überblendung)" data-ebene="nachher" src={nachher.bild} style={{ ...ebene, opacity: vorher ? blend / 100 : 1 }} />}
      {diff && <img alt="Unterschiede" data-ebene="overlay" src={diff.overlay} style={ebene} />}
      {diff && <Boxes art="diff" groesse={groesse} boxen={diff.boxen.map((b, i) => [String(i), b])} />}
      <Boxes art="teil" groesse={groesse} boxen={teile} />
    </Box>
  );
}

/** Pixel-Diff des aktuellen Frames, nur bei geänderten Assets und eingeschaltetem Schalter. */
function useDiff(cache: Map<string, Promise<PixelDiff>>, asset: ReviewAsset, anim: string | undefined, nr: number, basis: string, an: boolean) {
  const [diff, setDiff] = useState<{ wert?: PixelDiff; fehler?: string }>({});
  useEffect(() => {
    if (!an || anim === undefined || asset.status !== 'geaendert') return setDiff({});
    const key = `${asset.id}|${anim}|${nr}|${basis}`;
    let p = cache.get(key);
    if (!p) cache.set(key, (p = assetsPixelDiff(asset.id, anim, nr, basis)));
    let aktiv = true;
    // das alte Overlay bleibt stehen, bis das neue da ist
    p.then((wert) => aktiv && setDiff({ wert }), (e) => aktiv && setDiff({ fehler: text(e) }));
    return () => {
      aktiv = false;
    };
  }, [cache, an, asset.id, asset.status, anim, nr, basis]);
  return diff;
}

/** Zustand der Ansicht: Animation, Frame, Wiedergabe und die beiden Frames aus dem Cache. */
function useAnsicht(asset: ReviewAsset, basis: string) {
  const animationen = useAnimationen(asset);
  const [gewaehlt, setAnim] = useState<string>();
  const anim = animationen === undefined ? undefined : (gewaehlt ?? animationen[0]?.name ?? '');
  const [nr, setNr] = useState(0);
  const [spielt, setSpielt] = useState(false);
  const cache = frameCache;
  const vorher = useFrame(cache, asset.id, anim, nr, asset.status === 'neu' ? null : basis);
  const nachher = useFrame(cache, asset.id, anim, nr, asset.status === 'geloescht' ? null : '');
  return { animationen, anim, setAnim, nr, setNr, spielt, setSpielt, vorher, nachher };
}

/** Ids aus `teile`, die im gezeigten Frame eine Box haben. */
function markierteTeile(asset: ReviewAsset, frame?: Frame): [string, Rahmen][] {
  const rahmen = frame?.teile ?? {};
  return asset.teile.filter((t) => rahmen[t]).map((t): [string, Rahmen] => [t, rahmen[t]!]);
}

interface Steuerung {
  blend: number;
  onBlend: (v: number) => void;
  zeigeDiff: boolean;
  onDiff: () => void;
  diffMoeglich: boolean;
  diff: { wert?: PixelDiff; fehler?: string };
}

function Regler({ blend, onBlend, zeigeDiff, onDiff, diffMoeglich, diff }: Steuerung) {
  return (
    <Flex gap="3" align="center">
      <Box style={{ width: 200 }}>
        <ValueSlider aria-label="Überblendung" value={blend} min={0} max={100} step={5} onChange={onBlend} />
      </Box>
      <Button size="1" variant={zeigeDiff ? 'solid' : 'soft'} aria-pressed={zeigeDiff} disabled={!diffMoeglich} onClick={onDiff}>
        Unterschiede
      </Button>
      {diff.wert && <Text size="1" color="gray" style={{ fontVariantNumeric: 'tabular-nums' }}>{Math.round(diff.wert.anteil * 1000) / 10} % der Pixel</Text>}
      {diff.fehler && <Text size="1" color="red">{diff.fehler}</Text>}
    </Flex>
  );
}

/** Vorher/Nachher eines Review-Eintrags: synchron abspielbar, mit Überblendregler, Pixel-Diff-Overlay und markierten Teilen. */
export function Compare({ asset, basis }: { asset: ReviewAsset; basis: string }) {
  const a = useAnsicht(asset, basis);
  const [blend, setBlend] = useState(50);
  const [zeigeDiff, setZeigeDiff] = useState(false);
  const diffs = diffCache;
  const diff = useDiff(diffs, asset, a.anim, a.nr, basis, zeigeDiff);
  const zoom = useZoom('review', '1');
  return (
    <Flex direction="column" gap="3">
      {zoom.schalter}
      <Flex gap="4" align="end" className="as-zoom">
        {asset.status !== 'neu' && <Seite titel="Vorher" stand={a.vorher} zoom={zoom.faktor} />}
        {asset.status !== 'geloescht' && <Seite titel="Nachher" stand={a.nachher} zoom={zoom.faktor} />}
      </Flex>
      {a.animationen && a.anim && (
        <AnimationSync animationen={a.animationen} anim={a.anim} nr={a.nr} spielt={a.spielt} onAnim={a.setAnim} onNr={a.setNr} onSpielt={a.setSpielt} />
      )}
      <div className="as-zoom">
        <Stapel vorher={a.vorher.frame} nachher={a.nachher.frame} blend={blend} diff={zeigeDiff ? diff.wert : undefined}
          teile={markierteTeile(asset, a.nachher.frame ?? a.vorher.frame)} zoom={zoom.faktor} />
      </div>
      <Regler blend={blend} onBlend={setBlend} zeigeDiff={zeigeDiff} onDiff={() => setZeigeDiff((z) => !z)} diffMoeglich={asset.status === 'geaendert'} diff={diff} />
    </Flex>
  );
}
