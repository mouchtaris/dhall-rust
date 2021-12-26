let Head = { head : Bool }

in  λ(ls : Head) → let ls = ls.head in ls { head = True }
