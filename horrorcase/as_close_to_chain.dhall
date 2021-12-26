let n = Natural

let Class
    : Kind
    = Type → { Data : Type, Proto : Type }

let Class/Read
    : Kind
    = Class → Type → Type

let Data
    : Class/Read
    = λ(f : Class) → λ(a : Type) → (f a).Data

let Proto
    : Class/Read
    = λ(f : Class) → λ(a : Type) → (f a).Proto

let Change/
    : Class/Read → Class/Read → Class/Read
    = λ(a : Class/Read) →
      λ(b : Class/Read) →
      λ(f : Class) →
      λ(x : Type) →
        a f x → b f x

let Option
    : Class
    = λ(a : Type) →
        let Data = λ(a : Type) → < some : a | none >

        let Proto =
              let map = λ(b : Type) → (a → b) → Data b

              in  { map_to : ∀(b : Type) → map b, map : map a }

        in  { Data = Data a, Proto }

let Map =
      λ(f : Class) →
        let of = λ(a : Type) → λ(b : Type) → (f a).Data → (a → b) → (f b).Data

        in  { type = ∀(a : Type) → ∀(b : Type) → of a b, of }

let Option/map
    : (Map Option).type
    = λ(a : Type) →
      λ(b : Type) →
      λ(fa : (Option a).Data) →
      λ(f : a → b) →
        merge
          { some = λ(x : a) → (Option b).Data.some (f x)
          , none = (Option b).Data.none
          }
          fa

let Option/
    : ∀(a : Type) → Change/ Data Proto Option a
    = λ(a : Type) →
      λ(fa : Data Option a) →
        let map_to = λ(b : Type) → Option/map a b fa

        let map = map_to a

        in  { map, map_to }

let f = λ(x : n) → x + 1

let d = (Data Option n).some 12

let a = Option/map n n d f

let b = (Option/ n d).map_to n f

let n_ = Option/ n

let c = n_ d

let e = c.map f

let h = (n_ ((n_ ((n_ ((n_ (c.map f)).map f)).map f)).map f)).map f

in  { a, b, e, h }
