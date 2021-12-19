{- vim: set ft=dhall : -}
let json/ =
        https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/JSON/package.dhall
          sha256:5f98b7722fd13509ef448b075e02b9ff98312ae7a406cf53ed25012dbc9990ac
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/JSON/package.dhall

let fun/compose
    : ∀(a : Type) → ∀(b : Type) → ∀(c : Type) → (a → b) → (b → c) → a → c
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Function/compose.dhall
          sha256:65ad8bbea530b3d8968785a7cf4a9a7976b67059aa15e3b61fcba600a40ae013
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Function/compose.dhall

let fun/id
    : ∀(a : Type) → ∀(x : a) → a
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Function/identity.dhall
          sha256:f78b96792b459cb664f41c6119bd8897dd04353a3343521d436cd82ad71cb4d4
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Function/identity.dhall

let list/append
    : ∀(a : Type) → List a → a → List a
    =   ./list/append.dhall
          sha256:c15695120f116d4dbbb79f9de4d3b745bd2c73626324ca8739b0c79a55a89734
      ? ./list/append.dhall

let list/append2 = λ(a : Type) → λ(x : a) → λ(l : List a) → list/append a l x

let list/concat
    : ∀(a : Type) → List (List a) → List a
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/concat.dhall
          sha256:54e43278be13276e03bd1afa89e562e94a0a006377ebea7db14c7562b0de292b
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/concat.dhall

let list/concat_map
    : ∀(a : Type) → ∀(b : Type) → (a → List b) → List a → List b
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/concatMap.dhall
          sha256:3b2167061d11fda1e4f6de0522cbe83e0d5ac4ef5ddf6bb0b2064470c5d3fb64
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/concatMap.dhall

let list/drop
    : ∀(n : Natural) → ∀(a : Type) → List a → List a
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/drop.dhall
          sha256:af983ba3ead494dd72beed05c0f3a17c36a4244adedf7ced502c6512196ed0cf
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/drop.dhall

let list/filter
    : ∀(a : Type) → (a → Bool) → List a → List a
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/filter.dhall
          sha256:8ebfede5bbfe09675f246c33eb83964880ac615c4b1be8d856076fdbc4b26ba6
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/filter.dhall

let list/fold
    : ∀(a : Type) →
      List a →
      ∀(list : Type) →
      ∀(cons : a → list → list) →
      ∀(nil : list) →
        list
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/fold.dhall
          sha256:10bb945c25ab3943bd9df5a32e633cbfae112b7d3af38591784687e436a8d814
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/fold.dhall

let list/fold2
    : ∀(a : Type) →
      ∀(list : Type) →
      ∀(nil : list) →
      ∀(cons : a → list → list) →
      List a →
        list
    = λ(a : Type) →
      λ(list : Type) →
      λ(nil : list) →
      λ(cons : a → list → list) →
      λ(la : List a) →
        list/fold a la list cons nil

let list/fold_left
    : ∀(a : Type) →
      List a →
      ∀(list : Type) →
      ∀(cons : list → a → list) →
      ∀(nil : list) →
        list
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/foldLeft.dhall
          sha256:3c6ab57950fe644906b7bbdef0b9523440b6ee17773ebb8cbd41ffacb8bfab61
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/foldLeft.dhall

let list/fold_left2
    : ∀(a : Type) →
      ∀(list : Type) →
      ∀(nil : list) →
      ∀(cons : list → a → list) →
      List a →
        list
    = λ(a : Type) →
      λ(list : Type) →
      λ(nil : list) →
      λ(cons : list → a → list) →
      λ(la : List a) →
        list/fold_left a la list cons nil

let list/head
    : ∀(a : Type) → List a → Optional a
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/head.dhall
          sha256:0d2e65ba0aea908377e46d22020dc3ad970284f4ee4eb8e6b8c51e53038c0026
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/head.dhall

let Idx
    : Type → Type
    = λ(a : Type) → { index : Natural, value : a }

let list/indexed
    : ∀(a : Type) → List a → List (Idx a)
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/indexed.dhall
          sha256:58bb44457fa81adf26f5123c1b2e8bef0c5aa22dac5fa5ebdfb7da84563b027f
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/indexed.dhall

let list/init
    : ∀(a : Type) → a → List a
    =   ./list/init.dhall
          sha256:9fadcc569cde4e323ebf20992e4bb14c9a5b7b84ee9b245bb748dbd1a17d205c
      ? ./list/init.dhall

let list/map
    : ∀(a : Type) → ∀(b : Type) → (a → b) → List a → List b
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/map.dhall
          sha256:dd845ffb4568d40327f2a817eb42d1c6138b929ca758d50bc33112ef3c885680
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/map.dhall

let list/nth
    : Natural → ∀(a : Type) → List a → Optional a
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/index.dhall
          sha256:e657b55ecae4d899465c3032cb1a64c6aa6dc2aa3034204f3c15ce5c96c03e63
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/index.dhall

let list/flat_map
    : ∀(a : Type) → ∀(b : Type) → (a → List b) → List a → List b
    = list/concat_map

let list/null
    : ∀(a : Type) → List a → Bool
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/null.dhall
          sha256:2338e39637e9a50d66ae1482c0ed559bbcc11e9442bfca8f8c176bbcd9c4fc80
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/null.dhall

let list/reverse
    : ∀(a : Type) → List a → List a
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/reverse.dhall
          sha256:ad99d224d61852de6696da5a7d04c98dbe676fe67d5e4ef4f19e9aaa27006e9d
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/reverse.dhall

let list/take
    : ∀(n : Natural) → ∀(a : Type) → List a → List a
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/take.dhall
          sha256:b3e08ee8c3a5bf3d8ccee6b2b2008fbb8e51e7373aef6f1af67ad10078c9fbfa
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/List/take.dhall

