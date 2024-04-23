use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
struct FontAliasAdjust {
    weight: u32,
    to: u32,
}

#[derive(Deserialize)]
struct FontList {
    family: String,
    alias: Vec<HashMap<String, u32>>,
    #[serde(default)]
    adjust: Vec<FontAliasAdjust>,
}

#[derive(Deserialize)]
struct FcFallback {
    #[serde(rename = "")]
    inner: Vec<HashMap<String,String>>
}

#[derive(Deserialize)]
struct FontConfig {
    fontdir: Vec<PathBuf>,
    generic: Vec<FontList>,
    fallback: Vec<FcFallback>,
}

#[cfg(test)]
mod test {
    /// Example of available fonts on OpenHarmony are described in JSON format in
    /// `/system/etc/fontconfig.json`
    static EXAMPLE_JSON: &str = r###"{
  "fontdir": ["/system/fonts/"],
  "generic": [
    {
      "family": "HarmonyOS Sans",
      "alias": [
        {
          "HarmonyOS-Sans": 0
        },
        {
          "HarmonyOS-Sans-Light": 100
        },
        {
          "HarmonyOS-Sans-Regular": 400
        },
        {
          "HarmonyOS-Sans-Medium": 700
        },
        {
          "HarmonyOS-Sans-Bold": 900
        }
      ],
      "adjust": [
        {
          "weight": 50, "to": 100
        },
        {
          "weight": 80, "to": 400
        },
        {
          "weight": 100, "to": 700
        },
        {
          "weight": 200, "to": 900
        }
      ]
    },
    {
      "family": "HarmonyOS Sans Condensed",
      "alias": [
        {
          "HarmonyOS-Sans-Condensed": 0
        }
      ]
    },
    {
      "family": "HarmonyOS Sans Digit",
      "alias": [
        {
          "HarmonyOS-Sans-Digit": 0
        }
      ]
    }
  ],
  "fallback": [
    {
      "": [
        {
          "zh-Hans": "HarmonyOS Sans SC"
        },
        {
          "zh-Hant": "HarmonyOS Sans TC"
        },
        {
          "und-Arab": "HarmonyOS Sans Naskh Arabic UI"
        },
        {
          "und-Ethi": "Noto Sans Ethiopic"
        },
        {
          "und-Hebr": "Noto Sans Hebrew"
        },
        {
          "ja": "Noto Sans JP"
        },
        {
          "ko": "Noto Sans KR"
        },
        {
          "und-Zsye": "HMOS Color Emoji"
        },
        {
          "und-Zsye": "HMOS Color Emoji Flags"
        },
        {
          "und-Zsym": "Noto Sans Symbols"
        },
        {
          "und-Zsym": "HM Symbol"
        }
      ]
    }
  ]
}"###;


    #[test]
    fn test_deser() {
        use super::FontConfig;
        let value: FontConfig = serde_json::from_str(EXAMPLE_JSON).expect("Failed to parse");
    }
}