# SI4.1 · Regeltext Reittiere je Welt (B-970)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/si4
- **Abhängig von:** –
- **Tickets:** B-970
- **Kriterien:** AC-01

## Ziel

`docs/rules/monarch.md` § 7 beschreibt Reittier-Katalog, Gatter, Kauf, Mitfahren und die Stärken der Welt-Reittiere (B-970/AC-01).

## Kontext

Quelle ist `B-970 › Anforderungen` samt Defaults in `B-970 › Offene Fragen` (Entscheidungen 🧑 2026-11-15; Defaults vorläufig). Heute sagt `docs/rules/monarch.md` § 7: jeder Monarch reitet das Standard-Pferd (`data/monarch.json` › `mount`, Faktoren 1,0), „immer beritten: das Reittier wird weder gekauft noch verloren“, „weitere Reittiere gibt es erst später“. Diese beiden Zeilen werden angepasst. Der Sprint hat heute keine Ausdauer (kein solches Feld in `data/monarch.json`); deshalb ist die Stärke des Kamels vorläufig ein höheres Grundtempo. Eine Welt Polar gibt es nicht; das Rentier steht als geplant im Text. Tabellenform der Datei beibehalten (Regel, Begründung, Daten, Zielkorridor).

## Erlaubte Dateien

- `docs/rules/monarch.md`

## Nicht-Ziele

Code, Daten-Dateien, `welten.md` (Feld `mount` trägt SI4.3 ein), Sprint-Ausdauer einführen, Werte abstimmen.

## Schritte

1. `B-970` und `monarch.md` § 7 vollständig lesen.
2. Zeilen ergänzen: Katalog `data/monarch.json` › `mounts`, Welt wählt ein Reittier oder keins, Gatter an der Oberfläche jeder Insel, Kauf je Monarch, Reittier bleibt bis zum Ende der Welt und belegt keinen Frachtplatz (B-967).
3. Stärken: Sengoku-Pferd `sprintFactor`, Kamel `speedFactor`, Rentier geplant; kein Nachteil.
4. Zeilen „immer beritten“ und „weitere Reittiere“ anpassen (kein Absteigen bleibt, Kauf ist neu).
5. Ausnahmefälle aus B-970; alle Zahlen als vorläufig; Stand-Zeile nachziehen.

## Fertig, wenn

- [ ] AC-01: § 7 enthält alle Regeln aus B-970, die alten Zeilen widersprechen nicht mehr.
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
