import { Button, Flex, IconButton, StatusBadge, Text, type Tone } from '@webkit/atoms';
import { CheckField } from '@webkit/molecules';
import type { LicenseMark } from '../api';
import { fileUrl } from '../assets/assets';
import { formatSize, MARK_LABEL } from '../lib/license';
import type { Group } from './filter';
import { stageLabel, type FileRow } from './model';

const MARK_TONE: Record<LicenseMark, Tone> = { frei: 'ok', sa: 'warn', pruefen: 'warn', gesperrt: 'error' };

interface Props {
  groups: readonly Group<FileRow>[];
  selected: string;
  picked: ReadonlySet<string>;
  offline: boolean;
  onPick: (keys: string[], on: boolean) => void;
  onSelect: (slotId: string) => void;
  onPlay: (row: FileRow) => void;
}

function Preview({ f, onPlay }: { f: FileRow; onPlay: () => void }) {
  if (!f.vorschau) return <span className="ap-thumb ap-thumb-s"><span className="ap-gap">–</span></span>;
  if (f.art === 'sound') return <IconButton size="1" variant="soft" aria-label={`${f.titel} abspielen`} onClick={onPlay}>▶</IconButton>;
  return <span className="ap-thumb ap-thumb-s"><img src={fileUrl(f.vorschau)} alt="" loading="lazy" /></span>;
}

function Row({ f, on, checked, offline, onPick, onSelect, onPlay }: { f: FileRow; on: boolean; checked: boolean; offline: boolean; onPick: (on: boolean) => void; onSelect: () => void; onPlay: () => void }) {
  const dim = f.stufe === 'netz' && (offline || !f.ladbar);
  return (
    <div className={`ap-file${on ? ' ap-file-on' : ''}`} data-testid="file-row" data-key={f.key} style={{ opacity: dim ? 0.55 : 1 }}>
      <CheckField size="1" label="" aria-label={`${f.titel} wählen`} checked={checked} disabled={!f.ladbar || offline} onCheckedChange={(c) => onPick(c === true)} />
      <Preview f={f} onPlay={onPlay} />
      <Button variant="ghost" color="gray" highContrast className="ap-file-main" onClick={onSelect} title={f.grund ?? f.datei}>
        <span className="ap-file-title">{f.titel}</span>
        <Text size="1" color="gray">{f.slotId} · {f.quelle}</Text>
      </Button>
      <StatusBadge tone={f.stufe === 'spiel' ? 'ok' : f.stufe === 'vorrat' ? 'info' : 'neutral'}>{stageLabel(f)}</StatusBadge>
      <StatusBadge tone={MARK_TONE[f.marke]}>{MARK_LABEL[f.marke]}</StatusBadge>
      <Text size="1" color="gray" className="ap-size">{formatSize(f.groesse)}</Text>
    </div>
  );
}

/** Sicht Dateien: jede Datei mit Stufe (Netz, Vorrat, Spiel, Präf. n), Lizenz-Marke und Größe; ladbare Netz-Dateien lassen sich wählen. */
export function FilesView({ groups, selected, picked, offline, onPick, onSelect, onPlay }: Props) {
  if (groups.length === 0) return <Text color="gray" className="ap-empty">Keine Dateien für diesen Filter.</Text>;
  return (
    <div className="ap-groups">
      {groups.map((g) => {
        const ladbar = g.items.filter((f) => f.ladbar).map((f) => f.key);
        const all = ladbar.length > 0 && ladbar.every((k) => picked.has(k));
        return (
          <section key={g.key} className="ap-group" data-group={g.key}>
            {g.title && (
              <Flex gap="2" align="center" className="ap-group-title">
                <Text size="2" weight="bold">{g.title}</Text>
                <Text size="1" color="gray">{g.items.length}</Text>
                {!offline && ladbar.length > 0 && <CheckField size="1" label={`ladbare wählen (${ladbar.length})`} checked={all} onCheckedChange={(c) => onPick(ladbar, c === true)} />}
              </Flex>
            )}
            {g.items.map((f) => <Row key={f.key} f={f} on={f.slotId === selected} checked={picked.has(f.key)} offline={offline} onPick={(on) => onPick([f.key], on)} onSelect={() => onSelect(f.slotId)} onPlay={() => onPlay(f)} />)}
          </section>
        );
      })}
    </div>
  );
}
