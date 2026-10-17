🛩️ Tenvan CC · Features

Band über der Eingabe
• Kontext – zeigt, wie voll das Kontextfenster ist; ab 85 % wird es gelb.
• Kompaktieren – legt /compact samt Ziel, Dateien und offenen Schritten ins Eingabefeld.
• Limits 5h/7d – zeigt den Verbrauch und ob du schneller bist als die Zeit.
• Cache – Restzeit und Trefferquote; ist er kalt, steht da, wie viel neu gelesen wird.
• Kosten – was die Session bisher gekostet hat.
• Schleifen-Warnung – schlägt an, wenn derselbe Aufruf mehrmals scheitert oder eine Datei ständig geändert wird.
• Effort – Stufe und Modell; „Auto“ lässt Haiku je Prompt Low/Medium/High wählen.
• Handoff – bei großem, kaltem Kontext: HANDOFF.md schreiben und kopieren statt teuer weiterzumachen.
• Sparmodus – ab 80 % bei 5h oder 7d deckelt ein Knopf an den Limits den Auto-Effort auf Low.
• Atemübung – solange Claude arbeitet, atmet ein Kreis im Band; die Arbeitszeile zählt die Phase mit (Übung in den Einstellungen).
• Merge Requests – auf GitLab-, GitHub- und Bitbucket-Repos MR/PR des Branches im Band (Pipeline, Freigaben, Threads), Hinweise bei Änderungen, im Panel Rebase, Mergen, Pipeline neu, Anmerkungen an Claude und „MR anlegen“ mit Entwurf.
• Folge-Schritte – drei Vorschläge nebeneinander nach jeder Antwort, Tasten 1–3.
• Checkliste – ganz oben Claudes Aufgabenliste mit Status (arbeitet, braucht dich, hängt?, fertig), Fortschritt, Restzeit, Pause/Weiter und Plan-Karte; /cockpit-verlauf mit Tages- und Wochenbericht zum Kopieren.

Panel rechts (/cockpit)
• Agent arbeitet – Laufzeit, Tool-Calls und Edits, solange ein Turn läuft.
• Arbeitsverzeichnis – Branch, Commit, Änderungen und Abstand zum Upstream.
• Kontextfenster – wofür der Kontext gerade draufgeht.
• Kontext-Diät – MCP-Server, die Tokens fressen, aber nie benutzt wurden.
• Turn-Bilanz – Kosten je Turn; teure Turns ohne Edit sind gelb.
• Tool-Freigaben – was erlaubt, nachgefragt oder abgelehnt wurde.
• Subagents – eine Kachel je Agent mit Tokens, Schritten und Laufzeit.
• Agent-Bremse – sperrt weitere Tool-Calls, wenn ein Subagent zu teuer wird.
• Hintergrundaufgaben – Shell-Jobs im Hintergrund mit Status.
• Log – die letzten Ereignisse.

Im Hintergrund
• Cache-Wächter – schlägt kurz vor Ablauf einen Prompt vor, damit der Cache warm bleibt.
• Zusammenfassung – fasst die Session in einem eigenen Panel zusammen.
• Chat-Skins – Symbole sowie Karten für Diffs, Code, Tabellen, Mermaid und Formeln.
