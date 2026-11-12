package view

// Tastenbelegung für die Seite „Tasten“ im Pause-Menü (B-807): game baut die Zeilen aus input (Tasten) und hud
// (Beschriftungen) und reicht sie an menu (menu.New); menu kennt weder input noch hud.

// Aktionen der Tastenbelegung (KeyRow.Action) in der Reihenfolge der Seite; menu beschriftet sie in der Sprache.
const (
	KeyMove       = "move"   // laufen (links/rechts)
	KeySprint     = "sprint" // Galopp
	KeyPay        = "pay"    // zahlen, bestätigen
	KeyAttack     = "attack" // Schlag
	KeySkill1     = "skill1" // Skill-Slot 1 … 4
	KeySkill2     = "skill2"
	KeySkill3     = "skill3"
	KeySkill4     = "skill4"
	KeySkillMenu  = "skillMenu"  // Skill-Menü
	KeyPause      = "pause"      // Pause-Menü
	KeyFullscreen = "fullscreen" // Vollbild
)

// KeyActions sind alle Aktionen der Tastenbelegung in der Reihenfolge der Seite.
var KeyActions = [...]string{KeyMove, KeySprint, KeyPay, KeyAttack, KeySkill1, KeySkill2, KeySkill3, KeySkill4,
	KeySkillMenu, KeyPause, KeyFullscreen}

// KeyRow ist eine Zeile der Tastenbelegung: Aktion (KeyActions) und die fertig beschrifteten Tasten der linken und
// rechten Tastatur-Hälfte und des Gamepads (Standardbelegung, auch ohne angeschlossenes Pad); "" = nicht belegt.
type KeyRow struct {
	Action, Left, Right, Pad string
}

// KeyTable liefert die Tastenbelegung in der Sprache lang (DefaultLanguage oder LanguageEnglish), eine Zeile je
// Eintrag von KeyActions. Der Slice gehört dem Aufrufer.
type KeyTable func(lang string) []KeyRow
