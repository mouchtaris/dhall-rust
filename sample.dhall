let Typ = ./Type.dhall
let F = Typ.F
let T = Typ.T
let make/ = λ(f : F) → λ(g : F) → ∀(x : T) → f x → g x
let fog/ = λ(f : F) → λ(g : F) → ∀(x : T) → f x → f (g x)
let Open = λ(f : F) → ∀(x : T) → f x
let Apply- = λ(f : F) → λ(x : T) → f x → x
let Apply = λ(f : F) → Open (Apply- f)
let Bind = λ(f : F) → f (Apply f)
in  { make/, fog/, Apply-, Apply, Bind, Open } : Typ.Type
