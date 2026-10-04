---
name: agent-serena-navigation
version: 1.0.0
description: Verbindliche Reihenfolge für Code-Navigation und symbolische Edits mit Serena statt Volltextsuche und zeilenbasierten Edits. Gilt beim Verstehen fremden Codes, beim Suchen von Aufrufern und beim Ändern von Funktionen, Methoden oder Klassen in TypeScript, JavaScript und Python.
user-invocable: false
---

# Serena-Navigation

Serena ist das führende Werkzeug für Code-Navigation und symbolische Änderungen. `Grep` und `Glob` bleiben für Discovery zuständig, nicht für das Nachlesen von Code.

## Routing

| Aufgabe | Werkzeug |
| --- | --- |
| Neue Code-Datei verstehen | `get_symbols_overview` |
| Bekanntes Symbol verstehen | `find_symbol` mit `include_body` |
| Aufrufer oder Verwendungen finden | `find_referencing_symbols` |
| Ganze Funktion oder Klasse ändern | `replace_symbol_body` |
| Kleine Änderung innerhalb eines Symbols | `replace_content` |
| Unbekannten Namen oder Datei finden | `Glob`, `Grep`, danach die Symboltools |

## Harte Grenzen

- Wird eine Code-Datei gelesen, um eine Funktion, Methode oder Klasse zu verstehen, gilt `find_symbol` oder `get_symbols_overview` — nicht `Read`.
- `Read` auf Code ist nur erlaubt bei einer Nicht-Code-/Config-Datei, oder wenn bereits ein Symbol-Overview der Datei besteht **und** mehr als drei zusammenhängende Symbole derselben Datei gebraucht werden, etwa für eine Aufrufkette.
- „Wer ruft X auf" wird immer mit `find_referencing_symbols` beantwortet, nie mit `Grep` zum Nachlesen von Code.
- `Grep` und `Glob` bleiben erlaubt für Dateinamen- und Textmuster-Discovery sowie für Laufzeit- und Config-Diagnose (`.env`, JSON-Config, Datei-Existenz) — dort ist Serena nicht zuständig.

## Vor einem Edit

Vor dem Ändern einer Funktion `find_referencing_symbols` auf sie ausführen. Eine Guard in der gemeinsamen Funktion ist ein kleinerer Diff als eine pro Aufrufer — und ein Fix nur auf dem gemeldeten Pfad lässt jeden Geschwister-Aufrufer kaputt.

## Wenn Serena fehlt

Ist kein Serena-Werkzeug in der Session registriert, diesen Abschnitt überspringen statt danach zu suchen. Dann gilt: `Grep` und `Glob` für Discovery, `Read` gezielt für den Edit-Kontext der bereits identifizierten Datei — nicht für explorative Mehrfach-Reads.
