import { useState } from 'react';
import { Box, Button, Flex, Heading, SelectField, Text, TextField } from '@webkit/atoms';
import { StatusBadge } from '@webkit/atoms';
import { useWeltFilter } from '@webkit/lib/useWeltFilter';
import { zuordnung, type WeltZuordnung } from '@webkit/lib/weltFilter';
import { CheckField, LoadingNote, WeltFilter } from '@webkit/molecules';
import type { AssetEintrag } from '../api';
import { useWeltSicht } from '../useWeltSicht';

interface Props {
  list: AssetEintrag[];
  /** Liste lädt noch: Lade-Anzeige statt „Keine Assets“ (WB2/AC-03). */
  loading?: boolean;
  inDiff: Set<string>;
  /** Noch ungeprüfte Einträge (B-732): Fortschritt über der Liste. */
  open?: number;
  /** „Neu laden“: Cache leeren und den Katalog neu holen (B-732). */
  onReload?: () => void;
  selected: string;
  onSelect: (id: string) => void;
}

/** Schlüssel des Assets in der Welt-Zuordnung (`<gruppe>/<asset>` wie in data/paper.json). */
const schluessel = (a: AssetEintrag) => `${a.gruppe}/${a.id}`;

const ALLE_GRUPPEN = 'Alle Gruppen';
const ALLE_ARTEN = 'Alle Arten';

/** Liste der Specs nach Gruppe; ungültige rot mit `feld · regel` und Text (005 › 7), Abzeichen „im Diff“. Filter: Text,
 *  Welt (gesamt oder exklusiv, Herkunft geerbter Assets am Eintrag), Gruppe, Art und „nur geänderte“; der Zustand steht in
 *  der Adresse (B-1169, WZ62.2). */
export function AssetList({ list, loading = false, inDiff, open = 0, onReload, selected, onSelect }: Props) {
  const [query, setQuery] = useState('');
  const [f, setF] = useWeltFilter();
  const { welten, sicht, laedt, error: weltFehler } = useWeltSicht(f.welt);
  const q = query.trim().toLowerCase();
  const zu = (a: AssetEintrag, modus = f.modus) => zuordnung(sicht, modus, schluessel(a));
  const andere = (a: AssetEintrag) =>
    (!f.geaendert || inDiff.has(a.id)) && (!f.gruppe || a.gruppe === f.gruppe) && (!f.art || a.rig === f.art) &&
    (!q || a.id.includes(q) || a.rig.includes(q) || a.gruppe.includes(q));
  const shown = laedt ? [] : list.filter((a) => andere(a) && zu(a).sichtbar);
  const counts = sicht ? { gesamt: list.filter((a) => andere(a) && zu(a, 'gesamt').sichtbar).length, exklusiv: list.filter((a) => andere(a) && zu(a, 'exklusiv').sichtbar).length } : undefined;
  const gruppenListe = [...new Set(list.map((a) => a.gruppe))].sort();
  const arten = [...new Set(list.map((a) => a.rig))].sort();
  const groups = [...new Set(shown.map((a) => a.gruppe))].sort();
  return (
    <Flex direction="column" gap="3">
      <Box className="as-list-filter">
        <Flex gap="2" align="center" mb="1">
          <Box flexGrow="1">
            <TextField.Root size="1" placeholder="Filter: ID, Rig, Gruppe" aria-label="Filter" value={query} onChange={(e) => setQuery(e.target.value)} />
          </Box>
          {onReload && <Button size="1" variant="soft" disabled={loading} onClick={onReload}>Neu laden</Button>}
        </Flex>
        <WeltFilter welten={welten} welt={f.welt} modus={f.modus} counts={counts}
          onWelt={(welt) => setF({ welt })} onModus={(modus) => setF({ modus })}
          hint={sicht?.fehlt ? 'Welt noch nicht umgesetzt: nur Basis und Einträge aus data/paper.json' : undefined} />
        <Flex gap="2" align="center" wrap="wrap" my="1">
          <SelectField size="1" aria-label="Gruppe" value={f.gruppe} onChange={(gruppe) => setF({ gruppe })}
            options={[{ value: '', label: ALLE_GRUPPEN }, ...gruppenListe.map((g) => ({ value: g, label: g }))]} />
          <SelectField size="1" aria-label="Art" value={f.art} onChange={(art) => setF({ art })}
            options={[{ value: '', label: ALLE_ARTEN }, ...arten.map((r) => ({ value: r, label: r }))]} />
        </Flex>
        {/* nur Assets mit Änderung gegen HEAD, wie das Abzeichen „im Diff“ (B-732) */}
        <CheckField size="1" label={`nur geänderte (${list.filter((a) => inDiff.has(a.id)).length})`} checked={f.geaendert} onCheckedChange={(c) => setF({ geaendert: c === true })} />
        {weltFehler && <Text as="div" size="1" color="red">{weltFehler}</Text>}
        {open > 0 && <Text as="div" size="1" color="gray" data-checking>Prüft {list.length - open}/{list.length} …</Text>}
      </Box>
      {groups.map((g) => (
        <Flex key={g} direction="column" gap="1">
          <Heading size="2" color="gray">{g}</Heading>
          {shown.filter((a) => a.gruppe === g).map((a) => (
            <Entry key={a.id} asset={a} diff={inDiff.has(a.id)} active={a.id === selected} onSelect={onSelect} welt={sicht ? zu(a) : undefined} />
          ))}
        </Flex>
      ))}
      {loading && <LoadingNote id="list" text="Lädt Assets …" />}
      {!loading && laedt && <LoadingNote id="welt" text="Lädt Welt …" />}
      {!loading && !laedt && !shown.length && <Text size="1" color="gray">Keine Assets</Text>}
    </Flex>
  );
}

function Entry({ asset, diff, active, onSelect, welt }: { asset: AssetEintrag; diff: boolean; active: boolean; onSelect: (id: string) => void; welt?: WeltZuordnung }) {
  const invalid = !asset.gueltig;
  return (
    <Box role="button" tabIndex={0} data-asset={asset.id} data-invalid={String(invalid)} aria-pressed={active}
      className={`as-entry${active ? ' as-entry-active' : ''}${invalid ? ' as-entry-invalid' : ''}`}
      onClick={() => onSelect(asset.id)} onKeyDown={(e) => (e.key === 'Enter' || e.key === ' ') && onSelect(asset.id)}>
      <Flex gap="2" align="center" wrap="wrap">
        <Text size="2" weight="bold" color={invalid ? 'red' : undefined}>{asset.id}</Text>
        <Text size="1" color="gray">{asset.rig}</Text>
        {welt?.exklusiv && <StatusBadge tone="ok">exklusiv</StatusBadge>}
        {welt?.geerbt && <StatusBadge tone="neutral">geerbt von {welt.herkunft}</StatusBadge>}
        {diff && <StatusBadge tone="info">im Diff</StatusBadge>}
        {asset.offen && <StatusBadge tone="neutral">prüft …</StatusBadge>}
        {invalid && <StatusBadge tone="error">ungültig</StatusBadge>}
      </Flex>
      {asset.fehler?.map((f) => (
        <Text key={`${f.feld}|${f.regel}`} as="div" size="1" color="red">
          {f.feld} · {f.regel}: {f.text}
        </Text>
      ))}
    </Box>
  );
}
