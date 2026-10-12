# GR7.3 · Figuren-Lücken: Kandidatenlisten und Auswahl

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** gr7/3-figuren-kandidaten
- **Abhängig von:** –
- **Tickets:** B-193
- **Kriterien:** AC-01

## Ziel

Jede Figuren-Lücke hat drei Kandidaten auf einer Referenzseite unter `docs/funde/`, 🧑 hat je Lücke gewählt (oder „kein Treffer“ bestätigt) und die gewählten Figuren liegen mit Credit unter `public/sprites/`.

## Kontext

- **Agent: Mensch**, weil 🧑 wählt (Q14). Ein Agent darf die Suche **vorbereiten** (Kandidatenlisten) und nach der Wahl einbauen; die Wahl selbst nimmt er nie vor.
- **Figuren-Lücken** (Stand 2026-10-12, prüfen): `boss` (`docs/assets/zuordnung-welt.md`, „Lücke: später, B-130“, Bosse fehlen in `data/`; nur wenn K2 sie liefert) und die sechs Gegner mit Platzhalter-Sheet (B-329): `lavaSlime`, `ironBeetle`, `fireSpirit`, `crystalSpider`, `shardling`, `crystalGuardian` (Zeilen in `docs/assets/zuordnung-objekte.md`, Vermerk „Platzhalter (B-326)“). Der Monarch auf dem Reittier (B-173) ist umgesetzt, keine Lücke.
- **Verfahren** (B-193, Q14): 3 Kandidaten je Lücke, Referenzseite unter `docs/funde/` (Vorbild `docs/funde/gr2-grafik-funde.html`), Lizenzregel Q70 (CC0, CC-BY, OGA-BY, CC-BY-SA; CC-BY-NC nur markiert), Stil Q13 (16/32 px, ×2 bis ×3), Credits sofort in `public/sprites/CREDITS.md`. Ziel: `public/sprites/<sheet>/` mit `idle/run/attack.png`, Eintrag in `data/sprites.json` (Sheet, `enemies.<id>`), Zeile in der Zuordnung ohne Platzhalter-Vermerk. Der ResourcenManager (M10, B-298) steht noch nicht, die Suche läuft von Hand.
- Kein Treffer → Lücke bleibt mit Vermerk, Platzhalter bleibt (Rückfall der Spec).
- Credits-Test: `src/tools/credits.test.ts` verlangt für jedes Verzeichnis unter `public/sprites/` einen Eintrag.

## Erlaubte Dateien

- `docs/funde/` (neue Referenzseite), `docs/assets/zuordnung-objekte.md`, `docs/assets/zuordnung-welt.md`
- `public/sprites/` (neue Ordner, `CREDITS.md`), `data/sprites.json` (nur Sheets und `enemies`), `src/tools/credits.ts`
- `docs/sprints/geplant/GR7-figuren-schrift/`, `docs/sprints/aktiv/GR7-figuren-schrift/` (nur Status), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Umgebungs-Assets (B-162), Einbau neuer Animationen im Renderer (B-010), selbst gezeichnete Figuren, Namen und Werte der Gegner (BR2), Pipeline mit Ziel-Palette (B-331).

## Schritte

1. Lücken-Liste aus den Zuordnungsdateien prüfen und in der Referenzseite festhalten.
2. Je Lücke drei Kandidaten suchen (OpenGameArt, itch.io; nur erlaubte Lizenzen), Quelle, Urheber, Lizenz, Rastergröße und Vorschau auf der Referenzseite `docs/funde/gr7-figuren-funde.html` zeigen (Agent-Vorbereitung).
3. 🧑 wählt je Lücke einen Kandidaten oder bestätigt „kein Treffer“ (Workshop, kein Agent).
4. Gewählte Figuren nach `public/sprites/<sheet>/` kopieren, `CREDITS.md`, `data/sprites.json` und die Zuordnung nachziehen; Platzhalter-Vermerke entfernen oder für Lücken ohne Treffer das Ticket nennen.
5. `task check` (Credits-Test und `src/tools/zuordnung.test.ts` grün), Ergebnis mit Nachweis je Kriterium, `Status: fertig`.

## Fertig, wenn

- [ ] AC-01 (B-193/AC-01): Zu jeder Figuren-Lücke liegt eine Kandidatenliste mit drei Einträgen unter `docs/funde/` vor (weniger nur mit Vermerk).
- [ ] AC-01 (B-193/AC-02): 🧑 hat je Lücke gewählt oder „kein Treffer“ bestätigt.
- [ ] AC-01 (B-193/AC-03): Gewählte Figuren liegen unter `public/sprites/` mit Credit-Eintrag; `task check` grün (Credits- und Zuordnungs-Test).

## Prüfen

```bash
task test -- credits
task test -- zuordnung
task check
```

Die Wahl der Figuren ist eine manuelle Entscheidung von 🧑.

## Ergebnis

**Vorbereitet (2026-10-12), Session bleibt `offen`:** Schritt 1 und 2 erledigt. Referenzseite `docs/funde/gr7-figuren-funde.html` mit bis zu drei Kandidaten je Lücke (`lavaSlime`, `ironBeetle`, `fireSpirit`, `crystalSpider`, `shardling`, `crystalGuardian`) und Auswahlfeld; `boss` ausgelassen (B-130). Lizenzen und Eignung stammen aus Suchergebnissen und sind **ungeprüft**; einige Links sind OpenGameArt-Suchlinks. Für `ironBeetle` gibt es nur zwei Kandidaten (kein CC0-Treffer), `fireSpirit` nur Effekte. Offen: Wahl durch 🧑 (Schritt 3), Einbau (Schritt 4), `task check` (Schritt 5).
