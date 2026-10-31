import { endSourcesChat } from '../chat/chatBridge';
import { FACET_KEYS, noFacets, pruneFacets, type FocusChip } from './filter';
import { applyChip, type ViewState } from './viewState';

type Facet = (typeof FACET_KEYS)[number];

interface Args {
  view: ViewState;
  patch: (p: Partial<ViewState>) => void;
  chips: readonly FocusChip[];
  quellen: readonly string[];
  onNotice: (text: string) => void;
}

/** Aktionen auf Fokus-Chips und Facetten: Chip wählen (veraltete Facetten fallen weg, der Chat wird wieder frei), Facette schalten, alle leeren. */
export function useViewActions({ view, patch, chips, quellen, onNotice }: Args) {
  const choose = (id: string) => {
    const found = chips.find((c) => c.id === id);
    if (!found) return;
    const applied = applyChip(view, found);
    const next = pruneFacets(applied.facets, quellen);
    patch({ ...applied, facets: next.facets });
    onNotice(next.dropped.length ? `Nicht mehr vorhandene Facette entfernt: ${next.dropped.join(', ')}` : '');
    endSourcesChat();
  };
  const toggle = (facet: string, value: string, on: boolean) => {
    const key = facet as Facet;
    patch({ facets: { ...view.facets, [key]: on ? [...view.facets[key], value] : view.facets[key].filter((v) => v !== value) } });
  };
  return { choose, toggle, clear: () => patch({ facets: noFacets() }) };
}