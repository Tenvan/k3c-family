import { useEffect, type RefObject } from 'react';
import type { AssetKind, AssetSlot } from '../api';
import { assignContext } from '../chat/assignContext';
import { endSourcesChat, useAssignContext } from '../chat/chatBridge';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { stopAll } from '../sound/audio';
import { noFacets, pruneFacets, type Facets } from './filter';
import type { ViewState } from './viewState';

interface Args {
  /** Slot des Sprungs aus einer anderen Seite (leer = kein Sprung); der Sprung setzt die Filter zurück */
  jump: RefObject<string>;
  view: ViewState;
  patch: (p: Partial<ViewState>) => void;
  /** alle Daten geladen: erst dann sind Quellen-Facetten verlässlich */
  loaded: boolean;
  quellen: readonly string[];
  slot: AssetSlot | undefined;
  art: AssetKind | undefined;
  onArt?: (art: AssetKind | undefined) => void;
  onNotice: (text: string) => void;
}

/** Wirkungen der Assets-Seite: Sprung, veraltete Facetten, Abbau (Töne, Chat), Art für den Rahmen, Ticket- und Chat-Kontext. */
export function usePageEffects({ jump, view, patch, loaded, quellen, slot, art, onArt, onNotice }: Args): void {
  useEffect(() => { if (jump.current) patch({ facets: noFacets(), query: '' }); }, []); // eslint-disable-line react-hooks/exhaustive-deps
  // Eine gemerkte Quelle, die es nicht mehr gibt, würde still alles ausblenden und ließe sich in den Facetten nicht abwählen.
  useEffect(() => {
    if (!loaded) return;
    const pruned = pruneFacets(view.facets, quellen);
    if (pruned.dropped.length === 0) return;
    patch({ facets: pruned.facets });
    onNotice(`Nicht mehr vorhandene Facette entfernt: ${pruned.dropped.join(', ')}`);
  }, [loaded, view.facets, quellen]); // eslint-disable-line react-hooks/exhaustive-deps
  useEffect(() => () => { stopAll(); endSourcesChat(); }, []);
  useEffect(() => onArt?.(art), [art, onArt]);
  useTicketPage('assets', slot?.id ?? '');
  useAssignContext(assignContext(art ?? 'grafik', slot?.id ?? '', { kategorie: view.facets.kategorie.join(','), typ: '', status: view.facets.status.join(','), suche: view.query }));
}

export type { Facets };
