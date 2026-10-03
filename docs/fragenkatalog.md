# Fragenkatalog

Zu klärende Punkte für die Planung nach den Grundlagen (Plan: [`plan-weiterentwicklung.md`](plan-weiterentwicklung.md)). Eine Frage = ein Absatz mit
Optionen und einer **Empfehlung** (🤖), die 🧑 übernimmt oder ändert. Die Antwort wandert in das genannte Ticket (Feld
`Offene Fragen` → Anforderung/Regel) und in `docs/rules/` bzw. `docs/game-design.md`; danach wird die Zeile hier mit Datum
auf `geklärt` gesetzt. Kein Prozess-Dokument: Der Prozess steht in [`arbeitsweise.md`](arbeitsweise.md), Fragen sind sonst Tickets vom Typ `Frage`.

**Ablauf der nächsten Session (Vorschlag, ca. 90 Minuten):** Block 1 (blockiert F1) → Block 2 (blockiert Phase 1) → Block 3
(Schienen Grafik/Sound) → Block 4 (Betrieb und später). Pro Frage: Option wählen, Zahl nennen, Ticket aktualisieren.
Sprints bleiben `Spec: Entwurf`, bis 🧑 sie je Sprint freigibt.

| Nr. | Thema | Blockiert | Block | Status |
|---|---|---|---|---|
| Q01 | Pause im gemeinsamen Raum | F1, S5 | 1 | offen |
| Q02 | Zielkorridore als Zahlen | F1, BAL2, BR1, BR2 | 1 | offen |
| Q03 | Mindest-Schriftgröße je Split-Viertel | F1, S4 | 1 | offen |
| Q04 | Verbindungsverlust und Eingabe-Latenz | F1 | 1 | offen |
| Q05 | Nur Deutsch? | F1 | 1 | offen |
| Q06 | Skill-Tasten am Controller | X1, S3 | 1 | offen |
| Q08 | Feedback-Events: Liste und Bandbreite | F3, F4, SO1 | 2 | offen |
| Q09 | Golden-Hash ändern: wer bestätigt | F2 | 2 | offen |
| Q10 | Wann wird gespeichert | S2 | 2 | offen |
| Q11 | Onboarding-Umfang und Freundlich-Grad | S6 | 2 | offen |
| Q12 | Spielmetrik: was wird erfasst | S2, BAL4 | 2 | offen |
| Q24 | Spieleabend: Termin, Teilnehmer, Fragebogen | P1 | 2 | offen |
| Q22 | Reihenfolge Phase 2 | W1–W6 | 2 | offen |
| Q13 | Grafik-Stil: Raster, Palette, Skalierung | GR1 | 3 | offen |
| Q14 | Grafik-Auswahl je Lücke | GR2, GR3 | 3 | offen |
| Q15 | Sound-Quellen und Stil | SO2 | 3 | offen |
| Q16 | Musik-Zustände und Hörproben | SO3, SO4 | 3 | offen |
| Q19 | Bot-Profile für den Balancing-Tester | BAL3 | 3 | offen |
| Q07 | Registry und Pi-Modell | SP11 | 4 | offen |
| Q17 | Absicherung von Restore und Heimnetz | F4 | 4 | offen |
| Q18 | Backup-Ziel für den Pi | F4 | 4 | offen |
| Q20 | Release-Rhythmus | RL1 | 4 | offen |
| Q21 | itch.io und Englisch | später | 4 | offen |
| Q23 | Reittiere | später | 4 | offen |

## Block 1 – blockiert F1 und den Spieleabend-Build

**Q01 · Pause im gemeinsamen Raum** (B-135). Die Aktion `pause` (Menu-Taste) ist in `src/input/playerInput.ts` definiert, keine Szene nutzt sie; View + Menu zusammen ist „zurück zur Landingpage“. Fragen: Wer darf pausieren? Hält der Raum für alle an? Wie trennt man Menu allein von der Kombi?
Optionen: (a) Pause nur lokal: Menü öffnet sich, der Raum läuft weiter, der Monarch steht still und ist geschützt. (b) Raum pausiert, wenn alle verbundenen Geräte lokal pausiert haben. (c) Jeder Spieler kann den Raum anhalten, andere sehen „X pausiert“.
🤖 Empfehlung: (a) für Online, (b) im reinen Couch-Raum (ein Gerät). Menu allein (kurz, < 600 ms) öffnet das Menü, die Kombi bleibt reserviert.

**Q02 · Zielkorridore als Zahlen** (B-134). Ohne Zahlen hat der Balancing-Tester (B-099) keinen Abnahmemaßstab. Gesucht: je Kennzahl ein Korridor, z. B. „Grad Normal, 2 Spieler, Nacht 3 überleben ≥ 80 % der Seeds“, Gold je Tag, Zeit bis zur ersten Mauer, Anteil gewonnener Nächte je Grad.
🤖 Vorschlag: Agent liefert aus `docs/rules/wirtschaft.md`, `gegner.md`, `stufen.md` eine Tabelle mit Startwerten; 🧑 streicht, ändert, bestätigt. Pass/Fail je Kennzahl für 100 feste Seeds.

