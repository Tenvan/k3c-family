# B-431 · Raum-Optionen des Anlegen-Dialogs wirken nicht, wenn „Spielen“ einen vorhandenen Spielstand lädt

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit K5.2 öffnet „Spielen ({save})“ den Anlegen-Dialog und sendet `create` mit `grade`, `goal`, `defeat`. Ohne `?fresh=1` ist `fresh` aber `false`: Existiert der Spielstand (Standard `familie`), lädt der Server ihn und ignoriert die Optionen (`docs/protocol.md`). Der Spieler wählt dann z. B. „Hart“, bekommt aber den Grad des gespeicherten Stands, ohne Hinweis. Neu angelegt wird nur, wenn der Stand fehlt (`save_not_found` → zweiter Versuch mit `fresh: true`, die Optionen gehen mit).

## Ziel

Die im Anlegen-Dialog gewählten Raum-Optionen gelten immer, oder der Dialog sagt klar, wann sie nicht gelten. 🧑 entscheidet den Weg (Offene Fragen).

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

- Soll „Spielen“ immer einen neuen Spielstand anlegen (eigener Name, `fresh: true`) und das Fortsetzen nur über die Spielstand-Einträge der Liste laufen?
- Oder soll der Dialog nur erscheinen, wenn der Spielstand nicht existiert (Client kennt die Liste aus `listSaves()`)?
- Oder reicht ein Hinweis im Dialog „Spielstand {save} wird geladen, Optionen gelten nur neu“?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
