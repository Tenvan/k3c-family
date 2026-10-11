# WZ2.3 · Planungsseite: Flags bearbeiten

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** sprint/wz2
- **Abhängig von:** WZ2.1
- **Tickets:** B-363
- **Kriterien:** AC-04

## Ziel

Im Detail eines Tickets lassen sich Prio, Umgebung und Projekt ändern, im Detail einer offenen Session Agent und Umgebung; abgelehnte Werte ändern nichts.

## Kontext

- Spec: `docs/backlog/B-363-planung-flags-bearbeiten.md` (AC-01 bis AC-03); Entscheidungen 🧑 2026-10-11: nur offene Sessions, keine Sprint-Felder.
- `SessionDetail` in `tools/k3c-dev/frontend/src/planning/Backlog.tsx` stellt `Agent` und `Umgebung` per `FieldMenu` um, nur für offene Sessions (`pickable`). Tickets haben keine Bearbeitung.
- Änderungen laufen über `backend.planningSet` → `plan_set` (wie der Rang); Index und Tabellen zieht k3c-dev nach.
- Gültige Werte: Prio hoch/mittel/niedrig, Umgebung offline/live/?, Projekt = Kürzel der Projekte (`projects.ts`).
- Die Meldung des Tools bei abgelehntem Wert wird unverändert angezeigt.

## Erlaubte Dateien

- `tools/k3c-dev/frontend/src/planning/*.tsx`, `*.ts` und zugehörige `*.test.ts(x)`
- `tools/k3c-dev/internal/planning/` nur, falls ein Go-Test auf einer Kopie von `docs/` nötig ist

## Nicht-Ziele

Agent und Umgebung bei `fertig` oder `verworfen` änderbar machen; Felder am Sprint; weitere Felder (`Status`, `Rang`); Optik der Planungsseite umbauen.

## Schritte

1. Test zuerst (rot): Ticket-Detail zeigt Menüs für Prio, Umgebung, Projekt; Auswahl ruft `planningSet` mit dem Feld (B-363/AC-01).
2. Menüs im Ticket-Detail ergänzen (`FieldMenu` wiederverwenden).
3. Test: Session-Detail bietet Agent/Umgebung nur bei offenen Sessions an (B-363/AC-02).
4. Test: abgelehnter Wert (Projekt unbekannt) zeigt die Tool-Meldung und ändert nichts (B-363/AC-03).
5. Mit Mock-Daten prüfen, dass Ticket-Datei und `docs/backlog/README.md` nach der Änderung stimmen (Go-Test auf Kopie von `docs/`, falls noch nicht vorhanden).
6. `task check:dev` ausführen, Ergebnis eintragen. Die Abnahme am Bildschirm gehört 🧑 (WZ2.7).

## Fertig, wenn

- [ ] AC-04: Tests für Ticket-Menüs, Session-Menüs nur bei offenen Sessions und abgelehnten Wert grün (`B-363/AC-01`–`AC-03`).
- [ ] `task check:dev` grün; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Prüfen

```bash
task check:dev
```

Bei laufendem k3c-dev stattdessen `check_run` mit `task:check:dev`. Abnahme am Bildschirm nur durch 🧑.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
