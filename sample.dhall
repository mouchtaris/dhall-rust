let l = ../lib/...

let x = l.x

let u/ = l.u/

let xs/ron = l.xs/ron : x.Script → Text

let dhalls = ../etc/dhalls.dhall

let bin/format-dhalls
    : x.Script
    = let script/
          : Text → x.Script
          = λ(path : Text) →
              [ x.write_file
                  [ "", "dev", "stderr" ]
                  ( x.fat3
                      ''
                      🎐 :: Format ${path}
                      ''
                  )
              , x.exec
                  (   x.cmd::{
                      , cmd = [ "dhall" ]
                      , args = x.strlist [ "format", path, "--unicode" ]
                      }
                    ⫽ x.out/display
                  )
              ]

      let scripts
          : List x.Script
          = u/.list/map Text x.Script script/ dhalls

      let parallel_script
          : x.Script
          = l.parallel/
              scripts
              ( λ(args : List x.Expr) →
                    x.cmd::{
                    , cmd = [ "env" ]
                    , args =
                          x.strlist
                            [ "RUST_LOG=info", "parallel", "-n1", "-j0", "-u" ]
                        # args
                    }
                  ⫽ x.out/display
              )

      in  parallel_script

let bin/format-dhalls/xs/ron
    : Text
    = xs/ron bin/format-dhalls

in  { bin/format-dhalls, bin/format-dhalls/xs/ron }
