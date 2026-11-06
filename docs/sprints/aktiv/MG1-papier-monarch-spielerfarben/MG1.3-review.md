# MG1.3 · Sprint-Review MG1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** monarch
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/mg1
- **Abhängig von:** MG1.2
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Sprint MG1 ist reviewt (frischer Reviewer), Befunde behoben oder als Ticket, Abnahme in der README.

## Kontext

Diff `origin/feature/pap..sprint/mg1`: nur `public/grafik/figuren/**` und Planung. Kriterien AC-01–AC-03 der README.

## Erlaubte Dateien

- `public/grafik/figuren/**`
- `docs/sprints/aktiv/MG1-papier-monarch-spielerfarben/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine Sichtprüfung durch 🧑 (PS1).

## Schritte

1. Reviewer-Agent über den Sprint-Diff.
2. Befunde beheben (über MCP) oder Ticket.
3. `task check`, Abnahme-Abschnitt füllen, Commit.

## Fertig, wenn

- [x] AC-01–AC-03 mit Nachweis aus MG1.1/MG1.2 bestätigt.
- [x] `task check` grün, Abnahme ausgefüllt.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- Review durch einen frischen Opus-Agenten über `44f2d30c..352c3042`: nichts Kritisches. Ausgabe konsistent (renderKey, 25 Golden-Dateien mit SHA-256, Varianten); B-707–B-711 am Code bestätigt.
- Behoben: B-711 auf Prio mittel; Beispiel in der README, Häkchen und Branch-Feld in MG1.1/MG1.2; B-710 korrigiert (780 Zeilen) und um die doppelten `@spieler1`-PNGs ergänzt; fehlende Previews nachgeholt (spieler3 idle 4, spieler4 galopp 1, spieler1 galopp 7).
- Vorgabe Ralf (über die Haupt-Session): Umhang `smaragd`, Länge 79.8 (D-MG1-05). Mit `assets_set` umgesetzt, Rahmen geprüft: Umhang in allen Frames innerhalb (Idle min. x 9 px, Reiten ohne grüne Randpixel). Danach Rockfarben neu: spieler2 `koenigsrot`, spieler3 `purpur`, spieler4 `leder` (D-MG1-03), weil spieler2 mit `smaragd` kaum Kontrast zum Umhang hätte.
- Offen, nicht behoben: Unterscheidbarkeit (spieler1/spieler3, spieler4 gegen das Pferd) → Sichtprüfung 🧑 in PS1; Satteldecke blau → B-711.
- `task check`: unter Last 1 Timeout (wechselnde Tests), einzeln 3178/3178 grün; `tools/paper` + `planning` 1115/1115 grün; `check:race` hier nicht nötig (kein Go-Code).
