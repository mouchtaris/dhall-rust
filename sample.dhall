{- vim: set ft=dhall : -}
λ(io : Type) →
λ(dt : Type) →
  let Unit = < unit >

  let wrt = io → dt → io

  let seq = List io → io

  let opn = Unit → io

  let mty = io

  let cls = io → io

  let Proto = { wrt : wrt, seq : seq, opn : opn, cls : cls, mty : mty }

  let proto = { wrt, seq, opn, cls, mty }

  let params = { io, dt, Unit }

  let class = { Proto, proto, params }

  let fmt/deco =
        λ(ft : Type) →
          let fmt = ft → List dt

          let prt = io → ft → io

          let Proto = Proto ⩓ { fmt : fmt }

          let proto =
                  proto
                ∧ { fmt
                  , prt =
                      λ(proto : Proto) →
                        let map = (../util/...).list/map dt io

                        let seq = proto.seq

                        let wrt = proto.wrt

                        let mty = proto.mty

                        let fmt = proto.fmt

                        in  λ(ios : io) →
                            λ(t : ft) →
                              seq [ ios, seq (map (wrt mty) (fmt t)) ]
                  }

          let Proto = Proto ⩓ { prt : prt }

          let params = params ∧ { ft }

          in  class ⫽ { Proto, proto, params }

  in  { class, fmt/deco }
