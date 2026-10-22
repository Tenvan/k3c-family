# GEB9 · DEV, CLI · Ausbaustufen 1 bis 5 für alle Hub-Gebäude

- **Status:** aktiv
- **Projekt:** GEB
- **Domäne:** DEV, CLI
- **Reife:** bereit
- **Tickets:** B-435
- **Start-Commit:** 013dcb34
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-22 (Auftrag von Ralf: GEB9 autonom abarbeiten)

## Ausgangslage

Nach GEB1–GEB8 haben nur Burg, Mauer, Turm, Tor und Hintergrundmauer Grafik in fünf Stufen (`<id>-<n>-32.png`, `src/scenes/render/buildingSprites.ts` › `stufen`). Lager, Kaserne, Farm, Werkstatt, Taverne, Schmiede, Heilplatz, Rüstkammer und das Camp (Rekrutierungslager, `recruitCamp`) liegen nur als Grundform vor (`public/grafik/gebaeude/<id>-32.png`, keine `stufen` in der JSON).

- `data/buildings.json` führt `levels` nur für `wall` und `tower`. Die Hub-Stufe (`data/hub.json` › `levels`, heute 1–5; `engine/sim/hub_level.go`) steht im Snapshot (`World.hubLevel`); der Client nutzt sie nur für die Burg (`siteView.ts` › `updateCastleView`, `buildingSprites.ts` › `hubTexture`). `siteTexture` bekommt sonst die Stufe des Bauplatzes.
- Material je Hub-Stufe (`docs/rules/materialien-gebaeude.md` § 2): 1 Holz, 2 Stein, 3 Kupfer, 4 Eisen, 5 Kristall.
- **Freischaltung:** Quelle ist `data/hub.json` (`sites`, `islandSites`). Die Tabelle in `materialien-gebaeude.md` § 3 wird daraus erzeugt (`task gen:rules`, WZ15), bearbeitet wird sie im **Gebäude-Planer** des Asset-Studios (WZ9, B-462). Stand heute: Werkstatt und Farm ab Hub-Stufe 1; Kaserne, Lager, Taverne ab 2; Schmiede und Heilplatz ab 3; Rüstkammer ab 4. Die Zuordnung kann sich mit dem Planer (und B-458) ändern; GEB9 liest sie nicht und ändert sie nicht, die Grafik hängt nur an der Hub-Stufe.
- Die Grundformen von Kaserne, Taverne, Schmiede und Rüstkammer enthalten schon Stein- bzw. Eisenfarben; „Grundform = Holz“ gilt nicht für alle.
- Das Camp ist kein Hub-Gebäude, sondern ein Welt-Objekt; seine Grafik kommt über `src/scenes/render/natureSprites.ts`.
- Ausbaustufen entstehen nach `docs/standards/gebaeude-grafik.md` § 6 aus **einer** JSON (Grundform = Stufe 1, `stufen` lückenlos ab 2). Mit `stufen` schreibt `tools/buildings` nur noch `<id>-<n>-<dichte>.png` und löscht `<id>-<dichte>.png`.
- **Vorschau:** Die Asset-Vorschau ist mit WZ6 in die Grafik-Seite des Asset-Studios umgezogen (`tools/k3c-studio/frontend/src/graphics/BuildingStrip.tsx`, `szenenLogic.ts` › `stufenKarten`). Die Stufenreihe aus `stufen` zeigt sie schon für Mauer, Turm, Burg und Tor; neue Gebäude erscheinen dort, sobald ihre JSON `stufen` führt.
- Ralf (2026-10-17): Alle Gebäude sollen für die fünf Stufen gerendert werden. B-458 (Hub wächst kleinteiliger, Lagerfeuer bis Burg) kann die Zahl der Hub-Stufen später ändern; die Freischaltung dafür pflegt der Gebäude-Planer.

## Ziel

Alle neun Hub-Gebäude haben Grafik für die Hub-Stufen 1–5 im Material der Stufe (Holz bis Kristall), passend zu Burg, Mauer und Turm, und das Spiel zeigt je Gebäude die Stufe des Hubs. Am Ende sichtbar: Stufenreihe je Gebäude in der Asset-Vorschau und im Spiel, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

