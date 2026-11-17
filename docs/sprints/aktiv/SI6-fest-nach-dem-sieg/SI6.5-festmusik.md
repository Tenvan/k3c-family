# SI6.5 · Festmusik je Welt (B-1002)

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si6
- **Abhängig von:** SI6.4
- **Tickets:** B-1002
- **Kriterien:** AC-05

## Ziel

Von Sieg bis Abfahrt spielt die Festmusik der Welt statt der Tages- und Nachtmusik (B-1002/AC-04).

## Kontext

Ton im nativen Spiel: `native/k3c-game/audio/`; Slots in `data/sounds.json` (Kategorie `musik`, Bus `music`, z. B. `music:tag`, `music:abend`; `wahl` nennt Dateien unter `public/`), Dateien unter `public/audio/`, Klangbild und Mischpegel in `docs/rules/klang.md`. Musik-Varianten gibt es seit B-936 (Sprint SR1 in `docs/sprints/erledigt/` nachlesen, wie die Varianten entstanden sind und abgespielt werden). Neu (Spec `B-1002`): Slots `music:fest` (allgemein) und `music:fest:<welt>` für `sengoku` und `nil`; gespielt als Schleife, solange der Engine-Zustand `festival` gilt (SI6.3), ausgelöst durch `festivalStart` oder beim Laden. Keine neue Abhängigkeit; lässt sich die Musik nicht auf dem Weg von B-936 erzeugen, anhalten und melden. Fachprüfung: Agent `k3c-sound`.

## Erlaubte Dateien

- `native/k3c-game/audio/**`
- `data/sounds.json`
- `public/audio/**`

## Nicht-Ziele

Änderung der übrigen Musik oder der Mischpegel, Festmusik für Polar, neue Werkzeuge oder Abhängigkeiten.

## Schritte

1. Weg der Musik-Varianten aus B-936 prüfen (Sprint SR1, `docs/rules/klang.md`).
2. Drei kurze Fest-Schleifen erzeugen oder auswählen (allgemein, Sengoku, Nil), unter `public/audio/` ablegen.
3. `data/sounds.json`: Slots `music:fest`, `music:fest:sengoku`, `music:fest:nil`.
4. Abspielen: mit `festival` auf die Festmusik wechseln (Rückfall `music:fest`), beim Inselwechsel zurück.
5. Go-Tests: Wechsel bei `festivalStart`, Rückfall, Ende beim Inselwechsel, Laden während des Fests.
6. Fachprüfung `k3c-sound`, `check:diff`.

## Fertig, wenn

- [ ] AC-05: Tests zu B-1002/AC-04 grün (Testnamen im Ergebnis), Slots und Dateien vorhanden, `k3c-sound` ohne schweren Befund.
- [ ] `check_run task:check:diff` grün.

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
