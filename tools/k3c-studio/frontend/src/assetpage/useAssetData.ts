import { useCallback, useEffect, useMemo, useState } from 'react';
import { backend, type CatalogAsset } from '../api';
import { useAssets } from '../assets/useAssets';
import { useSources } from '../sources/useSources';
import { buildRows } from './model';

const NO_CATALOG: CatalogAsset[] = [];

/** Katalog beider Arten, einmal geladen (Quelle, Pack und Lizenz der zugeordneten Dateien); lädt bei assets:changed neu. */
function useCatalog(): { list: CatalogAsset[]; ready: boolean } {
  const [state, setState] = useState({ list: NO_CATALOG, ready: false });
  useEffect(() => {
    let live = true;
    const load = () => void Promise.all([backend.assetsCatalog('', 'grafik', 100_000), backend.assetsCatalog('', 'sound', 100_000)])
      .then(([a, b]) => live && setState({ list: [...a, ...b], ready: true }), () => live && setState({ list: NO_CATALOG, ready: true }));
    load();
    const off = backend.on('assets:changed', load);
    return () => { live = false; off(); };
  }, []);
  return state;
}

/** Daten der Assets-Seite: Slot-Register, Vorschlagsliste (Vorrat, Netz) und Katalog zu den Zeilen beider Sichten. */
export function useAssetData() {
  const assets = useAssets();
  const sources = useSources();
  const { list: catalog, ready } = useCatalog();
  const slots = assets.view?.slots;
  const proposals = sources.view?.vorschlaege;
  const rows = useMemo(() => buildRows(slots ?? [], proposals ?? [], catalog), [slots, proposals, catalog]);
  const used = useMemo(() => new Set((slots ?? []).flatMap((s) => s.wahl.map((c) => c.datei))), [slots]);
  const reloadAssets = assets.reload;
  const reloadSources = sources.reload;
  const reload = useCallback(() => { void reloadAssets(); void reloadSources(); }, [reloadAssets, reloadSources]); // stabil: useDownloads hängt einen Effekt daran
  /** loaded: alle drei Quellen geladen, erst dann sind Quellen-Facetten verlässlich. */
  const loaded = ready && !!assets.view && !!sources.view;
  return { assets, sources, rows, used, reload, loaded, offline: sources.view?.offline ?? false };
}
