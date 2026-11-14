# 010 · Asset-Pakete je Welt

- **Status:** Entwurf (wartet auf Entscheidung von Ralf)
- **Datum:** 2026-11-14
- **Projekt:** PAP · Tickets: B-880 (dieser Plan), B-879 (Welten-Editor), B-827 (`game:dist`)
- **Vorgänger:** 005 (Papierschnitt-Assets nur über MCP), 008 (natives Spiel), 009 (nur noch das native Spiel am PC)

## Ausgangslage

Stand der Bestandsaufnahme (PT6.1, 2026-11-14), nur gelesen, nichts geändert:

- **Zuordnung Rolle → Asset** steht global in `data/paper.json` (Spieler, Reittiere, Truppen, Gegner, Figurenhöhen,
  Hintergrund-Gruppe). Sie kennt keine Welt.
- **Grafik** liegt je Gruppe unter `public/grafik/<gruppe>/` (Gruppen: bosse, figuren, gegner, gebaeude-paper,
  hintergruende, icons, objekte, reittiere, truppen). Jede Gruppe hat `index.paper.json`; je Asset gibt es `manifest.json`
  und PNG-Sheets. PNG und Manifest sind nicht eingecheckt (B-737): `task gen:paper` rendert sie nach `.work/grafik/` und
  kopiert Geändertes nach `public/grafik/`.
- **Welten** sind Dateien unter `data/worlds/<id>.json` (B-680; `extends` für Kind-Welten, Inseln, Tiefen, `depthScaling`,
  `plannedChunks`). Einzige eingebettete Welt ist `standard` (`data/embed.go`: `*.json biomes/*.json worlds/standard.json`);
  weitere Welten und `data/` insgesamt liest die Engine von der Platte (`K3C_DATA_DIR`).
- **Werte** (Biome, Gegner, Truppen, Wellen, Ton `sounds.json`, Licht, Balance) liegen global in `data/*.json`; eine Welt
  überschreibt sie über Weltwerte (`engine/sim/world_values*.go`), Grafik und Ton überschreibt sie nicht.
- **Laden im nativen Spiel:** `native/k3c-game/art` (`Library`, `Config` mit `GrafikDir = public/grafik`, `RigDir`,
  `FontPath`) liest relativ zur Repo-Wurzel bzw. `K3C_ROOT`. Es lädt je Level genau die Sheets, die das Level braucht
  (`SetLevels`, Level → `gruppe/id`). `k3c-game --assets` nennt alle Assets aller Level und des Menüs.
- **Auslieferung:** `game:dist` (`native/Taskfile.yml`) schreibt `assets.txt` aus `--assets` und kopiert über
  `cmd/sync` nur diese Ordner nach `dist/k3c-game/public/grafik/` (dazu EXE, Ton, Schrift, Rig-Teile). Das Spiel ist damit
  heute schon ein Paket, aber **ein** Paket für **alle** Welten.

Lücke: Es gibt keine Einheit „Welt + ihre Assets“. Welche Assets eine Welt braucht, ergibt sich nur indirekt (Biome der
Inseln → Level → Assets); Ton und Werte hängen nicht an der Welt; eine zweite Welt mit eigener Optik müsste die globale
`paper.json` verbiegen.

## Anforderungen an die Lösung

1. Eine Welt lässt sich samt Grafik (und, zu klären, Ton und Werten) als Einheit ausliefern und laden.
2. Entscheidung 005 bleibt: Assets ändern nur die MCP-Tools `assets_*`; das Paket ist eine Ableitung, keine zweite Quelle.
3. Entscheidung 008/009: Das native Spiel liest Dateien vom Datenträger, kein Server, kein Netz.
4. Der Welten-Editor (B-879) soll eine Welt anlegen und Assets zuordnen können, ohne `paper.json` von Hand zu ändern.

## Varianten

### A · Manifest je Welt, Assets bleiben im gemeinsamen Pool (logisches Paket)

Neue Datei `data/worlds/<id>.assets.json` (oder Feld `assets` in `<id>.json`): Liste der Assets (`gruppe/id`),
Rollen-Überschreibungen (wie `paper.json`, nur Abweichungen), optional Ton-Überschreibungen. `public/grafik/` bleibt der
eine Pool. Das Spiel löst beim Laden einer Welt Rolle → Asset zuerst im Welt-Manifest, dann in `paper.json` auf.

- Vorteile: kleinste Änderung; kein Kopieren, kein doppeltes Rendern; `gen:paper` unverändert; Pflege von Assets bleibt an
  einer Stelle; Editor schreibt nur eine kleine JSON-Datei.
- Nachteile: Kein physisches Paket; Weitergeben einer Welt heißt „Welt-Datei plus passende Teilmenge des Pools“ und braucht
  trotzdem einen Exporter; Konsistenz (Asset im Manifest, aber nicht gerendert) muss ein Test prüfen.
- `game:dist`: kopiert weiter die Vereinigung aller Welt-Manifeste (statt `--assets`, das sich daraus ableiten lässt).

### B · Paketordner je Welt `worlds/<id>/` (physisches Paket)

