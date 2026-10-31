import { Button, Flex, IconButton, Text, type Tone } from '@webkit/atoms';
import type { AssetStatus } from '../api';
import { fileUrl, typOf } from '../assets/assets';
import type { Group } from './filter';
import { stageCounts, type SlotRow } from './model';

const TONE: Record<AssetStatus, Tone> = { zugeordnet: 'ok', luecke: 'warn', fehlt: 'error' };
const LABEL: Record<AssetStatus, string> = { zugeordnet: 'aktiv', luecke: 'Lücke', fehlt: 'Datei fehlt' };

interface Props {
  groups: readonly Group<SlotRow>[];
  selected: string;
  onSelect: (id: string) => void;
  onPlay: (row: SlotRow) => void;
}

/** Hinweis „2 im Netz · 1 im Vorrat“ unter dem Slot (leer ohne Kandidaten). */
function Stages({ row }: { row: SlotRow }) {
  const { netz, vorrat } = stageCounts(row);
  const parts = [netz > 0 && `${netz} im Netz`, vorrat > 0 && `${vorrat} im Vorrat`].filter(Boolean);
  return parts.length ? <Text size="1" color="gray" className="ap-stages">{parts.join(' · ')}</Text> : null;
}

function Dot({ status }: { status: AssetStatus }) {
  return <span className={`as-dot as-dot-${TONE[status]}`} role="img" aria-label={LABEL[status]} title={LABEL[status]} />;
}

/** Grafik-Slot als Karte mit Bild der aktiven Zuordnung. */
function Card({ row, on, onSelect }: { row: SlotRow; on: boolean; onSelect: () => void }) {
  const first = row.slot.wahl[0];
  return (
    <Button variant={on ? 'soft' : 'outline'} color={on ? undefined : 'gray'} highContrast={!on} className="ap-card" aria-pressed={on} onClick={onSelect} title={`${row.slot.beschreibung}${first ? `\n${first.datei}` : ''}`}>
      <span className="ap-thumb">{first ? <img src={fileUrl(first.datei)} alt="" loading="lazy" /> : <span className="ap-gap">Lücke</span>}</span>
      <span className="ap-card-id"><Dot status={row.slot.status} />{row.slot.id}</span>
      <Stages row={row} />
    </Button>
  );
}

/** Sound-Slot als Zeile mit ▶ für die aktive Zuordnung. */
function Line({ row, on, onSelect, onPlay }: { row: SlotRow; on: boolean; onSelect: () => void; onPlay: () => void }) {
  const first = row.slot.wahl[0];
  return (
    <Flex gap="1" align="center" className="ap-line">
      <IconButton size="1" variant="soft" disabled={!first} aria-label={`${row.slot.id} abspielen`} onClick={onPlay}>▶</IconButton>
      <Button variant={on ? 'soft' : 'ghost'} color={on ? undefined : 'gray'} highContrast={!on} className="ap-line-main" aria-pressed={on} onClick={onSelect} title={row.slot.beschreibung}>
        <Dot status={row.slot.status} />
        <span className="as-row-id">{row.slot.id}</span>
        <span className="as-row-desc">{typOf(row.slot)}{row.slot.wahl.length > 1 ? ` · ${row.slot.wahl.length} Präf.` : ''}</span>
        <Stages row={row} />
      </Button>
    </Flex>
  );
}

/** Sicht Slots: Grafik als Raster von Karten, Sound als Liste mit ▶ in der Zeile; gruppiert nach der gewählten Gruppierung. */
export function SlotsView({ groups, selected, onSelect, onPlay }: Props) {
  if (groups.length === 0) return <Text color="gray" className="ap-empty">Keine Asset-Slots für diesen Filter.</Text>;
  return (
    <div className="ap-groups" role="listbox" aria-label="Asset-Slots">
      {groups.map((g) => {
        const cards = g.items.filter((r) => r.slot.art === 'grafik');
        const lines = g.items.filter((r) => r.slot.art === 'sound');
        return (
          <section key={g.key} className="ap-group" data-group={g.key}>
            {g.title && <Text as="div" size="2" weight="bold" className="ap-group-title">{g.title} <Text size="1" color="gray">{g.items.length}</Text></Text>}
            {cards.length > 0 && <div className="ap-cards">{cards.map((r) => <Card key={r.slot.id} row={r} on={r.slot.id === selected} onSelect={() => onSelect(r.slot.id)} />)}</div>}
            {lines.length > 0 && <div className="ap-lines">{lines.map((r) => <Line key={r.slot.id} row={r} on={r.slot.id === selected} onSelect={() => onSelect(r.slot.id)} onPlay={() => onPlay(r)} />)}</div>}
          </section>
        );
      })}
    </div>
  );
}
