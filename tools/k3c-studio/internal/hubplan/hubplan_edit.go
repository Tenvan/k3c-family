package hubplan

import "context"

// Bearbeiten im Gebäude-Planer (B-462/AC-02, WZ9.3): Das Frontend ändert den Kandidaten (Text der hub.json) und lässt
// ihn hier prüfen; Matrix und Befunde kommen aus derselben Engine-Funktion wie beim Laden (k3c-game sim hub).

// Check prüft einen Kandidaten des Planers und liefert Matrix und Befunde, ohne etwas zu schreiben.
func (e Editor) Check(ctx context.Context, candidate string) HubPlanView {
	return e.planOf(ctx, hubFile, []byte(candidate))
}