Ein Ordner je Welt: `world.json`, `assets/` (Grafik-Teilmenge, Ton, Rig-Teile), `paper.json` der Welt. Das Spiel
bekommt einen Welt-Pfad und liest nur daraus. `gen:paper` rendert je Welt (oder rendert einmal und der Paketbau kopiert).

- Vorteile: Klare Einheit, trivial zu kopieren oder zu entfernen; Welt ist ohne den Rest des Repos lauffähig; passt zu
  Veröffentlichung (PB1) und Mods.
- Nachteile: Größter Umbau (`art.Config`, `paper.json`-Lesen, `data/embed.go`, `game:dist`, `gen:paper`, Tests mit
  Golden); doppelte Assets bei geteilten Figuren (Monarch, Truppen), falls kein gemeinsamer Basis-Pool; die Frage
  „wo wird gerendert und wo editiert“ (005: Specs in `public/grafik`) wird schwerer.
- Zusatzfrage: eine Basis-Welt mit gemeinsamem Pool plus Delta-Pakete (Kind-Welten über `extends`) löst die Dopplung, ist
  aber ein eigenes Teilprojekt.

### C · Paketdatei als Build-Artefakt (Zip/Archiv), Quelle bleibt Variante A

Wie A als Quelle (Manifest je Welt, gemeinsamer Pool), dazu ein Task `world:pack <id>`, der ein Archiv
`dist/welten/<id>.k3cw` (Zip: `world.json`, Manifest, benötigte Grafik, Ton) baut; das Spiel lädt entweder aus dem Repo
(Entwicklung) oder aus einem Archiv (`--world datei`, Zip entpackt im Speicher oder in einem Cache).

- Vorteile: Entwicklung bleibt einfach (A), Weitergabe und Veröffentlichung bekommen ein echtes Paket; Paketbau ist
  getrennt vom Format der Quelle und kann später nachgezogen werden.
- Nachteile: zwei Ladewege im Spiel (Ordner und Archiv); Archivformat und Versionierung sind eine zusätzliche
  Schnittstelle; sinnvoll erst, wenn es mehr als eine Welt oder eine Weitergabe gibt.

## Auswirkungen im Überblick

| Bereich | A Manifest | B Paketordner | C Archiv (auf A) |
| --- | --- | --- | --- |
| `gen:paper` | unverändert | je Welt oder Kopierstufe | unverändert |
| `game:dist` (B-827) | Vereinigung der Manifeste statt `--assets` | je Welt ein Ordner | wie A, plus `world:pack` |
| Spiel (`art.Library`) | Auflösung Rolle → Asset mit Welt-Vorrang | Welt-Pfad statt `public/grafik` | wie A, plus Archiv-Leser |
| Welten-Editor (B-879) | schreibt Manifest | schreibt Ordner, kopiert Assets | wie A |
| Entscheidung 005 | unberührt | Quelle und Paket auseinanderhalten | unberührt |
| Aufwand (grob) | S bis M | L | A plus M |

## Empfehlung

**Variante A jetzt, Variante C als späterer Ausbau.** Die Welt bekommt ein Asset-Manifest (Rollen-Überschreibungen, Liste
der Assets, später Ton), der Pool unter `public/grafik/` und die Regel „Assets nur über MCP“ bleiben unberührt. `game:dist`
leitet die Auslieferung aus den Manifesten ab. Ein physisches Archiv (C) entsteht erst, wenn es eine zweite Welt oder eine
Weitergabe gibt; Variante B lohnt sich nur, wenn Welten unabhängig vom Repo-Pool leben sollen. Begründung: Heute gibt es
eine Welt (`standard`), ein Umbau auf Paketordner würde Aufwand ohne Nutzen erzeugen und Entscheidung 005 komplizierter machen.

## Folgen bei Annahme (Entwurf der Folge-Tickets, noch nicht angelegt)

1. Format des Welt-Manifests und Auflösungsreihenfolge festlegen (Spec, Domäne PAP/CLI).
2. `art.Library` und `backdrop.go` lesen Rollen über das Welt-Manifest; Test, dass jedes Asset des Manifests existiert.
3. `game:dist` und `--assets` aus den Manifesten ableiten (B-827 anpassen).
4. Welten-Editor (B-879): Assets je Welt zuordnen.
5. Später: `world:pack` und Archiv-Ladeweg (C) hinter einem eigenen Ticket, abhängig von PB1.

## Offene Fragen an Ralf

1. **Umfang des Pakets:** Grafik allein oder auch Ton (`sounds.json`, `public/audio`) und Werte (Weltwerte gibt es schon
   über `extends`)? Empfehlung: Grafik und Ton ins Manifest, Werte bleiben in der Welt-Datei.
2. **Geteilte Assets:** Sollen Monarch, Truppen und Icons immer aus einem Basis-Pool kommen (Empfehlung) oder je Welt
   vollständig mitgeliefert werden?
3. **Ziel der Pakete:** Nur eigene Entwicklung (A genügt) oder Weitergabe an Dritte/Veröffentlichung (dann C)?
4. **Kind-Welten (`extends`):** Erben sie das Asset-Manifest der Eltern-Welt (Empfehlung: ja, mit Überschreibung)?
5. **Eingebettete Welten:** Bleibt nur `standard` eingebettet, alle anderen nur von der Platte?
