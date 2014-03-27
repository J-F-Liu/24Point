package main

import (
	"fmt"
	. "math/big"
)

type calculation struct {
	result     *Rat
	expression string
}

func (c calculation) String() string {
	return fmt.Sprintf("{%s %s}", c.result.RatString(), c.expression)
}

func findSolution(numbers ...int64) map[string]bool {
	solutions := make(map[string]bool)
	steps := make([]calculation, len(numbers))
	for index, number := range numbers {
		steps[index] = calculation{NewRat(number, 1), fmt.Sprint(number)}
	}
	results := make(chan calculation, 6)
	go func() {
		process(steps, results)
		close(results)
	}()
	for result := range results {
		if result.result.Cmp(NewRat(24, 1)) == 0 {
			solutions[result.expression] = true
		}
	}
	return solutions
}

func process(steps []calculation, results chan calculation) {
	if len(steps) > 1 {
		for numbers := range comb(steps) {
			for merged := range calc(numbers[0], numbers[1]) {
				remained := make([]calculation, 0, len(steps)-1)
				remained = append(remained, merged)
				remained = append(remained, numbers[2:]...)
				process(remained, results)
			}
		}
	} else if len(steps) == 1 {
		//fmt.Printf("%s\n", steps[0])
		results <- steps[0]
	}
}

func comb(steps []calculation) chan []calculation {
	queue := make(chan []calculation)
	go func() {
		n := len(steps)
		for i := 0; i < n; i++ {
			for j := i + 1; j < n; j++ {
				selected := make([]calculation, 0, n)
				selected = append(selected, steps[i], steps[j])
				for k := 0; k < n; k++ {
					if k != i && k != j {
						selected = append(selected, steps[k])
					}
				}
				queue <- selected
			}
		}
		close(queue)
	}()
	return queue
}

func calc(stepA, stepB calculation) chan calculation {
	queue := make(chan calculation, 6)
	go func() {
		a := stepA.result
		b := stepB.result
		queue <- calculation{NewRat(0, 1).Add(a, b), fmt.Sprintf("(%s + %s)", stepA.expression, stepB.expression)}
		queue <- calculation{NewRat(0, 1).Mul(a, b), fmt.Sprintf("(%s * %s)", stepA.expression, stepB.expression)}
		if a.Cmp(b) >= 0 {
			queue <- calculation{NewRat(0, 1).Sub(a, b), fmt.Sprintf("(%s - %s)", stepA.expression, stepB.expression)}
		} else {
			queue <- calculation{NewRat(0, 1).Sub(b, a), fmt.Sprintf("(%s - %s)", stepB.expression, stepA.expression)}
		}
		if b.Cmp(NewRat(0, 1)) != 0 {
			queue <- calculation{NewRat(0, 1).Quo(a, b), fmt.Sprintf("(%s / %s)", stepA.expression, stepB.expression)}
		}
		if a.Cmp(NewRat(0, 1)) != 0 && a.Cmp(b) != 0 {
			queue <- calculation{NewRat(0, 1).Quo(b, a), fmt.Sprintf("(%s / %s)", stepB.expression, stepA.expression)}
		}
		close(queue)
	}()
	return queue
}
