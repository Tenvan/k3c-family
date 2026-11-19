# B-940 · Papierschnitt-Assets für Krone, Banner, Schild, Zahlstellen, Portal-HP und Münzschauer ersetzen die gezeichneten Formen

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** RU60
- **Projekt:** RSU
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

KS5 zeichnet Krone, Monarch ohne Krone, Banner, Schild, Gruppenmarke, Portal-HP-Balken, Zahlstellen-Symbole und Münzschauer vorläufig aus Grundformen; es gibt keine Papierschnitt-Assets dafür.

## Ziel

Papierschnitt-Assets über `assets_*` (Entscheidung 005) ersetzen die gezeichneten Formen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Umgehängt auf Rust (B-1143, 2026-11-19): Das Spiel ist `rust/crates/k3c-game`; die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `public/grafik/**`, `data/paper.json`, `rust/crates/k3c-game/src/**`
- **Abhängig von:** B-934
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Assets angelegt und im Bereich Assets der Workbench geprüft.
- **AC-02** Das Spiel nutzt sie über `data/paper.json`.
- **AC-03** Sichtprüfung am TV durch 🧑.

## Offene Fragen

keine

## Notizen

Sichtprüfung 🧑 2026-11-15: Auslöser als Horn auf Pfahl vor dem Tor → B-941; Schild-Gestell an der Burg ab höherer Burgstufe → B-942 (beide KS7).
