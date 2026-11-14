# NG7.3 · Spielstand-Menü: Liste, Weiter, Laden, Löschen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** menue
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** ng7/3-spielstaende
- **Abhängig von:** NG7.2
- **Tickets:** B-800
- **Kriterien:** AC-03

## Ziel

Das Spielstand-Menü listet, lädt und löscht Spielstände (Löschen nur nach Rückfrage); „Weiter“ lädt den neuesten.

## Kontext

Spec: B-800. Vorbild: `src/scenes/saveMenu.ts`, `loadLogic.ts`, `LoadScene.ts` samt Tests. Daten nur über `view.Session` (Liste mit Zeit, Insel, ladbar ja/nein). Vertrag und Importregel wie NG7.1.

## Erlaubte Dateien

- `native/k3c-game/menu/**`
- Planung NG7, B-800 (über MCP)

## Nicht-Ziele

Speichern selbst (`local`, NG1.2); Migration alter Stände.

## Schritte

1. Tests gegen eine Schein-`view.Session` (rot): leere Liste, mehrere Stände, nicht ladbar, Löschen mit Rückfrage.
2. Spielstand-Bildschirm.
3. „Weiter“ im Hauptmenü an den neuesten Stand binden.

## Fertig, wenn

- [ ] AC-03: Tests grün.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-03 umgesetzt: Spielstand-Liste aus `Session.List` (neueste zuerst, rollt), Laden mit dem bestätigenden Gerät, nicht ladbare Stände grau mit Grund, Löschen mit Rückfrage, Fehler als Meldung, „Weiter“ lädt den neuesten ladbaren Stand. Geprüft mit `saves_test.go`, `main_test.go` und an Bildern `ng7-3-saves*.png`.
Abweichung: Löschen liegt auf View, weil der Vertrag keine X/Y-Aktion hat.
