let dl/ = ../dhallib/...

let u/ = dl/.v/0.u/

let tls/ = ./tls.dhall

let s/ = ./system.dhall

let h/ = ./host.dhall

let aln/ = ./alloc-net.dhall

let TARGET = "/etc/nomad.d/"

let Host-T = s/.Host-T

let Map = λ(V : Type) → u/.Map Text V

let TlsFiles
    : Type
    = { ca_file : Text, cert_file : Text, key_file : Text }

let Tls
    : Type
    =   TlsFiles
      ⩓ { http : Bool
        , rpc : Bool
        , verify_server_hostname : Bool
        , verify_https_client : Bool
        }

let Advertise
    : Type
    = { bind_addr : Text, advertise : { http : Text } }

let HostVolume
    : Type
    = { path : Text, read_only : Bool }

let HV/ro/
    : Text → HostVolume
    = λ(path : Text) → { path, read_only = True }

let Server
    : Type
    = { enabled : Bool, bootstrap_expect : Natural }

let Proxy
    : Type
    =   {}
      ⩓ TlsFiles
      ⩓ { address : Text
        , share_ssl : Bool
        , ssl : Bool
        , verify_ssl : Bool
        , checks_use_advertise : Bool
        }

let Vault
    : Type
    = {} ⩓ Proxy ⩓ { enabled : Bool }

let Consul
    : Type
    = {} ⩓ Proxy ⩓ { grpc_address : Text }

let Client
    : Type
    = { enabled : Bool, node_class : Text, host_volume : Map HostVolume }

let Docker/Config
    : Type
    = { infra_image : Optional Text }

let Plugin/docker
    : Type
    = { config : Optional Docker/Config }

let Plugin
    : Type
    = { docker : Optional Plugin/docker }

let `Type`
    : Type
    =   {}
      ⩓ Advertise
      ⩓ { datacenter : Text
        , data_dir : Text
        , acl : { enabled : Bool }
        , server : Optional Server
        , client : Optional Client
        , vault : Optional Vault
        , consul : Optional Consul
        , tls : Optional Tls
        , plugin : Plugin
        }

let TlsFiles/default
    : TlsFiles
    = { ca_file = "", cert_file = "", key_file = "" }

let Tls/default
    : Tls
    =   TlsFiles/default
      ∧ { http = True
        , rpc = True
        , verify_server_hostname = True
        , verify_https_client = True
        }

let Advertise/default
    : Advertise
    = { bind_addr = "", advertise.http = "" }

let Server/default
    : Server
    = { enabled = True, bootstrap_expect = 3 }

let Proxy/default
    : Proxy
    =   TlsFiles/default
      ∧ { address = ""
        , share_ssl = True
        , ssl = True
        , verify_ssl = True
        , checks_use_advertise = True
        }

let Vault/default
    : Vault
    = Proxy/default ∧ { enabled = True }

let Consul/default
    : Consul
    = Proxy/default ∧ { grpc_address = "" }

let Client/default
    : Client
    = { enabled = True, node_class = "", host_volume = [] : Map HostVolume }

let Docker/Config/default
    : Docker/Config
    = { infra_image = Some "dr.test.x/pause-amd64:3.1" }

let Plugin/docker/default
    : Plugin/docker
    = { config = Some Docker/Config/default }

let Plugin/default
    : Plugin
    = { docker = Some Plugin/docker/default }

let default
    : `Type`
    =   Advertise/default
      ∧ { datacenter = "dc-0"
        , data_dir = "/opt/nomad"
        , acl.enabled = True
        , server = Some Server/default
        , client = None Client
        , vault = Some Vault/default
        , consul = Some Consul/default
        , tls = Some Tls/default
        , plugin = Plugin/default
        }

let Config = `Type`

let Mod = s/.Mod Config

let opt/mod = s/.opt/mod

let dom = (tls/.dom/ s/.Domain.nomad) with dc = "global"

let ca = tls/.ca/ dom

let dom-consul = tls/.dom/ s/.Domain.consul

let ca-consul = tls/.ca/ dom-consul

let dom-vault = tls/.dom/ s/.Domain.vault

