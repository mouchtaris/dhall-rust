let T = ./Policy.Type.dhall

let u/ = T.u/

let to_json/ = T.to_json/

let json/ = to_json/.json/

let Capability = T.Capability

let Path/unix = u/.text/concat_sep "/"

let Capability/show =
      λ(cap : Capability) →
        merge
          { create = "create"
          , read = "read"
          , update = "update"
          , list = "list"
          , delete = "delete"
          , sudo = "sudo"
          , deny = "deny"
          }
          cap

let Capability/to_json = to_json/.string Capability Capability/show

let Capabilities/to_json = to_json/.list Capability Capability/to_json

let AllowedParamsList/to_json = to_json/.list Text json/.string

let AllowedParams/to_json =
      to_json/.map T.AllowedParams AllowedParamsList/to_json

let to_json =
      λ(entry : T.Entry) →
        json/.object
          ( toMap
              { path =
                  json/.object
                    [ { mapKey = Path/unix entry.path
                      , mapValue =
                          json/.object
                            ( toMap
                                { capabilities =
                                    Capabilities/to_json entry.capabilities
                                , allowed_parameters =
                                    AllowedParams/to_json
                                      entry.allowed_parameters
                                }
                            )
                      }
                    ]
              }
          )

let to_hcl =
      λ(entry : T.Entry) →
        ''
        path "${Path/unix entry.path}"
        {
          capabilities = ${json/.render
                             (Capabilities/to_json entry.capabilities)}
          allowed_parameters = ${json/.render
                                   ( AllowedParams/to_json
                                       entry.allowed_parameters
                                   )}
        }
        ''

let param =
    -- Name	Description
    -- identity.entity.id	The entity's ID
    -- identity.entity.name	The entity's name
    -- identity.entity.metadata.<metadata key>	Metadata associated with the entity for the given key
    -- identity.entity.aliases.<mount accessor>.id	Entity alias ID for the given mount
    -- identity.entity.aliases.<mount accessor>.name	Entity alias name for the given mount
    -- identity.entity.aliases.<mount accessor>.metadata.<metadata key>	Metadata associated with the alias for the given mount and metadata key
    -- identity.groups.ids.<group id>.name	The group name for the given group ID
    -- identity.groups.names.<group name>.id	The group ID for the given group name
    -- identity.groups.ids.<group id>.metadata.<metadata key>	Metadata associated with the group for the given key
    -- identity.groups.names.<group name>.metadata.<metadata key>
      let Prefix
          : Type
          = Text → Text

      let _/new = λ(p : Text) → λ(s : Text) → p ++ s

      let _/with = λ(p : Text) → _/new 
      "${p}."

      let _/plus = λ(a : Prefix) → λ(b : Prefix) → λ(s : Text) → a (b s)

      let _/pw
          : Text → Prefix → Prefix
          = λ(b : Text) → λ(a : Prefix) → _/plus a (_/with b)

      let _/identity = _/pw "identity"

      let _/entity = _/pw "entity"

      let _/metadata = _/pw "metadata"

      let _/aliases = _/pw "aliases"

      let _/groups = _/pw "groups"

      let _/ids = _/pw "ids"

      let _/names = _/pw "names"

      let _/ = _/new ""

      in  { identity =
              let _/ = _/identity _/

              in  { entity =
                      let _/ = _/entity _/

                      in  { id = _/ "id"
                          , name = _/ "name"
                          , metadata = _/metadata _/
                          , aliases =
                              let _/ = _/aliases _/

                              in  λ(mount_accessor : Text) →
                                    let _/ = _/pw mount_accessor _/

                                    in  { id = _/ "id"
                                        , name = _/ "name"
                                        , metadata = _/metadata _/
                                        }
                          }
                  , groups =
                      let _/ = _/groups _/

                      in  { ids =
                              let _/ = _/ids _/

                              in  λ(group_id : Text) →
                                    let _/ = _/pw group_id _/

                                    in  { name = _/ "name"
                                        , metadata = _/metadata _/
                                        }
                          , names =
                              let _/ = _/names _/

                              in  λ(group_name : Text) →
                                    let _/ = _/pw group_name _/

                                    in  { id = _/ "id"
                                        , metadata = _/metadata _/
                                        }
                          }
                  }
          }

in    { param
      , to_json
      , to_hcl
      , Capability/show
      , AllowedParams/to_json
      , AllowedParamsList/to_json
      , Capabilities/to_json
      , Capability/to_json
      }
    : T.Type