**Q03 · Mindest-Schriftgröße je Split-Viertel** (B-136). Heute 20–26 px; im Viertel-Split schrumpft das auf etwa 10 px effektiv (`src/scenes/HudScene.ts`, `worldRenderer.ts`). `game-design.md` fordert „Mindestgröße“ ohne Zahl.
🤖 Empfehlung: ≥ 28 px bei 1080p im Vollbild, ≥ 24 px nach Skalierung im Viertel, Kontrast ≥ 4,5:1; bei 3–4 Spielern HUD-Text kürzen statt verkleinern. Optionaler Test in `tests/projectRules.test.ts`.

**Q04 · Verbindungsverlust und Eingabe-Latenz** (B-144). Wenn ein Gerät minutenlang weg ist, steht sein Monarch (`WaitFor` = 60 s, `engine/room/room.go`). Nachts schutzlos? Außerdem gibt es kein Latenz-Ziel (30 Hz plus WLAN, keine Vorhersage des eigenen Monarchen).
Optionen: (a) getrennter Monarch wird unverwundbar und ausgeblendet; (b) bleibt verwundbar (härter, fair?); (c) übernimmt eine KI.
🤖 Empfehlung: (a) bis zur Frist, Latenz-Ziel „Eingabe → Bild ≤ 100 ms im Heim-WLAN“, gemessen mit dem Debug-Overlay.

**Q05 · Nur Deutsch?** (B-145). Texte stehen hart kodiert in den Szenen. Für die Familie reicht Deutsch; ein itch.io-Release (Q21) bräuchte Englisch.
🤖 Empfehlung: „nur Deutsch“ als Nicht-Ziel festhalten, Texte aber in neuen Dateien zentral sammeln (billig, solange es wenige sind); Englisch erst mit Q21.

**Q06 · Skill-Tasten am Controller** (B-026, X1). Belegung der Skill-Slots ohne B (reserviert) und View + Menu. Hängt am Xbox-Test (B-006).
🤖 Empfehlung: erst X1 mit Testseite durchführen (Y/X/RB/LB-Kandidaten), dann festlegen; Tastatur und Touch parallel (S3).

## Block 2 – blockiert Phase 1 (Spieleabend-Build)

**Q08 · Feedback-Events** (B-139, B-140). `GameEvent` in `src/model/types.ts` kennt nur Makro-Ereignisse. Für Sound und Juice fehlen: Treffer, Kill, Münze aufgehoben/gegeben, Pfeil, Schlag, Bau-Fortschritt, Tod, Wiederbeleben.
Fragen: Welche Liste ist Pflicht für den Spieleabend-Build? Wie viel Bandbreite je Tick darf das kosten (Pi 3, WLAN)?
🤖 Empfehlung: 12 Ereignisse als Minimum (siehe Plan SO2), Budget ≤ 200 Byte je Tick und Client im Mittel, gemessen mit dem Benchmark in F4; Ereignisse gehen im Delta mit, nicht als eigene Nachricht.

**Q09 · Golden-Hash ändern** (B-137). Mit jeder Regeländerung ändern sich Golden-Läufe. Wer bestätigt eine Änderung, wie wird sie begründet?
🤖 Empfehlung: ein `task golden:update`, Begründung im Commit-Text (Pflicht: welche Regel, welche Kennzahl), Review-Session prüft. Kein Freigabe-Zwang durch 🧑 bei Werteänderungen aus Beschlüssen.

**Q10 · Wann wird gespeichert** (B-147). Heute bei Tagesanbruch, Stufenwechsel und wenn das letzte Gerät geht. Kinder brechen mitten in der Nacht ab.
🤖 Empfehlung: zusätzlich beim Verlassen jedes Geräts und alle 60 s im Betrieb; maximaler Verlust 60 s, im HUD „gesichert“ nach Speichern.

**Q11 · Onboarding und Freundlich-Grad** (B-148). Umfang: nur Hinweise über Objekten oder auch ein geführter Tag? Grad: ohne Niederlage?
🤖 Empfehlung: kontextuelle Hinweise mit Controller-Glyphen (B-149), geführte erste Nacht optional, Grad „leicht“ aus `data/difficulty.json` ohne Niederlage.

**Q12 · Spielmetrik** (B-150). Was wird je Sitzung erfasst? Vorschlag: Tod durch was, Nächte überlebt, Zeit bis zum ersten Bau, Gold je Tag, Verbindungsabbrüche. Daten bleiben im Heimnetz (`reports/`), keine Namen, nur Geräte-Kürzel.
🤖 Empfehlung: Vorschlag übernehmen, JSON je Sitzung, Auswertung über k3c-dev.

