let Option = λ(option_t : Type) → < some : option_t >

let Option/map =
      λ(___OPTION_A___: Type) →
      λ(fa : Option ___OPTION_A___) →
         merge
           { some = λ(x : ___OPTION_A___) → x
           }
           fa

in  Option/map
      Natural
      ((Option Natural).some 12)