🧑 entscheidet die offenen Fragen (Stufen-Looks, Abgrenzung zu B-458) und nimmt Vorschau und Spiel ab; Agenten erweitern die Daten-JSONs und die Zuordnung im Client; Spieler sehen am Hub, wie weit er ausgebaut ist.

## Anforderungen

B-435 › Anforderungen; Regeln aus `docs/standards/gebaeude-grafik.md` (§ 3 Materialsprache, § 5 Vorlage, § 6 Ausbaustufen). Sprint-eigen (Annahmen, siehe Offene Fragen):
- **Gebäude:** `lager.json`, `kaserne.json`, `farm.json`, `werkstatt.json`, `taverne.json`, `schmiede.json`, `heilplatz.json`, `ruestkammer.json` bekommen `stufen` 2–5 (lückenlos nach § 6); die Grundform bleibt Stufe 1 und unverändert. Stufen vor der Freischaltung erscheinen nur in der Vorschau. Das Camp nach Entscheidung zu Frage 4.
- **Stufen-Look:** Farbziele Stein, Kupfer, Eisen, Kristall wie bei Mauer, Turm und Burg. Welche Teile tauschen (tragendes Mauerwerk, Dach) und welche Holz bleiben (Gestell, Zaun, Tresen), legt GEB9.1 je Gebäude fest (Frage 5). Dazu höchstens wenige `zusaetze` je Stufe, damit der Ausbau auch ohne Farbe erkennbar ist (Frage 3).
- **Ausgabe:** `<id>-<n>-<dichte>.png` für n = 1–5, `index.json` mit `stufen`; die Grafik-Seite des Asset-Studios zeigt die Stufenreihe je Gebäude.
- **Übergang:** Bis GEB9.5 bleibt `<id>-<dichte>.png` (= Stufe 1) erhalten, damit das Spiel zwischen den Sessions grün bleibt: `tools/buildings` schreibt sie für die GEB9-Gebäude zusätzlich, § 6 vermerkt den Übergang. GEB9.5 stellt die Zuordnung um und löscht die Grundform-Dateien samt Verweisen (Aufräum-Regel aus Projekt GEB › Notizen: `data/assets.json`, `docs/assets/zuordnung-*.md`).
- **Spiel:** Die acht Hub-Gebäude wählen die Grafik nach der Hub-Stufe des Snapshots (wie die Burg); fehlt die Stufe, gilt die höchste vorhandene, ohne Stufen die Stufe 1.
- **Determinismus:** gleiche JSON → byte-gleiche PNGs.
- **Erweiterbar:** Kommt mit B-458 eine andere Stufenzahl, entstehen weitere Stufen in derselben JSON, ohne Neuzeichnen.

## Nicht-Ziele

Kleinteiligere Hub-Stufen oder neue Stufenfolge (Lagerfeuer bis Burg) und Freischaltungen je Stufe (B-458); eigene Ausbaustufen der Gebäude mit Werten in `data/buildings.json` (SIM/REG, Ticket noch nicht angelegt, Frage 1); Bäume, Höhlen-Flora (B-454); Truhe geschlossen (B-455); Wimpel in Spielerfarbe (B-456); Treppen und Natur-Objekte; Änderungen an `engine/`, `data/hub.json`, `data/buildings.json` oder am Protokoll; Animationen.

## Regeln und Einschränkungen