**Q24 · Spieleabend 1** (B-151, P1). Termin, Teilnehmer, Dauer („ein Tag und eine Nacht“), Fragebogen. Kindgerecht: Daumen hoch/runter je Frage, ein Satz offenes Feedback.
🤖 Empfehlung: Fragebogen mit 8 Fragen (Spaß, Verständlichkeit, zu schwer/leicht, Lieblingsmoment, Ärgernis, Ton, Lesbarkeit, nochmal spielen?).

**Q22 · Reihenfolge Phase 2.** Wirtschaft zuerst (W1–W3: Hub-Ausbau, Plantage/Adern, Gebäude) oder Bürger zuerst (W4: Wiederbeleben, Berufe, Händler)?
🤖 Empfehlung: Wirtschaft zuerst (Spieler sehen Fortschritt am Hub), Wiederbeleben vorgezogen, weil es den Koop-Spaß am Spieleabend stärkt.

## Block 3 – Schienen Grafik, Sound, Balancing

**Q13 · Grafik-Stil** (B-161). Mehrere Packs (Sunnyland, Gothicvania, LuizMelo, …) mischen Raster und Palette. Vorschlag: Grundraster 16/32 px, gemeinsame Skalierung ×2 bis ×3, Palettenbruch nur bei Hintergründen; Packs, die nicht passen, werden Lücke.
Fragen: Wie strikt (Ausschlusskriterium) und welche Packs sind gesetzt?

**Q14 · Auswahl je Grafik-Lücke** (B-162). Die Suche liefert je Lücke 3 Kandidaten mit Vorschau, Lizenz (CC0/CC-BY), Stilbewertung. 🧑 wählt aus einer Referenzseite (G1-Muster). Frage: Auswahl am Bildschirm im Termin oder asynchron per Ticket-Kommentar?

**Q15 · Sound-Quellen und Stil** (B-167). Quellen: Kenney (CC0), OpenGameArt, freesound (CC0/CC-BY), selbst erzeugte Retro-SFX per Web Audio. Frage: Stilrichtung (Retro-Chiptune, orchestral, gemischt?), Gewicht auf Kindertauglichkeit (nicht erschreckend, Nacht leise statt laut).

**Q16 · Musik-Zustände und Hörproben** (B-168, B-169). Zustände Tag, Abend, Nacht, Kampf, Tiefe/Höhle, Boss, Lobby, Niederlage/Sieg; Crossfade; Ducking bei Warnungen. Hörproben auf `soundtest.html` auf dem TV. Frage: Anzahl Stücke je Zustand (Vorschlag 1–2), Lautheit am TV prüfen.

**Q19 · Bot-Profile** (B-158). Vorschlag: passiv, sparsam, Mauern zuerst, Wirtschaft zuerst, kooperativ (2 und 4 Spieler), „Kind-Bot“ (macht Fehler, vergisst Münzen). Frage: Welche sind für die Zielkorridore Pflicht?

## Block 4 – Betrieb und später

**Q07 · Registry und Pi-Modell** (SP11). Ist `ghcr.io/tenvan/k3c-family` öffentlich (Pi zieht ohne Anmeldung) oder privat (`docker login` mit Token)? Welches Pi-Modell genau (Pi 3 mit 64-Bit-OS)? Ohne Antwort gilt die Annahme der Sprint-Spec.

**Q17 · Absicherung von Restore und Heimnetz** (B-143). `/api/save/restore`, `/api/save`, `/api/report`, `/api/clientlog` ohne Token; Gäste im WLAN könnten Stände überschreiben.
🤖 Empfehlung: Restore nur mit `K3C_STATUS_TOKEN`; Report/Clientlog nur Größenlimit und Rotation; kein Port-Forwarding (dokumentieren).

**Q18 · Backup-Ziel für den Pi** (B-142). SD-Karten sterben. Wohin kopiert ein Task `saves/`? PC (SMB), NAS, USB-Stick am Pi? Häufigkeit (täglich) und Restore-Probe einmal durchspielen.

**Q20 · Release-Rhythmus** (B-170). Tag `v0.<n>.0` nach jeder Phase und jedem Spieleabend; Checkliste (Golden amd64/arm64, `task check:all`, Save-Migration, Dev-Reste aus, Pi-Image, Version im Status, Credits).

**Q21 · itch.io und Englisch** (B-023). Ob und wann. Hängt an Q05 (Sprache), CC-BY-Credits, Lizenz (PolyForm Noncommercial, Spiel kostenlos).

**Q23 · Reittiere** (B-152). Grafiken und Sattelpunkte liegen in `data/sprites.json` › `mounts`; Mechanik (Tempo, Kosten, Tod) fehlt. Wann, welcher Nutzen, Balancing?
