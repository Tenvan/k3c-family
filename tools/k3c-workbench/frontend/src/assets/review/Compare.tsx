import { Box, Button, Flex, Text, ValueSlider } from '@webkit/atoms';
import { useEffect, useState } from 'react';
import { reviewApi } from './api';
import { text } from './fehlertext';
import { Boxes, pngGroesse } from './Boxes';
import type { Box as Rahmen, Frame, PixelDiff, ReviewAsset } from './types';

interface Stand {
  frame?: Frame;
  fehler?: string;
}

/** Erste Animation der Spec im Arbeitsstand; gelöschte Assets gibt es dort nicht (leer = erste der Basis, RV1.2). */
function useAnimation(asset: ReviewAsset): string | undefined {
  const [anim, setAnim] = useState<string>();
  useEffect(() => {
    let aktiv = true;
    setAnim(undefined);
    if (asset.status === 'geloescht') setAnim('');
    else reviewApi.get(asset.id).then((s) => aktiv && setAnim(Object.keys(s.animationen ?? {})[0] ?? ''), () => aktiv && setAnim(''));
    return () => {
      aktiv = false;
    };
  }, [asset.id, asset.status]);
  return anim;
}

/** Lädt einen Frame (Basis oder Arbeitsstand) und merkt den Fehlertext. */
function useFrame(id: string, anim: string | undefined, nr: number, basis: string | null): Stand {
  const [stand, setStand] = useState<Stand>({});
  useEffect(() => {
    if (anim === undefined || basis === null) return setStand({});
    let aktiv = true;
    reviewApi.frame(id, anim, nr, '', basis).then((frame) => aktiv && setStand({ frame }), (e) => aktiv && setStand({ fehler: text(e) }));
    return () => {
      aktiv = false;
    };
  }, [id, anim, nr, basis]);
  return stand;
}

function Seite({ titel, stand }: { titel: 'Vorher' | 'Nachher'; stand: Stand }) {
  return (
    <Flex direction="column" gap="1" align="center">
      <Text size="1" color="gray">{titel}</Text>
      {stand.frame && <img alt={titel} src={stand.frame.bild} style={{ maxHeight: 240, imageRendering: 'pixelated' }} />}
      {stand.fehler && <Text size="1" color="red">{stand.fehler}</Text>}
    </Flex>
  );
}

/** Überblendung, Pixel-Diff-Overlay und Rahmen auf einem gemeinsamen Bild. */
function Stapel({ vorher, nachher, blend, diff, teile }: { vorher?: Frame; nachher?: Frame; blend: number; diff?: PixelDiff; teile: [string, Rahmen][] }) {
  const basis = nachher ?? vorher;
  if (!basis) return null;
  const groesse = pngGroesse(basis.bild);
  const ebene = { position: 'absolute', inset: 0, width: '100%', height: '100%', imageRendering: 'pixelated' } as const;
  return (
    <Box position="relative" style={{ width: 'fit-content', height: 240, aspectRatio: `${groesse.b} / ${groesse.h}` }}>
      {vorher && <img alt="Vorher (Überblendung)" data-ebene="vorher" src={vorher.bild} style={ebene} />}
      {nachher && <img alt="Nachher (Überblendung)" data-ebene="nachher" src={nachher.bild} style={{ ...ebene, opacity: vorher ? blend / 100 : 1 }} />}
      {diff && <img alt="Unterschiede" data-ebene="overlay" src={diff.overlay} style={ebene} />}
      {diff && <Boxes art="diff" groesse={groesse} boxen={diff.boxen.map((b, i) => [String(i), b])} />}
      <Boxes art="teil" groesse={groesse} boxen={teile} />
    </Box>
  );
}

/** Vorher/Nachher eines Review-Eintrags mit Überblendregler, Pixel-Diff-Overlay und markierten geänderten Teilen. */
export function Compare({ asset, basis, frame = 0 }: { asset: ReviewAsset; basis: string; frame?: number }) {
  const anim = useAnimation(asset);
  const vorher = useFrame(asset.id, anim, frame, asset.status === 'neu' ? null : basis);
  const nachher = useFrame(asset.id, anim, frame, asset.status === 'geloescht' ? null : '');
  const [blend, setBlend] = useState(50);
  const [zeigeDiff, setZeigeDiff] = useState(false);
  const [diff, setDiff] = useState<PixelDiff>();
  const [diffFehler, setDiffFehler] = useState('');
  useEffect(() => {
    setDiff(undefined);
    setDiffFehler('');
    if (!zeigeDiff || anim === undefined || asset.status !== 'geaendert') return;
    let aktiv = true;
    reviewApi.pixelDiff(asset.id, anim, frame, basis).then((d) => aktiv && setDiff(d), (e) => aktiv && setDiffFehler(text(e)));
    return () => {
      aktiv = false;
    };
  }, [zeigeDiff, asset.id, asset.status, anim, frame, basis]);
  const rahmen = (nachher.frame ?? vorher.frame)?.teile ?? {};
  const teile = asset.teile.filter((t) => rahmen[t]).map((t): [string, Rahmen] => [t, rahmen[t]!]);
  return (
    <Flex direction="column" gap="3">
      <Flex gap="4" align="end">
        {asset.status !== 'neu' && <Seite titel="Vorher" stand={vorher} />}
        {asset.status !== 'geloescht' && <Seite titel="Nachher" stand={nachher} />}
      </Flex>
      <Stapel vorher={vorher.frame} nachher={nachher.frame} blend={blend} diff={zeigeDiff ? diff : undefined} teile={teile} />
      <Flex gap="3" align="center">
        <Box style={{ width: 200 }}>
          <ValueSlider aria-label="Überblendung" value={blend} min={0} max={100} step={5} onChange={setBlend} />
        </Box>
        <Button size="1" variant={zeigeDiff ? 'solid' : 'soft'} aria-pressed={zeigeDiff} disabled={asset.status !== 'geaendert'} onClick={() => setZeigeDiff((z) => !z)}>
          Unterschiede
        </Button>
        {diff && <Text size="1" color="gray">{Math.round(diff.anteil * 1000) / 10} % der Pixel</Text>}
        {diffFehler && <Text size="1" color="red">{diffFehler}</Text>}
      </Flex>
    </Flex>
  );
}
