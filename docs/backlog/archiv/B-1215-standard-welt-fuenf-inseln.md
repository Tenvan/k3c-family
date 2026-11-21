# B-1215 · Die Standard-Welt besteht aus fünf Inseln mit wachsender Größe

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** WL2
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ralf 2026-11-21: Die Standard-Welt (data/worlds/standard.json) soll aus fünf Inseln bestehen, deren Größe von Insel 1 (Minimum) bis Insel 5 (Maximum) steigt.

## Ziel

Fünf Inseln island1 bis island5, je Insel alle fünf Tiefen gleich groß: sehr klein (16), klein (24), mittel (40), groß (64), mega groß (160 Chunks).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **AC-01** Standard-Welt mit fünf Inseln, Größen 16/24/40/64/160 über `chunkCounts`; Hafen in Tiefe 0 jeder Insel außer der letzten.
- **AC-02** Go-Datenbaum (`testdata/rust/data-tree.json.gz`) und Tests angepasst; Tests der Ein-Insel-Annahme nutzen eine Ein-Insel-Welt.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `world_check standard` gültig. **AC-02** Rust-Tests ohne neue rote (Stand vorher: vier vorhandene).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Annahmen: Hafen auf Insel 1 auf Platz -6 (Größe 16 erlaubt höchstens ±6), sonst -7. Geschichte der Inseln 2 bis 5 fehlt noch (Hinweis von `world_check`). Balance (Gegner, Preise je Insel) bleibt offen. Fünf Inseln mit Größen 16 bis 160 sind eine Annahme: die sechs Größen der Tabelle werden ohne `groß`/`sehr groß`-Lücke nur teils genutzt (100 fehlt).
