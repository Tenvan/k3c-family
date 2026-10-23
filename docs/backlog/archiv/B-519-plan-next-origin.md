# B-519 · plan_next berücksichtigt den Stand der Sprint-Branches auf origin

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** WZ26
- **Projekt:** WZG
- **Erstellt:** 2026-10-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`plan_next` (B-518) liest nur den ausgecheckten Stand. `in Arbeit` und `fertig` stehen bis zum Sprint-PR nur auf `origin/sprint/<präfix>`; auf `develop` liefert das Tool deshalb erledigte Sessions, und die Domänen-Sperre greift kaum.

## Ziel

`plan_next` überlagert je aktivem Sprint mit Branch auf `origin` die README und Session-Dateien von dort (`git show`), wie `docs/arbeitsweise.md` › Autonomer Ablauf Schritt 1 es verlangt.

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

- **AC-01** Liegt `origin/sprint/<präfix>` vor, gilt dessen Status der Sessions für Auswahl, Abhängigkeiten und Domänen-Sperre (Go-Test mit Test-Repo).
- **AC-02** Ohne Netz oder ohne Branch fällt `plan_next` auf den lokalen Stand zurück und sagt das in der Antwort.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
