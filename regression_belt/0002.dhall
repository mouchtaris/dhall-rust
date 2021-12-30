\(x : Natural) ->
	let f = \(x: Natural) -> x + 1
	let g = \(f: Natural -> Natural) -> f x
	in g f
