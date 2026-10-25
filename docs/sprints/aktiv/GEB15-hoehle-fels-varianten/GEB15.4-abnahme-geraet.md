<<<<<<<< HEAD:docs/sprints/aktiv/GEB16-bauschilder-wald-geruest/GEB16.7-abnahme-geraet.md
# GEB16.7 · Abnahme am Gerät: Schilder, Wald, Gerüst, Abbau-Symbole
|||||||| 819c53d3:docs/sprints/geplant/GEB17-hub-ausbau-je-inselstufe/GEB17.6-grafik-wachstumsreihe.md
# GEB17.6 · Eigene Grafik je Schritt der Wachstumsreihe
========
# GEB15.4 · Abnahme am Gerät: Wurzeln, Felsen, Minen
>>>>>>>> origin/develop:docs/sprints/aktiv/GEB15-hoehle-fels-varianten/GEB15.4-abnahme-geraet.md

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
<<<<<<<< HEAD:docs/sprints/aktiv/GEB16-bauschilder-wald-geruest/GEB16.7-abnahme-geraet.md
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb16/7-abnahme-geraet
- **Abhängig von:** GEB16.6
- **Tickets:** B-584, B-585
- **Kriterien:** AC-05
|||||||| 819c53d3:docs/sprints/geplant/GEB17-hub-ausbau-je-inselstufe/GEB17.6-grafik-wachstumsreihe.md
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb17/6-grafik-wachstumsreihe
- **Abhängig von:** GEB17.3
- **Tickets:** B-590
- **Kriterien:** AC-05
========
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** geb15/4-abnahme-geraet
- **Abhängig von:** GEB15.3
- **Tickets:** B-582, B-583
- **Kriterien:** AC-03
>>>>>>>> origin/develop:docs/sprints/aktiv/GEB15-hoehle-fels-varianten/GEB15.4-abnahme-geraet.md

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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
