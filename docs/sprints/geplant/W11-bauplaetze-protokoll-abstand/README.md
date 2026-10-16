# W11 · REG, SRV, CLI · Bauplätze im Protokoll, Abstand der Treppen, Ausbilden und Tauschen, Figuren

- **Status:** geplant
- **Projekt:** WRT
- **Domäne:** REG, SRV, CLI
- **Reife:** Entwurf
- **Tickets:** B-409, B-208, B-410, B-408, B-207
- **Start-Commit:** a8d46d9e
- **Spec:** Entwurf
- **Revision:** 3
- **Freigabe:** –

## Ausgangslage

Aus der Sichtung von W6 (2026-10-11): Bauplätze stehen im Hub zu eng, besonders die Treppen (B-409, `data/hub.json › sites`, je 4 Units Abstand); Plätze „aller Stufen“ erscheinen schon im Hub einer niedrigen Stufe (B-408); das Protokoll trägt gestreute Bauplätze, Platz- und Hub-Stufe nicht zum Client (B-208); `actions` enthält Ausbilden und Tauschen nicht (B-410); gesperrte Plätze zeigen keinen Grund (B-207, seit 2026-10-14 in W11, siehe B-422).

**Stand nach Revision 3 (2026-10-16):** W11.1 bis W11.3 sind fertig (W11.1 per PR #295 auf `develop`, W11.2 und W11.3 nur auf `sprint/w11`, der 12 Commits hinter `develop` liegt); offen sind W11.4 (CLI) und das Review. Parallel hat das Projekt GEB (GEB4, GEB5) Maße und Grafik der Hub-Gebäude geändert (`docs/standards/gebaeude-grafik.md` § 1, Gebäude-JSONs mit `kacheln.b/h`; 1 Kachel = 1 Unit). Prüfung gegen `data/hub.json` und die Maße (Beleg `origin/develop`, 2026-10-16):
- **W11.1 bleibt gültig.** `data/hub.json` ändern weder GEB4 noch GEB5 (GEB5 › Nicht-Ziele: keine Hub-Plätze verschieben). Treppe hoch 4 × 4 (+16: 14 … 18) und Treppe runter 4 × 3 (+8: 6 … 10) lassen zu allen Nachbarn mindestens 2 Units Rand (Burg 8 × 9: −4 … 4, Kaserne +24: 21 … 27); die 8 Units zwischen Zahlzielen reichen also auch für die neuen Breiten.
- **Rest von B-409:** Kaserne (+24) und Schmiede (+28), beide 6 Kacheln breit, überdecken sich um 2 Units; Rüstkammer ↔ Turm Linie 1 sowie Taverne/Heilplatz ↔ Turm Linie 1 um je 0,5. Das hält das Zahlziel-Mindestmaß (≥ 4), ist aber der Grund von B-409 (Beschluss 🧑 2026-10-11: Abstand nach Gebäude-Breite). Ausgelagert nach B-446.
- **W11.2 und W11.3 bleiben gültig:** Protokoll (`sites[].hubLevel`, `players[].actions`) und Server berühren GEB nicht (GEB5 › Nicht-Ziele: keine Sim- oder Protokolländerung).
- **W11.4 ist betroffen:** Es ändert `src/scenes/siteView.ts`, `worldRenderer.ts` und `src/model/types.ts`; GEB5.4 (CLI) ändert `siteView.ts` (Treppe hoch als ein Bild statt `STAIRS_UP_TILES`), `worldRenderer.ts` und `buildingSprites.ts` (GEB5 › GEB5.4 › Erlaubte Dateien). Dazu kam B-412 (Figuren für Lager, Plantage, Adern) in W11.4, das GEB5 (Lager, Plantage) und GEB6 (Adern) als Grafik schon abdecken.
- **B-207** (gesperrt mit Grund) braucht für „Linie fehlt“ ein Server-Feld, das W11.2 nicht liefert (`Site` ohne Linie und Seite) → B-447.

## Ziel

Der Client zeigt nur Bauplätze, die zur Hub- und Stufen-Lage passen, die gesperrten mit Grund „ab Hub-Stufe n“, in lesbarem Abstand an den Treppen, dazu Ausbilden und Tauschen im Aktionen-Overlay. Am Ende sichtbar: Hub am TV ohne überlappende Preisschilder an den Treppen, Aktionen-Overlay mit Ausbilden und Tauschen, gesperrte Plätze mit Grund; von 🧑 abgenommen. Adern, Plantage und Lager als Figuren (B-412) gehören nicht mehr dazu.

## Beteiligte und Zielgruppen

Spielende am TV (bis 2 lokal, PC und Xbox); Agent in REG, SRV und CLI nacheinander; 🧑 nimmt am Gerät ab und entscheidet die Offenen Fragen. Die Sessions GEB5.4 und W11.4 teilen sich Dateien in `src/scenes/` (siehe Regeln).

## Anforderungen

B-409, B-208, B-410, B-408, B-207 › Anforderungen (Specs der Tickets in Revision 2 bzw. 3, Entwurf). Sprint-eigen:
- Treppen ≥ 8 Units zu jedem Zahlziel derselben Tiefe (W11.1, fertig); Maße der Treppen im Spiel nach GEB5 (4 × 4 und 4 × 3 Kacheln) bleiben im Rand.
- Der Client zeichnet jeden Platz, den der Server schickt, ohne eigenen Filter (B-408) und beschriftet gesperrte Plätze mit „ab Hub-Stufe n“ aus `sites[].hubLevel` (B-207); „Linie fehlt“ und „Tor nur außen“ kommen mit B-447.
- `PlayerAction` in `src/model/types.ts` kennt `train` und `trade`, `actionHints.ts` zeigt sie (B-410, aus W11.3 übernommen, B-434).
- `src/model/types.ts` (414 Zeilen, Grenze 400) wird in W11.4 geteilt, bevor Felder dazukommen (Hinweis des Reviews von W8).

## Nicht-Ziele

Neue Gebäude oder Wirtschaftsregeln; Rohstoff-Typen (W8, erledigt); Ausrüstung und Spielstand (W7, SV1); Grafik der Gebäude (GEB). Aus Revision 3 ausgenommen:
- **B-412** (Lager, Plantage, Adern als Figuren mit Zustand): Grafik liefern GEB5.4 und GEB6.5; die Zustands-Anzeige wird nach GEB6 neu eingeplant (B-412 `offen`, `Sprint: –`).
- **Gebäude-Überdeckung** (Kaserne ↔ Schmiede, Turm-Ränder): B-446.
- **Sperrgrund „Linie fehlt“ und „Tor nur außen“:** B-447 (Server-Feld nötig).

## Regeln und Einschränkungen

Reihenfolge REG → SRV → CLI. Mindestmaß der Zahlziele ≥ 4 Units (Glossar › Zahlziel) bleibt, für Treppen 8. Protokoll-Änderungen (W11.2, W11.3) mit Beispielen unter `testdata/protocol/`, keine neue Version (beschlossen). Der Client rechnet nichts. Der Spielstand bleibt kompatibel oder wird migriert (Golden, F2); alte Stände ohne Rücksicht (Beschluss 🧑).

**Abhängigkeit von GEB5 (Merge-Reihenfolge, Entwurf):**
- GEB5 kommt zuerst nach `develop`. W11.4 startet erst, wenn GEB5.4 `fertig` ist **und** `sprint/w11` `origin/develop` eingemergt hat (`git merge origin/develop`, nie Rebase); danach liest die Session `siteView.ts`, `worldRenderer.ts` und `buildingSprites.ts` im neuen Stand (Treppe hoch als ein Bild, Maßstab 1) und baut darauf auf.
- Die Domänensperre CLI (`docs/arbeitsweise.md` › Autonomer Ablauf, Schritt 1) greift nur, solange eine CLI-Session `in Arbeit` ist; steht GEB5.4 noch auf `offen`, erzwingt sie nichts. Diese Regel ist deshalb die eigentliche Absicherung.
- W11.1 bis W11.3 (REG, SRV) sind von GEB unabhängig; `engine/`, `data/` und Protokoll fassen GEB5 und GEB6 nicht an. GEB6.5 (CLI) ändert später dieselben Dateien: Der Sprint, der zuerst `develop` erreicht, gewinnt; der andere merged zuerst.
- Der PR von W11 entsteht erst nach dem Review (wie bisher); `develop` enthält dann GEB5.

## Beispiele

Hub-Stufe 1 → der Zustand trägt nur Plätze mit `hubLevel` ≤ 2 (und bezahlte oder gebaute); der Platz mit `hubLevel` 2 zeigt „ab Hub-Stufe 2“, Plätze mit `hubLevel` 3 sind nicht da. Treppe hoch und runter stehen mindestens 8 Units von jedem anderen Zahlziel derselben Tiefe entfernt (Burg, Händler, Plätze, Angebote, Linien); Treppe hoch 4 × 4 bei +16 und Treppe runter 4 × 3 bei +8 lassen zur Burg und zur Kaserne mindestens 2 Units Rand. Spieler am Bergmann-Angebot hält A → das Overlay zeigt „Ausbilden: Bergmann“ mit Preis; am Händler „Kaufen“ bzw. „Verkaufen“ mit Kurs.

## Ausnahme- und Fehlerfälle

Alter Server ohne `hubLevel` → Client zeigt Plätze wie bisher, ohne Sperr-Hinweis, wie bisher (Log, keine Warnung im Bild). Unbekannte Aktion in `actions` → ignorieren, Log (`actionHints.ts`; gilt nach W11.4 für alles außer `train`/`trade`). Grafik der Gebäude noch nicht geladen → Platzhalter wie heute (GEB5).

## Akzeptanzkriterien

- **AC-01** Die Bauplätze im Hub haben bei den Treppen mindestens 8 Units Abstand, `task balance` und Golden bleiben grün (B-409). Nachweis W11.1; Geltung gegen die Maße aus GEB5 in Revision 3 geprüft (Treppen 4 × 4 und 4 × 3 als GEB5-Annahmen, Rand ≥ 2 Units; GEB5.3 liefert die JSONs erst noch, daher gegen geplante Werte geprüft).
- **AC-02** Das Protokoll trägt Bauplätze mit Platz- und Hub-Stufe; nicht erreichbare Plätze erscheinen nicht (B-208, B-408). Nachweis W11.2.
- **AC-03** Der Server liefert Ausbilden und Tauschen in `actions` (B-410). Nachweis W11.3.
- **AC-04** Der Client kennt `train` und `trade` in `PlayerAction` und zeigt sie im Aktionen-Overlay; er zeichnet alle Plätze aus dem Zustand ohne eigenen Filter (B-410/AC-03, B-408). Reine Funktionen getestet (`task test`), `task check` grün. *(Revision 3: ohne Adern, Plantage, Lager; die gehören nicht mehr zu W11, B-412.)*
- **AC-05** Gesperrte Plätze zeigen „ab Hub-Stufe n“, freie bleiben unbeschriftet; eine reine Funktion bildet Platz und Hub-Stufe darauf ab (B-207/AC-01, AC-02). *(neu in Revision 3, B-422)*
- **AC-06** 🧑 hat am TV (PC und Xbox) abgenommen: Hub ohne überlappende Preisschilder an den Treppen, Ausbilden und Tauschen im Overlay, gesperrte Plätze mit Grund (B-409, B-410, B-408, B-207/AC-03). *(aus dem Abnahme-Teil von AC-04 herausgelöst, Revision 3)*

## Offene Fragen

Geklärt mit 🧑 (2026-10-14): W11 vor W8; Treppen ≥ 8 Units zu **allen** Zahlzielen derselben Tiefe. „Linien +8“ verworfen (Kristallhöhle: Portale ab 135, Camps ohne freien Chunk); stattdessen teilen sich Treppe hoch (ab Tiefe 1) und Händler-Kaufen (nur Tiefe 0) ein Raster, Linien bleiben. B-408: Der Server filtert; „erreichbar“ heißt Platz-Stufe ≤ Hub-Stufe + 1 (bezahlte und gebaute Plätze immer), damit die nächste Stufe als gesperrt gezeigt werden kann (B-207, jetzt W11.4). Protokoll: keine neue Version (additiv). Mit 🧑 (2026-10-15): `train` und `trade` wie in B-410 Rev. 2; `PlayerAction` mit der Anzeige in W11.4 (B-434).

**Offen für 🧑 (Revision 3, 2026-10-16, Vorschläge in Klammern):**
1. **W11 nicht neu starten, nur anpassen** (Vorschlag): W11.1 bis W11.3 bleiben `fertig`, W11.1 wird nicht wiederholt (Beleg in Ausgangslage). Einverstanden? Der Sprint steht dafür wieder in `geplant/`, weil `aktiv` eine freigegebene Spec verlangt (das Werkzeug lehnt sonst ab); der Lebenslauf kennt keinen Rückweg aktiv → geplant und zählt einen Sprint mit Branch auf `origin` als aktiv: gilt bis zur Freigabe, danach zurück nach `aktiv/`. Die Änderung liegt auf `sprint/w11-plan`, nicht auf `sprint/w11`.
2. **B-412 aus W11 herausnehmen** (Vorschlag), weil die Grafik GEB5/GEB6 liefern und dieselben Dateien betroffen sind; neu einplanen nach GEB6.
3. **B-207 auf „frei“ und „ab Hub-Stufe n“ begrenzen** (Vorschlag); „Linie fehlt“ und „Tor nur außen“ brauchen ein Server-Feld (B-447, später) oder eine SRV-Session in W11 vor W11.4 (dann 6 Sessions).
4. **Gebäude-Überdeckung** (B-446): eigener Sprint nach GEB5 (Vorschlag) oder als REG-Session in W11 vor W11.4?
5. **Merge-Reihenfolge:** GEB5 zuerst, dann `git merge origin/develop` in `sprint/w11`, dann W11.4 (siehe Regeln). Einverstanden?
7. **AC-06 ohne Mensch-Session:** W11 hat keine Session `Agent: Mensch` für die TV-Abnahme (anders als W6.4); der Sprint würde nie `erledigt`. Vorschlag: W11.6 `Workshop`/`Mensch` „Abnahme am Gerät“ ergänzen (Review steht dann vor ihr, Hardware entkoppelt) oder 🧑 trägt AC-06 beim Review ein.
6. SV1-Verweis auf B-208 anpassen (blieb offen aus 2026-10-14; Beleg SV1 nicht geprüft, ungeprüft ob noch nötig).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W11.1 | `W11.1-treppen-abstand.md` | Umsetzung | autonom | fertig |
| W11.2 | `W11.2-bauplaetze-im-zustand.md` | Umsetzung | autonom | fertig |
| W11.3 | `W11.3-aktionen-ausbilden-tauschen.md` | Umsetzung | autonom | fertig |
| W11.4 | `W11.4-bauplaetze-figuren-client.md` | Umsetzung | autonom | offen |
| W11.5 | `W11.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
