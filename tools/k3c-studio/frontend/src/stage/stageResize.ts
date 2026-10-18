// Größe der Phaser-Bühne an ihr Element anpassen (Fehler aus dem Live-Test, WZ17.5). Phasers ScaleManager.refresh()
// skaliert mit der zuletzt gemessenen Elterngröße; neu gemessen wird sie nur im eigenen Takt (alle 500 ms, und nur,
// solange die Game-Loop läuft). Ändert sich die Höhe der Bühne durch Zeilen darüber (Editor, Meldungen, Streifen), blieb
// das Canvas deshalb bei der alten Größe, während die Chunk-Leiste schon der neuen folgte.

/** Der Teil des ScaleManagers, den die Bühne braucht */
export interface ParentScale {
  getParentBounds(): boolean;
  refresh(): unknown;
}

/** Elterngröße neu messen, dann neu skalieren (refresh auch ohne Größenänderung: die Lage kann sich geändert haben). */
export function refreshToParent(scale: ParentScale): void {
  scale.getParentBounds();
  scale.refresh();
}
