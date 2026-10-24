# GEB18 · DEV, SIM, CLI · Hintergründe der fünf Stufen: vierfache Auflösung, fünf Varianten je Stufe nach Seed

- **Status:** aktiv
- **Projekt:** GEB
- **Domäne:** DEV, SIM, CLI
- **Reife:** bereit
- **Tickets:** B-618
- **Start-Commit:** f8bee41c
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-24, Revision 1 (Auftrag Ralf im Chat: autonom im Plan eintragen); umfasst B-618 Revision 1

## Ausgangslage

Ralf (2026-10-24): Die Hintergründe der fünf Stufen sind zu grob und zu eintönig. Heute gibt es je Biom eine Fassung mit drei Schichten in 32 px je Unit aus `tools/buildings` (GEB7); Details in B-618.

## Ziel

Die fünf Stufen haben detailreiche Hintergründe in vierfacher Auflösung mit fünf Varianten je Stufe, die nach Seed abwechseln. Am Ende sichtbar: Beim Laufen durch eine Stufe wechseln die Hintergründe sichtbar zwischen Varianten, gleicher Seed zeigt auf PC und Xbox dieselbe Folge; das Studio zeigt alle Varianten in der Szene „Umgebung“.

## Beteiligte und Zielgruppen

Spielende; Agenten DEV, SIM, CLI (mit Protokoll-Teil SRV in GEB18.5); 🧑 beurteilt das Aussehen am Gerät.

## Anforderungen

B-618 › Anforderungen.

## Nicht-Ziele

Neue Biome; Vordergrund-Objekte (GEB15, GEB16); Parallax-Faktoren; Balancing.

## Regeln und Einschränkungen

Auswahl der Variante nur in Go mit `engine/rng`, kein Zufall im Client; Protokolländerung additiv, Version bleibt; Golden nur mit Begründung; Palette und Regeln aus GEB7; selbst erzeugt oder CC0; Xbox-Speicher im Budget (nur Varianten der gezeigten Stufen laden); Aufräum-Regel des Projekts GEB: alte Schichten ohne Variante löscht GEB18.5, sobald der Client sie nicht mehr nutzt; GEB18.2 und GEB18.3 laufen parallel (verschiedene Vorlagen), GEB18.4 parallel dazu.

## Beispiele

B-618 › Beispiele.

## Ausnahme- und Fehlerfälle

B-618 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Generator: vierfache Auflösung (Dichte 128) und Varianten (B-618/AC-01).
- **AC-02** Je Stufe mindestens fünf Varianten mit drei Schichten in vierfacher Auflösung, im Katalog und in der Studio-Szene „Umgebung“ (B-618/AC-02).
- **AC-03** Variante je Abschnitt deterministisch nach Seed, Nachbarn verschieden (B-618/AC-03).
- **AC-04** Protokoll-Feld beschrieben, Client zeichnet die gemeldete Variante und lädt nur Benötigtes (B-618/AC-04).
- **AC-05** 🧑 nimmt am Gerät ab, Xbox-Speicher und Ladezeit im Budget (B-618/AC-05).
- **AC-06** Alte Schichten ohne Variante gelöscht, keine Verweise mehr (B-618/AC-06).

## Offene Fragen

keine blockierende. Defaults (Claude, autonom, 2026-10-24): Dichte 128; Auswahl im Go-Level-Generator; Abschnittslänge = Kachelbreite der Schicht „mitte“. Ralf bestätigt oder ändert in GEB18.7.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB18.1 | `GEB18.1-generator-dichte-varianten.md` | Umsetzung | autonom | fertig |
| GEB18.2 | `GEB18.2-varianten-wald-hoehle-mine.md` | Umsetzung | autonom | fertig |
| GEB18.3 | `GEB18.3-varianten-eisenstollen-kristall.md` | Umsetzung | autonom | fertig |
| GEB18.4 | `GEB18.4-auswahl-nach-seed.md` | Umsetzung | autonom | fertig |
| GEB18.5 | `GEB18.5-protokoll-client-varianten.md` | Umsetzung | autonom | fertig |
| GEB18.6 | `GEB18.6-review.md` | Review | autonom | fertig |
| GEB18.7 | `GEB18.7-abnahme-geraet.md` | Workshop | Mensch | offen |

- GEB18.1 (DEV) `tools/buildings`: Dichte 128 erlauben, Vorlagen-Format und Ablage je Variante, Go-Tests (AC-01).
- GEB18.2 (DEV) Vorlagen Wald, Höhle, Mine: je fünf detailreiche Varianten, drei Schichten (AC-02).
- GEB18.3 (DEV) Vorlagen Eisenstollen, Kristall: je fünf Varianten; Katalog-Slots und Studio-Szene „Umgebung“ mit allen Varianten (AC-02).
- GEB18.4 (SIM) Level-Generator wählt Variante je Abschnitt nach Seed, Nachbarn verschieden, Golden (AC-03).
- GEB18.5 (CLI mit SRV-Teil) Feld im Level und `docs/protocol.md`, Client zeichnet die Variante, lädt nur Benötigtes, Speichermessung (AC-04).
- GEB18.6 Review: alle Kriterien.
- GEB18.7 (Mensch) Abnahme am Gerät (AC-05).

## Abnahme

2026-10-24, Claude (GEB18.6): `task check`, `task check:go` und `task check:race` grün. Der Race-Lauf kam aus dem Go-Testcache, siehe B-619.
AC-01 bis AC-04 und AC-06 sind laut den Ergebnissen von GEB18.1 bis GEB18.5 erfüllt. Claude hat den Diff gelesen (backdrop.go, objectView/stageView) und keine schweren Befunde gefunden.
AC-05 fehlt noch, das ist die Geräte-Abnahme durch Ralf in GEB18.7. Der Sprint bleibt aktiv, bis sie erledigt ist.
