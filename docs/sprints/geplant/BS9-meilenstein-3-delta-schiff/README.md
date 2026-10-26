# BS9 · DEV · Meilenstein 3: Bots bauen das Schiff, Delta-Runde Schiff gegen Baseline M2

- **Status:** geplant
- **Projekt:** BAS
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-638, B-640
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach BS5 kann man das Schiff bauen und am Hafen wechseln; Bots bauen es nicht, der Bericht kennt nur „Schiff bezahlbar ab Tag“ (BS3). Baseline `M2` mit `basis` und `holz` eingefroren (BS8).

## Ziel

**Meilenstein 3:** Bots bauen das Schiff, der Bericht nennt den Bau-Tag je Abschnitt, und eine Delta-Runde justiert nur die Gruppe `schiff` gegen `M2`, bis `coop2` standard Insel 1 per Schiff verlässt; danach `M3` sichern.

**Am Ende sichtbar:** Delta-Bericht `baseline=M2`: `basis` und `holz` unverändert, `schiff` ✅, Inselwechsel im Lauf; Basis-Lauf `M3` im Editor.

## Beteiligte und Zielgruppen

Claude, 🧑 nimmt Meilenstein 3 ab.

## Anforderungen

B-638 › Anforderungen Stufe 2, B-640 › Anforderungen (Delta-Runde).

## Nicht-Ziele

Volle Werte-Runde; Client (BS6).

## Regeln und Einschränkungen

Abhängig von BS5 und BS8. Nur Hebel der Gruppe `schiff`; Kippen eingefrorener Gruppen = Befund. Alle Läufe über `sim_test`. ⚠️ Bot-/Bericht-Änderung (Go) → Workbench-Neustart deutlich melden, Delta-Lauf erst danach.

## Beispiele

B-638 › Beispiele.

## Ausnahme- und Fehlerfälle

Schiff nie fertig → Ampel rot mit fehlendem Material; Justierung nur in `schiff`.

## Akzeptanzkriterien

- **AC-01** Bots bauen Schiff-Abschnitte; Bericht nennt Bau-Tag je Abschnitt mit Ampel (B-638/AC-03).
- **AC-02** Delta-Lauf `coop2` standard `baseline=M2`: `basis`, `holz` unverändert, `schiff` grün, Insel 1 wird per Schiff verlassen.
- **AC-03** Geändert wurden nur Hebel der Gruppe `schiff`.
- **AC-04** Baseline `M3` gesichert, alle Gruppen eingefroren, im Editor sichtbar.

## Offene Fragen

keine.

## Sessions

Reife Entwurf, Stichpunkte:
- BS9.1 · DEV · Bots bauen das Schiff, Bericht Bau-Tag (AC-01)
- BS9.2 · REG · Delta-Runde `schiff` gegen `M2` (AC-02, AC-03)
- BS9.3 · Review, Baseline `M3` sichern (alle)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
