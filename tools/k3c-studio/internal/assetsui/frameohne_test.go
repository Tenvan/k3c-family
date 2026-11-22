package assetsui

import "testing"

// TestAssetsFrameOhne: das ausgeblendete Teil fehlt in den Boxen, das Bild weicht ab; ein ungültiger Name wird abgelehnt (B-734).
func TestAssetsFrameOhne(t *testing.T) {
	t.Parallel()
	d := dienst(testRepo(t), false)
	mit, err := d.AssetsFrame("monarch", "idle", 0, "", "")
	if err != nil {
		t.Fatal(err)
	}
	ohne, err := d.AssetsFrameOhne("monarch", "idle", 0, []string{"krone"})
	if err != nil {
		t.Fatal(err)
	}
	if _, ok := mit.Teile["krone"]; !ok {
		t.Fatal("krone fehlt schon mit allen Teilen")
	}
	if _, ok := ohne.Teile["krone"]; ok || ohne.Bild == mit.Bild {
		t.Fatalf("krone nicht ausgeblendet: %v", ohne.Teile)
	}
	if _, err := d.AssetsFrameOhne("monarch", "idle", 0, []string{"--out"}); err == nil {
		t.Fatal("ungültiger Teilname nicht abgelehnt")
	}
}
