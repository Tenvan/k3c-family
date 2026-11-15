import { Button, VisuallyHidden } from '@webkit/atoms';
import type { HubCell, HubMatrix, HubRow } from '../api';
import { depthText, levelColumns, offsetText, rowExtra, rowLabel, rowPlace } from './plannerLogic';

/** Zelle als farbiges Kästchen (B-670): ab der Stufe des Platzes kräftig, danach hell, vorher leer; Platz und Tiefe stehen im
 *  Tooltip und für Screenreader, eine Tiefen-Bedingung markiert das Kästchen. */
function Cell({ row, cell, level, chosen }: { row: HubRow; cell: HubCell; level: number; chosen: boolean }) {
  const depth = cell.available ? depthText(cell) : '';
  const offset = cell.available ? offsetText(cell.offsetUnits, row.bothSides) : '';
  const classes = ['pl-cell', cell.available ? 'pl-on' : 'pl-off', level === row.hubLevel ? 'pl-from' : '', chosen ? 'pl-chosen' : '', depth ? 'pl-cond' : ''];
  return (
    <td className={classes.filter(Boolean).join(' ')} data-level={level} title={[offset, depth].filter(Boolean).join(', ') || undefined}>
      <span className="pl-box" aria-hidden />
      {cell.available && (
        <VisuallyHidden>
          <span className="pl-offset">{offset}</span>
          {depth && <span className="pl-depth">{depth}</span>}
        </VisuallyHidden>
      )}
    </td>
  );
}

/** Kennung einer Zeile für die Auswahl: bleibt, wenn sich Platz oder Stufe ändern (Schlüssel kind@x tut das nicht). */
export function rowId(row: Pick<HubRow, 'source' | 'index' | 'kind'>): string {
  return `${row.source}#${row.index}#${row.kind}`;
}

interface MatrixProps {
  matrix: HubMatrix;
  hubLevel: number;
  onHubLevel: (level: number) => void;
  /** gewählte Zeile (rowId) und ihre Wahl per Klick auf den Namen (WZ9.3) */
  selected?: string;
  onSelect?: (id: string) => void;
  /** „+ Platz“ im Kopf (B-670); ohne gesperrt */
  onAdd?: () => void;
}

/** Matrix Hub-Stufe × Bauplatz (B-462/AC-01): Zeilen aus dem Matrix-Modell des Backends, Spalten = Hub-Stufen (beliebig
 *  viele, AC-08). Ein Klick auf eine Spalte wählt die Hub-Stufe der Live-Vorschau, ein Klick auf den Namen die Zeile; unter
 *  dem Namen stehen Platz und Liste (B-670). */
export function HubMatrixView({ matrix, hubLevel, onHubLevel, selected, onSelect, onAdd }: MatrixProps) {
  const columns = levelColumns(matrix.levels);
  return (
    <div className="pl-scroll">
      <table className="pl-matrix" aria-label="Matrix Hub-Stufe × Bauplatz" style={{ minWidth: `calc(14rem + ${columns.length} * 2.6rem)` }}>
        <thead>
          <tr>
            <th scope="col" className="pl-head">
              Bauplatz <Button size="1" variant="soft" className="pl-add" disabled={!onAdd} onClick={onAdd}>+ Platz</Button>
            </th>
            {columns.map((n) => (
              <th key={n} scope="col" className={n === hubLevel ? 'pl-chosen' : undefined}>
                <Button size="1" variant="ghost" className="pl-level" data-chosen={n === hubLevel || undefined} aria-label={`Hub-Stufe ${n}`} aria-pressed={n === hubLevel} onClick={() => onHubLevel(n)}>{n}</Button>
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {matrix.rows.map((row, i) => (
            <tr key={`${row.key}#${i}`} data-key={row.key} className={rowId(row) === selected ? 'pl-selected' : undefined}>
              <th scope="row">
                <Button variant="ghost" color="gray" highContrast className="pl-row" aria-pressed={rowId(row) === selected} onClick={() => onSelect?.(rowId(row))}>
                  {rowLabel(row)}
                  <span className="pl-extra">{[rowPlace(row, hubLevel), rowExtra(row)].filter(Boolean).join(', ')}</span>
                </Button>
              </th>
              {row.cells.map((cell, j) => <Cell key={j} row={row} cell={cell} level={j + 1} chosen={j + 1 === hubLevel} />)}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
