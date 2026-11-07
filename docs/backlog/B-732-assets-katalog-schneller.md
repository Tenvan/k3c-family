# B-732 · Der Assets-Katalog zeigt Einträge sofort, cacht sie und hat eine größere Vorschau

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-11-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Seite Katalog im Bereich Assets der Workbench (`tools/k3c-workbench/frontend/src/assets/catalog/`) ruft `AssetsList` auf; das Backend (`internal/assetsui/assets.go`) prüft dabei jede Spec mit `bun tools/paper/bin/paper.ts validate` (je Spec ein Prozess, 4 parallel). Bei 137 Specs dauert ein kalter Aufruf gut 10 s, warm (Prüf-Memo im Prozess) 0,4 s. Bis die ganze Antwort da ist, bleibt die Liste leer. Die Seite liegt in einem Tab, der beim Wechsel abgebaut wird: Liste, Specs und gerenderte Frames werden bei jeder Rückkehr neu geholt. Die Vorschau zeigt den Frame in Originalgröße (Bühne mindestens 160 px).

## Ziel

Der Katalog ist sofort bedienbar, lädt nicht bei jedem Seitenwechsel neu und zeigt die Vorschau groß genug zum Beurteilen.

## Beteiligte und Zielgruppen

🧑 und Claude-Sessions, die Assets im Katalog ansehen.

## Anforderungen

- Liste erscheint vor der Prüfung: neue RPC-Methode `AssetsIndex` (ID, Rig, Gruppe, Pfad, geändert; Prüfergebnis nur aus dem Memo) und `AssetsCheck(pfade)` für die Prüfung in Häppchen; `AssetsList` bleibt (Vertrag).
- Ungeprüfte Einträge tragen ein Abzeichen „prüft …“, geprüfte das Ergebnis wie bisher.
- Katalog, Specs und Frames liegen in einem Cache im Frontend (Modul-Ebene), der den Tab-Wechsel überlebt; ein Knopf „Neu laden“ leert ihn und holt neu, ebenso ein Neuverbinden zum Backend.
- Vorschaubild doppelt so breit und hoch wie der gerenderte Frame (höchstens Spaltenbreite), Bühne mindestens 320 px; die Teil-Auswahl per Klick trifft weiter.
- Review-Seite (Nachtrag 🧑 2026-11-07): Liste breiter (520 px); die Feldliste des gewählten Eintrags höchstens so hoch wie die Ansicht rechts, darüber hinaus „…“; nicht gewählte Einträge zeigen nur zwei Zeilen. Voller Text im Tooltip.
- Katalog-Filter „nur geänderte (n)“ (Nachtrag 🧑 2026-11-07): zeigt nur Einträge mit Abzeichen „im Diff“; n zählt sie.
- Review-Seite cacht (Nachtrag 🧑 2026-11-07): Ergebnis je Basis, Specs, Frames und Pixel-Diffs überleben den Tab-Wechsel; das Review gegen HEAD rechnet schon im Hintergrund, sobald der Bereich Assets offen ist; „Neu laden“ lässt das alte Ergebnis stehen, bis das neue da ist.

## Nicht-Ziele

Automatisches Neuladen bei Dateiänderungen unter `public/grafik/` (bräuchte ein Ereignis vom Backend; bei Bedarf eigenes Ticket). Schnellere Prüfung in der CLI selbst. Review je Asset gestreamt (heute rechnet `AssetsReview` alle Assets in einem Aufruf, 4–6 s; Aufteilen im Paket `internal/review` als eigenes Ticket).

## Regeln und Einschränkungen

Domäne DEV. Oberfläche nur über `@webkit` (Atomic Design, `tests/atomicDesign.test.ts`). Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Vertrag `docs/standards/paper-mcp.md` › Oberfläche wird um die zwei Methoden ergänzt.

## Beispiele

Kalter Start, Katalog öffnen → Liste mit allen Assets nach Bruchteilen einer Sekunde, Abzeichen „prüft …“ verschwinden schrittweise. Wechsel auf Planung und zurück → Liste und zuletzt gesehene Frames sofort da, kein neuer RPC.

## Ausnahme- und Fehlerfälle

Fällt `AssetsCheck` für ein Häppchen aus, bleibt die Liste stehen, die Einträge zeigen den Fehler wie bei einem CLI-Absturz. Fehlt eine Spec beim Prüfen (gelöscht), entfällt ihr Eintrag nicht stillschweigend: Fehler „cli“ am Eintrag.

## Akzeptanzkriterien

- **AC-01** Katalog-Tests (`frontend/src/assets/catalog/*.test.tsx`): Einträge sind sichtbar, bevor die Prüfung fertig ist; Abzeichen „prüft …“ bis zum Ergebnis.
- **AC-02** Go-Test `internal/assetsui`: `AssetsIndex` ruft keinen Validate auf, `AssetsCheck` liefert dieselben Einträge wie `AssetsList`.
- **AC-03** Test: Nach Abbau und neuem Mount der Katalog-Seite ruft sie weder `assetsIndex` noch `assetsFrame` für schon geladene Frames erneut auf; „Neu laden“ holt neu.
- **AC-04** Im Web-Client geprüft: Vorschaubild doppelt so groß wie `naturalWidth`/`naturalHeight`, Klick ins Bild wählt das Teil.
- **AC-05** Test `frontend/src/assets/review/ReviewList.test.tsx` und Web-Client: gewählter Eintrag mit Zeilenzahl aus der Höhe rechts, die übrigen mit zwei Zeilen.
- **AC-06** Test `CatalogCache.test.tsx` und Web-Client: Filter „nur geänderte“ blendet Einträge ohne „im Diff“ aus.
- **AC-07** Test `review/ReviewCache.test.tsx` und Web-Client: nach Abbau der Review-Seite kein neuer `AssetsReview`/`AssetsFrame`; „Neu laden“ holt neu, die Liste bleibt dabei stehen.

## Offene Fragen

keine

## Notizen

Messung 2026-11-07 (Dev-Workbench, warm): AssetsList 0,41 s / 19 KB, AssetsReview 0,26 s, AssetsPrompts 8 ms; `paper validate` je Spec ~0,3 s.
