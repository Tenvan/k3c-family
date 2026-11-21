# B-1195 · Gebäude reparieren kostet ein Grundmaterial der Ausbaustufe und zwei Gold

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** ECO1
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Heute reparieren Bauern beschädigte Gebäude kostenlos zwischen den Wellen (`docs/rules/materialien-gebaeude.md` § 4). 🧑-Idee 2026-11-21: Gebäude reparieren kostet immer genau eine Ressource von den Grundmaterialien der Ausbaustufe und zwei Gold.

## Ziel

Eine Reparatur kostet immer genau **ein** Grundmaterial der aktuellen Ausbaustufe des Gebäudes und **zwei Gold**; Reparatur wird so zur Entscheidung statt zum Gratis-Heilen.

## Beteiligte und Zielgruppen

🧑 entscheidet über Kosten und Balance; Spieler zahlen an der Zahlstelle des Gebäudes.

## Anforderungen

- Kosten je Reparatur: 1 Stück des Grundmaterials der Ausbaustufe (Holz oder Stein, laut `buildings.json › levels`), 2 Gold; Material kommt wie beim Bau aus dem Insel-Vorrat.
- Fehlt Material oder Gold, wartet die Reparatur und das wird angezeigt (wie „Wartegrund“ beim Bauplatz).
- Die Reparatur selbst macht ein Handwerker (B-1194).
- Werte in den Daten (`economy.json`), nicht im Code; 2+ Spieler.

## Nicht-Ziele

Zerstörte Gebäude (Wiederaufbau bleibt wie heute), Balancing-Läufe (gesperrt).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, nur Rust.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-sim/src/building/**`, `rust/crates/k3c-sim/src/economy/**`, `data/buildings.json`, `data/economy.json`
- **Abhängig von:** B-1194
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** golden | spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Eine Reparatur bucht genau 1 Grundmaterial der Stufe und 2 Gold (Test je Material).
- **AC-02** Ohne Mittel keine Reparatur, Wartegrund wird angezeigt.
- **AC-03** Regeltext `materialien-gebaeude.md` § 4 und Glossar angepasst.

## Offene Fragen

Defaults (autonom entscheidbar): „Grundmaterial der Ausbaustufe“ = das Material, das der Bau dieser Stufe als erstes verlangt (Holz bei Holzstufe, Stein bei Steinstufe); Kosten gelten je Reparaturvorgang bis volle HP. Balancing erst im Abnahme-Ticket, nicht vorher.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
