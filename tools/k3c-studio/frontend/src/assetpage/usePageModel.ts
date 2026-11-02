import { useMemo } from 'react';
import type { FocusOption } from '@webkit/molecules';
import {
  activeChip, chipCount, facetGroups, fileGroup, fileItem, groupItems, matches, quellenOf, slotGroup, slotItem, suggestName,
  type Item,
} from './filter';
import type { Rows } from './model';
import { allChips, type ViewState } from './viewState';
import type { CustomChip } from './filter';

/** Abgeleitete Daten der Assets-Seite aus Zeilen und Ansicht: Chips mit Zählern, Facetten, sichtbare Gruppen beider Sichten. */
export function usePageModel(rows: Rows, view: ViewState, custom: readonly CustomChip[]) {
  const slotItems = useMemo(() => rows.slots.map(slotItem), [rows.slots]);
  const fileItems = useMemo(() => rows.files.map(fileItem), [rows.files]);
  const items: readonly Item[] = view.sicht === 'dateien' ? fileItems : slotItems;
  const chips = useMemo(() => allChips(custom), [custom]);
  const quellen = useMemo(() => quellenOf(rows.files), [rows.files]);

  const chipOptions: FocusOption[] = useMemo(
    () => chips.map((c) => ({ id: c.id, label: c.label, count: chipCount(items, c, view.query), custom: c.id.startsWith('eigen:') })),
    [chips, items, view.query],
  );
  const chip = activeChip(view.facets, chips);
  const facets = useMemo(() => facetGroups(items, view.facets, view.query, quellen), [items, view.facets, view.query, quellen]);

  const slots = useMemo(() => rows.slots.filter((_, i) => matches(slotItems[i]!, view.facets, view.query)), [rows.slots, slotItems, view.facets, view.query]);
  const files = useMemo(() => rows.files.filter((_, i) => matches(fileItems[i]!, view.facets, view.query)), [rows.files, fileItems, view.facets, view.query]);
  const slotGroups = useMemo(() => groupItems(slots, slotGroup(view.gruppe)), [slots, view.gruppe]);
  const fileGroups = useMemo(() => groupItems(files, fileGroup(view.gruppe)), [files, view.gruppe]);
  const takenNames = custom.map((c) => c.label);
  return { chips, chipOptions, chip, facets, slots, files, slotGroups, fileGroups, quellen, suggested: suggestName(view.facets, takenNames) };
}
