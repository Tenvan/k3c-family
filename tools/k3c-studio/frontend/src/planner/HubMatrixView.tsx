import type { HubCell, HubMatrix, HubRow } from '../api';
import { depthText, levelColumns, offsetText, rowExtra, rowLabel } from './plannerLogic';

/** Zelle: leer, solange der Platz noch nicht verfügbar ist; sonst Platz und Tiefe, die erste Stufe hervorgehoben. */
function Cell({ row, cell, level, chosen }: { row: HubRow; cell: HubCell; level: number; chosen: boolean }) {
  const classes = ['pl-cell', cell.available ? 'pl-on' : 'pl-off', level === row.hubLevel ? 'pl-from' : '', chosen ? 'pl-chosen' : ''];
  const depth = cell.available ? depthText(cell) : '';
  return (
    <td className={classes.filter(Boolean).join(' ')} data-level={level}>
      {cell.available && <span className="pl-offset">{offsetText(cell.offsetUnits, row.bothSides)}</span>}
      {depth && <span className="pl-depth">{depth}</span>}
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
}

/** Matrix Hub-Stufe × Bauplatz (B-462/AC-01): Zeilen aus dem Matrix-Modell des Backends, Spalten = Hub-Stufen (beliebig
 *  viele, AC-08). Ein Klick auf eine Spalte wählt die Hub-Stufe der Live-Vorschau, ein Klick auf den Namen die Zeile. */
export function HubMatrixView({ matrix, hubLevel, onHubLevel, selected, onSelect }: MatrixProps) {
  const columns = levelColumns(matrix.levels);
  return (
    <div className="pl-scroll">
      <table className="pl-matrix" aria-label="Matrix Hub-Stufe × Bauplatz">
        <thead>
          <tr>
            <th scope="col">Bauplatz</th>
            <th scope="col">Herkunft</th>
            {columns.map((n) => (
              <th key={n} scope="col" className={n === hubLevel ? 'pl-chosen' : undefined}>
                <button type="button" className="pl-level" aria-pressed={n === hubLevel} onClick={() => onHubLevel(n)}>Hub-Stufe {n}</button>
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {matrix.rows.map((row, i) => (
            <tr key={`${row.key}#${i}`} data-key={row.key} className={rowId(row) === selected ? 'pl-selected' : undefined}>
              <th scope="row">
                <button type="button" className="pl-row" aria-pressed={rowId(row) === selected} onClick={() => onSelect?.(rowId(row))}>
                  {rowLabel(row)}
                  {rowExtra(row) && <span className="pl-extra">{rowExtra(row)}</span>}
                </button>
              </th>
              <td className="pl-source"><code>{row.source}</code></td>
              {row.cells.map((cell, j) => <Cell key={j} row={row} cell={cell} level={j + 1} chosen={j + 1 === hubLevel} />)}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