- Domänen nacheinander: **DEV** (GEB9.1–9.4: `public/grafik/gebaeude/`, `tools/buildings/` für den Übergang, Regeldatei) → **CLI** (GEB9.5–9.6: `src/scenes/render/buildingSprites.ts`, `src/scenes/render/siteView.ts`, ggf. `src/scenes/render/natureSprites.ts`, `data/assets.json`, `docs/assets/zuordnung-*.md`, Löschen der Grundform-Dateien). `engine/` bleibt unberührt.
- **Vorschau:** Die Stufenreihe liegt auf der Grafik-Seite des Asset-Studios (`tools/k3c-studio`, `GebaeudeStreifen.tsx`); es gibt keine Vorschau mehr in `src/tools/`. Die Reihe entsteht aus `stufen` in der Gebäude-JSON und braucht höchstens Test-Ergänzungen.
- Stufen nur über `farben` und `zusaetze` (§ 6), kein Neuzeichnen; Farben nur aus DB32.
- Die Hub-Stufe kommt aus dem Snapshot; im Client keine Würfel, keine eigene Stufenlogik außer der Zuordnung. Stufenzahl nicht fest verdrahten (`data/hub.json` › `levels`, B-458); fehlt eine Stufe, gilt die höchste vorhandene.
- 2 Spieler: Die Stufe gilt je Hub, beide Spieler sehen dasselbe.
- Keine neue Abhängigkeit. Datei ≤ 400, Funktion ≤ 60 Zeilen.
- **Abstimmung:** B-458 kann die Stufenzahl ändern; GEB9 baut auf den heutigen fünf Hub-Stufen auf und bleibt über `stufen` erweiterbar. Die Freischaltung je Stufe pflegt der Gebäude-Planer (WZ9, REG); GEB9 ändert `data/hub.json` und die Regeltabelle nicht. B-454 (Natur) ist ein eigener Sprint.

## Beispiele

- `task gen:gebaeude` mit `taverne.json` (mit `stufen`) → `taverne-1-32.png` bis `taverne-5-32.png` plus 16/64/128; während des Übergangs zusätzlich `taverne-32.png` (= Stufe 1), nach GEB9.5 nicht mehr.
- Hub-Stufe 3, Schmiede gebaut → Schmiede in Kupfer (`schmiede-3-32.png`) neben Burg Stufe 3.
- Hub-Stufe 2 → Rüstkammer gibt es noch nicht (heute ab Hub-Stufe 4, `data/hub.json`); ihre Stufen 1–3 erscheinen nur in der Vorschau. Verschiebt der Gebäude-Planer die Rüstkammer auf Stufe 3, zeigt das Spiel sie ohne Änderung an GEB9 in Kupfer.
- Hub steigt von 2 auf 3 → alle gebauten Hub-Gebäude wechseln mit dem nächsten Snapshot auf Stufe 3.
- Grafik-Seite des Asset-Studios › Gebäude → je Gebäude eine Reihe Stufe 1–5 nebeneinander.

## Ausnahme- und Fehlerfälle

- Snapshot ohne Hub-Stufe oder unbekannte Stufe → Stufe 1 (Grundform).
- Gebäude im Bau oder zerstört → Platzhalter wie heute, unabhängig von der Stufe.
- Grafik einer Stufe nicht geladen → Grundform statt leerer Fläche.
- Camp außerhalb des Hubs → folgt der Entscheidung zu Frage 4; bis dahin Grundform.

## Akzeptanzkriterien

- **AC-01** Stufen-Looks (Farbtausch und Zusätze je Gebäude), Umgang mit Stufen vor der Freischaltung, Camp und Abgrenzung zu B-458 sind als Entscheidung von 🧑 in der Spec festgelegt.
- **AC-02** Lager, Kaserne, Farm, Werkstatt, Taverne, Schmiede, Heilplatz und Rüstkammer haben `stufen` 2–5 in ihrer JSON und PNGs für Stufe 1–5 (`B-435`); das Camp nach AC-01.
- **AC-03** Die Grafik-Seite des Asset-Studios zeigt je Gebäude die Stufenreihe 1–5 (vorhandene Darstellung, Test in `szenenLogic.test.ts` mit einem der neuen Gebäude).
- **AC-04** Im Spiel zeigen die acht Hub-Gebäude die Grafik der aktuellen Hub-Stufe; ohne Stufe die Grundform (Test der Zuordnung).
- **AC-05** Gleiche JSON → byte-gleiche PNGs; nach GEB9.5 gibt es keine Grundform-Dateien `<id>-<dichte>.png` der GEB9-Gebäude und keine Verweise mehr darauf; `task check` ist nach jeder Session grün.
- **AC-06** 🧑 nimmt Stufenreihe in der Vorschau und das Spiel in mindestens zwei Hub-Stufen ab (Review-Session).

