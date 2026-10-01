# B-083 · Die Lobby zeigt nach `replaced` oder `version` keinen bedienbaren Eintrag mehr

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-01
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Beim Browser-Lauf zu SP08 (2026-10-01, `task serve:go`, Pane gesteuert durch einen Agenten): Nach `replaced` („An anderer Stelle geöffnet“) wechselt
`GameScene` zur `LobbyScene`. Die Lobby zeigt den Hinweis und eine (alte) Raumliste, der Eintrag „Spielen“ sieht bedienbar aus. Der Client ist aber
`ended` und ignoriert `create`/`join`; es passiert nichts, bis man die Seite neu lädt.

## Ziel

Im Zustand `ended` (und `lost`) ist die Lobby erkennbar nicht bedienbar und sagt, was zu tun ist („Seite neu laden“ bzw. „Erneut versuchen“ mit `retry()`).

## Beteiligte und Zielgruppen

Spieler am TV und Handy; Entwickler.

## Anforderungen

- `ended` blendet „Spielen“ und die Raumliste aus oder grau und zeigt „Seite neu laden“ (Button/A lädt neu).
- `lost` bietet „Erneut versuchen“ (`RoomClient.retry()`).

## Nicht-Ziele

Automatisches Neuverbinden nach `replaced` (Protokoll verbietet es).

## Regeln und Einschränkungen

Domäne CLI; B nicht belegen; Seiten-Regeln aus `CLAUDE.md`.

## Beispiele

Zweiter Tab desselben Browsers tritt dem Raum bei → der erste Tab zeigt „An anderer Stelle geöffnet“ und „Seite neu laden“.

## Ausnahme- und Fehlerfälle

nicht relevant: Randfall der Bedienung.

## Akzeptanzkriterien

- **AC-01** Im Zustand `ended` und `lost` zeigt die Lobby keinen wirkungslosen Eintrag, sondern die passende Aktion (Test der Lobby-Logik).

## Offene Fragen

keine

## Notizen

Gefunden beim Browser-Lauf zu SP08 (siehe SP08-README, Abnahme).
