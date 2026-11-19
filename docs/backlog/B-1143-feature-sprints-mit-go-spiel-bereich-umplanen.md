# B-1143 · Offene Sprints und Tickets mit Bereich native/k3c-game müssen nach dem Löschen des Go-Spiels auf Rust umgeplant oder verworfen werden

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU37
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19 (Ralf: ausdrücklich)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ralf hat am 2026-11-19 ausdrücklich entschieden, das Go-Spiel (`native/k3c-game`, `native/k3c-rig`) jetzt zu entfernen (RU58 Referenz, RU37 Löschen). Offene Sprints und Tickets ändern aber Spielfunktionen genau dort.

**Aktive Sprints mit `native/k3c-game` im Bereich:** GS1, GS2, KS3, KS4, KS5, KS6, KS7, WE4, WI1. **Geplant:** GS3, GS4, WE6, WE8.

**Offene und eingeplante Tickets mit `native/` im Bereich (42, Stand 2026-11-19):** B-780, B-885, B-888, B-889, B-890, B-891, B-899, B-910, B-911, B-912, B-936, B-938, B-940, B-941, B-942, B-943, B-944, B-947, B-948, B-949, B-953, B-962, B-964, B-966, B-974, B-977, B-978, B-980, B-993, B-994, B-995, B-997, B-998, B-999, B-1000, B-1001, B-1002, B-1004, B-1005, B-1006, B-1011, B-1012 (Sprints u. a. NA1, CH1, BI1, BI2, SR1, WG2, FB4, SI1 bis SI6; die Sprint-Zuordnung steht je Ticket).

Zufällig erledigte Sprints (z. B. NG*, RU0) bleiben unberührt (historisch).

Befund aus der Planung 2026-11-19: Mit dem Löschen verlieren diese Arbeiten ihren Zielort. Engine-Anteile (`engine/sim`, `engine/room`, `data/**`) bleiben gültig, die Spiel-Anteile (HUD, Ansicht, Zeichnen, Eingabe, Menü) müssten im Rust-Spiel (`rust/crates/k3c-game/src/**`) neu entstehen oder entfallen.

## Ziel

Für jeden der genannten Sprints und Tickets ist entschieden: auf Rust umplanen (neuer Bereich, Port der Spiel-Anteile) oder verwerfen; kein Ticket zeigt nach RU37 auf einen nicht mehr vorhandenen Ordner.

## Beteiligte und Zielgruppen

🧑 Ralf entscheidet je Sprint (Rust nachziehen oder verwerfen); der Hauptrechner setzt das mit `plan_set`/`plan_create` um (der Second Worker legt keine Tickets an).

## Anforderungen

- Liste der betroffenen Sprints und Tickets mit Vorschlag (umplanen auf Rust oder verwerfen) überlegt vorlegen (Hauptrechner, Ralf-Queue).
- Nach Ralfs Entscheidung: `Bereich` der behaltenen Sprints und Tickets auf `rust/crates/k3c-game/src/<modul>/` umstellen; Ports als Ticket je Spiel-Anteil, im selben Sprint oder in einem Rust-Folgesprint des Projekts RSU; verworfene mit `plan_set Status verworfen` und Grund.
- Das Löschen in RU37 wartet nicht auf diese Entscheidung (Ralf 2026-11-19); die Spiel-Anteile der Sprints leben im Tag `go-referenz-final` weiter und lassen sich von dort als Vorlage lesen.

## Nicht-Ziele

Die Spielfunktionen selbst umsetzen. Das Löschen verzögern (RU37 läuft unabhängig von dieser Entscheidung).

## Regeln und Einschränkungen

Planung nur über MCP `plan_*`. Entscheidung nur durch 🧑.

- **Bereich:** `docs/backlog/**`, `docs/sprints/**`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Größe:** M
- **Sperre:** –

## Beispiele

KS5 (Kampf-Anzeige, nur `native/k3c-game/**`): wird als Rust-Sprint auf `rust/crates/k3c-game/src/hud` umgeplant, oder verworfen, wenn Rust die Anzeige schon hat.

## Ausnahme- und Fehlerfälle

Ein Sprint mit Anteilen in `engine/` und `native/`: der Engine-Anteil bleibt, der Spiel-Anteil wird ein Rust-Ticket.

## Akzeptanzkriterien

- **AC-01** Jeder genannte Sprint und jedes Ticket hat einen Entscheid (Rust-Bereich oder verworfen), per `plan_list` prüfbar.
- **AC-02** `grep -rl "native/k3c-game" docs/sprints/aktiv docs/sprints/geplant docs/backlog/B-*.md` liefert nur historische Erwähnungen.

## Offene Fragen

Welche der 13 Sprints und 42 Tickets werden auf Rust umgeplant, welche verworfen? Entscheidung 🧑 Ralf.

## Notizen

Gefunden bei der Planung des Go-Abbaus (RU58, RU37, RU59) am 2026-11-19.
