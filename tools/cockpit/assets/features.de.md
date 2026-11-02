🛩️ Tenvan CC · Features

Ein Cockpit: Band über der Eingabe (nur Links), „Tenvan“ unter dem Prompt, ein Panel mit sechs Reitern.

Band über der Eingabe
• Checkliste – Claudes Aufgabenliste mit Status (arbeitet, braucht dich, hängt?, fertig), Fortschritt, Restzeit; Pause, Weiter, Plan als Links; Antworten auf Rückfragen direkt unter der Frage (Tasten 4–6).
• Atemübung – solange Claude arbeitet, atmet der Statusring der Checkliste; die ganze Übung steht im Reiter Jetzt.
• Messwerte – Limits 5h/7d mit Reset, Effort und Modell mit „Auto“, Kontext mit „Kompaktieren“ (ab 85 % gelb), Cache mit Trefferquote, Kosten. Jeder Wert führt zum Reiter Verbrauch.
• Handoff – bei großem, kaltem Kontext: HANDOFF.md schreiben und kopieren statt teuer weiterzumachen.
• Sparmodus – ab 80 % bei 5h oder 7d deckelt ein Link an den Limits den Auto-Effort auf Low.
• Merge Request – MR des Branches im Kopf (Pipeline, Freigaben, Threads), Links „Anmerkungen → Claude“, „Konflikt lösen“, „MR anlegen“.
• Meldung – nur bei Gelb oder Rot die wichtigste Meldung (Schleife, Verzeichniswechsel, Fehler, Wächter, Tipps, Diagnose), dahinter „+n Hinweise“.
• Folge-Schritte – drei Vorschläge nach jeder Antwort, Tasten 1–3.

Panel (/cockpit oder „Tenvan“)
• Jetzt – Agent arbeitet, Plan mit Zeiten und Dateien, Subagents mit Bremse, Hintergrundaufgaben, Letzter Turn mit Turn-Bilanz.
• Verbrauch – Übersicht mit Takt und Reset, Kontextfenster mit Kontext-Diät, Kontext-Inspektor (/cockpit-kontext).
• Diagnose – Hinweise (alle Meldungen, Tipps, Nutzungswächter), Protokoll (Log, Fehler-Eingang, Tool-Freigaben, Zeitreise mit Diff und Zurück), Profil (/cockpit-profil), Blast-Radius (/cockpit-blast).
• Git – Arbeitsverzeichnis, Ablauf Commit → MR → Merge → Aufräumen, Branches, MR-Karten (/cockpit-git).
• Rückblick – Zusammenfassung dieser Session und Verlauf mit Tages- und Wochenbericht (/cockpit-verlauf).
• Einstellungen – jeder Schalter, Sprache, Skin-Demo (/cockpit-einstellungen).

Im Hintergrund
• Cache-Wächter – schlägt kurz vor Ablauf einen Prompt vor, damit der Cache warm bleibt.
• Chat-Skins – Symbole sowie Karten für Diffs, Code, Tabellen, Mermaid und Formeln.
