//go:build !windows

package main

import "time"

var clockStart = time.Now()

// clock liefert die echte Zeit in Sekunden seit einem festen, beliebigen Nullpunkt.
func clock() float64 { return time.Since(clockStart).Seconds() }
