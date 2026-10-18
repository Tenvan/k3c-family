# WZ8.1 · Workshop Format und Regeln geplanter Chunks

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** wz8/1-workshop-format
- **Abhängig von:** –
- **Tickets:** B-461
- **Kriterien:** AC-01

## Ziel

Format und Regeln geplanter Chunks sind von 🧑 beschlossen (AC-01): Regeltext in `docs/rules/`, Glossar-Einträge und korrigierte Chunk-Breite sind committet. Erst danach beginnt WZ8.2.

## Kontext

Spec: B-461 und die Sprint-README WZ8 (Entwurf). Der Workshop entscheidet 🧑, Claude bereitet vor und schreibt auf. Claude beschließt nichts selbst.

**Vorbereitung durch Claude:** B-461 › Anforderungen (Format, Stromnamen, Lage-Regeln) und Messwerte-Tabelle bereitlegen, `docs/rules/stufen.md` und `docs/glossar.md` lesen, Format-Vorschlag mit den Annahmen aus B-461 als Entscheidungsvorlage.

**Zu entscheiden (B-461 › Offene Fragen 6–9):**
1. Chunk-Platz relativ zu den Hub-Chunks (Annahme) oder zu Tiefen-Eingang und Rand.
2. Objektliste ersetzt die gewürfelten Ressourcen der Chunk-Art (Annahme) oder ergänzt sie; feste Skill-Punkte (Annahme: nein).
3. Lage-Regeln: 4 Units Abstand zu Bauplätzen nur für Adern (Annahme) oder auch für Truhen und Camps; Stromnamen `<biom>:<seed>:chunk:<platz>` und `<biom>:chunk:<chunk-seed>`.
4. Übrige Portale abwechselnd, beginnend auf der Seite mit weniger festen Portalen (Annahme); Anrechnung fester Truhen und Camps auf die Ereignis-Chunks.

Außerdem: Format-Felder für Teil 2 (Gebäude, Gegner, B-468) reservieren; Glossar-Befund: Chunk ist in Eisenstollen und Kristallhöhle 25 Units breit.

Weicht der Beschluss von den Annahmen in B-461 ab, wird zuerst die Spec angepasst (Revision + 1, neue Freigabe), dann beginnt WZ8.2.

## Erlaubte Dateien

- `docs/rules/` (neue Regeldatei für geplante Chunks, z. B. `docs/rules/geplante-chunks.md`, Format Regel · Begründung · Daten)
- `docs/glossar.md`
- Planungsdateien von WZ8 und B-461 (über k3c-dev)

## Nicht-Ziele

Code oder `data/*.json` ändern; die Spec freigeben (nur 🧑); Ruinen und Gegnerlager regeln (B-468); Werte oder Balancing ändern; ohne 🧑 entscheiden.

## Schritte

1. Claude bereitet die Entscheidungsvorlage vor (Kontext) und legt sie 🧑 vor.
2. Workshop: 🧑 entscheidet die Fragen 6–9; Claude hält Beschluss, Datum und Quelle fest.
3. Regeltext in `docs/rules/` schreiben: Ort und Format (JSON-Beispiel), Chunk-Platz mit Formel, Chunk-Art und -Seed, feste Objekte mit Lage, Stromnamen, Anrechnung auf die Biom-Summen, Lage-Regeln, Fehlerfälle, reservierte Felder für Teil 2.
4. Glossar: „geplanter Chunk“, „Chunk-Platz“, „Chunk-Seed“ neu; „Ruine“ und „Gegnerlager“ als geplant (B-468); Chunk-Breite korrigieren (25 Units in Eisenstollen und Kristallhöhle).
5. Weicht der Beschluss von B-461 ab: B-461 und Sprint-README anpassen, Revision + 1, `Spec: Entwurf`; Freigabe holt 🧑.
6. `task test -- planning` grün; Commit `docs(reg): Regeln geplanter Chunks (WZ8.1)`.

## Fertig, wenn

- [ ] AC-01: Beschluss von 🧑 zu Frage 6–9 mit Datum und Quelle im Ergebnis.
- [ ] AC-01: Regeltext in `docs/rules/` enthält Format, Platzformel, Stromnamen, Summen-Anrechnung, Lage-Regeln.
- [ ] AC-01: Glossar enthält die drei neuen Begriffe, „Ruine“/„Gegnerlager“ als geplant und die korrigierte Chunk-Breite.
- [ ] `task test -- planning` grün.

## Prüfen

```bash
task test -- planning
```

Der Workshop selbst ist die manuelle Prüfung von 🧑.

## Ergebnis

- **AC-01 umgesetzt:** Workshop am 2026-10-17 im Chat: 🧑 (Ralf) übernimmt alle Annahmen aus B-461 › Offene Fragen 6–9 unverändert (Beschluss 🧑, Quelle Chat). Keine Abweichung, daher keine Spec-Revision.
  - Frage 6: Chunk-Platz relativ zu den Hub-Chunks.
  - Frage 7: Objektliste ersetzt die gewürfelten Ressourcen; keine festen Skill-Punkte.
  - Frage 8: 4 Units Abstand zu Bauplätzen nur für Adern; Stromnamen `<biom>:<seed>:chunk:<platz>` und `<biom>:chunk:<chunk-seed>`; leerer Chunk-Seed = kein Chunk-Seed.
  - Frage 9: übrige Portale abwechselnd, beginnend auf der Seite mit weniger festen Portalen; feste Truhen und Camps rechnen auf die Ereignis-Chunks an.
- **Regeltext:** `docs/rules/geplante-chunks.md` (Format, Platzformel, Inhalt, Stromnamen, Summen, Lage-Regeln, Fehlerfälle, Schreiben).
- **Glossar:** neu „geplanter Chunk“, „Chunk-Platz“, „Chunk-Seed“, „Ruine (geplant)“, „Gegnerlager (geplant)“; Chunk-Breite korrigiert (25 Units in Eisenstollen und Kristallhöhle).
- **Geprüft:** `tests/planning.test.ts` grün. Entschieden von 🧑, aufgeschrieben von Claude. **Neue Tickets:** keine.
