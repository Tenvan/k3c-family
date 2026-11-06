import { Badge, CardButton, Flex, Strong, Text } from '@webkit/atoms';
import type { ReviewAsset, ReviewStatus } from './types';

const STATUS: Record<ReviewStatus, { text: string; farbe: 'red' | 'amber' | 'green' | 'gray' }> = {
  ungueltig: { text: 'ungültig', farbe: 'red' },
  geaendert: { text: 'geändert', farbe: 'amber' },
  neu: { text: 'neu', farbe: 'green' },
  geloescht: { text: 'gelöscht', farbe: 'gray' },
};

const wert = (v: unknown) => (typeof v === 'string' ? v : JSON.stringify(v));

/** Feldänderungen kurz: `pfad: alt → neu`. */
export const felderText = (a: ReviewAsset) => a.felder.map((f) => `${f.pfad}: ${wert(f.alt)} → ${wert(f.neu)}`).join('; ');

function Eintrag({ asset, aktiv, onSelect }: { asset: ReviewAsset; aktiv: boolean; onSelect: () => void }) {
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
        {asset.felder.length > 0 && <Text size="1" color="gray">{felderText(asset)}</Text>}
      </Flex>
    </CardButton>
  );
}

/** Liste der geänderten Assets; ungültige stehen rot mit Fehlertext, die übrigen bleiben sichtbar. */
export function ReviewList({ assets, auswahl, onSelect }: { assets: ReviewAsset[]; auswahl: string | null; onSelect: (id: string) => void }) {
  if (assets.length === 0) return <Text color="gray">Keine Asset-Änderung gegen diese Basis.</Text>;
  return (
    <Flex direction="column" gap="2" style={{ minWidth: 280, maxWidth: 360 }}>
      {assets.map((a) => (
        <Eintrag key={a.id} asset={a} aktiv={a.id === auswahl} onSelect={() => onSelect(a.id)} />
      ))}
    </Flex>
  );
}
