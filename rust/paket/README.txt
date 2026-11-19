K3C – Family Three Crowns (Couch-Koop-Side-Scroller)
=====================================================

Start
-----
Doppelklick auf k3c-game.exe. Das Spiel läuft unter Windows 10/11 (64 Bit) ohne Installation; der ganze Ordner
kann an einen anderen Ort oder auf einen anderen PC kopiert werden. Die Ordner data, public und native müssen
neben der EXE liegen bleiben. Eine Grafikkarte mit DirectX 12, Vulkan oder OpenGL wird gebraucht.

Spieler
-------
1 bis 2 Spieler am selben PC (geteilter Bildschirm): ein Spieler kann die Tastatur nutzen, alle anderen brauchen
einen Controller; wer beitritt, wählt es in der Lobby. Die Tastenbelegung zeigt das Spiel im Menü unter den Optionen (Steuerung); ein Controller-Test steht im Menü.
Im Menü geht die Controller-Taste B (oder Esc) einen Schritt zurück.

Spielstände
-----------
Die Spielstände liegen im Ordner saves neben der EXE (er entsteht beim ersten Speichern). Zum Sichern oder
Mitnehmen den Ordner saves kopieren. Ein anderer Ort: k3c-game.exe --saves <ordner>

Inhalt des Ordners
------------------
  k3c-game.exe        das Spiel
  data/               Spielwerte und Welten (JSON)
  public/grafik/      Papierschnitt-Grafik
  public/audio/       Klänge und Musik
  public/schrift/     Schrift samt Lizenz
  public/rig/         Teile der Figuren
  LICENSE             Lizenz des Spiels (PolyForm Noncommercial 1.0.0)
  LICENSE-ASSETS      Lizenz der eigenen Grafiken, Animationen, Klänge und Musik
  LIZENZEN.txt        Lizenzen der verwendeten Fremdwerke (Rust-Bibliotheken, Schrift, Klänge)

Das Spiel ist nicht für den kommerziellen Gebrauch freigegeben; Weitergabe nur unter den Bedingungen von LICENSE,
LICENSE-ASSETS und LIZENZEN.txt.

Probleme
--------
Startet das Spiel nicht oder bleibt der Bildschirm schwarz: Grafikkartentreiber aktualisieren, dann erneut
starten. Neben dem Spiel öffnet sich ein Konsolenfenster mit Meldungen; es kann offen bleiben.
