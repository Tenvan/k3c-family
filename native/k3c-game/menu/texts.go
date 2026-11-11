package menu

import (
	"strings"

	"k3c/native/k3c-game/view"
)

// Spieltexte der Menüs in Deutsch und Englisch (wie src/core/texts.de.ts und texts.en.ts). Die Debug-Schrift kennt nur
// Latin-1: statt ◀ ▶ stehen < >. Fehler der Session sind deutsch (view.Session) und erscheinen unübersetzt.

type textKey int

const (
	txtTitle textKey = iota
	txtContinue
	txtContinueSub
	txtContinueNone
	txtNew
	txtNewSub
	txtLoad
	txtLoadSub
	txtOptions
	txtOptionsSub
	txtQuit
	txtQuitSub
	txtMainHint
	txtLobbyTitle
	txtGrade
	txtGoal
	txtDefeat
	txtWorld
	txtSeed
	txtSeedAuto
	txtStart
	txtCancel
	txtPlayer
	txtPlayerFree
	txtLobbyHint
	txtTooMany
	txtSavesTitle
	txtSavesEmpty
	txtSavesLine
	txtDepth
	txtSavesHint
	txtDeleteAsk
	txtDeleteNo
	txtDeleteYes
	txtDeleted
	txtAskHint
	txtMusic
	txtEffects
	txtFullscreen
	txtLanguage
	txtBack
	txtOn
	txtOff
	txtOptionsHint
	txtPauseTitle
	txtResume
	txtRemove
	txtLeave
	txtPauseHint
	txtNoSeat
	txtLost
	txtWon
	txtBy
	txtKeysLeft
	txtKeysRight
	txtPad
	txtGradeEasy
	txtGradeNormal
	txtGradeHard
	txtGradeUltra
	txtGoalEndboss
	txtGoalGold
	txtGoalDays
	txtGoalMineAll
	txtGoalBuildAll
	txtDefeatResources
	txtDefeatStage
	txtDefeatLost
	txtLangName
	textCount
)

