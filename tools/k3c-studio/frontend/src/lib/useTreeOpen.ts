import { useCallback, useState } from 'react';

// Auf- und Zuklappzustand eines Baums (Katalog, Werte, Geschichte): Standard ist alles zugeklappt; `reveal` öffnet beim
// Start den Ast der schon gewählten Auswahl. `expandAll`/`collapseAll` gehören zur Leiste `TreeBar`.

export interface TreeOpen {
  isOpen: (key: string) => boolean;
  flip: (key: string) => void;
  open: (key: string) => void;
  expandAll: () => void;
  collapseAll: () => void;
}

export function useTreeOpen(allKeys: readonly string[], reveal?: readonly string[]): TreeOpen {
  const [set, setSet] = useState<ReadonlySet<string>>(() => new Set(reveal ?? []));
  const flip = useCallback((k: string) => setSet((s) => { const n = new Set(s); if (!n.delete(k)) n.add(k); return n; }), []);
  const open = useCallback((k: string) => setSet((s) => (s.has(k) ? s : new Set(s).add(k))), []);
  const expandAll = useCallback(() => setSet(new Set(allKeys)), [allKeys]);
  const collapseAll = useCallback(() => setSet(new Set()), []);
  return { isOpen: (k) => set.has(k), flip, open, expandAll, collapseAll };
}
