# WI2 · SIM · Werkzeuggürtel und Handwerker: ein Beruf für alle Bauarbeiten, Reparatur mit Kosten

- **Status:** geplant
- **Projekt:** WIR
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** B-1193, B-1194, B-1195
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ein Handwerker ist heute Bauer + Hammer (B-950); die Werkstatt macht Hämmer und Schwerter; Roden und Felsen hauen übernehmen Berufe; Reparatur ist kostenlos (`docs/rules/buerger.md`, `materialien-gebaeude.md`). 🧑-Ideen 2026-11-21: Werkzeuggürtel, Handwerker für alle Bauarbeiten, Reparatur mit Kosten.

## Ziel

Der Werkzeugmacher stellt einen Werkzeuggürtel mit Hammer, Spitzhacke und Axt her; ein Bürger mit Gürtel ist Handwerker und macht alle Bauarbeiten (roden, Felsen hauen, bauen, reparieren); eine Reparatur kostet ein Grundmaterial der Ausbaustufe und zwei Gold.

## Beteiligte und Zielgruppen

🧑 gibt die Spec frei (Spielregeln) und nimmt am PC ab; Hauptrechner setzt in Rust (`k3c-sim`) um.

## Anforderungen

`B-1193`, `B-1194`, `B-1195`. Sessions (bei Aktivierung): WI2.1 Gürtel und Werkzeugmacher (B-1193), WI2.2 Handwerker macht alle Bauarbeiten (B-1194), WI2.3 Reparaturkosten (B-1195), WI2.4 Review. Reihenfolge nach Abhängigkeit: B-1194 braucht B-1193, B-1195 braucht B-1194.

## Nicht-Ziele

Tiefe Ebenen und Minen (WI3), Grafik des Gürtels (eigenes Ticket, Lane zweit), Balancing (Abnahme-Ticket).

## Regeln und Einschränkungen

Neue Mechanik nur in Rust (Entscheidung 013), Go-Engine eingefroren; deterministisch (`k3c-rng`), 2 Spieler; Spielstand-Version erhöhen (alte Stände egal, 🧑 2026-11-21); Regeltexte `buerger.md`, `materialien-gebaeude.md` und Glossar ziehen mit; Datei ≤ 400 Zeilen; Sim und Balancing gesperrt.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-sim/src/**`, `rust/crates/k3c-data/src/**`, `data/buildings.json`, `data/economy.json`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/actors/**`
- **Abhängig von:** LA3
- **Braucht 🧑:** Freigabe | Abnahme
- **Sperre:** golden | spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** (B-1193) Die Werkstatt stellt Gürtel her; Spielstand-Roundtrip grün.
- **AC-02** (B-1194) Ein Bürger mit Gürtel macht Bau, Reparatur, Rodung und Fels; ohne Gürtel nichts davon.
- **AC-03** (B-1195) Reparatur bucht 1 Grundmaterial der Stufe und 2 Gold.
- **AC-04** Abnahme 🧑 am PC: Handwerker trägt sichtbar das Werkzeug.

## Offene Fragen

🧑 muss die Spielregeln freigeben. Vorschläge (Defaults) stehen in B-1193 bis B-1195: Werkzeugmacher = die Werkstatt, Gürtel ersetzt den Einzelhammer, Bergmann und Baumeister gehen im Handwerker auf, Reparatur je Vorgang bis volle HP.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
