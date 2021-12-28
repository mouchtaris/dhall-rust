let T = Type
let F = T -> T
let G = T -> F

let Arrow: T -> T -> G -> T = \(a: T) -> \(b: T) -> \(g: G) -> g a b

let BindLeft: T -> G -> G = \(a: T) -> \(g: G) -> \(r: T) -> \(b: T) -> r -> g a b

let func-: G = \(a: T) -> \(b: T) -> a -> b

let Nil = Bool
let nil = False

let Read: F -> F = \(f: F) -> \(r: T) -> f r -> r
let Bake: F -> T = \(f: F) -> forall(r: T) -> Read f r
let Make: F -> T = \(f: F) -> f (Bake f)

let User2: G -> G = \(f: G) -> BindLeft Text (BindLeft Natural (BindLeft Bool f))
let User/read: F = User2 func- Nil
let User/new: T = Make User/read
let User/nu: User/new =
	\(_: Nil) ->
	\(name: Text) ->
	\(age: Natural) ->
	\(blue: Bool) ->
	\(r: T) ->
	\(read: User/read r) ->
	  read _ name age blue

let User/name: T = Bake User/read -> Text
let User/nam: User/name =
	\(r: Bake User/read) ->
	  r Text
	  (\(_: Nil) -> \(name: Text) -> \(_: Natural) -> \(_: Bool) -> name)
let User/age: Bake User/read -> Natural =
	\(r: Bake User/read) ->
	  r Natural
	  (\(_: Nil) -> \(_: Text) -> \(age: Natural) -> \(_: Bool) -> age)
let Jil = Natural/show 3111
let Chris = Natural/show 5415
let user-0: Bake User/read = User/nu nil Jil 26 False
let user-1: Bake User/read = User/nu nil Chris 34 True
let resevil = {
, jil = User/nam user-0
, chris = User/age user-1
}

let all = { Arrow, BindLeft, User2, User/read, User/new, User/nu, user-0, user-1, resevil }

let Weapon2: G -> G = \(f: G) -> BindLeft Natural (BindLeft Natural (BindLeft Natural f))
let Weapon/: F = Weapon2 func- Nil
let Weapon/new: Make Weapon/ =
	\(_: Nil) ->
	\(dmg: Natural) ->
	\(mgz: Natural) ->
	\(rld: Natural) ->
	\(r: T) ->
	\(read: Weapon/ r) ->
	  read _ dmg mgz rld

let Weapon/damage: Bake Weapon/ -> Natural =
	\(r: Bake Weapon/) ->
	  r Natural
	  (
	  	\(_: Nil) ->
		\(dmg: Natural) ->
		\(mgz: Natural) ->
		\(rld: Natural) ->
		dmg
	  )

let Weapon/magazine: Bake Weapon/ -> Natural =
	\(r: Bake Weapon/) ->
	  r Natural
	  (
	  	\(_: Nil) ->
		\(dmg: Natural) ->
		\(mgz: Natural) ->
		\(rld: Natural) ->
		mgz
	  )

let Weapon/reload_speed: Bake Weapon/ -> Natural =
	\(r: Bake Weapon/) ->
	  r Natural
	  (
	  	\(_: Nil) ->
		\(dmg: Natural) ->
		\(mgz: Natural) ->
		\(rld: Natural) ->
		rld
	  )

let cobra: Bake Weapon/ = Weapon/new nil 23 6 9
let berreta: Bake Weapon/ = Weapon/new nil 11 23 4
let magnum: Bake Weapon/ = Weapon/new nil 18 15 6

let Stat: T = { dmg: Natural, rld: Natural, mgz: Natural }
let stats: Bake Weapon/ -> Stat = \(w: Bake Weapon/) ->
	{ dmg = Weapon/damage w
	, rld = Weapon/reload_speed w
	, mgz = Weapon/magazine w
	}

let resevil = {
	, resevil
	, cobra_stats = stats cobra
	, magnum_stats = stats magnum
	, berreta_stats = stats berreta
}
let resevil = {
	, resevil
}
in resevil
