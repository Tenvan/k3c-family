import { Badge, CardButton, Flex, Strong, Text } from '@webkit/atoms';
import { felderText } from './prompt';
import type { ReviewAsset, ReviewStatus } from '../api';

const STATUS: Record<ReviewStatus, { text: string; farbe: 'red' | 'amber' | 'green' | 'gray' }> = {
  ungueltig: { text: 'ungültig', farbe: 'red' },
  geaendert: { text: 'geändert', farbe: 'amber' },
  neu: { text: 'neu', farbe: 'green' },
  geloescht: { text: 'gelöscht', farbe: 'gray' },
};


/** Zeilenhöhe von Text size="1" und Platz für Kopfzeile und Fehler über der Feldliste. */
const ZEILE = 16;
const KOPF = 64;

/** Nicht gewählte Einträge zeigen nur den Anfang der Feldliste. */
const KURZ = 2;

/** Feldliste: gewählt höchstens so hoch wie die Ansicht rechts, sonst {@link KURZ} Zeilen, darüber hinaus „…“; der volle
 *  Text steht im Tooltip. */
function Felder({ asset, aktiv, hoehe }: { asset: ReviewAsset; aktiv: boolean; hoehe: number }) {
  const text = felderText(asset);
  const lang = hoehe > 0 ? Math.max(3, Math.floor((hoehe - KOPF) / ZEILE)) : undefined;
  const zeilen = aktiv ? lang : KURZ;
  return (
    <Text size="1" color="gray" title={text} data-felder data-zeilen={zeilen} className="rv-felder" style={zeilen ? { WebkitLineClamp: zeilen } : undefined}>
      {text}
    </Text>
  );
}

function Eintrag({ asset, aktiv, onSelect, felderHoehe }: { asset: ReviewAsset; aktiv: boolean; onSelect: () => void; felderHoehe: number }) {
  const s = STATUS[asset.status];
  return (
    <CardButton data-asset={asset.id} data-status={asset.status} aria-current={aktiv} variant={aktiv ? 'classic' : 'surface'}
      style={asset.status === 'ungueltig' ? { boxShadow: 'inset 3px 0 0 var(--red-9)' } : undefined} onClick={onSelect}>
      <Flex direction="column" gap="1" align="start">
        <Flex gap="2" align="center">
          <Strong>{asset.id}</Strong>
          <Badge color={s.farbe}>{s.text}</Badge>
          {asset.stil && <Badge color="violet">Stil</Badge>}
          {asset.status === 'geaendert' && <Text size="1" color="gray">{Math.round(asset.pixelAnteil * 1000) / 10} % Pixel</Text>}
        </Flex>
        {asset.fehler?.map((f) => (
          <Text key={`${f.feld}·${f.regel}`} size="1" color="red">
            {f.text}
          </Text>
        ))}
        {asset.felder.length > 0 && <Felder asset={asset} aktiv={aktiv} hoehe={felderHoehe} />}
      </Flex>
    </CardButton>
  );
}

/** Liste der geänderten Assets; ungültige stehen rot mit Fehlertext, die übrigen bleiben sichtbar. */
interface Props {
  assets: ReviewAsset[];
  auswahl: string | null;
  onSelect: (id: string) => void;
  /** Höhe der Ansicht rechts in Pixeln (0 = unbekannt, dann ohne Grenze). */
  felderHoehe?: number;
}

export function ReviewList({ assets, auswahl, onSelect, felderHoehe = 0 }: Props) {
  if (assets.length === 0) return <Text color="gray">Keine Asset-Änderung gegen diese Basis.</Text>;
  return (
    <Flex direction="column" gap="2">
      {assets.map((a) => (
        <Eintrag key={a.id} asset={a} aktiv={a.id === auswahl} onSelect={() => onSelect(a.id)} felderHoehe={felderHoehe} />
      ))}
    </Flex>
  );
}