let list/take/2
    : ∀(a : Type) → ∀(n : Natural) → List a → List a
    = λ(a : Type) → λ(n : Natural) → list/take n a

let Map/Entry =
        https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Map/Entry.dhall
          sha256:f334283bdd9cd88e6ea510ca914bc221fc2dab5fb424d24514b2e0df600d5346
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Map/Entry.dhall

let Map =
        https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Map/Type.dhall
          sha256:210c7a9eba71efbb0f7a66b3dcf8b9d3976ffc2bc0e907aadfb6aa29c333e8ed
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Map/Type.dhall

let map/keys
    : ∀(k : Type) → ∀(v : Type) → Map k v → List k
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Map/keys.dhall
          sha256:d13ec34e6acf7c349d82272ef09a37c7bdf37f0dab489e9df47a1ff215d9f5e7
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Map/keys.dhall

let map/values
    : ∀(k : Type) → ∀(v : Type) → Map k v → List v
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Map/values.dhall
          sha256:ae02cfb06a9307cbecc06130e84fd0c7b96b7f1f11648961e1b030ec00940be8
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Map/values.dhall

let map/map
    : ∀(k : Type) → ∀(a : Type) → ∀(b : Type) → (a → b) → Map k a → Map k b
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Map/map.dhall
          sha256:23e09b0b9f08649797dfe1ca39755d5e1c7cad2d0944bdd36c7a0bf804bde8d0
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Map/map.dhall

let nat/eq
    : Natural → Natural → Bool
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Natural/equal.dhall
          sha256:7f108edfa35ddc7cebafb24dc073478e93a802e13b5bc3fd22f4768c9b066e60
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Natural/equal.dhall

let bool/not
    : Bool → Bool
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Bool/not.dhall
          sha256:723df402df24377d8a853afed08d9d69a0a6d86e2e5b2bac8960b0d4756c7dc4
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Bool/not.dhall

let opt/default
    : ∀(a : Type) → a → Optional a → a
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Optional/default.dhall
          sha256:5bd665b0d6605c374b3c4a7e2e2bd3b9c1e39323d41441149ed5e30d86e889ad
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Optional/default.dhall

let opt/to_list
    : ∀(a : Type) → Optional a → List a
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Optional/toList.dhall
          sha256:d78f160c619119ef12389e48a629ce293d69f7624c8d016b7a4767ab400344c4
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Optional/toList.dhall

let opt/map
    : ∀(a : Type) → ∀(b : Type) → (a → b) → Optional a → Optional b
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Optional/map.dhall
          sha256:501534192d988218d43261c299cc1d1e0b13d25df388937add784778ab0054fa
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Optional/map.dhall

let opt/null
    : ∀(a : Type) → Optional a → Bool
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Optional/null.dhall
          sha256:3871180b87ecaba8b53fffb2a8b52d3fce98098fab09a6f759358b9e8042eedc
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Optional/null.dhall

let opt/is_some
    : ∀(a : Type) → Optional a → Bool
    = λ(a : Type) → fun/compose (Optional a) Bool Bool (opt/null a) bool/not

let opt/len
    : ∀(a : Type) → Optional a → Natural
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Optional/length.dhall
          sha256:f168337c5244ded68c05ecf32ce068b6b87158881d07e87b8cb6853fc6982a85
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Optional/length.dhall

let op/ =
        https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Operator/package.dhall
          sha256:861f724704a7b4755c96f173e54d03f314492a2d046723404c31ff612b7bf2e6
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Operator/package.dhall

let text/concat
    : List Text → Text
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Text/concat.dhall
          sha256:731265b0288e8a905ecff95c97333ee2db614c39d69f1514cb8eed9259745fc0
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Text/concat.dhall

let text/concat_sep
    : ∀(separator : Text) → ∀(elements : List Text) → Text
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Text/concatSep.dhall
          sha256:e4401d69918c61b92a4c0288f7d60a6560ca99726138ed8ebc58dca2cd205e58
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Text/concatSep.dhall

let text/rep
    : ∀(num : Natural) → ∀(repeatance : Text) → Text
    =   https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Text/replicate.dhall
          sha256:1b398b1d464b3a6c7264a690ac3cacb443b5683b43348c859d68e7c2cb925c4f
      ? https://raw.githubusercontent.com/dhall-lang/dhall-lang/v21.0.0/Prelude/Text/replicate.dhall

let text/append
    : Text → Text → Text
    = λ(a : Text) → λ(t : Text) → t ++ a

let text/prepend
    : Text → Text → Text
    = λ(p : Text) → λ(t : Text) → p ++ t

let tuple/ =
        ./tuple.dhall
          sha256:1bc2c1595209cba1c49dfd486cca91968d7d8b5dbac56fc7ed25e5a75b06737e
      ? ./tuple.dhall

in  { json/
    , fun/compose
    , fun/id
    , Idx
    , list/append
    , list/append2
    , list/concat
    , list/concat_map
    , list/drop
    , list/head
    , list/indexed
    , list/init
    , list/filter
    , list/fold
    , list/fold2
    , list/fold_left
    , list/fold_left2
    , list/map
    , list/nth
    , list/flat_map
    , list/null
    , list/reverse
    , list/take
    , list/take/2
    , Map
    , Map/Entry
    , map/keys
    , map/values
    , map/map
    , nat/eq
    , opt/default
    , opt/to_list
    , opt/map
    , opt/null
    , opt/is_some
    , opt/len
    , op/
    , text/append
    , text/concat
    , text/concat_sep
    , text/rep
    , text/prepend
    , tuple/
    }
