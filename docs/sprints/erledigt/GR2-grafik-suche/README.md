# GR2 · CLI · Grafik-Suche für Lücken

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-162
- **Start-Commit:** e317292
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 2, durch 🧑 (nicht gewählte Kandidaten in den Bestand, AC-06, für einen späteren Auswahl-Workshop auf einer GrafikManager-Seite, B-252); Revision 1: 2026-10-03, Chat (Ralf); umfasst B-162; mit Änderungen aus dem Spec-Review (3 Kandidaten nach Q14, Figuren-Lücken als B-193, Reihenfolge nach GR1)

## Ausgangslage

Lücken ohne Treffer laut B-010: Mine-Hintergrund, Rekrutierungslager, Werkstatt, Farm, Kaserne, Treppen; weitere kommen mit der Zuordnungstabelle (GR1). Details in B-162.

## Ziel

Zu jeder Lücke gibt es Kandidaten, 🧑 hat gewählt, die Assets liegen mit Credits im Repo. Am Ende sichtbar: Kandidatenseite unter `docs/funde/`, neue Packs unter `public/grafik/`, Tabelle ohne offene Lücke außer „kein Treffer“.

## Beteiligte und Zielgruppen

Der Agent recherchiert; 🧑 wählt je Lücke (Q14).

## Anforderungen

B-162 › Anforderungen.

## Nicht-Ziele

Einbau (GR3), Atlas (GR4), selbst gezeichnete Grafiken, Lizenzen außer CC0 und CC-BY, Figuren-Lücken unter `public/sprites/` (B-193).

## Regeln und Einschränkungen

Credits sofort in `public/grafik/CREDITS.md` und `lizenzen.html` (Test `src/tools/grafikPacks.test.ts`); keine Musik und kein Demo-Code im Repo. Läuft nach GR1, braucht dessen Lückenliste (GR2.1 nach GR1.3).

## Beispiele

Lücke „Werkstatt“ → drei Kandidaten → Wahl → Pack unter `public/grafik/` mit Credits.

## Ausnahme- und Fehlerfälle

Kein Treffer → Platzhalter bleibt, Vermerk in der Tabelle. Widersprüchliche Lizenzangaben → strengere gilt.

## Akzeptanzkriterien

- **AC-01** Zu jeder Lücke liegt eine Kandidatenliste mit drei Einträgen vor, weniger nur mit Vermerk, wenn die Suche nicht mehr hergibt (Q14, B-162/AC-01).
- **AC-02** 🧑 hat je Lücke gewählt oder „kein Treffer“ bestätigt (B-162/AC-02).
- **AC-03** Gewählte Assets liegen unter `public/grafik/` mit Lizenzdatei, Index- und Credit-Eintrag, `task test` ist grün (B-162/AC-03).
- **AC-04** Die Zuordnungstabelle zeigt für jede entschiedene Lücke den neuen Status (B-162/AC-04).
- **AC-05** `task check` ist grün.
- **AC-06** Nicht gewählte Kandidaten mit CC0 oder CC-BY liegen ebenfalls unter `public/grafik/` (Gruppe `kandidaten` in `index.json`) mit Lizenzdatei und Credit; sie sind nicht zugeordnet. CC-BY-SA bleibt draußen.

## Offene Fragen

- Welcher Kandidat je Lücke? Entscheidet 🧑 (`docs/fragenkatalog.md` Q14).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GR2.1 | `GR2.1-recherche-kandidaten.md` | Umsetzung | autonom | fertig |
| GR2.2 | `GR2.2-workshop-auswahl.md` | Workshop | Mensch | fertig |
| GR2.3 | `GR2.3-assets-einbinden.md` | Umsetzung | autonom | fertig |
| GR2.4 | `GR2.4-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-10-04, Agent (Claude Opus) in GR2.4, leichtes Review. AC-01 bis AC-04: Ergebnisse GR2.1 bis GR2.3 (Teil 1 schon mit #126 auf `develop`; AC-02 Workshop mit 🧑); AC-06: Diff `origin/develop...origin/sprint/gr2` geprüft (20 Packs, 93 PNG, je `LICENSE.txt`, Credit und `lizenzen.html`-Eintrag, nur CC0/CC BY, Gruppe `kandidaten`, in keiner Zuordnungsdatei, Test ergänzt statt gelockert); AC-05: `task check` und `task check:go` grün.
Schwere Befunde: keine. Offen für 🧑: Ansicht `grafiken.html` im Browser (GR2.3 Schritt 5). B-162 archiviert.
Version: v0.6.1 vorgeschlagen (Patch: nur Bild-Assets, Credits, Doku und Test, keine neue Funktion; v0.6.1 ist der GR1-Vorschlag, falls dessen Tag noch fehlt, beide zusammen); gesetzt erst nach Bestätigung durch 🧑.
