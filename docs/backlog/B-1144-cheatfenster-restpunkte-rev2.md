# B-1144 · Cheat-Fenster: Zielspieler und Schalterzustand bleiben erhalten, goto, Fokus und Skillpunkte fehlen

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU56
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Review RU56.12 (k3c-hud, k3c-reviewer): (1) Die Zielspieler-Auswahl und alle Auswahlen/Umschalter liegen in den Controls der Seite; Reiterwechsel und set_catalog bauen sie neu (init_combo setzt Index 0), danach geht ein Befehl still an Spieler 1. (2) Umschalter (Baukosten 1, Zeitraffer endet mit der Tageszeit) und Zeitraffer-Auswahl, Debug-Seite, Leistung, Fluss zeigen den Spielzustand nicht (State enthaelt die Schalter nicht). (3) Ohne Spielpfad: goto (Welt, Insel, Stufe), Spielfenster nach vorn aus dem Fenster und Skillpunkte +n liefern ok:false; die Buttons sind trotzdem aktiv. (4) Die Spielerauswahl bietet immer 4 Spieler; save und speed melden ok, auch wenn dev_session_cmd nichts tut.

## Ziel

Das Cheat-Fenster zeigt und behaelt den Zustand: Zielspieler und Auswahlen ueberleben den Reiterwechsel, Umschalter, Zeitraffer und Anzeige-Tasten spiegeln State, wirkungslose Controls sind ausgegraut oder fehlen bis ein Pfad existiert.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **AC-01** Auswahlen und Eingaben einer Seite bleiben beim Reiterwechsel und nach set_catalog erhalten (Werte je Seite im Modell).
- **AC-02** State traegt cheapBuild, timescaleAuto; Umschalter, Zeitraffer-Auswahl und Debug-/Leistungs-/Fluss-Tasten zeigen den Stand.
- **AC-03** goto, Fokus aus dem Fenster und Skillpunkte: Session-Pfad beziehungsweise Dev-Aktion in der Sim schaffen oder Controls ausblenden; Spielerauswahl nach Spielerzahl aus dem state.
- **AC-04** speed und save melden den echten Ausgang der Aktion.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

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

- **AC-01** Test: Zielspieler 2 gewaehlt, Reiterwechsel, danach Gold geht an Spieler 2.
- **AC-02** Test: State mit cheapBuild an zeigt den Umschalter gedrueckt.
- **AC-03** Test: Ohne Pfad ist das Control deaktiviert.
- **AC-04** Test: Fehlschlag von speed liefert ok:false.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
