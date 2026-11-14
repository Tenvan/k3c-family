# B-852 · Abnahme-Läufe WZ43

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** WZ43
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)

## Ausgangslage

Sprint WZ43 (Asset-Tools-Review) ist bis auf die Sichtprüfung WZ43.4 umgesetzt; er ändert die Anbindung an die Paper-CLI (Review, Vorschau, Teil-Patch). Nach Regel B-742 laufen Gen und Race einmal zum Abschluss.

## Ziel

Die Abnahme-Läufe von WZ43 sind grün.

## Beteiligte und Zielgruppen

Claude autonom; 🧑 liest das Ergebnis.

## Anforderungen

Läufe laut `docs/arbeitsweise.md` › Sim, Balance und Gen, nur über `check_run`.

## Nicht-Ziele

Nachbessern im Ticket; Rot wird ein Problem-Ticket.

## Regeln und Einschränkungen

Sim und Balancing gesperrt (Beschluss 🧑 2026-11-11).

- **Bereich:** `public/grafik/**`, `tools/paper/golden/**`
- **Abhängig von:** WZ43
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** golden

## Beispiele

nicht relevant: reine Prüfläufe.

## Ausnahme- und Fehlerfälle

Lauf rot → neues Problem-Ticket im Projekt WZG.

## Akzeptanzkriterien

- **AC-01** Sim und Balance verschoben (Sperre 2026-11-11).
- **AC-02** Gen: `check_run task:test` samt `gen:paper` und Grafik-Golden grün.
- **AC-03** Race: `check_run task:check:race` für die Workbench-Pakete (lokal ohne gcc als übersprungen nennen).

## Offene Fragen

keine

## Notizen

Vorschlag aus WZ43.5 (2026-11-14).
