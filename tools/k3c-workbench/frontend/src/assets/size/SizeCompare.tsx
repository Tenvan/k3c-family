import { useState } from 'react';
import { Button, Flex, Text } from '@webkit/atoms';
import { ChipSwitch, LoadingNote } from '@webkit/molecules';
import { figur, hintergrund, hintergruende, MONARCH, stufen, stufeVon, useGeladen, type Figur, type Hintergrund, type Stufen } from './sizeData';
import { groesseresMass, SizeScene, szenenMass, type SzenenMass } from './SizeScene';

/** Ohne Biom: neutraler Himmel und Boden. */
const OHNE = '';
/** Zuletzt gewähltes Biom, gilt für alle Assets und beide Seiten (Katalog, Review). */
let letztesBiom = 'wald';

/** Lineal der Größenstufen für die Szene: Daten, Stufe des Assets und die gewählte Stufe. */
interface Lineal {
  stufen?: Stufen;
  assetStufe?: string;
  markiert?: string;
  mass?: SzenenMass;
}

interface Props {
  id: string;
  /** Basis des Reviews (B-853): zeigt das Vorher-Bild neben dem Arbeitsstand; leer = nur Arbeitsstand. */
  basis?: string;
  /** Arbeitsstand zeigen (gelöschte Assets haben keinen, B-853); Standard ja. */
  nachher?: boolean;
}

/** Eine Szene mit Überschrift; ohne Figur der Fehler oder die Lade-Anzeige. */
function Stand({ titel, id, asset, monarch, hg, lineal }: {
  titel: string; id: string; asset: { wert?: Figur; fehler?: string }; monarch?: Figur; hg?: Hintergrund; lineal: Lineal;
}) {
  return (
    <Flex direction="column" gap="1" data-size-stand={titel} className="sz-stand">
      <Text size="2" weight="bold">{titel}</Text>
      {asset.fehler && <Text color="gray" size="2">{titel}: {asset.fehler}</Text>}
      {asset.wert && monarch ? (
        <>
          <SizeScene asset={asset.wert} monarch={monarch} hintergrund={hg} {...lineal} />
          <Text size="1" color="gray">{id}: {asset.wert.quelle}, Animation {asset.wert.animation} · Monarch: {monarch.quelle}</Text>
        </>
      ) : (
        !asset.fehler && <LoadingNote id={`size-${titel}`} text={`Lädt Größenvergleich ${id} …`} />
      )}
    </Flex>
  );
}

/** Größenvergleich (B-733): das Asset in Spielgröße am Boden, der Monarch rechts daneben, Biom als Hintergrund wählbar.
 *  Mit Basis (B-853) stehen Vorher (Stand der Basis) und Nachher (Arbeitsstand) nebeneinander, beide mit dem Monarchen
 *  des Arbeitsstands als Maßstab; fehlt das Asset in der Basis, steht dort der Grund. */