let ca-vault = tls/.ca/ dom-vault

let with_host =
      λ(host : Host-T) →
        let host = host with domains = [ s/.Domain.nomad ]

        let addr = aln/.host/ip-text host

        let set-addr/vault
            : s/.Mod Vault
            = λ(config : Vault) → config with address = "https://${addr}:8200"

        let set-addr/consul
            : s/.Mod Consul
            = λ(config : Consul) →
                config
                with address = "${addr}:8501"
                with grpc_address = "${addr}:8502"

        let set-addr
            : Mod
            = λ(config : Config) →
                config
                with bind_addr = addr
                with advertise.http = "${addr}:4646"
                with vault = opt/mod Vault set-addr/vault config.vault
                with consul = opt/mod Consul set-addr/consul config.consul

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

        let req-consul
            : tls/.Obj
            = let host =
                    host
                    with roles = [ s/.Role.client ]
                    with domains = host.domains # [ s/.Domain.consul ]

              in  tls/.dom-req-with-host/ host dom-consul

        let req-vault
            : tls/.Obj
            = let host =
                    host
                    with roles = [ s/.Role.client ]
                    with domains = host.domains # [ s/.Domain.vault ]

              in  tls/.dom-req-with-host/ host dom-vault

        let update-tls/files
            : tls/.Obj → tls/.Obj → s/.Mod TlsFiles
            = λ(ca : tls/.Obj) →
              λ(req : tls/.Obj) →
              λ(conf : TlsFiles) →
                conf
                with ca_file = TARGET ++ ca.ca
                with cert_file = TARGET ++ req.crt
                with key_file = TARGET ++ req.key

        let update-tls/tls
            : s/.Mod Tls
            = λ(conf : Tls) → conf ⫽ update-tls/files ca req conf.(TlsFiles)

        let update-tls/vault
            : s/.Mod Vault
            = λ(conf : Vault) →
                conf ⫽ update-tls/files ca-vault req-vault conf.(TlsFiles)

        let update-tls/consul
            : s/.Mod Consul
            = λ(conf : Consul) →
                conf ⫽ update-tls/files ca-consul req-consul conf.(TlsFiles)

        let update-tls
            : Mod
            = λ(conf : Config) →
                conf
                with tls = opt/mod Tls update-tls/tls conf.tls
                with vault = opt/mod Vault update-tls/vault conf.vault
                with consul = opt/mod Consul update-tls/consul conf.consul

        let tls-files
            : List s/.Path
            =   [ [ ca.ca ]
                , [ req.crt ]
                , [ req.key ]
                , [ ca-vault.ca ]
                , [ req-vault.crt ]
                , [ req-vault.key ]
                , [ ca-consul.ca ]
                , [ req-consul.crt ]
                , [ req-consul.key ]
                ]
              : List s/.Path

        let tls-requests
            : List tls/.SignReq
            = [ { req, ca }
              , { req = req-consul, ca = ca-consul }
              , { req = req-vault, ca = ca-vault }
              ]

        in  { req, req-consul, tls-files, set-addr, update-tls, tls-requests }

in  { Type
    , default
    , dom
    , ca
    , dom-consul
    , ca-consul
    , dom-vault
    , ca-vault
    , with_host
    , TARGET
    , TlsFiles = { Type = TlsFiles, default = TlsFiles/default }
    , Tls = { Type = Tls, default = Tls/default, none = None Tls }
    , Advertise = { Type = Advertise, default = Advertise/default }
    , HostVolume =
      { Type = HostVolume
      , default = {=}
      , none = [] : Map HostVolume
      , ro/ = HV/ro/
      }
    , Server = { Type = Server, default = Server/default, none = None Server }
    , Proxy = { Type = Proxy, default = Proxy/default }
    , Vault = { Type = Vault, default = Vault/default, none = None Vault }
    , Consul = { Type = Consul, default = Consul/default, none = None Consul }
    , Client =
      { Type = Client
      , default = Client/default
      , none = None Client
      , or_default = u/.opt/default Client Client/default
      }
    , Docker/Config
    , Plugin/docker
    , Plugin
    }