## Offene Fragen

Alle Fragen sind am 2026-10-22 autonom von Claude mit Default entschieden (Auftrag „autonom abarbeiten“); 🧑 ändert, was ihm nicht gefällt.

1. **Stufe = Hub-Stufe:** entschieden wie angenommen. Die Gebäude folgen der Hub-Stufe (wie die Burg); eigene `levels` je Gebäude in `data/buildings.json` sind Nicht-Ziel (eigenes Ticket, falls gewünscht).
2. **Reihenfolge zu B-458:** GEB9 jetzt mit fünf Stufen, B-458 (GEB11) erweitert später über `stufen` in derselben JSON. Begründung: Planer und Regeltabelle stehen bereit, die Stufenzahl ist nicht fest verdrahtet.
3. **Zusätze:** Farbtausch auf allen Stufen 2–5, dazu je Gebäude höchstens ein sichtbarer Zusatz (Schild, Banner, Anbau) ab Stufe 3, damit der Ausbau auch ohne Farbe erkennbar ist.
4. **Camp:** bleibt Grundform (Welt-Objekt, kein Hub-Gebäude). Es fällt aus AC-02 und GEB9.4 heraus; ein Biom- oder Stufen-Look wäre ein eigenes Ticket.
5. **Welche Teile tauschen:** Holzteile (Gestell, Zaun, Tresen) bleiben Holz, Mauerwerk und Dach wechseln das Material der Stufe; bei Gebäuden mit Stein-Grundform (Kaserne, Taverne, Schmiede, Rüstkammer) wechselt der Stein. Die Feinheiten je Gebäude legt GEB9.1 in `docs/standards/gebaeude-grafik.md` § 6 fest.
6. **Stufen vor der Freischaltung:** behalten (Vorschau, Erweiterbarkeit mit B-458); der Gebäude-Planer kann die Freischaltung jederzeit verschieben.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB9.1 | `GEB9.1-stufen-looks-regeln.md` | Umsetzung | autonom | fertig |
| GEB9.2 | `GEB9.2-werkstatt-farm-uebergang.md` | Umsetzung | autonom | fertig |
| GEB9.3 | `GEB9.3-kaserne-lager-taverne.md` | Umsetzung | autonom | fertig |
| GEB9.4 | `GEB9.4-schmiede-heilplatz-ruestkammer.md` | Umsetzung | autonom | offen |
| GEB9.5 | `GEB9.5-spiel-zuordnung-aufraeumen.md` | Umsetzung | autonom | offen |
| GEB9.6 | `GEB9.6-review-abnahme.md` | Review | autonom | offen |

- GEB9.1 (DEV) Stufen-Looks je Gebäude, Zusätze und Übergang in `docs/standards/gebaeude-grafik.md` § 6 festschreiben (Entscheidungen aus Offene Fragen), kein Workshop nötig (AC-01).
- GEB9.2 (DEV) Übergang in `tools/buildings` (Grundform-Datei bleibt), dann Werkstatt und Farm: Stufen 2–5, PNGs, Stufenreihe im Asset-Studio prüfen (AC-02, AC-03, AC-05).
- GEB9.3 (DEV) Kaserne, Lager, Taverne: Stufen 2–5 (AC-02, AC-03, AC-05).
- GEB9.4 (DEV) Schmiede, Heilplatz, Rüstkammer: Stufen 2–5; das Camp bleibt Grundform (AC-02, AC-03, AC-05).
- GEB9.5 (CLI) Spiel wählt die Grafik der acht Hub-Gebäude nach Hub-Stufe (`siteView.ts`, `buildingSprites.ts`), Zuordnung und Test; Grundform-Dateien und Verweise löschen (AC-04, AC-05).
- GEB9.6 (CLI) Review und Abnahme durch 🧑 in Vorschau und Spiel (AC-06, alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
