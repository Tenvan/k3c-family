# LA2 · CLI, PAP · Look-Nachträge: Cheat-Panel für Umgebungseffekte, Wolken und Wellenstriche, Licht-Assets

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI, PAP
- **Reife:** bereit
- **Tickets:** B-1179, B-1181, B-1183
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude (2026-11-21, Auftrag von 🧑)

## Ausgangslage

LA1 hat Licht, Himmel und Fackeln im Rust-Spiel angepasst (PR #795). Offen blieben die Teile, die Grafik oder das Cheat-Fenster brauchen: ein Panel für die Umgebungseffekte (B-1179), Wolken, Wellenstriche und ein Monumentalbild (B-1183) und echte Licht-Assets (B-1181). Alle Sessions laufen beim Second Worker (Lane zweit, 🧑 2026-11-21: Asset-Änderungen immer für den Second Worker).

## Ziel

Das Cheat-Fenster schaltet die Umgebungseffekte live, der Himmel hat Wolken, das Wasser ruhigere Wellenstriche, jede feste Welt ein Monumentalbild, und Lichtquellen sind animierte Assets mit Ankerpunkten an den Gebäuden. Am Ende sichtbar: am PC nachts und tags im Hub jeder festen Welt.

## Beteiligte und Zielgruppen

Second Worker setzt autonom um; 🧑 nimmt am PC ab und entscheidet Look-Fragen, die die Sessions mit Default beantworten und im Ergebnis melden.

## Anforderungen

`B-1179 › Anforderungen`, `B-1183 › Anforderungen`, `B-1181 › Anforderungen`. Sprint-eigen: Asset-Änderungen nur über MCP `assets_*` (Entscheidung 005), Anbindung im Spiel in den Dateien dieses Sprints.

## Nicht-Ziele

Neue Wetter- oder Lichtmechanik, Regeln, Bauplatz-Abstände (LA3, B-1182).

## Regeln und Einschränkungen

Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; 2 Spieler; Frame-Budget (60 Hz, schlechtester Frame); Go-Golden werden nie zugunsten von Rust geändert; Gen und Bildvergleiche laufen in diesen Sessions (Lane zweit), `check:race` im Abnahme-Ticket.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-devproto/**`, `rust/crates/k3c-devwin/**`, `rust/crates/k3c-game/src/devwin/**`, `rust/crates/k3c-game/src/game/devwinhost.rs`, `rust/crates/k3c-game/src/world/**`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-platform/**`, `public/grafik/**`, `data/paper.json`, `data/look.json`
- **Abhängig von:** LA1
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

Nacht im Hub Sengoku: Papierlaterne mit animierter Flamme neben der Tür, Vulkan als Monumentalbild am Horizont, Wolken am Himmel.

## Ausnahme- und Fehlerfälle

Biome ohne Himmel (`tint.noSky`): keine Wolken, kein Monumentalbild. Ohne Cheat-Fenster keine Mehrarbeit je Takt.

## Akzeptanzkriterien

- **AC-01** (B-1179/AC-01) Das Cheat-Fenster zeigt je Effekt aus `fx::EFFECTS` einen Schalter, Umschalten wirkt im nächsten Bild (Test mit Mock-Host).
- **AC-02** (B-1183/AC-01) Wolken im oberen Himmel bei Tag und Abend, feste Lage (Test), Aufnahme zeigt sie.
- **AC-03** (B-1183/AC-03) Wellenstriche nachts ruhiger, Go-Golden unverändert.
- **AC-04** (B-1183/AC-02) Je feste Welt ein Monumentalbild als Asset angebunden.
- **AC-05** (B-1181/AC-01, AC-02) Je Welt ein animiertes Flammen-Asset, mindestens drei Halter-Arten, Ankerpunkte je Gebäude, Test: keine Lichtquelle im Türbereich.
- **AC-06** Abnahme 🧑 am PC (B-1179/AC-02, B-1183/AC-03, B-1181/AC-03).

## Offene Fragen

Keine offen. Defaults (autonom, Claude, 2026-11-21; 🧑 bestätigt sie bei der Abnahme oder ändert sie):

- **Monument je Welt:** Oberwelt Berg, Sengoku Vulkan, Nil/Ägypten Pyramide, Britannien Berg, Standard Berg.
- **Halter-Arten:** Fackel am Pfahl, Laterne (Papier bzw. Metall je Welt), Feuerschale; Zuordnung: Wohnhäuser Laterne, Wehrbauten Fackel, Plätze und Burg Feuerschale. Nie im Türbereich.
- **Wolken:** prozedural, ohne Asset; im Go-Profil aus.
- **Panel:** Schalter gelten nur für die Sitzung, kein Speichern über Programmstarts.
- Weicht eine Session davon ab, meldet sie das im Ergebnis.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| LA2.1 | `LA2.1-cheat-panel-umgebungseffekte.md` | Umsetzung | autonom | offen |
| LA2.2 | `LA2.2-wolken-wellenstriche.md` | Umsetzung | autonom | offen |
| LA2.3 | `LA2.3-licht-assets-flammen.md` | Umsetzung | autonom | offen |
| LA2.4 | `LA2.4-monumentalbild-je-welt.md` | Umsetzung | autonom | offen |
| LA2.5 | `LA2.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
