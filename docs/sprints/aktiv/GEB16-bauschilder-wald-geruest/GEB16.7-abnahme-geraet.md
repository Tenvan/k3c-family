<<<<<<<< HEAD:docs/sprints/aktiv/GEB16-bauschilder-wald-geruest/GEB16.6-review.md
# GEB16.6 · Review GEB16
|||||||| 819c53d3:docs/sprints/geplant/GEB17-hub-ausbau-je-inselstufe/GEB17.4-sim-protokoll-hubstep.md
# GEB17.4 · Daten, Simulation und Protokoll setzen die Ausbauschritte um (hubStep)
========
# GEB16.7 · Abnahme am Gerät: Schilder, Wald, Gerüst, Abbau-Symbole
>>>>>>>> origin/develop:docs/sprints/aktiv/GEB16-bauschilder-wald-geruest/GEB16.7-abnahme-geraet.md

<<<<<<<< HEAD:docs/sprints/aktiv/GEB16-bauschilder-wald-geruest/GEB16.6-review.md
- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb16/6-review
- **Abhängig von:** GEB16.4, GEB16.5
- **Tickets:** B-584, B-585, B-586, B-587, B-588
- **Kriterien:** AC-01, AC-02, AC-03, AC-04
|||||||| 819c53d3:docs/sprints/geplant/GEB17-hub-ausbau-je-inselstufe/GEB17.4-sim-protokoll-hubstep.md
- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** geb17/4-sim-protokoll-hubstep
- **Abhängig von:** GEB17.3
- **Tickets:** B-592
- **Kriterien:** AC-03
========
- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb16/7-abnahme-geraet
- **Abhängig von:** GEB16.6
- **Tickets:** B-584, B-585
- **Kriterien:** AC-05
>>>>>>>> origin/develop:docs/sprints/aktiv/GEB16-bauschilder-wald-geruest/GEB16.7-abnahme-geraet.md

## Ziel

Ein bis zwei Sätze: welches Ergebnis nach dieser Session auf `develop` liegt.

## Kontext

Alles, was ein Agent **ohne Vorwissen** braucht: Stand des Codes, relevante Dateien mit Pfad, Entscheidungen
(`docs/decisions/…`), Fallstricke. Keine Verweise auf Chat-Verläufe.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `pfad/…`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Konkrete, nummerierte Schritte in sinnvoller Reihenfolge.

## Fertig, wenn

- [ ] AC-01: Jeder Punkt ist prüfbar: ein Befehl mit erwartetem Ergebnis, ein Test, eine Datei, eine Beobachtung.
- [ ] Punkte ohne Kriterium sind technische Voraussetzungen (z. B. CI grün).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 bis AC-04: geprüft mit `task check` und `task check:go` (grün), per Diff-Lesung, Sichtprüfung der Grafiken und den Review-Befunden zu GEB16.1/.2.
- Die Abnahme steht in der Sprint-README. Neues Ticket B-620.
