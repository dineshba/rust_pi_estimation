package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {
	now := time.Now()
	estimatePi(160000000, 16)
	fmt.Println(time.Now().Sub(now))
}

func estimatePi(totalPoints int, totalThreads int) {
	ch := make(chan int)

	for i := 0; i < totalThreads; i++ {
		go computeInSingleThread(totalPoints/totalThreads, ch)
	}

	pointsInCircle := 0
	for i := 0; i < totalThreads; i++ {
		a := <-ch
		pointsInCircle += a
	}

	fmt.Println(4.0 * float64(pointsInCircle) / float64(totalPoints))
}

func computeInSingleThread(totalPoints int, ch chan int) {
	pointsInCircle := 0
	for totalPoints > 0 {
		x := rand.Float64()*2.0 - 1
		y := rand.Float64()*2.0 - 1
		if x*x+y*y < 1.0 {
			pointsInCircle++
		}
		totalPoints--
	}
	ch <- pointsInCircle
}