export function SizeCompare({ id, basis = '', nachher = true }: Props) {
  const [biom, setBiom] = useState(letztesBiom);
  const jetzt = useGeladen(nachher ? `figur|${id}` : '', () => figur(id));
  const vorher = useGeladen(basis ? `figur|${id}|${basis}` : '', () => figur(id, basis));
  const monarch = useGeladen(`figur|${MONARCH}`, () => figur(MONARCH));
  const biome = useGeladen('hintergruende', hintergruende);
  const gueltig = biome.wert?.includes(biom) ? biom : OHNE;
  const hg = useGeladen(gueltig ? `hg|${gueltig}` : '', () => hintergrund(gueltig));
  const waehle = (b: string) => {
    letztesBiom = b;
    setBiom(b);
  };
  const fehler = monarch.fehler ?? hg.fehler;
  const st = useGeladen('stufen', stufen);
  const [markiert, setMarkiert] = useState('');
  const assetStufe = stufeVon(st.wert, id);
  // Vorher und Nachher teilen Maße und damit den Maßstab: gleiche M sehen gleich groß aus
  const mass = basis && vorher.wert && jetzt.wert && monarch.wert
    ? groesseresMass(szenenMass(vorher.wert, monarch.wert, st.wert, assetStufe, markiert || undefined), szenenMass(jetzt.wert, monarch.wert, st.wert, assetStufe, markiert || undefined))
    : undefined;
  const lineal: Lineal = { stufen: st.wert, assetStufe, markiert: markiert || undefined, mass };
  const szene = (titel: string, asset: { wert?: Figur; fehler?: string }) => (
    <Stand titel={titel} id={id} asset={asset} monarch={monarch.wert} hg={gueltig ? hg.wert : undefined} lineal={lineal} />
  );
  return (
    <Flex direction="column" gap="2" data-size-compare>
      {biome.wert && (
        <ChipSwitch label="Biom" value={gueltig} onChange={waehle}
          options={[{ value: OHNE, label: 'ohne' }, ...biome.wert.map((b) => ({ value: b, label: b }))]} />
      )}
      {fehler && <Text color="red" size="2">{fehler}</Text>}
      {st.fehler && <Text color="gray" size="1">Kein Lineal in M: {st.fehler}</Text>}
      {basis ? (
        <Flex gap="4" wrap="wrap" align="start">
          {szene(`Vorher (${basis})`, vorher)}
          {nachher && szene('Nachher', jetzt)}
        </Flex>
      ) : (
        <Einzeln id={id} asset={jetzt} monarch={monarch.wert} hg={gueltig ? hg.wert : undefined} lineal={lineal} fehler={fehler} />
      )}
      {st.wert && <Stufenwahl stufen={st.wert} eigene={lineal.assetStufe} markiert={markiert} onWahl={setMarkiert} />}
    </Flex>
  );
}

interface EinzelnProps {
  id: string;
  asset: { wert?: Figur; fehler?: string };
  monarch?: Figur;
  hg?: Hintergrund;
  lineal: Lineal;
  fehler?: string;
}

/** Ohne Basis: nur der Arbeitsstand neben dem Monarchen. */
function Einzeln({ id, asset, monarch, hg, lineal, fehler }: EinzelnProps) {
  return (
    <>
      {asset.fehler && <Text color="red" size="2">{asset.fehler}</Text>}
      {asset.wert && monarch ? (
        <>
          <SizeScene asset={asset.wert} monarch={monarch} hintergrund={hg} {...lineal} />
          <Text size="1" color="gray">{id}: {asset.wert.quelle}, Animation {asset.wert.animation} · Monarch: {monarch.quelle}</Text>
        </>
      ) : (
        !asset.fehler && !fehler && <LoadingNote id="size" text={`Lädt Größenvergleich ${id} …`} />
      )}
    </>
  );
}

interface WahlProps {
  stufen: Stufen;
  /** Stufe des Assets laut sizes.json */
  eigene?: string;
  markiert: string;
  onWahl: (id: string) => void;
}

/** Größenstufen als Wahl unter der Szene: ein Klick zeichnet die Höhe der Stufe als Linie ein (zweiter Klick nimmt sie
 *  zurück); die Stufe des Assets ist hervorgehoben. */
function Stufenwahl({ stufen, eigene, markiert, onWahl }: WahlProps) {
  return (
    <Flex gap="1" align="center" wrap="wrap" role="group" aria-label="Größenstufen" data-stufenwahl>
      <Text size="1" color="gray">Stufen (1 M = {Math.round(stufen.bezugPx)} px):</Text>
      {stufen.stufen.map((s) => (
        <Button key={s.id} size="1" variant={s.id === markiert ? 'solid' : 'soft'} color={s.id === eigene ? 'orange' : 'gray'} title={s.was}
          aria-pressed={s.id === markiert} data-stufe={s.id} onClick={() => onWahl(s.id === markiert ? '' : s.id)}>
          {s.id} · {s.m.toLocaleString('de-DE')} M
        </Button>
      ))}
    </Flex>
  );
}
