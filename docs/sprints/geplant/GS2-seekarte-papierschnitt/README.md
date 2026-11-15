# GS2 · PAP, CLI · Seekarte im Spiel als Papierschnitt-Diorama mit Schiffsfahrt

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** PAP, CLI
- **Reife:** bereit
- **Tickets:** B-964, B-977
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Seekarte am Hafen (B-962, Session WE4.2, in `main`) zeichnet Kreise und Linien (`native/k3c-game/hud/drawsea.go`). 🧑 2026-11-15 will sie als Papierschnitt-Diorama mit Schiffsfahrt (B-964). B-964 ist aus WE4 hierher verschoben und zerlegt: Assets und Diorama bleiben in B-964, die Fahrt im Vollbild ist B-977.

## Ziel

Die Seekarte im Spiel ist ein Diorama aus Papierschichten mit Insel-Miniaturen je Welt, und der Inselwechsel zeigt die Schiffsfahrt im Vollbild als Übergang.

## Beteiligte und Zielgruppen

Spielende am TV (1–2 Monarchen). Claude legt Assets über `assets_*` an und setzt um; 🧑 gibt frei, sieht die Assets im Review der Workbench und nimmt am TV ab.

## Anforderungen

- Assets und Diorama im Overlay: siehe `B-964 › Anforderungen`.
- Schiffsfahrt im Vollbild beim Inselwechsel: siehe `B-977 › Anforderungen`.

## Nicht-Ziele

Regeln der Zielwahl (B-961), Bedienung der Karte (B-962), Editor-Seekarte (B-963), Andenken und Chronik (GS3), Miniatur für „Polar“, Ton der Fahrt.

## Regeln und Einschränkungen

- Reihenfolge PAP → CLI → CLI → Review; ein Commit je Session auf `sprint/gs2`, ein PR.
- Assets nur über `assets_*` (Entscheidung 005), Fachprüfung `k3c-grafik`; HUD und Szene prüft `k3c-hud`.
- Gen-Läufe (`gen:paper`, Grafik-Golden) in GS2.1 am Feature, sonst im Abnahme-Ticket (B-742).
- Überlappt im Bereich `native/k3c-game/hud/**` mit GS1 (Einblendung): die beiden Sprints laufen nicht in derselben Welle.

- **Bereich:** `public/grafik/hintergruende/**`, `public/grafik/objekte/**`, `data/paper.json`, `native/k3c-game/hud/**`, `native/k3c-game/art/**`, `native/k3c-game/view/**`, `native/k3c-game/game/**`
- **Abhängig von:** WE4.2
- **Braucht 🧑:** Abnahme
- **Sperre:** golden

## Beispiele

Sengoku zu zweit, Insel 1 abgeschlossen: Spieler 1 öffnet am Steg das Diorama in seiner Hälfte und wählt die Bambuswälder. Beide reiten auf den Steg, die Karte übernimmt den Bildschirm, das Schiff fährt in 3 s hinüber, die neue Insel beginnt.

## Ausnahme- und Fehlerfälle

- Assets nicht gerendert → Overlay mit den bisherigen Formen, Inselwechsel mit dem bisherigen Ladebildschirm.
- Welt ohne eigene Miniatur → Miniatur von `standard`.
- Einzelheiten: `B-964`, `B-977 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Assets der Seekarte sind über `assets_*` angelegt und in `data/paper.json` › `seekarte` zugeordnet (B-964/AC-01).
- **AC-02** Das Overlay zeichnet das Diorama mit Miniatur, Größe, Schichten, Aufsatz und Zustandszeichen deterministisch; Tests aus B-962 bleiben grün (B-964/AC-02, AC-03).
- **AC-03** Rückfall auf Formen ohne Assets, keine Allokation je Frame (B-964/AC-04).
- **AC-04** Der Inselwechsel zeigt die Fahrt im Vollbild, endet nach Fahrzeit und Laden, nimmt keine Eingabe an; Laden und neues Spiel behalten den Ladebildschirm (B-977/AC-01 bis AC-04).
- **AC-05** 🧑 am TV: Diorama und Fahrt geprüft (B-964/AC-05, B-977/AC-05).

## Offene Fragen

Alle mit Default entschieden, 🧑 bestätigt bei der Freigabe:

- Die Fahrt startet mit dem Inselwechsel (alle am Steg), nicht schon mit A (B-977).
- Miniaturen für die vier vorhandenen Welten, drei neutrale Aufsätze (B-964).
- Fahrzeit 3 s, nicht abbrechbar (B-977).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GS2.1 | `GS2.1-assets-seekarte.md` | Umsetzung | autonom | offen |
| GS2.2 | `GS2.2-diorama-overlay.md` | Umsetzung | autonom | offen |
| GS2.3 | `GS2.3-schiffsfahrt-vollbild.md` | Umsetzung | autonom | offen |
| GS2.4 | `GS2.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
