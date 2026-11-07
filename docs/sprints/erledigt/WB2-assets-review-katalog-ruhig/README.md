# WB2 · DEV · Review und Katalog: feste Schieber, getrennte Scrollbereiche, Lade-Anzeige

- **Status:** erledigt
- **Projekt:** PAP
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** cb45f3f3
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-07, autonom, Claude

## Ausgangslage

Im Bereich Assets der Workbench (`tools/k3c-workbench/frontend/src/assets/`) springen auf Review und Katalog die Animationsschieber im Playmodus: wechselnde Zahlen (`Frame 9/10` → `10/10`), wechselnde Knopftexte (Abspielen/Anhalten) und im Katalog ein kurz leeres Bild beim Frame-Laden verschieben das Layout. Liste und Detail scrollen gemeinsam mit der Seite, rechts verschwindet das Asset beim Scrollen der Liste. Der Katalog lädt lange ohne Rückmeldung.

## Ziel

Review und Katalog stehen ruhig: Schieber, Zähler und Knöpfe haben feste Breiten, Liste und Detail haben je einen eigenen Scrollbereich in voller Höhe, der Katalog zeigt beim Laden eine Lade-Anzeige.

## Beteiligte und Zielgruppen

🧑 Ralf prüft Assets in der Workbench; Claude setzt autonom um.

## Anforderungen

Auftrag Ralf (über die Session „PR und mergen“, 2026-11-07), Punkte 1–5. Atomic Design über `@webkit` (`docs/arbeitsweise.md` › Oberflächen der Wails-Programme, `tests/atomicDesign.test.ts`).

## Nicht-Ziele

Styleguide-Prompts, Filter und Prompts für Sound und Quellen (nur Tickets, WB2.3); Backend-Tempo des Katalogs; Review-Seite auf WB1-Backend umstellen (B-703).

## Regeln und Einschränkungen

Gemeinsame Bausteine in `tools/webkit`: Molekül `FramePlayer` (Abspielen, Frame-Schieber, Zähler, alles mit fester Breite, Zähler `tabular-nums`), Template `ListDetail` (Liste | Detail, je eigener Scrollbereich, Höhe des Fensters minus Kopf). Datei ≤ 400, Funktion ≤ 60 Zeilen. Tests zuerst; vor dem Commit Sichtprüfung im Browser (Vite der Workbench des Worktrees).

## Beispiele

- Abspielen einer 12-Frame-Animation: Schieber und Zähler behalten Position und Breite von Frame 1 bis 12.
- 80 Assets in der Liste, ganz nach unten gescrollt: das Detail rechts bleibt oben sichtbar.
- Katalog öffnen: „Lädt Assets …“ mit Spinner, bis die Liste da ist; Asset wählen: Spinner bis Spec und erstes Bild da sind; Frame wechseln: altes Bild bleibt mit kleinem Spinner stehen.

## Ausnahme- und Fehlerfälle

- Laden schlägt fehl: Fehlertext statt Spinner, die Liste bleibt bedienbar.
- Schmale Fenster: die Spalten bleiben nebeneinander, die Liste behält ihre Breite.

## Akzeptanzkriterien

- **AC-01** Review: Frame-Schieber, Zähler und Abspiel-Knopf haben feste Breiten, der Zähler nutzt `tabular-nums`; beim Abspielen ändern Schieber und Zähler ihre Position nicht (Test und Browser-Messung).
- **AC-02** Review: Liste und Detail haben je einen eigenen Scrollbereich in voller Höhe; Scrollen der Liste bewegt das Detail nicht (Test und Browser).
- **AC-03** Katalog: Lade-Anzeige beim Laden der Liste, bei der Wahl eines Assets (auch aus einer anderen Gruppe) und beim Frame-Laden; das alte Bild bleibt beim Frame-Laden stehen (Test).
- **AC-04** Katalog: Liste und Detail getrennt scrollbar wie AC-02.
- **AC-05** Katalog: Schieber fest wie AC-01.
- **AC-06** Bausteine über `@webkit` (Molekül, Template), `task check:diff` grün, Review (Opus) ohne offene wichtige Befunde.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WB2.1 | `WB2.1-review-ruhig.md` | Umsetzung | autonom | fertig |
| WB2.2 | `WB2.2-katalog-ruhig.md` | Umsetzung | autonom | fertig |
| WB2.3 | `WB2.3-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-11-07, autonom (Claude). AC-01 und AC-02 WB2.1, AC-03 bis AC-05 WB2.2 (Tests und Browser-Messung im Worktree, Vite 5196), AC-06 WB2.3 (Review Opus ohne offene wichtige Befunde; Bausteine `FramePlayer`, `LoadingNote`, `ListDetail` über `@webkit`). Letztes `task check:diff` auf Wunsch von Ralf abgebrochen, gezielte Prüfungen grün (siehe WB2.3).
Hinweis Ralf umgesetzt: Lade-Anzeige über der Grafik. Zurückgestellt: Review-Befunde 3, 5, 7–10 (klein). Neue Tickets: B-727, B-728, B-729. Sichtprüfung im echten Workbench-Fenster durch Ralf nach dem Merge.
