# B-661 · Der Lauf-Editor setzt Varianten als Tabelle mit Basiswert und Daten-Browser

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ32
- **Projekt:** WZG
- **Erstellt:** 2026-10-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-29 🧑 Chat (Revision 1, offene Fragen mit Default von Claude)

## Ausgangslage

Ein Lauf entsteht heute im Formular `balancing/ConfigForm.tsx` (1.086 px hoch bei 1.000 px Fensterbreite). Unter jedem Textfeld stehen zwei bis drei Zeilen Hilfetext, daneben zusätzlich „?“. Beispiele und Basis-Läufe stecken im Dialog „Beispiele …“, Voreinstellungen unten im Formular.

Varianten sind Freitext (`datei.json:pfad=wert`, eine Zeile je Variante), geprüft erst beim Start und nur die erste falsche Zeile (`spec.ts` › `specFromForm`). Einen Pfad zu finden braucht sechs Schritte: Dialog „Konfigurationsdateien …“ öffnen, Datei suchen, „Werte“, Pfad aus einer Auswahl ohne Suche wählen (`DataFiles.tsx` › `ValuePicker`; `buildings.json` hat 252 Zahlenwerte, die sieben variierbaren Dateien zusammen 735), „Als Variante“, Dialog schließen. Der Basiswert und die Abweichung sind nicht sichtbar. Abstimm-Parameter (`TuneConfig.tsx`) sind ein zweites Textfeld mit eigenem Format (`=min..max`) hinter „Modus: Aus“.

## Ziel

Ein Lauf ist in wenigen Klicks aus einem Ausgangspunkt gebaut; jede Variante zeigt Basiswert und Abweichung und ist sofort geprüft; Pfade findet man über eine Suche statt über Dialoge. Varianten und Abstimmung sind ein Konzept mit einer Bedienung.

## Beteiligte und Zielgruppen

Ralf beim Balancing; Agent DEV.

## Anforderungen

### Lauf-Editor

- Öffnet im Detailbereich der Seite („Neuer Lauf“, „Bearbeiten und starten“, „Als Variante übernehmen“, „Im Editor übernehmen“ aus B-660), mit Weg zurück zur Liste.
- Ausgangspunkt als Umschalter: Leer, Beispiel, Basis-Lauf, Voreinstellung, Bericht; übernimmt alle Felder, danach frei änderbar. Der Dialog „Beispiele …“ entfällt.
- Abschnitte: 1 Szenario (Welt: eine Stufe oder ganze Insel 1; Tiefe nur bei einer Stufe; Dev-Hilfen nur mit Welt; Grad; Tage), 2 Spieler und Bots (Spieler 1–4 mit −/+, ein Profil für alle oder je Spieler), 3 Umfang (Seeds 1, 8, 20, Alle, Liste), 4 Varianten, 5 Vergleich und Name (Baseline, Label). Jeder Abschnitt zeigt rechts eine Zusammenfassung.
- Feste Auswahlen als Umschalter statt Auswahlliste, wo es höchstens fünf Werte gibt (Grad, Hilfen, Welt, Seeds).
- Hilfetexte nur noch im „?“-Popup (Texte aus `help.ts` bleiben); unter einem Feld nur ein Warnhinweis, wenn er gilt (z. B. Hilfen aktiv).
- Fußleiste fest: Zusammenfassung (Seeds × Spieler × Tage, bei Bereichen Auswertungen, geschätzte Dauer), Als Voreinstellung speichern, Zurücksetzen, Starten (Beschriftung je nach Art: Lauf, Abstimmung, n Läufe).

### Varianten-Tabelle

- Spalten: Wert (Datei und Pfad), Basis, Neu, Abweichung in %, Art (fest/Bereich), Entfernen.
- **Fest:** ein neuer Wert; Abweichung = (neu − Basis) / Basis. **Bereich:** von/bis, Abweichung als Spanne; der Umschalter wandelt eine Zeile ohne Neueingabe um (Bereich: ±25 % um den Basiswert wie `rangeAround`).
- Prüfung je Zeile beim Tippen: unbekannte Datei oder unbekannter Pfad (mit Vorschlag des ähnlichsten Pfads und „Übernehmen“), kein Zahlenwert, von ≥ bis. Starten ist aus, solange eine Zeile falsch ist.
- Mindestens ein Bereich → der Lauf wird zur Abstimmung: Kasten mit Modus (Suche/Sweep) und Budget bzw. Schritte erscheint; höchstens zwei Bereiche (Grenze der CLI).
- „Als Text bearbeiten“ schaltet auf das heutige Zeilenformat um und zurück, ohne Verlust (`=wert` = fest, `=min..max` = Bereich); Einfügen aus der Zwischenablage geht dort.
- Option „Jede feste Variante als eigener Lauf gegen die Basis“ (braucht B-662), sonst ausgegraut mit Hinweis.

### Daten-Browser

- Liste neben dem Editor (unter 1.100 px darunter): alle Zahlenwerte der variierbaren Dateien, gruppiert nach Datei, mit Pfad und aktuellem Wert; Suche über Datei und Pfad; Link zur Beschreibung der Datei (`docs/data/`).
- Je Wert „+ fest“ und „+ Bereich“; bereits genutzte Werte zeigen „in den Varianten“ statt der Knöpfe.
- Der Dialog „Konfigurationsdateien …“ bleibt für Zweck, Leser und Beschreibung der Dateien, ohne Werte-Auswahl.

