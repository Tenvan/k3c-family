//go:build !windows

package game

import "time"

var clockStart = time.Now()

// clock liefert die echte Zeit in Sekunden seit einem festen, beliebigen Nullpunkt (nur für Messwerte).
func clock() float64 { return time.Since(clockStart).Seconds() }
