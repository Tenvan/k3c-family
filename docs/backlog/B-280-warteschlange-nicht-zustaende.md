# B-280 · Die Warteschlange einer Verbindung läuft nicht voll, wenn andere Nachrichten zwischen Zuständen stehen

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NT1
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit N1.1 (B-276) ersetzt ein Zustand in der Warteschlange einer Verbindung nur einen wartenden Zustand **direkt davor**
(`engine/net/ws.go` › `push`). Steht dazwischen eine andere Nachricht (`seats`, `rooms`, `level` …), bleibt der alte
Zustand stehen und der neue wird angehängt. Hängt ein Gerät und kommen im Takt immer wieder andere Nachrichten, wächst
die Warteschlange abwechselnd aus Zustand und Nachricht und erreicht `sendBuffer` (64): Die Verbindung wird getrennt,
obwohl sie nur veraltete Zustände staut. Gefunden im Review N1.3.

## Ziel

Ein langsames Gerät wird nur getrennt, wenn sich wirklich nicht ersetzbare Nachrichten stauen.

## Beteiligte und Zielgruppen

Spieler mit wackligem WLAN; SRV-Agent setzt um.

## Anforderungen

- Veraltete Zustände in der Warteschlange zählen nicht zum Stau, auch wenn andere Nachrichten zwischen ihnen stehen.
- Reihenfolge Level → snap → delta und die anderer Nachrichten bleibt erhalten; Ereignisse verworfener Zustände gehen
  nicht verloren (wie N1.1).

## Nicht-Ziele

Protokolländerung (B-278 beschreibt nur das Verhalten).

## Regeln und Einschränkungen

Domäne SRV (`engine/net/`), Protokoll unverändert, Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Gerät hängt 5 s, in der Zeit wechselt ein anderes Gerät mehrmals den Platz (`seats`) → das Gerät bleibt verbunden und
bekommt danach `seats`-Nachrichten und ein Delta zum zuletzt gesendeten Zustand.

## Ausnahme- und Fehlerfälle

64 andere Nachrichten ohne Zustände dazwischen → weiterhin Trennung.

## Akzeptanzkriterien

- **AC-01** Test in `engine/net/`: abwechselnd Zustand und andere Nachricht, mehr als 64 Mal, ohne Senden → nicht
  getrennt; danach ist der Client-Zustand nach dem Anwenden der des Servers.

## Offene Fragen

keine

## Notizen

Review N1.3, Befund 9; bewusst nicht im Review umgebaut.
