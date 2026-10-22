# K5 · CLI · Anzeigen für Kampf, Bosse und Events, Anlegen-Dialog, Debug-Panel

- **Status:** aktiv
- **Projekt:** KMP
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-132, B-105, B-107, B-098
- **Start-Commit:** 1d368498
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-11, 🧑 im Chat, Revision 2 (3–4 Spieler, Touch und Handy entfallen)

## Ausgangslage

Der Client zeigt weder Warnkreise, Boss-Leisten, Phasen noch Events; der Anlegen-Dialog wählt keinen Grad, kein Ziel und keinen Niederlage-Modus; das Debug-Panel fehlt, das Debug-Overlay ist noch Standard (B-132, B-105, B-107, B-098).

## Ziel

Kämpfe sind lesbar, die Härte wird beim Anlegen gewählt, Entwickler wechseln den Grad im Debug-Panel, und das Debug-Overlay ist nur noch mit `?dev=1` verfügbar.

Am Ende sichtbar: Boss-Leiste, Warnkreis und Event-Banner am TV, Lobby-Dialog, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

Spieler (1 bis 2 am PC oder an der Xbox) und Entwickler; 🧑 nimmt am Gerät ab. 3 bis 4 Spieler, Touch und Handy entfallen seit 2026-10-11 (Beschluss 🧑).

## Anforderungen

B-132 › Anforderungen, B-105 › Anforderungen, B-107 › Anforderungen, B-098 › Anforderungen.

## Nicht-Ziele

Grafik-Anbindung (GR3), Ton, Protokoll (K4), Neustart und weitere Panel-Aktionen (Gold, Stufe).

## Regeln und Einschränkungen

Der Client rechnet nichts und zeichnet nur Server-Zustand (`src/scenes/noSim.test.ts`); Logik als reine Funktionen mit Test. Mindest-Schriftgrößen je Split-Viertel nach den Regeln aus F1 (B-136). Seiten-Regeln aus `CLAUDE.md`; B nicht belegen, View + Menu reserviert. Datei ≤ 400 Zeilen, Funktion ≤ 60. Der Sprint bleibt in der Domäne CLI. Abhängigkeit: K4 (B-080, Server-Aktion Grad).

## Beispiele

Endboss in Phase 2 → Leiste mit Phasen-Text, Warnkreis vor dem Flächenschlag.

## Ausnahme- und Fehlerfälle

Dev-Sperre aktiv → Debug-Panel und Grad „Dev“ sind nicht verfügbar.

## Akzeptanzkriterien

- **AC-01** Reine Funktionen für Boss-Leiste, Phasen-Text und Event-Banner sind getestet (`task test`) (B-132/AC-01).
- **AC-02** Warnkreis, Boss-Leiste und Event-Banner werden angezeigt (B-132/AC-02).
- **AC-03** Die Auswahl-Logik für Standards je Grad und Dev-Sperre ist getestet, die Lobby zeigt sie und sendet sie beim Anlegen (B-105/AC-01, B-105/AC-02).
- **AC-04** Das Debug-Panel wechselt den Grad über eine Server-Aktion und ist ohne Dev-Mode nicht verfügbar (Test) (B-107/AC-01, B-107/AC-02).
- **AC-05** `debugEnabled('')` ist `false`, `debugEnabled('?dev=1')` ist `true` (B-098/AC-01).
- **AC-06** 🧑 hat Anzeige, Dialog und Panel am PC (Tastatur) und an der Xbox (Controller) mit höchstens 2 Spielern abgenommen (B-132/AC-03, B-105/AC-03, B-107/AC-03). Touch, Handy und 3–4 Spieler entfallen (Beschluss 🧑 2026-10-11).

## Offene Fragen

Vorentschieden (autonom, Claude, 2026-10-23), Bestätigung durch 🧑 in K5.4 bzw. KR6:

- Tastenbelegung des Debug-Panels: das vorhandene Panel (D-Pad wählt, A löst aus, RB wechselt den Fokus, Mausklick) trägt die Gruppe „Grad“; B, View + Menu und X bleiben frei (B-107).
- Stick-Klick (Q06): bleibt Overlay-Umschalter wie heute, keine Änderung in K5.
- Dev-Sperre in der Lobby: Der Grad „Dev“ erscheint nur mit `?dev=1` in der Adresse; der Server bleibt die Sperre (`bad_request` → Hinweis in der Lobby).
- Die Server-Aktion für den Grad über WebSocket und das Zustandsfeld `grade` fehlen (nur `/api/dev`); sie kommen in KB2.6 (SRV, vorgezogen vor K5.3).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| K5.1 | `K5.1-boss-warnkreis-event.md` | Umsetzung | autonom | fertig |
| K5.2 | `K5.2-anlegen-dialog.md` | Umsetzung | autonom | offen |
| K5.3 | `K5.3-debug-panel-overlay.md` | Umsetzung | autonom | offen |
| K5.4 | `K5.4-abnahme-geraet.md` | Workshop | Mensch | offen |
| K5.5 | `K5.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
