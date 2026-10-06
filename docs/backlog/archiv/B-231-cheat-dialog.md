# B-231 · Der Cheat-Dialog ist modal, hält den Raum an und öffnet per Geste auf jedem Gerät

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** –
- **Erstellt:** 2026-10-04
- **Spec:** rückwirkend
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Debug-Overlay (B-093, B-179) zeigt Diagnose-Text und daneben eine kleine Dev-Aktionsliste; Umschalten per Ö oder
Klick auf den linken Stick, die Liste per RB-Fokus. Am Touch-Gerät gibt es keinen Aufruf, und der Raum läuft weiter,
während man Cheats wählt (`src/scenes/debugOverlay*.ts`).

## Ziel

Tester rufen die wichtigsten Cheats auf jedem Gerät schnell auf, ohne dass das Spiel weiterläuft; die Diagnose bleibt
getrennt davon abrufbar. Wunsch von 🧑 (Chat 2026-10-04), soll zeitnah in die aktuelle Version.

## Beteiligte und Zielgruppen

Tester und Entwickler an Xbox, PC, Handy und Tablet; 🧑 nimmt am Gerät ab.

## Anforderungen

- Cheat-Dialog modal, 80vw × 80vh, leicht durchsichtig; solange er offen ist, steht der Raum (neue Dev-Aktion `pause`).
- Aufruf Cheat-Dialog: Taste Ä, LB + RB 3 s halten (Xbox), Doppeltap mit zwei Fingern (Touch).
- Diagnose nur anzeigen: Taste Ö, RB 3 s halten (Xbox), Doppeltap mit einem Finger (Touch).
- Im Dialog nur die wichtigsten Cheats: Gold, Material, Zeitraffer, für den gewählten lokalen Spieler.
- Controller im Dialog: D-Pad wählen, A auslösen, LB + RB schließt; Maus und Touch tippen.

## Nicht-Ziele

Umfangreiche Live-Anpassungen und Diagnose auf einer eigenen Seite: B-232 (Dungeon-Master-Seite `/dm`).

## Regeln und Einschränkungen

Taste B, View + Menu und X bleiben frei (`CLAUDE.md`). Pause und Cheats nur im Dev-Mode des Servers
(`docs/protocol.md` › Dev-Aktionen, Protokoll bleibt v3). Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Xbox: LB + RB 3 s halten → Dialog, Raum steht; „Gold 50“ mit A → Monarch bekommt Münzen; LB + RB → Dialog zu, Raum läuft.

## Ausnahme- und Fehlerfälle

Server ohne Dev-Mode → Dialog öffnet mit Hinweis, Pause und Cheats werden abgelehnt (`forbidden`). Gerät trennt bei
offenem Dialog → der Server hebt die Pause auf, sobald kein Gerät mehr verbunden ist. Szene endet → Pause wird gelöst.

## Akzeptanzkriterien

- **AC-01** Dev-Aktion `pause` hält den Raum an und lässt ihn weiterlaufen, `devPaused` im Zustand (Go-Test `TestDevPauseHaeltRaumAn`).
- **AC-02** Gesten: LB + RB 3 s = Dialog, RB 3 s = Diagnose, Doppeltap 2 bzw. 1 Finger (Test `debugGestures.test.ts`).
- **AC-03** Ä/Ö am PC, Gesten an Xbox und Handy von 🧑 am Gerät abgenommen.

## Offene Fragen

keine

## Notizen

Ein-Finger-Doppeltap kann beim schnellen Antippen zum Laufen versehentlich die Diagnose umschalten (harmlos, nur Anzeige).
