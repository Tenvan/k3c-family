# B-884 · Abnahme MG11: Sichtprüfung der Welt-Grafik und Gen-Läufe sind erledigt

- **Domäne:** PAP
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** MG11
- **Projekt:** PAP
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint MG11 (B-721, B-722, B-723) ist umgesetzt und reviewt; offen sind die Sichtprüfung durch 🧑 und die Gen-Läufe am Sprint-Ende (`docs/arbeitsweise.md` › Sim, Balance und Gen).

## Ziel

MG11 ist abgenommen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- 🧑 sichtet im Review der Workbench die Gruppen `gebaeude-paper` (Stufen 1–5), `hintergruende` (75 Assets), `objekte` (Bäume, Wurzel, Höhlenpilze, Truhe, Axt-Schild) und `icons` (Holz, Kupfer, Eisen, Pokal, Münze).
- `task gen:paper` samt Grafik-Golden und `task check:race` einmal grün.

## Nicht-Ziele

Neue Grafik; Befunde der Sichtprüfung werden eigene Tickets.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Sichtprüfung durch 🧑 dokumentiert (je Gruppe ✅ oder Ticket).
- **AC-02** `task gen:paper` ohne Golden-Abweichung, `task check:race` grün (ohne gcc: als übersprungen nennen).

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Quelle: MG11.5. Hinweis (vermutet): 360-px-Bäume werden im Spiel auf ≈0,4 verkleinert, beim Sichten auf der Xbox auf Flimmern achten.