// texts[k] = {Deutsch, Englisch}; `{}` ist der Platzhalter für tr1.
var texts = [textCount][2]string{
	txtTitle:           {"Family Three Crowns", "Family Three Crowns"},
	txtContinue:        {"Weiter", "Continue"},
	txtContinueSub:     {"Spielstand {}", "Save {}"},
	txtContinueNone:    {"Kein Spielstand vorhanden", "No save yet"},
	txtNew:             {"Neues Spiel", "New game"},
	txtNewSub:          {"Grad, Ziel und Welt wählen", "Pick grade, goal and world"},
	txtLoad:            {"Laden", "Load"},
	txtLoadSub:         {"Spielstände", "Saves"},
	txtOptions:         {"Optionen", "Options"},
	txtOptionsSub:      {"Ton, Anzeige, Sprache", "Sound, display, language"},
	txtQuit:            {"Beenden", "Quit"},
	txtQuitSub:         {"Spiel schließen", "Close the game"},
	txtMainHint:        {"Hoch/Runter wählen · A / Enter bestätigen", "Up/Down select · A / Enter confirm"},
	txtLobbyTitle:      {"Neues Spiel", "New game"},
	txtGrade:           {"Grad", "Grade"},
	txtGoal:            {"Ziel", "Goal"},
	txtDefeat:          {"Niederlage", "Defeat"},
	txtWorld:           {"Welt", "World"},
	txtSeed:            {"Seed", "Seed"},
	txtSeedAuto:        {"Name des Spielstands", "Name of the save"},
	txtStart:           {"Spiel starten", "Start game"},
	txtCancel:          {"Abbrechen", "Cancel"},
	txtPlayer:          {"Spieler {}", "Player {}"},
	txtPlayerFree:      {"frei: A drücken", "free: press A"},
	txtLobbyHint:       {"A auf einem zweiten Gerät: mitspielen · Links/Rechts ändern · B zurück bzw. Spieler 2 geht", "A on a second device: join · Left/Right change · B back or player 2 leaves"},
	txtTooMany:         {"Höchstens 2 Spieler", "At most 2 players"},
	txtSavesTitle:      {"Spielstände", "Saves"},
	txtSavesEmpty:      {"Keine Spielstände", "No saves"},
	txtSavesLine:       {"Tag {}", "Day {}"},
	txtDepth:           {"Stufe", "Level"},
	txtSavesHint:       {"A laden · View / Tab löschen · B zurück", "A load · View / Tab delete · B back"},
	txtDeleteAsk:       {"Spielstand {} löschen? Er bleibt als Sicherung erhalten.", "Delete save {}? It is kept as a backup."},
	txtDeleteNo:        {"Abbrechen", "Cancel"},
	txtDeleteYes:       {"Löschen", "Delete"},
	txtDeleted:         {"Spielstand {} gelöscht", "Save {} deleted"},
	txtAskHint:         {"Hoch/Runter wählen · A bestätigen · B abbrechen", "Up/Down select · A confirm · B cancel"},
	txtMusic:           {"Musik", "Music"},
	txtEffects:         {"Effekte", "Effects"},
	txtFullscreen:      {"Vollbild", "Fullscreen"},
	txtLanguage:        {"Sprache", "Language"},
	txtBack:            {"Zurück", "Back"},
	txtOn:              {"an", "on"},
	txtOff:             {"aus", "off"},
	txtOptionsHint:     {"Hoch/Runter wählen · Links/Rechts ändern · A umlegen · B zurück", "Up/Down select · Left/Right change · A toggle · B back"},
	txtPauseTitle:      {"Pause", "Paused"},
	txtResume:          {"Fortsetzen", "Resume"},
	txtRemove:          {"Spieler abmelden", "Remove player"},
	txtLeave:           {"Spiel verlassen", "Leave game"},
	txtPauseHint:       {"Hoch/Runter wählen · A bestätigen · B / Menu zurück ins Spiel", "Up/Down select · A confirm · B / Menu back to the game"},
	txtNoSeat:          {"Dieses Gerät steuert keinen Spieler", "This device controls no player"},
	txtLost:            {"Die Insel ist verloren. Weiter lädt den letzten Spielstand.", "The island is lost. Continue loads the last save."},
	txtWon:             {"Ziel erreicht! Das Spiel ist gespeichert.", "Goal reached! The game is saved."},
	txtBy:              {"bedient: {}", "operated by: {}"},
	txtKeysLeft:        {"Tastatur links", "Keyboard left"},
	txtKeysRight:       {"Tastatur rechts", "Keyboard right"},
	txtPad:             {"Pad {}", "Pad {}"},
	txtGradeEasy:       {"Leicht", "Easy"},
	txtGradeNormal:     {"Normal", "Normal"},
	txtGradeHard:       {"Hart", "Hard"},
	txtGradeUltra:      {"Ultra", "Ultra"},
	txtGoalEndboss:     {"Endboss", "Final boss"},
	txtGoalGold:        {"Gold sammeln", "Collect gold"},
	txtGoalDays:        {"Tage überleben", "Survive days"},
	txtGoalMineAll:     {"Alles abbauen", "Mine everything"},
	txtGoalBuildAll:    {"Alles ausbauen", "Build everything"},
	txtDefeatResources: {"Gold/Material-Verlust", "Lose gold/material"},
	txtDefeatStage:     {"Stufenverlust", "Lose level"},
	txtDefeatLost:      {"Komplett verloren", "Game over"},
	txtLangName:        {"Deutsch", "English"},
}

// langIndex liefert den Index der Sprache in texts (unbekannt = Deutsch, wie view.Settings.Normalized).
func langIndex(lang string) int {
	if lang == view.LanguageEnglish {
		return 1
	}
	return 0
}

// tr liefert den Text k in der Sprache lang.
func tr(lang string, k textKey) string { return texts[k][langIndex(lang)] }

// tr1 liefert den Text k mit arg statt des Platzhalters `{}`.
func tr1(lang string, k textKey, arg string) string {
	return strings.Replace(tr(lang, k), "{}", arg, 1)
}
