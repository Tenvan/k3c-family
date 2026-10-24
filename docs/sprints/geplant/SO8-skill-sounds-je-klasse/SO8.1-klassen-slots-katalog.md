# SO8.1 · Vier Klassen-Slots im Katalog

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** so8/1-klassen-slots-katalog
- **Abhängig von:** –
- **Tickets:** B-576
- **Kriterien:** AC-01

## Ziel

Der Katalog hat vier Slots `sfx:skill:tank`, `sfx:skill:mage`, `sfx:skill:healer`, `sfx:skill:thief` mit je mindestens einem CC0-Kandidaten an Platz 1 und Credit; der Sound-Atlas ist neu erzeugt.

## Kontext

Spec `docs/sprints/geplant/SO8-skill-sounds-je-klasse/README.md` und B-576. Vorhandene Slots in `data/assets.json` (`sfx:skill`, `sfx:revived` …) dienen als Muster; Kandidaten liegen unter `public/audio/kandidaten/` (Kenney, CC0), Credits in `public/audio/README.md`. Linien in `data/monarch.json › lines`: tank, mage, healer, thief. Auswahl-Default: Tank dumpf/metallisch, Zauberer hell/mystisch, Heiler weich/aufsteigend, Dieb kurz/leise; kindgerecht, keine lauten Töne. 🧑 bestätigt später in SO8.4.

## Erlaubte Dateien

- `data/assets.json`, `public/audio/kandidaten/` (neue Kenney-Dateien), `public/audio/README.md`, `public/audio/atlas.*`, `src/tools/credits.ts` falls nötig
- Download nur von kenney.nl (CC0), keine anderen Quellen; Datei, Quelle und Größe im Ergebnis nennen

## Nicht-Ziele

Auslöser im Client (SO8.2), Ton je einzelnem Skill, Pegel-Anpassung.

## Schritte

1. Pro Linie einen passenden Kenney-Ton aus bereits vorhandenen Kenney-Paketen wählen (zuerst vorhandene Kandidaten wiederverwenden, nur sonst aus den Kenney-Paketen holen).
2. Slots `sfx:skill:<linie>` in `data/assets.json` anlegen (Format wie `sfx:skill`, `bus: sfx`), Credits eintragen.
3. `task gen:soundatlas` ausführen.
4. Test: Katalog und Credits (`task test -- credits`, `task test -- assets`) und `task check:diff`.

## Fertig, wenn

- [ ] AC-01: Vier Slots mit Kandidaten und Credit im Katalog, Atlas neu erzeugt, Credits-Test grün.

## Prüfen

`task test -- credits`, danach `task check:diff` einmal vor dem Commit.

## Ergebnis

–
