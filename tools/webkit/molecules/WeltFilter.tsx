import { Flex, SelectField, Text } from '../atoms';
import type { WeltModus } from '../lib/weltFilter';
import { ChipSwitch } from './ChipSwitch';

export interface WeltOption {
  id: string;
  name: string;
  /** Feste Welt ohne Datei („noch nicht umgesetzt“). */
  missing?: boolean;
}

export interface WeltFilterProps {
  welten: readonly WeltOption[];
  welt: string;
  modus: WeltModus;
  onWelt: (welt: string) => void;
  onModus: (modus: WeltModus) => void;
  /** Treffer je Modus hinter den Chips; fehlt = ohne Zähler. */
  counts?: { gesamt: number; exklusiv: number };
  /** Erklärung unter dem Filter, z. B. „keine Welt-Zuordnung in den Sound-Daten“. */
  hint?: string;
}

/** Welt wählen und Modus „gesamt“ (eigene plus geerbte) oder „exklusiv“ (nur für diese Welt); ohne Welt gibt es den Modus nicht. */
export function WeltFilter({ welten, welt, modus, onWelt, onModus, counts, hint }: WeltFilterProps) {
  const bekannt = !welt || welten.some((w) => w.id === welt);
  const options = [
    { value: '', label: 'Alle Welten' },
    ...welten.map((w) => ({ value: w.id, label: w.missing ? `${w.name} (noch nicht umgesetzt)` : w.name })),
    // eine Welt aus der Adresse, die es nicht (mehr) gibt, bleibt sichtbar und abwählbar
    ...(bekannt ? [] : [{ value: welt, label: `${welt} (unbekannt)` }]),
  ];
  return (
    <Flex direction="column" gap="1" data-welt-filter>
      <Flex gap="2" align="center" wrap="wrap">
        <Text size="1" color="gray">Welt</Text>
        <SelectField size="1" aria-label="Welt" value={welt} options={options} onChange={onWelt} />
        {welt && (
          <ChipSwitch label="Modus" value={modus} onChange={onModus}
            options={[
              { value: 'gesamt', label: 'gesamt', count: counts?.gesamt },
              { value: 'exklusiv', label: 'exklusiv', count: counts?.exklusiv },
            ]} />
        )}
      </Flex>
      {hint && <Text size="1" color="gray">{hint}</Text>}
    </Flex>
  );
}
