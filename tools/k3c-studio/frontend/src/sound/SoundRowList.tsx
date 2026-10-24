import { ActionButton, Text } from '@webkit/atoms';
import { ListFilter } from '@webkit/molecules';
import { useMemo } from 'react';
import { CREDITS } from '@game/tools/credits';
import type { AssetSlot } from '../api';
import { licenseFlag } from '../graphics/scenesLogic';
import { loadJSON, saveJSON } from '../lib/prefs';
import { HERKUNFT_LABEL, licenseText, rowFilter, soundRows, type RowScene, type SoundRow } from './soundRows';

// Zeilen der Szenen Skills, Gegner und Bosse, Bauen, Reiten, UI und Warnungen (WZ30.2, B-597/AC-01, AC-02): je Zeile Slot,
// Datei, Lizenz, Herkunft und Abspielen; Lücken stehen als Hinweis da statt als Fehler.

export function SoundRowList({ szene, slots, onPlay }: { szene: RowScene; slots: readonly AssetSlot[]; onPlay: (r: SoundRow, far: boolean) => Promise<void> }) {
  const rows = useMemo(() => soundRows(szene, slots), [szene, slots]);
  const filter = useMemo(() => rowFilter(rows, CREDITS), [rows]);
  return (
    <ListFilter
      key={szene} items={rows} {...filter} itemKey={(r) => r.key} renderItem={(r) => <Row r={r} onPlay={onPlay} />}
      storageKey={`sound.${szene}.filter`} prefs={{ loadJSON, saveJSON }} emptyHint="Keine Einträge in dieser Szene."
    />
  );
}

function Row({ r, onPlay }: { r: SoundRow; onPlay: (r: SoundRow, far: boolean) => Promise<void> }) {
  const nc = r.choice && licenseFlag(r.choice.datei, CREDITS);
  const where = r.slot ? r.slot.id : 'kein Slot';
  return (
    <div className="sd-row">
      <span className="sd-cap">
        <b>{r.titel}</b> <span className="sz-dim">{r.linie ? `${r.gruppe} · ${r.art} · ` : ''}{where} · {HERKUNFT_LABEL[r.herkunft]}</span>
        {nc && <span className="sz-nc"> NC: nur Lückenfüller</span>}
        <br />
        <span className="sz-dim">
          {r.choice
            ? `${r.choice.datei}${r.choice.sprite ? `#${r.choice.sprite}` : ''} · Lizenz ${licenseText(r.choice.datei, CREDITS)}${r.global ? ' · senkt die Musik' : ''}`
            : r.herkunft === 'still' ? 'still: kein Ton für diese Zeile' : `Lücke: ${where} hat keine Zuordnung`}
        </span>
      </span>
      {r.choice ? <ActionButton size="1" onClick={() => onPlay(r, false)}>Abspielen</ActionButton> : <Text size="1" color="gray">kein Ton</Text>}
      {r.choice?.sprite && <ActionButton size="1" variant="soft" onClick={() => onPlay(r, true)}>fern</ActionButton>}
    </div>
  );
}
