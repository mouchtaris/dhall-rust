let K = Kind → Kind

let Item/
    : Kind
    = { Item : Type }

let Option/
    : Kind
    = Item/

let Mod/
    : Kind
    = Item/

let State/
    : Kind
    = Item/ ⩓ { State : Type }

let Data/
    : K
    = λ(a : Kind) → a → Type

let Triplex/
    : K
    = λ(a : Kind) → a → a → a

let data_item/gof =
      λ(f : Data/ Item/) →
      λ(X : Kind) →
      λ(g : Data/ X) →
      λ(x : X) →
        f { Item = g x }

let Mod
    : Data/ Mod/
    = λ(m : Mod/) → m.Item → m.Item

let Option
    : Data/ Option/
    = λ(s : Option/) → < some : s.Item | none >

let State
    : Data/ State/
    = λ(s : State/) → { state : s.State, item : s.Item }

let State/new
    : ∀(s : State/) → s.State → s.Item → State s
    = λ(s : State/) → λ(state : s.State) → λ(item : s.Item) → { state, item }

let Option/Mod = data_item/gof Mod Item/ Option

let Option/func
    : ∀(s : Option/) → Mod s → Option/Mod s
    = λ(s : Option/) →
      λ(f : Mod s) →
      λ(d : Option s) →
        merge
          { some = λ(item : s.Item) → (Option s).some (f item)
          , none = (Option s).none
          }
          d

let naturals
    : State/
    = { State = Natural, Item = Natural }

let naturals/nil
    : State naturals
    = State/new naturals 1 0

let State/Mod = data_item/gof Mod State/ State

let naturals/next
    : State/Mod naturals
    = let N = Natural

      let S = { state : N, item : N }

      let _ =
          -- Option/func { Item = S }
            0

      in  λ(x : S) → { state = x.state + 1, item = x.item + 1 }

let a = naturals/nil

let a = naturals/next a

let a = naturals/next a

let a = naturals/next a

let a = naturals/next a

in  { State/new
    , State
    , Option
    , Data/
    , State/
    , Option/
    , Item/
    , naturals
    , naturals/nil
    , Option/func
    , a
    }
