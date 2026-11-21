# B-1193 · Der Werkzeugmacher stellt einen Werkzeuggürtel mit Hammer, Spitzhacke und Axt her

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

Die Werkstatt stellt heute Hämmer und Schwerter her (`docs/rules/materialien-gebaeude.md` § 3, Geselle in `buerger.md`); ein Handwerker ist Bauer + Hammer (B-950). Spitzhacke und Axt gibt es als Werkzeug nicht; Bergmann und Holzfäller sind Berufe mit Ausbildung. 🧑-Idee 2026-11-21: Ein Werkzeugmacher stellt einen Werkzeuggürtel her, der Hammer, Spitzhacke und Axt enthält.

## Ziel

Der Werkzeugmacher (Default: die Werkstatt mit ihrem Gesellen) stellt statt einzelner Hämmer einen Werkzeuggürtel her; ein Gürtel enthält Hammer, Spitzhacke und Axt und kostet Holz, Stein und Gold.

## Beteiligte und Zielgruppen

🧑 entscheidet über Regel und Preis; Spieler sehen den Gürtel im Regal der Werkstatt.

## Anforderungen

- Neue Mechanik nur in Rust (`k3c-sim`, Entscheidung 013), deterministisch, 2+ Spieler.
- Gürtel im Regal der Werkstatt (bis 3, wie die Hämmer heute), Preis in `data/buildings.json › workshop`.
- Spielstand-Format: Gürtel im Regal und am Bürger; Version erhöhen, alte Stände egal (🧑 2026-11-21 für Spielstände von LA3).
- Regeltexte `buerger.md`, `materialien-gebaeude.md` und Glossar ziehen mit.

## Nicht-Ziele

Was ein Bürger mit Gürtel tut (B-1194), Reparaturkosten (B-1195), Grafik des Gürtels (Lane zweit, eigenes Ticket bei der Umsetzung).

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

- **AC-01** Die Werkstatt stellt Gürtel her (Regal, Preis aus den Daten), Test in `k3c-sim`.
- **AC-02** Der Hammer-Regalplatz entfällt oder wird durch den Gürtel ersetzt; Regeltext und Glossar angepasst.
- **AC-03** Spielstand-Roundtrip mit Gürtel grün.

## Offene Fragen

Entschieden (Ralf 2026-11-21): Der Hersteller heißt **Handwerksmeister** (statt „Werkzeugmacher“) und ist die umbenannte Werkstatt (kein neues Gebäude). Weitere Defaults (autonom entscheidbar): der Gürtel ersetzt den Einzelhammer vollständig; Preis 30 Holz, 10 Stein, 3 Gold. 🧑 bestätigt bei der Freigabe. Das Schwert macht künftig der Waffenschmied (FG1, B-1200).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
