use pest::{error::Error as PestError, iterators::Pair, Parser};

use std::convert::TryFrom;

use crate::common::{Vdf, VdfPair, VdfValue};

#[derive(Parser)]
#[grammar = "grammars/text.pest"]
struct VdfParser;

impl<'a> TryFrom<&'a str> for Vdf<'a> {
    type Error = PestError<Rule>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        VdfPair::try_from(s).map(Self)
    }
}

impl<'a> TryFrom<&'a str> for VdfPair<'a> {
    type Error = PestError<Rule>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let unparsed = VdfParser::parse(Rule::vdf, s)?.next().unwrap();
        Ok(Self::from(unparsed))
    }
}

impl<'a> From<Pair<'a, Rule>> for VdfPair<'a> {
    fn from(pair: Pair<'a, Rule>) -> Self {
        if let Rule::pair = pair.as_rule() {
            let mut inner_rules = pair.into_inner();
            let key = inner_rules
                .next()
                .unwrap()
                .into_inner()
                .next()
                .unwrap()
                .as_str();
            let value = VdfValue::from(inner_rules.next().unwrap());

            Self(key, value)
        } else {
            unreachable!("Prevented by grammar")
        }
    }
}

impl<'a> From<Pair<'a, Rule>> for VdfValue<'a> {
    fn from(pair: Pair<'a, Rule>) -> Self {
        match pair.as_rule() {
            Rule::string => VdfValue::Str(pair.into_inner().next().unwrap().as_str()),
            Rule::obj => VdfValue::Obj(pair.into_inner().map(VdfPair::from).collect()),
            _ => unreachable!("Prevented by grammar"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_vdf = r#"
"896660"
{
	"common"
	{
		"name"		"Valheim Dedicated Server"
		"type"		"Tool"
		"parent"		"892970"
		"oslist"		"windows,linux"
		"osarch"		""
		"icon"		"1aab0586723c8578c7990ced7d443568649d0df2"
		"logo"		"233d73a1c963515ee4a9b59507bc093d85a4e2dc"
		"logo_small"		"233d73a1c963515ee4a9b59507bc093d85a4e2dc_thumb"
		"clienticon"		"c55a6b50b170ac6ed56cf90521273c30dccb5f12"
		"clienttga"		"35e067b9efc8d03a9f1cdfb087fac4b970a48daf"
		"ReleaseState"		"released"
		"associations"
		{
		}
		"gameid"		"896660"
	}
	"config"
	{
		"installdir"		"Valheim dedicated server"
		"launch"
		{
			"0"
			{
				"executable"		"start_server_xterm.sh"
				"type"		"server"
				"config"
				{
					"oslist"		"linux"
				}
			}
			"1"
			{
				"executable"		"start_headless_server.bat"
				"type"		"server"
				"config"
				{
					"oslist"		"windows"
				}
			}
		}
	}
	"depots"
	{
		"1004"
		{
			"name"		"Steamworks SDK Redist (WIN32)"
			"config"
			{
				"oslist"		"windows"
			}
			"manifests"
			{
				"public"		"6473168357831043306"
			}
			"maxsize"		"39546856"
			"depotfromapp"		"1007"
		}
		"1005"
		{
			"name"		"Steamworks SDK Redist (OSX32)"
			"config"
			{
				"oslist"		"macos"
			}
			"manifests"
			{
				"public"		"2135359612286175146"
			}
			"depotfromapp"		"1007"
		}
		"1006"
		{
			"name"		"Steamworks SDK Redist (LINUX32)"
			"config"
			{
				"oslist"		"linux"
			}
			"manifests"
			{
				"public"		"6688153055340488873"
			}
			"maxsize"		"59862244"
			"depotfromapp"		"1007"
		}
		"896661"
		{
			"name"		"Valheim dedicated server Linux"
			"config"
			{
				"oslist"		"linux"
			}
			"manifests"
			{
				"public"		"521795651741005384"
			}
			"maxsize"		"985409357"
			"encryptedmanifests"
			{
				"experimental"
				{
					"encrypted_gid_2"		"BEDF872D73873D16C025EF87E27C2BDB"
					"encrypted_size_2"		"2559486959C6E5DCEA5C71ED32BA9080"
				}
			}
		}
		"896662"
		{
			"name"		"Valheim dedicated server Windows"
			"config"
			{
				"oslist"		"windows"
			}
			"manifests"
			{
				"public"		"5449924312569304795"
			}
			"maxsize"		"963189471"
			"encryptedmanifests"
			{
				"experimental"
				{
					"encrypted_gid_2"		"9FD2B7B42FACB1D1FC439DD83ED2BED9"
					"encrypted_size_2"		"B2D602E667364DEDCB7C3D6EE9AA7374"
				}
			}
		}
		"branches"
		{
			"public"
			{
				"buildid"		"6246034"
				"timeupdated"		"1613558776"
			}
			"experimental"
			{
				"buildid"		"6263839"
				"description"		"Experimental version of Valheim"
				"pwdrequired"		"1"
				"timeupdated"		"1613728251"
			}
			"unstable"
			{
				"buildid"		"6246034"
				"description"		"Unstable test version of valheim"
				"pwdrequired"		"1"
				"timeupdated"		"1613469743"
			}
		}
	}
}
"#;

        fn find_by_key<'a>(pairs: &'a [VdfPair<'a>], name: &str) -> &'a VdfPair<'a> {
            pairs
                .into_iter()
                .find(|VdfPair(key, _)| key == &name)
                .unwrap()
        }

        let vdf = Vdf::try_from(sample_vdf).unwrap();

        if let VdfValue::Obj(pairs) = vdf.value() {
            let depots = find_by_key(pairs, "depots").value();

            if let VdfValue::Obj(pairs) = depots {
                let branches = find_by_key(pairs, "branches").value();

                if let VdfValue::Obj(pairs) = branches {
                    let public_branch = find_by_key(pairs, "public").value();

                    if let VdfValue::Obj(pairs) = public_branch {
                        let build_id = find_by_key(pairs, "buildid").value();

                        if let VdfValue::Str(id_str) = build_id {
                            println!("Build id: {}", id_str);
                        }
                    }
                }
            }
        }

        panic!();
    }
}
