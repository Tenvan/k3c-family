# GR2 · CLI · Grafik-Suche für Lücken

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-162
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Lücken ohne Treffer laut B-010: Mine-Hintergrund, Rekrutierungslager, Werkstatt, Farm, Kaserne, Treppen; weitere kommen mit der Zuordnungstabelle (GR1). Details in B-162.

## Ziel

Zu jeder Lücke gibt es Kandidaten, 🧑 hat gewählt, die Assets liegen mit Credits im Repo. Am Ende sichtbar: Kandidatenseite unter `docs/funde/`, neue Packs unter `public/grafik/`, Tabelle ohne offene Lücke außer „kein Treffer“.

## Beteiligte und Zielgruppen

Der Agent recherchiert; 🧑 wählt je Lücke (Q14).

## Anforderungen

B-162 › Anforderungen.

## Nicht-Ziele

Einbau (GR3), Atlas (GR4), selbst gezeichnete Grafiken, Lizenzen außer CC0 und CC-BY.

## Regeln und Einschränkungen

Credits sofort in `public/grafik/CREDITS.md` und `lizenzen.html` (Test `src/tools/grafikPacks.test.ts`); keine Musik und kein Demo-Code im Repo. Läuft parallel zu GR1, braucht dessen Lückenliste.

## Beispiele

Lücke „Werkstatt“ → drei Kandidaten → Wahl → Pack unter `public/grafik/` mit Credits.

## Ausnahme- und Fehlerfälle

Kein Treffer → Platzhalter bleibt, Vermerk in der Tabelle. Widersprüchliche Lizenzangaben → strengere gilt.

## Akzeptanzkriterien

- **AC-01** Zu jeder Lücke liegt eine Kandidatenliste mit mindestens zwei Einträgen vor (B-162/AC-01).
- **AC-02** 🧑 hat je Lücke gewählt oder „kein Treffer“ bestätigt (B-162/AC-02).
- **AC-03** Gewählte Assets liegen unter `public/grafik/` mit Lizenzdatei, Index- und Credit-Eintrag, `task test` ist grün (B-162/AC-03).
- **AC-04** Die Zuordnungstabelle zeigt für jede entschiedene Lücke den neuen Status (B-162/AC-04).
- **AC-05** `task check` ist grün.

## Offene Fragen

- Welcher Kandidat je Lücke? Entscheidet 🧑 (`docs/fragenkatalog.md` Q14).

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- GR2.1 Recherche: Kandidaten je Lücke mit Vorschau, Lizenz, Stilbewertung (AC-01).
- GR2.2 🧑 Workshop (Agent: Mensch): je Lücke wählen (AC-02).
- GR2.3 Gewählte Assets einbinden, Credits, Tabelle aktualisieren (AC-03, AC-04).
- GR2.4 Review (AC-05).

## Abnahme

–
