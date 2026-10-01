# G1 · PLAT · Referenzseite für die gewählten Grafik-Packs

- **Status:** erledigt
- **Domäne:** PLAT
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-087
- **Start-Commit:** 657658d
- **Spec:** freigegeben
- **Revision:** 3
- **Freigabe:** 2026-10-01 🧑 Chat („Auswahl B-010: …“, „Ja, alle zehn“, Einbau in eine Referenzseite wie die Spielfiguren; Revision 2: zwei weitere Packs, B-087 Revision 2; Revision 3: Auswahlfunktion, alle zwölf Packs vorläufig übernommen)

## Ausgangslage

B-010 will Pixel-Art für Gebäude, Ressourcen und Hintergründe (nur CC0). Die Recherche liegt in `docs/funde/b010-grafik-funde.html`, 🧑 hat zwölf Packs gewählt (zuerst zehn, dann zwei) und den Download freigegeben.
Für Figuren gibt es `figuren.html` und `aufstellung.html`, für diese Grafiken keine Seite.

## Ziel

Die zwölf Packs liegen mit Lizenz im Repo und sind auf `grafiken.html` ansehbar. Am Ende sichtbar: Kachel „Alle Grafiken“ → Seite mit allen Packs, Urhebern, Lizenzen, Quellen und den Bildern in wählbarer Vergrößerung.

## Beteiligte und Zielgruppen

🧑 entscheidet, was wofür ins Spiel kommt (B-010); Entwickler und Agenten binden später ein.

## Anforderungen

B-087 › Anforderungen. Sprint-eigen: Die Packs enthalten Musik, Code, PSD-Quellen und Demo-GIFs; übernommen werden nur die Umgebungs-Bilder (PNG) und die Lizenzdatei.

## Nicht-Ziele

Einbau ins Spiel (B-010); Figuren-Sprites der Packs; Musik; Lücken schließen (Mine-Hintergrund, Rekrutierungslager, Werkstatt, Farm, Kaserne, Treppen).

## Regeln und Einschränkungen

Seiten-Regeln aus `CLAUDE.md`; `public/grafik/` ist Fremdmaterial mit eigener Lizenz (wie `public/sprites/`); `lizenzen.html` und `public/grafik/CREDITS.md` müssen übereinstimmen (Test).
Prüfungen im Browser nur mit Freigabe durch 🧑. Die Auswahl gehört in `src/tools/` (PLAT), `spriteReference.ts` ist dafür mit angefasst.

## Beispiele

Kachel „Alle Grafiken“ → Abschnitt „Gothicvania Town“ zeigt `houses.png`, Props und Ebenen in 2×, darunter Quelle und „CC0 · ansimuz“.

## Ausnahme- und Fehlerfälle

Ein Bild fehlt → „Bild fehlt“ in der Karte statt einer leeren Stelle.

## Akzeptanzkriterien

- **AC-01** Alle zwölf Packs liegen unter `public/grafik/<pack>/` mit `LICENSE.txt`, ohne Musik, Code und Quelldateien (B-087/AC-01).
- **AC-02** `public/grafik/CREDITS.md` und `lizenzen.html` nennen jedes Pack mit Quelle, Urheber und Lizenz (B-087/AC-02).
- **AC-03** `grafiken.html` ist von der Landingpage erreichbar und zeigt alle Packs mit allen Bildern (B-087/AC-03).
- **AC-04** 🧑 hat die Seite angesehen und die Packs abgenommen (B-087/AC-04).
- **AC-05** Auf `grafiken.html`, `figuren.html` und `aufstellung.html` kann 🧑 Figuren, Packs und Bilder ankreuzen; unten steht die Auswahl als kopierbarer Satz mit stabilen IDs (B-087/AC-05).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| G1.1 | `G1.1-referenzseite.md` | Umsetzung | autonom | fertig |
| G1.2 | `G1.2-review.md` | Review | autonom | fertig |
| G1.3 | `G1.3-auswahl.md` | Umsetzung | autonom | fertig |
| G1.4 | `G1.4-review.md` | Review | autonom | fertig |
| G1.5 | `G1.5-abnahme.md` | Workshop | Mensch | fertig |

## Abnahme

Reviews 2026-10-01 (G1.2, G1.4): `task check` grün (701 Tests), `task build` nimmt Seite und `public/grafik/` (921 KB) auf; keine schweren Befunde; Kopieren der Auswahl über http ergänzt. Prüfer war dieselbe Sitzung wie die Umsetzung.
AC-01 bis AC-03 und AC-05 durch `grafikPacks.test.ts` und `selection.test.ts` belegt (zwölf Packs mit Lizenzdatei, Credits in `CREDITS.md` und `lizenzen.html`, Auswahl auf drei Seiten).
**AC-04 und AC-05 abgenommen 2026-10-01 durch 🧑 (Ralf)** am lokalen Browser: „passt alles“, alle zwölf Packs übernommen (auch Warped Caves unter CC BY 3.0). Einzelschritte und die Zuordnung Pack → Zweck hat 🧑 nicht gemeldet.
Offen für B-010: Verwendung je Pack (per Auswahl im Chat), Lücken Mine-Hintergrund, Rekrutierungslager, Werkstatt, Farm, Kaserne, Treppen; Blue Cave ohne bekannten Urheber. Neue Tickets: keine.
