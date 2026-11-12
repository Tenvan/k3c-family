// Package view ist der Vertrag zwischen Spielzustand und Darstellung des nativen Spiels (B-792). Er ist nach NG1
// eingefroren: Braucht ein Fachpaket mehr, gibt es ein Ticket an die NG1-Nachfolge, keinen Eingriff aus NG2–NG8.
// Additiv ergänzt bei der Zusammenführung der Welle 2b: Session.Settings und Session.Seats, Frame.Outcome mit
// EndDelay, Frame.Cells, Frame.Recording, StatsRect und die Rolle MenuPages.
// Additiv ergänzt in NG14 (B-810, B-816): Settings.River und Settings.Stats (F3 und F2, ohne Optionen-Menü),
// RiverShader und RiverTiles, Settings.NextRiver, StatsRectLines und StatsTextSizeLines (zweite Zeile der
// Leistungsanzeige: Update und Draw in ms, nach unten über StatsRect hinaus; Drawer halten weiter nur StatsRect frei).
// Additiv ergänzt in NG13 (B-807, B-809): KeyRow, KeyTable und KeyActions (Tastenbelegung: game baut sie aus input und
// hud, menu zeigt sie; menu.New bekommt die KeyTable), Settings.FlashesOff, Settings.GuideOff und Settings.GuideSeen
// (Null-Wert = Standard; hud meldet neu gesehene Hinweise über Session.SetSettings), MenuPages kennt "keys" und
// "licenses". Seit NG13 zeichnet game beide Zeilen der Anzeige mit der Schrift des Spiels innerhalb von StatsRect;
// über StatsRect hinaus (StatsRectLines) reicht nur noch die Debug-Schrift ohne Schriftdatei.
//
// Rollen:
//   - game (Schleife) besitzt Fenster, Zellen, Kameras, Menü-Stapel, Geräte-Zuordnung, Grafik der Level und die
//     Simulation (local); nur game kennt alle Pakete.
//   - local erfüllt Session (Spiel, Spielstände, Skills, Geräte der Spieler, Einstellungen) und liefert je Update einen Frame.
//   - Fachpakete (world, actors, props, input, hud, fx, menu, audio) liefern Drawer, Listener, Source oder Menus
//     (Einträge in game.Packages). Jeder dieser Beiträge kann zusätzlich optionale Rollen erfüllen (roles.go), game
//     meldet sie per Typ-Zusicherung selbst an: CameraRule (world, höchstens eine), AssetSource (wer Papier-Sheets
//     zeichnet), Overlay (hud: Skill-Menü im Spiel), Listener (Drawer mit Zustand). MenuPages (menu) fragt game nur
//     im Aufnahme-Modus (--menu) direkt an Menus ab.
//
// Importregel (geprüft in game/imports_test.go): Fachpakete importieren nur view, art, k3c/engine/sim (lesend) und
// k3c/data, dazu Standardbibliothek und Ebitengine; nie einander, nie game oder local. view selbst kennt nur sim.
// Rig-Figuren gehen nur über art (Actor, Painter, M, Point, ActorFrame, RigView), nie über k3c-rig direkt.
//
// Ablauf je Update (30 Hz, ein Simulationstakt):
//  1. game liest Source.Poll und vergleicht die Geräte mit dem vorigen Update (Cues CueDeviceConnected/Lost).
//  2. Liegt ein Screen oben, bekommt er die Eingabe (Screen.Update). Sonst im Spiel: Overlays zuerst (verbrauchte
//     Spieler: leerer Befehl, Aktionen zählen nicht), ActionMenu oder ActionHome öffnet das Pause-Menü, verwaiste
//     Plätze bekommen ein freies Gerät (Zuordnungs-Regel game.Rebinds, Session.Rebind), dann Befehle, dann Beitritt
//     freier Geräte mit Bestätigen. B (ActionBack) ist im Spiel unbelegt. Vorher: Trägt der Frame seit EndDelay
//     Takten einen Ausgang (Frame.Outcome), schließt game das Spiel (Session.Close), legt das Hauptmenü auf und tut in
//     diesem Update sonst nichts.
//  3. local taktet und baut den Frame; game ergänzt Unseated, Seat.Lost und die Cues, lädt bei geänderten Leveln die
//     Grafik (AssetSource → art.Library.SetLevels; im Fenster setzt der erste Update eines Stufenwechsels „lädt“, game
//     zeigt die Ladeanzeige und lädt im folgenden Update; fehlende Grafik zeigt game als Meldung mit Pfad im Fenster
//     und beendet erst auf Enter, Esc oder A, danach mit stderr und Exit 1), setzt je Zelle
//     die Kamera (CameraRule, danach --cam), trägt Frame.Cells und Frame.Recording ein, wendet Vollbild aus
//     Frame.Settings an und ruft Listener.Listen genau einmal je Frame (Ereignisse und Cues erscheinen in genau einem
//     Frame).
//
// Aufnahme-Modus (--shots, Frame.Recording): Der Vorlauf (--tick) rechnet jedes Update wie im Spiel (Eingabe,
// Overlays, Menüs, Listener), nur ohne Zeichnen; das Bild hängt nur vom Takt ab (local speichert im Takt, feste Uhr
// der Spielstände, kein Ton, kein Vollbild). game zeichnet oben rechts in StatsRect den Takt statt der FPS; Drawer
// halten die Fläche frei.
//
// Draw je Bild: für jede Zelle die Drawer der Zellen-Ebenen (LayerFar … LayerHUD) in Ebenen-Reihenfolge, danach einmal
// die Overlay-Drawer für das ganze Fenster, zuletzt der oberste Screen. Draw kann denselben Frame mehrmals zeichnen
// und ändert keinen Zustand (Kameras rechnet nur das Update). Zustand über Bilder hinweg (Animation, Umhang der
// Rig-Figuren in actors) schreibt ein Listener im Takt fort, nie Draw (einzige Ausnahme: game merkt sich, dass die
// Ladeanzeige gezeichnet wurde, und lädt erst danach); das Bild hängt nicht davon ab, wie oft Draw
// läuft, in welcher Reihenfolge die Zellen kommen oder was vorher sichtbar war. Erlaubt sind in Draw nur Puffer und
// Zwischenspeicher, die das Bild nicht ändern (Messwerte von Texten, Zeichenlisten je Zelle, Konturtexte über
// art.OutlineCache, das ein Listener je Takt mit Sweep aufräumt).
//
// Spieler, Zellen, Geräte: Seat.Player ist der Slot im Raum und bleibt fest; Zelle i zeigt Frame.Seats[i] (geht
// Spieler 0, steht Spieler 1 in Zelle 0). DeviceID.Index ist bei Pads die ebiten.GamepadID; ein Platz hängt nie an
// einem festen Pad-Index.
//
// Regeln für alle Drawer: *sim.World und alles im Frame nur lesen und nicht über das Update hinaus halten (Puffer
// werden wiederverwendet); keine Wanduhr (nur Frame.Clock), kein Zufall außerhalb der Simulation; Lage, Reihenfolge
// und Auswahl als reine Funktionen ohne ebiten.Image, mit Tests.
package view

// MaxPlayers ist die Höchstzahl lokaler Spieler (Zellen) im Split-Screen.
const MaxPlayers = 2
