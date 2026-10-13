# Tenvan CC 🛩️ (`tenvan-cc`)

Universal-Mod für **Claude Desktop**: ein schmales Band über der Prompt-Eingabe und ein ausführliches Panel rechts. Eigene Implementierung, die den Funktionsumfang dreier Community-Mods bündelt und verschlankt:

- [scasella/claude-flightdeck](https://github.com/scasella/claude-flightdeck) – Panel mit Turn-Beleg, Gate, Agents, Log
- [augiefra/token-weather-usage](https://github.com/augiefra/claude-mods/tree/main/plugins/token-weather-usage) – Band mit Kontext, Limits, Cache, Kosten
- [pawandeepdhall/next-steps](https://github.com/pawandeepdhall/claude-mods/tree/main/plugins/next-steps) – Folge-Schritte als Buttons

Kein Code aus diesen Projekten wurde übernommen. Kein Netzwerk, keine Telemetrie, kein Lesen von Transcripts – alle Werte kommen aus den Mod-Events der Engine und `$.session.usage()`. Einzige Ausnahme sind die Folge-Schritte: Sie kosten einen kurzen Haiku-Call pro Antwort über die eigene Claude-Anbindung.

## Anzeige

**Band über der Prompt-Eingabe**

| Teil | Inhalt |
| --- | --- |
| Titelleiste | `Tenvan CC (Version x, Build: Datum Uhrzeit)`, auch erste Zeile im Panel |
| Features | Knopf hinter der Legende: gibt alle Funktionen mit aktuellem Einstellungsstand als Bericht im Chat aus (`/cockpit-features`) |
| Kontext | Füllstand des Kontextfensters in Prozent; ab 85 % gelb mit Button „Kompaktieren“ |
| `5h` / `7d` | Verbrauch der Rate-Limit-Fenster; gelb, wenn mehr als 10 Punkte vor dem Takt, rot ab 90 % |
| Cache | Restzeit des Prompt-Caches seit dem letzten Request (Schätzung: 1 h mit Subscription, sonst 5 min) und Trefferquote des letzten Requests |
| Kosten | Bisherige Session-Kosten |
| Agents | Anzahl laufender Subagents, nur wenn > 0 |
| Details | Öffnet das Panel |
| Weiter: … | Nach jeder Antwort bis zu 3 Folge-Schritte als Buttons; ein Klick schreibt den Schritt ins Eingabefeld, Enter sendet ihn |

**Panel rechts** (`/cockpit` oder „Details“)

- Übersicht: Kontext in Tokens, Limits mit Takt und Reset-Uhrzeit, Kosten, Prompt-Cache
- Letzter Turn: Dauer, Tool-Calls, Edits, Fehler, Kosten-Zuwachs
- Tool-Freigaben: erlaubt / gefragt / abgelehnt
- Subagents: Typ, Aufgabe, Status, Laufzeit
- Log: die letzten Ereignisse der Session

„Takt“ = Verbrauch minus verstrichener Anteil des Fensters, in Prozentpunkten. `+12` heißt: 12 Punkte schneller als gleichmäßig.

## Grenzen

- Zeichnet nur auf der Desktop-Surface; im Terminal bleibt alles wie gewohnt.
- Die Cache-Restzeit ist geschätzt; ob der Cache wirklich traf, zeigt erst die Trefferquote des nächsten Requests.
- Ohne Subscription liefert die Engine keine Rate-Limits – der Limit-Teil entfällt dann.
- Tool-Freigaben werden nur gezählt, das Mod lehnt selbst nie etwas ab.
- Folge-Schritte sind Einzelklicks (keine Mehrfachauswahl) und lassen sich abschalten.

## Optionen

| Option | Standard | Wirkung |
| --- | --- | --- |
| `nextSteps` | `true` | Folge-Schritte nach jeder Antwort vorschlagen (ein Haiku-Call pro Antwort) |

Gesetzt über `/config` oder in den Settings unter `pluginConfigs.tenvan-cc`.

## Installation

Im Terminal (der Desktop-Code-Tab kennt `/plugin` nicht, lädt das Mod aber mit):

```shell
claude --plugin-dir tools/cockpit
```

## Entwicklung

Spezifikation, Backlog und Entscheidungen: [SPEC.md](SPEC.md). Arbeitsanleitung für Agenten (Prüfbefehle, API-Fallstricke, Test-Muster): [DEVELOPMENT.md](DEVELOPMENT.md).

Mod-API: Claude Code Function Hooks (Engine ≥ 2.1.295, Early Access). Struktur:

| Datei | Zweck |
| --- | --- |
| `hooks/register.tsx` | Hooks, Band und Panel |
| `hooks/core.ts` | Reine Rechen- und Formatfunktionen |
| `types/index.d.ts` | `$.state`-Vertrag |
| `hooks/register.test.ts` | Tests |

```shell
claude plugin validate plugins/tenvan-cc
claude plugin test plugins/tenvan-cc
```

Live ausprobieren ohne Installation: `claude --plugin-dir plugins/tenvan-cc`, oder den Pfad in `CLAUDE_CODE_PLUGIN_DIRS` (env-Block der `~/.claude/settings.json`) eintragen, damit die Desktop-App das Mod lädt.

Weitere Community-Mods werden als zusätzliche Abschnitte in Band oder Panel ergänzt, nicht als eigene Plugins.
