package net

// Delete gehört zu room.Store (B-086); der Fake in ws_test.go liegt knapp an der Dateigrenze.
func (s memSaves) Delete(name string) error { delete(s, name); return nil }
