# K5 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Anzeigen für Kampf, Bosse und Events, Anlegen-Dialog, Debug-Panel
- **Projekt:** KMP

## Ziel

Kämpfe sind lesbar, die Härte wird beim Anlegen gewählt, Entwickler wechseln den Grad im Debug-Panel, und das Debug-Overlay ist nur noch mit `?dev=1` verfügbar. Am Ende sichtbar: Boss-Leiste, Warnkreis und Event-Banner am TV, Lobby-Dialog, von 🧑 abgenommen.

## Ergebnis

2026-10-23, autonom (Claude, K5.5): AC-01–AC-05 mit Nachweis; Reviewer ohne schwere Befunde. 2026-10-24 (K5.4, Ralf am PC): AC-06 für Dialog (B-105) und Panel (B-107) abgenommen, B-105, B-107, B-098 erledigt. Die Anzeigen (B-132) sind noch nicht abgenommen: verschoben auf B-602 (Studio-Szene) und B-603 (Dev-Aktionen), Abnahme in WZ29.9; B-132 bleibt `eingeplant`.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Reine Funktionen für Boss-Leiste, Phasen-Text und Event-Banner sind getestet (`task test`) … |
| AC-02 | – | Warnkreis, Boss-Leiste und Event-Banner werden angezeigt (B-132/AC-02). |
| AC-03 | – | Die Auswahl-Logik für Standards je Grad und Dev-Sperre ist getestet, die Lobby zeigt sie u … |
| AC-04 | – | Das Debug-Panel wechselt den Grad über eine Server-Aktion und ist ohne Dev-Mode nicht verf … |
| AC-05 | – | `debugEnabled('')` ist `false`, `debugEnabled('?dev=1')` ist `true` (B-098/AC-01). |
| AC-06 | geprüft | 🧑 hat Anzeige, Dialog und Panel am PC (Tastatur) und an der Xbox (Controller) mit höchsten … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-132 | verworfen | AC-01 geprüft, AC-02 –, AC-06 geprüft | Der Client zeigt Gegner-Fähigkeiten, Bosse, Phasen und Events |
| B-105 | erledigt | AC-03 –, AC-06 geprüft | Der Anlegen-Dialog der Lobby wählt Grad, Ziel und Niederlage-Modus |
| B-107 | erledigt | AC-04 –, AC-06 geprüft | Ein Debug-Panel im Dev-Mode wechselt den Schwierigkeitsgrad und weitere Optionen |
| B-098 | erledigt | AC-05 – | Das Debug-Overlay ist vor dem Release wieder nur mit ?dev=1 verfügbar |
