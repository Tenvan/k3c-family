# BS8 · DEV · Meilenstein 2: Bots und Delta-Runde Baumschule gegen Baseline M1

- **Status:** geplant
- **Projekt:** BAS
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-636, B-640
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach BS4 gibt es die Baumschule im Server, aber Bots kennen sie nicht und ihre Werte sind Startwerte. Seit BS7 gibt es Werte-Gruppen, Baseline `M1` (Gruppe `basis` eingefroren) und den Delta-Bericht.

## Ziel

**Meilenstein 2:** Bots roden und kaufen Baumschulen, und eine Delta-Runde justiert nur die Gruppe `holz` gegen Baseline `M1`, bis `coop2` standard wieder durchläuft; danach wird `M2` gesichert und `holz` eingefroren.

**Am Ende sichtbar:** Delta-Bericht `baseline=M1`: `basis` ✅ unverändert, `holz` ✅; Basis-Lauf `M2` im Editor.

## Beteiligte und Zielgruppen

Claude, 🧑 nimmt Meilenstein 2 ab.

## Anforderungen

B-636 › Anforderungen (Bots, Werte der Baumschule), B-640 › Anforderungen (Delta-Runde).

## Nicht-Ziele

Volle Werte-Runde über alle Gruppen; Schiff (BS9).

## Regeln und Einschränkungen

Abhängig von BS4 und BS7. Nur Hebel der Gruppe `holz` ändern; kippt `basis`, ist das ein Befund (Ticket, 🧑 fragen), keine Voll-Runde. Alle Läufe über `sim_test`. ⚠️ Bot-Änderung (Go) → Workbench-Neustart deutlich melden, Delta-Lauf erst danach.

## Beispiele

Holz Tag 3 +40 % → Hinweis nennt `nursery.growSeconds`, `nursery.cost` → Anpassung → Lauf ✅.

## Ausnahme- und Fehlerfälle

`basis` kippt → Befund-Ticket, Session blockiert bis 🧑 entscheidet.

## Akzeptanzkriterien

- **AC-01** Bots (`coop2` u. a.) roden Baumschul-Plätze und kaufen Baumschulen; Unit-Test (B-636).
- **AC-02** Delta-Lauf `coop2` standard `baseline=M1`: `basis` unverändert, `holz` grün, Lauf läuft durch.
- **AC-03** Geändert wurden nur Hebel der Gruppe `holz` (Diff geprüft).
- **AC-04** Baseline `M2` gesichert, `frozen: [basis, holz]`, im Editor sichtbar.

## Offene Fragen

keine.

## Sessions

Reife Entwurf, Stichpunkte:
- BS8.1 · DEV · Bots: Baumschule roden und kaufen (AC-01)
- BS8.2 · REG · Delta-Runde `holz` gegen `M1` (AC-02, AC-03)
- BS8.3 · Review, Baseline `M2` sichern (alle)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
