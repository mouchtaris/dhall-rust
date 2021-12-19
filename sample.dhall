{- vim: set ft=dhall : -}
let util/ = ../util/...

let Path = List Text

let Path/Relation = < root | sibling | parent >

let Path/to_text
    : Path → Text
    = util/.text/concat_sep "/"

let Path/basename
    : Path → Text
    = let rev
          : Path → Path
          = util/.list/reverse Text

      let head
          : Path → Optional Text
          = util/.list/head Text

      let name
          : Optional Text → Text
          = util/.opt/default Text ""

      let `head >> name`
          : (Path → Optional Text) → (Optional Text → Text) → Path → Text
          = util/.fun/compose Path (Optional Text) Text

      let `rev >> ...`
          : (Path → Path) → (Path → Text) → Path → Text
          = util/.fun/compose Path Path Text

      in  `rev >> ...` rev (`head >> name` head name)

in  { Path, Path/Relation, Path/basename, Path/to_text }