## Nicht-Ziele

- Seitenrahmen (B-659), Liste und Bericht (B-660).
- Neue Werte in `data/` schreiben: Varianten ändern Werte nur im Speicher des Laufs (wie heute).
- Mehr als zwei Abstimm-Parameter (Grenze der CLI bleibt).
- Nicht-Zahlenwerte (Texte, Listen als Ganzes) als Variante.

## Regeln und Einschränkungen

- Das Format der Varianten bleibt mit Go gleich (`simrun/spec.go` › `varyArg`, Regex in `spec.ts`); der Editor erzeugt dieselbe `BalanceSpec`.
- Begriffe laut Glossar: Lauf-Editor, Daten-Browser, Variante (Balancing), Stellschraube.
- Atomic Design über `@webkit`; ein Umschalter (Segmented Control) und eine Wertetabelle als Bausteine in `molecules/`, falls es sie noch nicht gibt.
- Alte Voreinstellungen (`balancing.presets`) laden weiter (`isPresetList`).
- Komplexitäts-Budget: Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

- Bericht „Insel 1 solo · ohne Hilfen“ → „Bearbeiten und starten“ → Editor mit Ausgangspunkt Bericht, alle Felder gefüllt.
- Daten-Browser: Suche „markCost“ → `gatherables.tree.markCost = 1` → „+ fest“ → Zeile mit Basis 1; Neu 0.5 → Abweichung −50 %.
- `wall.cost.gold` (Basis 2) → „+ Bereich“ → von 1 bis 3, Abstimm-Kasten erscheint, Knopf heißt „Abstimmung starten“.
- Zeile `tower.cost.gld` → rot: „Pfad unbekannt. Meinten Sie tower.cost.gold (4)?“ → „Übernehmen“ korrigiert.
- „Als Text bearbeiten“ → drei Zeilen im heutigen Format; zurück → dieselben drei Tabellenzeilen.

## Ausnahme- und Fehlerfälle

- Basiswert 0 → Abweichung „–“, Bereich `0..1` (wie `rangeAround`).
- Ganze Zahl in der Datei und Dezimalwert in der Variante → Hinweis, dass die Abstimmung ganze Zahlen behält; feste Variante bleibt erlaubt.
- Drei Bereiche → dritte Zeile rot „höchstens zwei Bereiche“, Starten aus.
- Datei-Werte nicht ladbar → Daten-Browser zeigt den Fehler, die Tabelle prüft dann nur das Format.
- Dev-Hilfen ohne Welt → Hilfen ausgeblendet und geleert (heute lehnt der Lauf ab).

## Akzeptanzkriterien

- **AC-01** Vitest: Ausgangspunkt Bericht, Beispiel, Basis-Lauf und Voreinstellung füllen den Editor wie `formFromSpec`, `formFromExample`, `formFromBaseline`; alte Voreinstellungen laden.
- **AC-02** Vitest: Varianten-Tabelle zeigt Basis und Abweichung; Umschalten fest ↔ Bereich ohne Neueingabe; falscher Pfad mit Vorschlag sperrt Starten.
- **AC-03** Vitest: Ein Bereich macht aus dem Auftrag eine Abstimmung (`spec.tune` gesetzt, Modus und Budget), drei Bereiche sperren Starten.
- **AC-04** Vitest: „Als Text bearbeiten“ und zurück ist verlustfrei; der erzeugte Auftrag ist gleich dem aus dem heutigen Formular für dieselben Eingaben.
- **AC-05** Vitest: Daten-Browser filtert über alle variierbaren Dateien; „+ fest“ und „+ Bereich“ fügen Zeilen an, genutzte Werte zeigen „in den Varianten“.
- **AC-06** Beobachtung im Browser-Pane mit `?mock=1`: Aufbau wie im Mock-Board „Vorschlag: Lauf-Editor mit Varianten“, Fußleiste bleibt beim Scrollen stehen.

## Offene Fragen

- Dauer-Schätzung in der Fußleiste: aus Erfahrungswerten („40 Tage ≈ 1–4 min je Seed“ aus `help.ts`) oder aus der Dauer früherer Läufe? Default (Claude): Dauer früherer Läufe mit gleicher Welt und Tagen, sonst die Faustregel. Entscheidet 🧑.
- Soll der Daten-Browser auch auf der Seite ohne offenen Editor erreichbar sein (z. B. statt „Konfigurationsdateien …“)? Default (Claude): nur im Editor. Entscheidet 🧑.

## Notizen

- Mock: Design-Canvas „Balancing-Seite Mock“ (2026-10-29), Board „Vorschlag: Lauf-Editor mit Varianten“ (klickbar: Zeilen hinzufügen, fest/Bereich umschalten, Suche im Daten-Browser).
- Gezählte Zahlenwerte am 2026-10-29: economy 48, buildings 252, hub 81, troops 47, enemies 154, waves 22, monarch 131.
- Option „jede Variante als eigener Lauf“ und die Werte aller Dateien in einem Aufruf: B-662.
