# B-058 · requirements.md empfiehlt keine Sicherheitseinstellung ohne Entscheidung von 🧑

- **Domäne:** INF
- **Typ:** Frage
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`requirements.md` › „Installation unter Windows“ rät bei blockierten Skripten (`npm.ps1 kann nicht geladen werden`)
zuerst zu `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned`, erst danach zu `npm.cmd`. Das ist eine dauerhafte
Sicherheitseinstellung des Nutzerkontos, nicht nur für dieses Repo. Gefunden im Review L2.2.

## Ziel

Die Anleitung empfiehlt nur, was 🧑 für Entwickler-Rechner bewusst entschieden hat.

## Beteiligte und Zielgruppen

Entwickler unter Windows; 🧑 entscheidet.

## Anforderungen

- Der Hinweis nennt entweder zuerst den Weg ohne Systemänderung (`npm.cmd`) oder begründet die Policy-Änderung.

## Nicht-Ziele

Andere Abschnitte von `requirements.md` ändern.

## Regeln und Einschränkungen

Domäne INF; nur Doku, keine Skripte, die Einstellungen setzen.

## Beispiele

`npm run check` in PowerShell scheitert an der Execution Policy → die Anleitung führt ohne Systemänderung zum Ziel.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Doku-Entscheidung.

## Akzeptanzkriterien

- **AC-01** `requirements.md` › Hinweise nennt die von 🧑 gewählte Reihenfolge; eine Policy-Änderung steht nur mit Begründung dort.

## Offene Fragen

Soll `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned` empfohlen bleiben, nur als zweite Wahl nach `npm.cmd`
stehen oder entfallen? Entscheidet 🧑.

## Notizen

`RemoteSigned` für `CurrentUser` ist verbreitet und braucht keine Admin-Rechte, lockert aber die Voreinstellung
für alle Skripte des Kontos. `npm.cmd` (oder `cmd`/Git Bash) umgeht das Problem ohne Einstellung.
