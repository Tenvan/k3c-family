# BS7 · DEV · Delta-Balancing: Werte-Gruppen, Baseline je Meilenstein, Basis-Läufe im Editor

- **Status:** aktiv
- **Projekt:** BAS
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-640
- **Start-Commit:** 4b78a08c
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑, Revision 1, mit den Defaults der offenen Fragen

## Ausgangslage

Nach Meilenstein 1 (BS3) ändern Baumschule (BS4) und Schiff (BS5) die Wirtschaft erneut. Das Balancing kennt eine Baseline (`balance/insights.go` › `BaselineView`) und Sensitivität (`sensitivity.go`), aber keine Werte-Gruppen, keine benannten Meilenstein-Baselines und keine Auswahl durchgelaufener Läufe im Balancing-Editor (`frontend/src/balancing/Examples.tsx` kennt nur feste Beispiele).

## Ziel

Nach jeder Mechanik-Änderung reicht eine Delta-Runde: abgenommene Werte-Gruppen bleiben eingefroren, nur die Gruppe der neuen Mechanik wird justiert, verglichen mit der Baseline des letzten Meilensteins. Durchgelaufene Läufe sind im Balancing-Editor unter „Beispiele“ als Basis-Läufe wählbar.

**Am Ende sichtbar:** Baseline `M1` gesichert; `sim_test start bots=coop2 world=standard baseline=M1` liefert einen Delta-Bericht je Gruppe; im Editor „Beispiele › Basis-Läufe › M1 übernehmen“.

## Beteiligte und Zielgruppen

Claude (Delta-Runden in BS4/BS5), 🧑 (Editor, Abnahme je Meilenstein).

## Anforderungen

B-640 › Anforderungen.

## Nicht-Ziele

Automatisches Tuning, neue Bot-Profile, Werteänderungen.

## Regeln und Einschränkungen

Abhängig von BS1 (Seed-Modus, `sim_test`-Parameter); BS7.4 zusätzlich von BS3.4 (Meilenstein-Bericht). BS7.1–BS7.3 dürfen parallel zu BS3 laufen, sofern die Domänen-Sperre DEV frei ist (BS3.3 ist DEV). Muss vor der Delta-Runde von BS4 fertig sein.

**⚠️ Workbench-Neustart:** BS7.2 und BS7.3 ändern Go-Code bzw. die eingebettete Oberfläche der Workbench → Neustart deutlich melden (Ergebnis Zeile 1, `notify_ui` warn, Chat). BS7.4 prüft vorher `workbench_status`.

## Beispiele

B-640 › Beispiele.

## Ausnahme- und Fehlerfälle

B-640 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Jedes Ziel und jeder Hebel hat genau eine Werte-Gruppe, `frozen` steuerbar; Schema-Test (B-640/AC-01).
- **AC-02** `sim_test` nimmt `baseline`; ein durchgelaufener Bericht lässt sich mit Konfiguration als Baseline unter Namen sichern; Beschreibung und `instructions.md` erklären die Delta-Runde (B-640/AC-02).
- **AC-03** Delta-Bericht: Ampel und Trend je Gruppe, eingefrorene nur beim Kippen; Hinweise nur für freie Gruppen (B-640/AC-03, AC-04).
- **AC-04** Balancing-Editor › „Beispiele“ listet Basis-Läufe; „Übernehmen“ setzt Konfiguration und Baseline (B-640/AC-05).
- **AC-05** Baseline `M1` aus dem Meilenstein-Lauf von BS3.4 ist gesichert, Gruppe `basis` eingefroren, und erscheint im Editor.

## Offene Fragen

- Zuschnitt der Werte-Gruppen (B-640), Default: `basis`, `holz`, `schiff`. Nicht blockierend.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BS7.1 | `BS7.1-werte-gruppen.md` | Umsetzung | autonom | fertig |
| BS7.2 | `BS7.2-baseline-delta-bericht.md` | Umsetzung | autonom | fertig |
| BS7.3 | `BS7.3-basis-laeufe-editor.md` | Umsetzung | autonom | fertig |
| BS7.4 | `BS7.4-review-baseline-m1.md` | Review | autonom | fertig |
| BS7.5 | `BS7.5-simulationen.md` | Umsetzung | autonom | fertig |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
