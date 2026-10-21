# B-515 · Studio-Chat übernimmt die Umgebung einer Claude-Sitzung und zeigt Anmeldefehler doppelt

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-20
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Abnahme WZ19.5 (2026-10-20, Ralf, Screenshot): Der Chat im Studio antwortet auf „Erstelle eine Welt Nordsee“ mit „Failed to authenticate: OAuth session expired and could not be refreshed“. Das Studio wurde über k3c-dev (`task_start k3c-studio:run`) aus einer Claude-Desktop-Sitzung gestartet.

1. **Umgebung (vermutet, ungeprüft):** `chat_cli.go:82` reicht `os.Environ()` vollständig an `claude -p` weiter. Wird das Studio aus einer Claude-Sitzung gestartet, erbt die CLI deren Variablen (`ANTHROPIC_BASE_URL`, `CLAUDE_CODE_SDK_HAS_HOST_AUTH_REFRESH`, `CLAUDE_CODE_OAUTH_SCOPES`, `CLAUDECODE` …) und versucht die Anmeldung der Host-Sitzung statt der eigenen. Unabhängig davon meldet `claude auth status` auf dem PC `loggedIn: false`, die eigenständige CLI ist also nicht angemeldet.
2. **Anzeige:** Der Fehlertext steht zweimal da, einmal als Antwort von „Claude“ und einmal als oranger Fehler darunter.

Vorschlag: Die Variablen `CLAUDE_CODE_*`, `CLAUDECODE`, `ANTHROPIC_BASE_URL` (Test-Ersatz-Variablen ausgenommen) vor dem Start entfernen; bei Status `fehler` den Text der CLI nicht zusätzlich als Antwort zeigen.

## Ziel

Welches Ergebnis, welcher Nutzen (oder welcher Schaden verschwindet)? Kein Umsetzungsschritt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

2026-10-20: Nach `claude auth logout` und `claude auth login` von Ralf lief der Chat ohne Neustart des Studios. Ursache war die abgelaufene Anmeldung der eigenständigen CLI.

Behoben mit PR Tenvan/k3c-family#406 (Chat-Controls, ohne Sprint): Punkt 1 – `chatEnviron` (`tools/k3c-studio/backend/chat_cli.go`) entfernt geerbte `CLAUDE_CODE_*`, `CLAUDECODE` und `ANTHROPIC_BASE_URL` und behält Nutzer-Einstellungen aus `chatKeepEnv` (Git-Bash-Pfad, Bedrock/Vertex/Foundry, `MAX_OUTPUT_TOKENS`, `OAUTH_TOKEN`); ein selbst gesetzter Proxy über `ANTHROPIC_BASE_URL` fällt damit bewusst weg. Punkt 2 – `answerText` (`chatAdapter.ts`) zeigt den Fehler nur einmal.
