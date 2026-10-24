# BD9.4 · Abnahme am PC: Welt im Pausenmenü und in der Lobby, Raum schließen

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** –
- **Abhängig von:** BD9.3
- **Tickets:** B-604, B-581
- **Kriterien:** AC-04

## Ziel

Ralf hat Welt-Anzeige und Raum schließen am PC abgenommen.

## Kontext

Stand nach BD9.3 (`develop`); Server mit `task serve`, zwei Browser-Tabs als zwei Geräte.

## Erlaubte Dateien

- `docs/sprints/**/BD9-*/BD9.4-*.md` (Ergebnis), Tickets aus der Abnahme

## Nicht-Ziele

Code-Änderungen.

## Schritte

1. **Ralf:** Spiel mit einer eigenen Welt öffnen, Pausenmenü prüfen.
2. **Ralf:** In der Lobby die Welt bei Räumen und Spielständen prüfen.
3. **Ralf:** Mit zweitem Tab im Raum: offenen Raum per X schließen; zweiter Tab landet in der Lobby; danach Stand löschen.
4. **Claude:** Ergebnis eintragen.

## Fertig, wenn

- [ ] AC-04: Ralf bestätigt.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
