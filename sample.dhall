-- vim: ft=dhall
let dl/ = ../dhallib/...

let u/ = dl/.v/0.u/

let s/ = ./system.dhall

let aln/ = ./alloc-net.dhall

let h/ = ./host.dhall

let tls/ = ./tls.dhall

let TARGET = "/etc/consul.d/"

let Host-T = s/.Host-T

let `Type` =
      { datacenter : Text
      , data_dir : Text
      , encrypt : Optional Text
      , verify_incoming : Bool
      , verify_outgoing : Bool
      , verify_server_hostname : Bool
      , connect : { enabled : Bool }
      , ports : { http : Integer, https : Natural, grpc : Natural }
      , ui_config : { enabled : Bool }
      , node_name : Text
      , retry_join : List Text
      , advertise_addr : Text
      , bind_addr : Text
      , client_addr : Text
      , ca_file : Text
      , cert_file : Text
      , key_file : Text
      , server : Bool
      , bootstrap_expect : Optional Natural
      }

let default
    : `Type`
    = { datacenter = "dc-0"
      , data_dir = "/opt/consul"
      , encrypt = None Text
      , verify_incoming = True
      , verify_outgoing = True
      , verify_server_hostname = True
      , connect.enabled = True
      , ports = { http = -1, https = 8501, grpc = 8502 }
      , ui_config.enabled = True
      , node_name = "No-Name"
      , retry_join = [] : List Text
      , advertise_addr = ""
      , bind_addr = ""
      , client_addr = ""
      , ca_file = ""
      , cert_file = ""
      , key_file = ""
      , server = False
      , bootstrap_expect = None Natural
      }

let Mod
    : Type
    = s/.Mod `Type`

let dom
    : tls/.Dom
    = tls/.dom/ s/.Domain.consul

let ca
    : tls/.Obj
    = tls/.ca/ dom

let update-retry-join
    : List s/.Host-T → Mod
    = λ(servers : List s/.Host-T) →
      λ(conf : `Type`) →
        conf
        with retry_join = aln/.hosts-ips-text servers

let with_host =
      λ(host : s/.Host-T) →
        let host = host with domains = [ s/.Domain.consul ]

        let set-addr
            : Mod
            = λ(conf : `Type`) →
                let ip = aln/.host/ip-text host

                in  conf
                  with advertise_addr = ip
                  with bind_addr = ip
                  with client_addr = ip

        let update-node-name
            : Mod
            = λ(conf : `Type`) →
                let roles = s/.Role.join host.roles

                let i = Natural/show host.ix

                let z = s/.Az.show host.az

                in  conf
                  with node_name = "${roles}-${i}-${z}"

        let sanitize-node
            : Host-T → Host-T
            = let H = Host-T

              in  u/.fun/compose
                    H
                    H
                    H
                    h/.keep-only-server-client-ui/
                    h/.replace-agent-with-client/

        let req
            : tls/.Obj
            = let host = sanitize-node host in tls/.dom-req-with-host/ host dom

        let update-tls
            : Mod
            = λ(conf : `Type`) →
                conf
                with ca_file = TARGET ++ ca.ca
                with cert_file = TARGET ++ req.crt
                with key_file = TARGET ++ req.key

        let tls-files
            : List s/.Path
            = [ [ ca.ca ], [ req.crt ], [ req.key ] ] : List s/.Path

        let tls-requests
            : List tls/.SignReq
            = [ { req, ca } ]

        in  { set-addr
            , update-node-name
            , update-tls
            , req
            , tls-files
            , tls-requests
            }

in  { Type, Mod, default, update-retry-join, TARGET, dom, ca, with_host }
