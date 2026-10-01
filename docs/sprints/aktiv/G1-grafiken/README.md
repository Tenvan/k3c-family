# G1 · PLAT · Referenzseite für die gewählten Grafik-Packs

- **Status:** aktiv
- **Domäne:** PLAT
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-087
- **Start-Commit:** 657658d
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-01 🧑 Chat („Auswahl B-010: …“, „Ja, alle zehn“, Einbau in eine Referenzseite wie die Spielfiguren; Revision 2: zwei weitere Packs, B-087 Revision 2)

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
Prüfungen im Browser nur mit Freigabe durch 🧑.

## Beispiele

Kachel „Alle Grafiken“ → Abschnitt „Gothicvania Town“ zeigt `houses.png`, Props und Ebenen in 2×, darunter Quelle und „CC0 · ansimuz“.

## Ausnahme- und Fehlerfälle

Ein Bild fehlt → „Bild fehlt“ in der Karte statt einer leeren Stelle.

## Akzeptanzkriterien

- **AC-01** Alle zwölf Packs liegen unter `public/grafik/<pack>/` mit `LICENSE.txt`, ohne Musik, Code und Quelldateien (B-087/AC-01).
- **AC-02** `public/grafik/CREDITS.md` und `lizenzen.html` nennen jedes Pack mit Quelle, Urheber und Lizenz (B-087/AC-02).
- **AC-03** `grafiken.html` ist von der Landingpage erreichbar und zeigt alle Packs mit allen Bildern (B-087/AC-03).
- **AC-04** 🧑 hat die Seite angesehen und die Packs abgenommen (B-087/AC-04).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| G1.1 | `G1.1-referenzseite.md` | Umsetzung | autonom | fertig |
| G1.2 | `G1.2-review.md` | Review | autonom | offen |
| G1.3 | `G1.3-abnahme.md` | Workshop | Mensch | offen |

## Abnahme

–
