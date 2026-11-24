//! Die Texte zu `texts.rs`: `TEXTS[k] = [Deutsch, Englisch]`; `{}` ist der Platzhalter für `tr1`.

use super::texts::TEXT_COUNT;

pub static TEXTS: [[&str; 2]; TEXT_COUNT] = [
    ["Family Three Crowns", "Family Three Crowns"],               // Title
    ["Weiter", "Continue"],                                       // Continue
    ["Spielstand {}", "Save {}"],                                 // ContinueSub
    ["Kein Spielstand vorhanden", "No save yet"],                 // ContinueNone
    ["lädt ...", "loading ..."],                                  // ContinueLoading
    ["Neues Spiel", "New game"],                                  // New
    ["Grad, Ziel und Welt wählen", "Pick grade, goal and world"], // NewSub
    ["Laden", "Load"],                                            // Load
    ["Spielstände", "Saves"],                                     // LoadSub
    ["Optionen", "Options"],                                      // Options
    ["Ton, Anzeige, Sprache, Steuerung", "Sound, display, language, controls"], // OptionsSub
    ["Beenden", "Quit"],                                          // Quit
    ["Spiel schließen", "Close the game"],                        // QuitSub
    [
        "Hoch/Runter wählen · A / Enter bestätigen",
        "Up/Down select · A / Enter confirm",
    ], // MainHint
    ["Neues Spiel", "New game"],                                  // LobbyTitle
    ["Grad", "Grade"],                                            // Grade
    ["Ziel", "Goal"],                                             // Goal
    ["Niederlage", "Defeat"],                                     // Defeat
    ["Welt", "World"],                                            // World
    ["Seed", "Seed"],                                             // Seed
    ["Name des Spielstands", "Name of the save"],                 // SeedAuto
    ["Spiel starten", "Start game"],                              // Start
    ["Abbrechen", "Cancel"],                                      // Cancel
    ["Spieler {}", "Player {}"],                                  // Player
    ["frei: A drücken", "free: press A"],                         // PlayerFree
    [
        "A auf einem zweiten Gerät: mitspielen · Links/Rechts ändern · B zurück bzw. Spieler 2 geht",
        "A on a second device: join · Left/Right change · B back or player 2 leaves",
    ], // LobbyHint
    [
        "Links/Rechts blättern · A wählen · B zurück",
        "Left/Right browse · A choose · B back",
    ], // WorldHint
    ["Höchstens 2 Spieler", "At most 2 players"],                 // TooMany
    ["Spielstände", "Saves"],                                     // SavesTitle
    ["Keine Spielstände", "No saves"],                            // SavesEmpty
    ["Tag {}", "Day {}"],                                         // SavesLine
    ["Stufe", "Level"],                                           // Depth
    [
        "A laden · View / Tab löschen · B zurück",
        "A load · View / Tab delete · B back",
    ], // SavesHint
    [
        "Spielstand {} löschen? Er bleibt als Sicherung erhalten.",
        "Delete save {}? It is kept as a backup.",
    ], // DeleteAsk
    ["Abbrechen", "Cancel"],                                      // DeleteNo
    ["Löschen", "Delete"],                                        // DeleteYes
    ["Spielstand {} gelöscht", "Save {} deleted"],                // Deleted
    [
        "Hoch/Runter wählen · A bestätigen · B abbrechen",
        "Up/Down select · A confirm · B cancel",
    ], // AskHint
    ["Musik", "Music"],                                           // Music
    ["Effekte", "Effects"],                                       // Effects
    ["Vollbild", "Fullscreen"],                                   // Fullscreen
    ["Anzeige", "Display"],                                       // Display
    ["Fenster", "Windowed"],                                      // Windowed
    ["Rahmenlos", "Borderless"],                                  // Borderless
    ["Sprache", "Language"],                                      // Language
    ["Zurück", "Back"],                                           // Back
    ["an", "on"],                                                 // On
    ["aus", "off"],                                               // Off
    [
        "Hoch/Runter wählen · Links/Rechts ändern · A umlegen · B zurück",
        "Up/Down select · Left/Right change · A toggle · B back",
    ], // OptionsHint
    ["Pause", "Paused"],                                          // PauseTitle
    ["Fortsetzen", "Resume"],                                     // Resume
    ["Spieler abmelden", "Remove player"],                        // Remove
    ["Spiel verlassen", "Leave game"],                            // Leave
    [
        "Hoch/Runter wählen · A bestätigen · B / Menu zurück ins Spiel",
        "Up/Down select · A confirm · B / Menu back to the game",
    ], // PauseHint
    ["Dieses Gerät steuert keinen Spieler", "This device controls no player"], // NoSeat
    [
        "Die Insel ist verloren. Weiter lädt den letzten Spielstand.",
        "The island is lost. Continue loads the last save.",
    ], // Lost
    [
        "Ziel erreicht! Das Spiel ist gespeichert.",
        "Goal reached! The game is saved.",
    ], // Won
    ["bedient: {}", "operated by: {}"],                           // By
    ["Tastatur WASD", "Keyboard WASD"],                           // KeysWASD
    ["Tastatur Pfeile", "Keyboard arrows"],                       // KeysArrows
    ["Tastatur", "Keyboard"],                                     // Keyboard
    ["Gamepad", "Gamepad"],                                       // Gamepad
    ["Steuerung", "Controls"],                                    // Controls
    [
        "Hoch/Runter wählen · Links/Rechts Gerät · die Tastatur hat höchstens ein Spieler · B zurück",
        "Up/Down select · Left/Right device · only one player may use the keyboard · B back",
    ], // ControlsHint
    ["Pad {}", "Pad {}"],                                         // Pad
    ["Leicht", "Easy"],                                           // GradeEasy
    ["Normal", "Normal"],                                         // GradeNormal
    ["Hart", "Hard"],                                             // GradeHard
    ["Ultra", "Ultra"],                                           // GradeUltra
    ["Endboss", "Final boss"],                                    // GoalEndboss
    ["Gold sammeln", "Collect gold"],                             // GoalGold
    ["Tage überleben", "Survive days"],                           // GoalDays
    ["Alles abbauen", "Mine everything"],                         // GoalMineAll
    ["Alles ausbauen", "Build everything"],                       // GoalBuildAll
    ["Gold/Material-Verlust", "Lose gold/material"],              // DefeatResources
    ["Stufenverlust", "Lose level"],                              // DefeatStage
    ["Deutsch", "English"],                                       // LangName
    ["Blitze", "Flashes"],                                        // Flashes
    ["Anleitung", "Guide"],                                       // Guide
    ["Tasten", "Keys"],                                           // Keys
    ["B / Esc zurück", "B / Esc back"],                           // KeysHint
    ["Aktion", "Action"],                                         // KeyAction
    ["Gamepad", "Gamepad"],                                       // PadColumn
    ["Laufen", "Walk"],                                           // KeyMove
    ["Galopp", "Gallop"],                                         // KeySprint
    ["Zahlen, bestätigen", "Pay, confirm"],                       // KeyPay
    ["Schlag", "Attack"],                                         // KeyAttack
    ["Skill {}", "Skill {}"],                                     // KeySkill
    ["Skill-Menü", "Skill menu"],                                 // KeySkillMenu
    ["Pause", "Pause"],                                           // KeyPause
    ["Vollbild", "Fullscreen"],                                   // KeyFullscreen
    ["Lizenzen", "Licenses"],                                     // Licenses
    ["Engine, Schrift, Audio", "Engine, font, audio"],            // LicensesSub
    [
        "Lizenzdatei: LIZENZEN.txt im Paket · B zurück",
        "License file: LIZENZEN.txt in the package · B back",
    ], // LicensesHint
    ["Teil", "Part"],                                             // LicPart
    ["Werk", "Work"],                                             // LicWork
    ["Urheber", "Author"],                                        // LicHolder
    ["Lizenz", "License"],                                        // LicLicense
    ["Engine", "Engine"],                                         // LicEngine
    ["Schrift", "Font"],                                          // LicFont
    ["Grafik", "Graphics"],                                       // LicGraphics
    ["Audio", "Audio"],                                           // LicAudio
    ["Version {}", "Version {}"],                                 // Version
    ["unbekannt", "unknown"],                                     // VersionUnknown
    ["Testing", "Testing"],                                       // Testing
    ["Stresstest einstellen und starten", "Set up and start a stress test"], // TestingSub
    [
        "Hoch/Runter wählen · Links/Rechts ändern · A starten · B zurück",
        "Up/Down select · Left/Right change · A start · B back",
    ], // TestingHint
    ["Vorgabe", "Preset"],                                        // TstPreset
    ["Spieler", "Players"],                                       // TstPlayers
    ["Stufe (Tiefe)", "Stage (depth)"],                           // TstDepth
    ["Gegner je Welle", "Enemies per wave"],                      // TstEnemies
    ["Anstieg", "Growth"],                                        // TstGrowth
    ["Faktor je Welle", "Factor per wave"],                       // TstFactor
    ["Ausbau vorab", "Build up front"],                           // TstBuild
    ["Truppen vorab", "Troops up front"],                         // TstTroops
    ["Zeitraffer", "Time lapse"],                                 // TstTimescale
    ["Ende nach Wellen", "Stop after waves"],                     // TstWaves
    ["Ende nach Minuten", "Stop after minutes"],                  // TstMinutes
    ["Modus", "Mode"],                                            // TstMode
    ["normal", "normal"],                                         // TstModeNormal
    ["rein (Gegner an der Burg)", "pure (enemies at the castle)"], // TstModePure
    ["eigene Werte", "own values"],                               // TstOwn
    ["linear", "linear"],                                         // TstLinear
    ["exponentiell", "exponential"],                              // TstExp
    ["keine", "none"],                                            // TstNone
    [
        "Vorgaben nicht lesbar (data/stresstests.json)",
        "Presets not readable (data/stresstests.json)",
    ], // TstNoPresets
    ["Stresstest hier nicht verfügbar", "Stress test not available here"], // TstNoRunner
    ["Entwicklermodus", "Developer mode"],                        // DevMode
    ["Test starten", "Start test"],                               // TstStart
    ["Tasten-Test", "Button test"],                               // TstPads
    ["Tasten drücken · B zurück", "Press buttons · B back"],      // PadTestHint
    ["{} Tag", "{} day"],                                         // SavesDay
    ["{} Tage", "{} days"],                                       // SavesDays
    ["{} Monarch", "{} monarch"],                                 // SavesMonarch
    ["{} Monarchen", "{} monarchs"],                              // SavesMonarchs
];
