# B-498 · Das Asset-Studio ist eine Seite des Go-Servers statt einer eigenen EXE

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Asset-Studio war eine eigene Wails-EXE (`tools/k3c-studio`, `bin/k3c-studio.exe`) mit eigenem Go-Modul. Gamer sollen später selbst Welten bauen können; das braucht einen Zugang über die Landingpage des Spiels, nicht eine EXE neben dem Spiel.

## Ziel

Das Studio läuft als Seite des Go-Servers (`/studio/`), aufrufbar über die Kachel „Welten bauen“ der Landingpage. Keine EXE mehr.

## Beteiligte und Zielgruppen

Entwickler (🧑) heute; Gamer, die später Welten bauen (noch nicht umgesetzt).

## Anforderungen

- Backend als Go-Paket `engine/studio`, eingehängt unter `/api/studio/` (Aufrufe per JSON, Ereignisse per Server-Sent Events, Datei-Import per Upload).
- Frontend aus `tools/k3c-studio/frontend` nach `dist/studio/`; Kachel und Seite `studio.html` auf der Landingpage.
- Nur Dev-Mode, nur im Checkout, nur Loopback (LAN per `K3C_STUDIO_LAN=1`); schreibende Aufrufe brauchen die Kopfzeile `X-K3C-Studio`.

## Nicht-Ziele

Freigabe des Studios für Gamer (Rechte, Spielerwelten, Speicherort): eigenes Ticket, wenn es soweit ist.

## Regeln und Einschränkungen

Seiten-Regel aus `CLAUDE.md` (`installPageChrome()`, Eintrag in `src/landing/pages.ts`); Release-Image ohne Studio (kein Checkout).

## Beispiele

Landingpage → „Welten bauen“ → Studio mit den Seiten Grafik, Sound, Welten Editor, Gebäude-Planer, Monarch, Gegner.

## Ausnahme- und Fehlerfälle

Gerät im LAN, Release-Image oder Zugriff über den Vite-Port: `studio.html` zeigt den Hinweis „nur auf dem Rechner des Servers“; Aufrufe ohne Kopfzeile oder von fremden Adressen antworten 403.

## Akzeptanzkriterien

- **AC-01** `go test ./engine/studio` und Studio-Vitest grün; `/api/studio/rpc/AssetsList` antwortet mit dem Register, ohne Kopfzeile 403.
- **AC-02** Die Landingpage hat die Kachel „Welten bauen“, `studio.html` öffnet das Studio im Rahmen.
- **AC-03** `tools/k3c-studio` enthält keinen Go-Code und kein Wails mehr; k3c-dev „Asset-Studio öffnen“ öffnet `/studio/index.html`.

## Offene Fragen

keine

## Notizen

Umgesetzt im Sprint-Branch `sprint/ws1`; im Browser auf Port 8080 geprüft (Grafik-Seite mit echten Slots). Rückwärtsschritt gesehen: Importieren öffnet jetzt den Datei-Dialog des Browsers.
