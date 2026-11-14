import { useState } from 'react';
import { Flex, Text } from '@webkit/atoms';
import { ChipSwitch, LoadingNote } from '@webkit/molecules';
import { figur, hintergrund, hintergruende, MONARCH, useGeladen, type Figur, type Hintergrund } from './sizeData';
import { useZoom } from '../zoom';
import { SizeScene } from './SizeScene';

/** Ohne Biom: neutraler Himmel und Boden. */
const OHNE = '';
/** Zuletzt gewähltes Biom, gilt für alle Assets und beide Seiten (Katalog, Review). */
let letztesBiom = 'wald';

interface Props {
  id: string;
  /** Basis des Reviews (B-853): zeigt das Vorher-Bild neben dem Arbeitsstand; leer = nur Arbeitsstand. */
  basis?: string;
  /** Arbeitsstand zeigen (gelöschte Assets haben keinen, B-853); Standard ja. */
  nachher?: boolean;
}

/** Eine Szene mit Überschrift; ohne Figur der Fehler oder die Lade-Anzeige. */
function Stand({ titel, id, asset, monarch, hg, zoom }: {
  titel: string; id: string; asset: { wert?: Figur; fehler?: string }; monarch?: Figur; hg?: Hintergrund; zoom: number;
}) {
  return (
    <Flex direction="column" gap="1" data-size-stand={titel}>
      <Text size="2" weight="bold">{titel}</Text>
      {asset.fehler && <Text color="gray" size="2">{titel}: {asset.fehler}</Text>}
      {asset.wert && monarch ? (
        <>
          <SizeScene asset={asset.wert} monarch={monarch} hintergrund={hg} zoom={zoom} />
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
  const zoom = useZoom('groesse', '0.5');
  const szene = (titel: string, asset: { wert?: Figur; fehler?: string }) => (
    <Stand titel={titel} id={id} asset={asset} monarch={monarch.wert} hg={gueltig ? hg.wert : undefined} zoom={zoom.faktor} />
  );
  return (
    <Flex direction="column" gap="2" data-size-compare>
      {biome.wert && (
        <ChipSwitch label="Biom" value={gueltig} onChange={waehle}
          options={[{ value: OHNE, label: 'ohne' }, ...biome.wert.map((b) => ({ value: b, label: b }))]} />
      )}
      {zoom.schalter}
      {fehler && <Text color="red" size="2">{fehler}</Text>}
      {basis ? (
        <Flex gap="4" wrap="wrap" align="start">
          {szene(`Vorher (${basis})`, vorher)}
          {nachher && szene('Nachher', jetzt)}
        </Flex>
      ) : (
        <>
          {jetzt.fehler && <Text color="red" size="2">{jetzt.fehler}</Text>}
          {jetzt.wert && monarch.wert ? (
            <>
              <SizeScene asset={jetzt.wert} monarch={monarch.wert} hintergrund={gueltig ? hg.wert : undefined} zoom={zoom.faktor} />
              <Text size="1" color="gray">{id}: {jetzt.wert.quelle}, Animation {jetzt.wert.animation} · Monarch: {monarch.wert.quelle}</Text>
            </>
          ) : (
            !jetzt.fehler && !fehler && <LoadingNote id="size" text={`Lädt Größenvergleich ${id} …`} />
          )}
        </>
      )}
    </Flex>
  );
}
