import { useState } from 'react';
import { Flex, Text } from '@webkit/atoms';
import { ChipSwitch, LoadingNote } from '@webkit/molecules';
import { figur, hintergrund, hintergruende, MONARCH, useGeladen } from './sizeData';
import { useZoom } from '../zoom';
import { SizeScene } from './SizeScene';

/** Ohne Biom: neutraler Himmel und Boden. */
const OHNE = '';
/** Zuletzt gewähltes Biom, gilt für alle Assets und beide Seiten (Katalog, Review). */
let letztesBiom = 'wald';

/** Größenvergleich (B-733): das Asset in Spielgröße am Boden, der Monarch rechts daneben, Biom als Hintergrund wählbar. */
export function SizeCompare({ id }: { id: string }) {
  const [biom, setBiom] = useState(letztesBiom);
  const asset = useGeladen(`figur|${id}`, () => figur(id));
  const monarch = useGeladen(`figur|${MONARCH}`, () => figur(MONARCH));
  const biome = useGeladen('hintergruende', hintergruende);
  const gueltig = biome.wert?.includes(biom) ? biom : OHNE;
  const hg = useGeladen(gueltig ? `hg|${gueltig}` : '', () => hintergrund(gueltig));
  const waehle = (b: string) => {
    letztesBiom = b;
    setBiom(b);
  };
  const fehler = asset.fehler ?? monarch.fehler ?? hg.fehler;
  const zoom = useZoom('groesse', '0.5');
  return (
    <Flex direction="column" gap="2" data-size-compare>
      {biome.wert && (
        <ChipSwitch label="Biom" value={gueltig} onChange={waehle}
          options={[{ value: OHNE, label: 'ohne' }, ...biome.wert.map((b) => ({ value: b, label: b }))]} />
      )}
      {zoom.schalter}
      {fehler && <Text color="red" size="2">{fehler}</Text>}
      {asset.wert && monarch.wert ? (
        <>
          <SizeScene asset={asset.wert} monarch={monarch.wert} hintergrund={gueltig ? hg.wert : undefined} zoom={zoom.faktor} />
          <Text size="1" color="gray">
            {id}: {asset.wert.quelle}, Animation {asset.wert.animation} · Monarch: {monarch.wert.quelle}
          </Text>
        </>
      ) : (
        !fehler && <LoadingNote id="size" text={`Lädt Größenvergleich ${id} …`} />
      )}
    </Flex>
  );
}
