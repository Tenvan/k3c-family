# B-432 · Die Dev-Aktion grade wirkt über die Nachricht dev, der Zustand nennt den Grad des Raums

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Gefunden in K5.3 (B-107 blockiert). Die Dev-Aktion `grade` (B-080, K4.2) ist in `engine/room/dev.go` (`devApply`) umgesetzt und in `docs/protocol.md` nur unter *HTTP: Dungeon-Master-Seite* als „zusätzlich zu den Aktionen der Nachricht `dev`“ beschrieben. Über WebSocket kommt sie nicht an: `engine/net/dispatch.go:95` baut `room.DevAction` ohne `Grade` (`/api/dev` in `engine/net/dev.go:61` reicht es durch), ein `{"t":"dev","action":"grade","grade":"hard"}` endet daher in `SetGrade("")` → `bad_request`. Außerdem nennt der Zustand (`snap`/`delta`) den aktuellen Grad des Raums nicht, nur `rooms[].grade` in der Lobby. Das Debug-Panel im Client (K5.3, B-107) kann den Grad so weder wechseln noch den aktuellen markieren.

## Ziel

Ein Gerät im Raum wechselt im Dev-Mode den Grad über die Nachricht `dev` und sieht den aktuellen Grad im Zustand; damit ist B-107 (K5.3/AC-04) im Client umsetzbar.

## Beteiligte und Zielgruppen

Entwickler mit Debug-Overlay (`game.html?dev=1`); 🧑 entscheidet die Spec.

## Anforderungen

- `dispatch.go` reicht `grade` der Nachricht `dev` an `room.DevAction` weiter; Wirkung und Prüfung wie bei `/api/dev` (ab der nächsten Welle, ohne Dev-Mode `forbidden`, unbekannter Grad `bad_request`).
- Der Zustand nennt den Grad des Raums (Feldname, Ort in `s`, nur im Dev-Mode oder immer: entscheidet die Spec), im Delta nur bei Änderung.
- `docs/protocol.md` › Dev-Aktionen beschreibt `grade` in der Nachricht `dev` (Tabelle Zeile `dev` und Liste) samt Beispiel `c2s-dev-grade.json`.

## Nicht-Ziele

Das Panel im Client (B-107, folgt nach diesem Ticket in CLI). Neue Grade oder Faktoren.

## Regeln und Einschränkungen

Protokolländerung, Domäne SRV (`docs/arbeitsweise.md` › Grenzfälle). Server bleibt die eigentliche Dev-Sperre (`forbidden`).

## Beispiele

Dev-Mode, Raum auf `normal`, Gerät schickt `dev` mit `grade: "hard"` → Log „Dev-Aktion“ mit `grad=hard`, nächste Welle mit Faktoren von `hard`, Zustand nennt `hard`.

## Ausnahme- und Fehlerfälle

Ohne Dev-Mode → `forbidden`; unbekannter Grad oder fehlendes `grade` → `bad_request`.

## Akzeptanzkriterien

- **AC-01** Ein Go-Test in `engine/net` schickt `{"t":"dev","action":"grade","grade":"hard"}` im Dev-Mode und erwartet keinen Fehler und `hard` als Grad der Insel; ohne Dev-Mode `forbidden`.
- **AC-02** Ein Test belegt das Zustandsfeld für den Grad in `snap` und im Delta nach dem Wechsel.
- **AC-03** `docs/protocol.md` nennt `grade` in der Nachricht `dev` und das Zustandsfeld.

## Offene Fragen

Steht der Grad immer im Zustand oder nur im Dev-Mode (wie `devTimescale`)? Entscheidet 🧑 bei der Freigabe.

## Notizen

Blockiert K5.3/AC-04 (B-107).
