# SI6.4 · Jubel, Tanz und Himmels-Effekt je Welt im Spiel (B-1002)

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si6
- **Abhängig von:** SI6.3
- **Tickets:** B-1002
- **Kriterien:** AC-04

## Ziel

Solange das Fest gilt, zeigt das Spiel im Hub tanzende Bürger, jubelnde Truppen und den Himmels-Effekt der Welt (B-1002/AC-02, AC-03, AC-06).

## Kontext

Das native Spiel (`native/k3c-game`) zeichnet nur; Zustand aus der Engine (SI6.3: `festival`, Ereignis `festivalStart`). Figuren: `native/k3c-game/actors/`; Effekte: `native/k3c-game/fx/`; Himmel und Wasser: `native/k3c-game/world/` (`backdrop.go`, `water.go`); Darstellungs-Daten: `data/native.json`. Himmels-Effekt je Welt-ID aus `data/native.json` › `festival` (neu): `sengoku` steigende Laternen, `nil` Fackeln am Ufer und Blüten auf dem Wasser; Welt ohne Eintrag: kein Effekt. Assets `fest-laterne`, `fest-bluete`, `fest-fackel` aus SI6.2. Tanz: vorhandene Posen der Bürger gespiegelt und versetzt, keine neue Animation. Split-Screen: beide Hälften zeigen das Fest, wenn der Hub im Bild ist. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Erlaubte Dateien

- `native/k3c-game/**`
- `data/native.json`

## Nicht-Ziele

Festmusik (SI6.5), Nordlicht, neue Animationen, Feuerwerk, Engine-Änderungen (fehlt ein Zustand: Ticket).

## Schritte

1. Zustand aus SI6.3 lesen.
2. `data/native.json` › `festival`: Zuordnung Welt-ID → Effekt, Lader mit Prüfung.
3. Freie Bürger im Hub tanzen, Truppen jubeln am Posten; Arbeitende unverändert.
4. Himmels-Effekte: Laternen (steigen, begrenzte Zahl), Fackeln und Blüten (am Ufer und auf dem Wasser); bei Wetter-Darstellung hat das Wetter Vorrang am Himmel.
5. Go-Tests: mit und ohne `festival`, Zuordnung je Welt, Welt ohne Eintrag.
6. Fachprüfung `k3c-grafik`, `check:diff`.

## Fertig, wenn

- [ ] AC-04: Tests zu B-1002/AC-02 und AC-03 grün (Testnamen im Ergebnis); `check_run task:check:diff` grün.
- [ ] Die Beobachtung am TV (B-1002/AC-05) bleibt bei 🧑 und ist im Ergebnis als offen genannt.

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

–
