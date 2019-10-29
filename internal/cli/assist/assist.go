package assist

func OneTrueOf(states ...bool) (result bool) {
	for i, currentState := range states {
		var temp bool = currentState
		for k, state := range states {
			if k != i {
				temp = temp && !state
			}
		}

		result = result || temp
	}

	return result
}

func NoTrueOf(states ...bool) bool {
	var result bool = true
	for _, state := range states {
		result = result && !state
	}

	return result
}

func OneOrNoTrueOf(states ...bool) bool {
	return OneTrueOf(states...) || NoTrueOf(states...)
}