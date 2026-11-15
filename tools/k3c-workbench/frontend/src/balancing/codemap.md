# tools/k3c-workbench/frontend/src/balancing/

## Responsibility

Seite **Balancing** im Bereich Studio (WZ28.5, Rahmen B-659, Liste und Detail B-660): eine Liste für laufende Läufe und
fertige Berichte, rechts Bericht in Reitern, Lauf, Vergleich von 2–4 Berichten, Abstimm-Ergebnis oder der Lauf-Editor.
Startet und stoppt Balancing-Läufe, bewertet und löscht Berichte, setzt Baselines.

## Design

- `BalancingPage.tsx`: Wurzel; hält Auswahl (Reihenfolge der Wahl), Häkchen, Modus (Bericht oder Editor) und die
  Voreinstellungen (`balancing.presets` über `loadJSON`/`saveJSON`); `openEditor(form, origin)` öffnet `editor/RunEditor`.
  Werkzeugleiste mit „Neuer Lauf“ und Zustand der Warteschlange.
- `BalancingLayout.tsx`: Rahmen ohne Seiten-Scroll, Liste links und Detail rechts mit Splitter (`prefsStore`); unter
  1.100 px untereinander, unter 760 px genau eine Ansicht (`BalancingView`, Container Query in `styles/balancing.css`).
- Liste: `EntryList.tsx` (Suche und Chips über ListFilter, Gruppen Läuft/Heute/Gestern/Älter, „Älter“ ab
  `OLDER_COLLAPSE = 30` eingeklappt), `EntryRow.tsx`, `SelectionBar.tsx` (Vergleichen mit `MIN_COMPARE`–`MAX_COMPARE`,
  Bewerten, Löschen mit einer Rückfrage über `git/Confirm`).
- Detail: `BalancingDetail.tsx` wählt `CompareView`, `RunDetail`, `TuningView` oder `ReportDetail`. `ReportDetail.tsx`
  hat Kopf (`DetailHead`: Name, Status, Auftrag, Aktionen, Baseline-Name wie in Go geprüft) und fünf Reiter (`TABS`:
  `OverviewTab`, `TargetsTab`, `FiguresTab`, `HintsTab`, `ConfigTab`), der Reiter bleibt gemerkt.
- `DataFiles.tsx`: Popup „Konfigurationsdateien“ mit allen Balancing-relevanten Dateien in `data/` und ihrer
  Beschreibung aus `docs/data/` (`MarkdownView`). `HelpButton.tsx`: `HelpButton` und `HelpField` (Hilfetexte aus `help.ts`).
- Reine Logik:
  - `entries.ts`: `ListEntry` aus Läufen und Berichten (ein Lauf mit Bericht erscheint nur als Bericht), Status, Art, Tagesgruppe.
  - `compare.ts`: `compareSummaries` wie `balance.Compare` in Go, `targetKey` wie in Go, `tippedRows`.
  - `export.ts`: Markdown/JSON eines Berichts und eines Vergleichs, `downloadText` (Blob).
  - `figures.ts`: Reihen für Überleben, Wartezeit, Gold und Rohstoff-Fluss.
  - `help.ts`: `FIELD_HELP`, Beispiele (`EXAMPLES`), Listen `BOTS`, `GRADES`, `WORLDS` wie in Go.
  - `spec.ts`: `BalanceForm`, `specFromForm`/`formFromSpec`/`formFromBaseline`, Voreinstellungen (`savePreset`, `dropPreset`).
  - `tune.ts`: Abstimm-Formular, `paramError`, `tuneFromForm` wie `simrun/spec.go`.
  - `variant.ts`: Hebel aus den Hinweisen als Variante (`leverValue`, `leverLine`, `leverText`).
- Hooks: `useBalancing.ts` (Läufe, Berichte, Baselines und Aktionen; kein Abfrage-Takt, lädt bei Sichtbarkeit und den
  Ereignissen `balance:runs`, `balance:reports`, `ui:reconnect`), `useSummary`, `useTuning`, `useInsights`,
  `useSummaries.ts` (mehrere Berichte für den Vergleich).
- Unterordner `editor/` (eigene codemap): Lauf-Editor.

## Flow

1. `App` hängt `BalancingPage active` dauerhaft ein -> `useBalancing(active)` -> `balanceRuns`, `balanceReports`,
   `balanceBaselines`.
2. Auswahl -> `BalancingDetail` -> `useSummary` (`balanceReport`) bzw. `useInsights` (`balanceInsights`) bzw.
   `useTuning` (`balanceTuning`); Vergleich -> `useSummaries` -> `compareSummaries` -> `CompareView`.
3. „Neuer Lauf“ / „Im Editor übernehmen“ / „Als Variante übernehmen“ -> `openEditor` -> `RunEditor.onStart(spec)` ->
   `backend.balanceStart` -> Ereignis `balance:runs` -> Liste lädt neu; `RunDetail.onStop` -> `balanceStop`.
4. Bewerten, Löschen, Baseline: `balanceReportRate`/`balanceReportsRate`, `balanceReportDelete`/`balanceReportsDelete`,
   `balanceSaveBaseline`; Befehle der Hinweise kopieren oder `openInClaude`.

## Integration

- Konsument: `src/App.tsx` (Reiter `balancing` im Bereich Studio).
- Backend: Bindings `Balance*` in `tools/k3c-workbench/app_balance.go` (Pakete `internal/balance`, `internal/simrun`,
  `internal/datadoc`).
- Abhängigkeiten: `src/api`, `src/lib/prefs`, `ui/molecules/MarkdownView`, `git/Confirm`, `@webkit/*`.
- Stile: `../styles/balancing.css`, `../styles/balancing-editor.css`.
