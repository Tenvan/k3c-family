# B-1145 · Abnahme-Läufe BB1: gen:paper und Grafik-Golden

- **Domäne:** PAP
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** PN1
- **Projekt:** NAC
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

BB1 (B-1038 bis B-1042) hat Maßstab, Waldwand, 8 Baumvarianten und 4 Leitmotive nur über Spec und `data/paper.json` umgesetzt; kein `gen:paper` und kein Grafik-Golden-Lauf (B-742). Die Golden-Einträge der neuen Assets schrieb das Tool `assets_*`.

## Ziel

Die Grafik von BB1 rendert (`gen:paper`), die Grafik-Golden stimmen, `check:race` ist grün.

## Beteiligte und Zielgruppen

Second Worker (Lane `zweit`, Rechner mit Go und C-Compiler); Abnahme von Golden-Änderungen: Ralf.

## Anforderungen

- Lauf auf dem zweiten Rechner, Umgebung live (Gen, Race).
- Golden-Änderung nur gewollt (`docs/arbeitsweise.md` › Golden aktualisieren) und mit Begründung im Commit.

## Nicht-Ziele

Sim und Balance: nicht relevant (BB1 ändert nur Grafik-Daten, keine Regeln oder Werte). Anbindung im Spiel: Sprint LA1.

## Regeln und Einschränkungen

Entscheidung 005 (Papierschnitt aus Specs), B-742 (Gen nur im Abnahme-Ticket). Planungsangaben:
- **Bereich:** `public/grafik/truppen/**`, `public/grafik/gebaeude-paper/**`, `public/grafik/objekte/**`, `public/grafik/hintergruende/**`
- **Abhängig von:** BB1
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** golden

## Beispiele

`check_run task:test` rendert die vier Gruppen und bleibt grün; ein Spec-Fehler eines der 12 Bäume oder 5 Hintergründe zeigt sich als roter Golden-Test.

## Ausnahme- und Fehlerfälle

Golden weicht ab: Ursache klären (Spec oder Renderer), nur bei gewollter Änderung `task go:golden`-Weg der Grafik nach `docs/arbeitsweise.md` › Golden aktualisieren; sonst Spec korrigieren.

## Akzeptanzkriterien

- **AC-01** Sim: nicht relevant, BB1 ändert keine Regeln oder Werte.
- **AC-02** Balance: nicht relevant, kein Wert in `data/` mit Wirkung auf Balancing.
- **AC-03** Gen: `check_run task:test` mit `gen:paper` und Grafik-Golden für `truppen`, `gebaeude-paper`, `objekte`, `hintergruende` grün.
- **AC-04** Race: `check_run task:check:race` grün (läuft ohne C-Compiler still durch: dann als übersprungen nennen).

## Offene Fragen

keine

## Notizen

Lane `zweit`. Angelegt von BB1.5.
