use serde::Serialize;

use crate::{error::CollectResult, utils};

/// Crypto holds info parsed from /proc/crypto.
#[derive(Debug, Serialize, Clone, Default)]
pub struct Crypto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignmask: Option<u64>,

    pub cryptoasync: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocksize: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunksize: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctzsize: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub digestsize: Option<u64>,

    pub driver: String,
    pub geniv: String,
    pub internal: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ivsize: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_authsize: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keysize: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_keysize: Option<u64>,

    pub module: String,

    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refcnt: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seedsize: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statesize: Option<u64>,

    pub selftest: String,

    pub cryptotype: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub walksize: Option<u64>,
}

impl Crypto {
    fn new() -> Self {
        Default::default()
    }
}

/// collects crypto information
/// # Example
/// ```
/// use procsys::crypto;
///
/// let crypto_info = crypto::collect().expect("crypto information");
/// let json_output = serde_json::to_string_pretty(&crypto_info).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<Vec<Crypto>> {
    collect_from("/proc/crypto")
}

fn collect_from(filename: &str) -> CollectResult<Vec<Crypto>> {
    let mut crypto_info: Vec<Crypto> = Vec::new();
    let mut info_index = 0;

    for line in utils::read_file_lines(filename)? {
        if line.trim().is_empty() {
            continue;
        }

        let item_fields: Vec<&str> = line.trim().split(':').filter(|s| !s.is_empty()).collect();
        if item_fields.len() < 2 {
            continue;
        }

        let metric = item_fields[0].trim();
        let metric_value = item_fields[1].trim();

        match metric {
            "alignmask" => {}
            "async" => {
                let mut casync = false;
                if metric_value == "yes" {
                    casync = true;
                }

                crypto_info[info_index].cryptoasync = casync;
            }
            "blocksize" => {
                crypto_info[info_index].blocksize = Some(utils::convert_str_to_u64(metric_value)?);
            }
            "chunksize" => {
                crypto_info[info_index].chunksize = Some(utils::convert_str_to_u64(metric_value)?);
            }
            "ctxsize" => {}
            "digestsize" => {
                crypto_info[info_index].digestsize = Some(utils::convert_str_to_u64(metric_value)?);
            }
            "driver" => {
                crypto_info[info_index].driver = metric_value.to_string();
            }
            "geniv" => {
                crypto_info[info_index].geniv = metric_value.to_string();
            }
            "internal" => {
                crypto_info[info_index].internal = metric_value.to_string();
            }
            "ivsize" => {
                crypto_info[info_index].ivsize = Some(utils::convert_str_to_u64(metric_value)?);
            }
            "maxauthsize" => {
                crypto_info[info_index].max_authsize =
                    Some(utils::convert_str_to_u64(metric_value)?);
            }
            "max keysize" => {
                crypto_info[info_index].max_keysize =
                    Some(utils::convert_str_to_u64(metric_value)?);
            }
            "min keysize" => {
                crypto_info[info_index].min_keysize =
                    Some(utils::convert_str_to_u64(metric_value)?);
            }
            "module" => {
                crypto_info[info_index].module = metric_value.to_string();
            }
            "name" => {
                let new_crypto = Crypto::new();
                if !crypto_info.is_empty() {
                    info_index += 1;
                }
                crypto_info.push(new_crypto);
                crypto_info[info_index].name = metric_value.to_string();
            }
            "priority" => {
                crypto_info[info_index].priority = Some(utils::convert_str_to_i64(metric_value)?);
            }
            "refcnt" => {
                crypto_info[info_index].refcnt = Some(utils::convert_str_to_i64(metric_value)?);
            }
            "seedsize" => {
                crypto_info[info_index].seedsize = Some(utils::convert_str_to_u64(metric_value)?);
            }
            "statesize" => {
                crypto_info[info_index].statesize = Some(utils::convert_str_to_u64(metric_value)?);
            }
            "selftest" => {
                crypto_info[info_index].selftest = metric_value.to_string();
            }
            "type" => {
                crypto_info[info_index].cryptotype = metric_value.to_string();
            }
            "walksize" => {
                crypto_info[info_index].walksize = Some(utils::convert_str_to_u64(metric_value)?);
            }
            _ => {}
        }
    }

    Ok(crypto_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crypto_information() {
        let crypto_info =
            collect_from("test_data/fixtures/proc/crypto").expect("collecting crypto information");

        assert_eq!(crypto_info.len(), 2);

        for cryptinfo in crypto_info {
            match cryptinfo.name.as_ref() {
                "ccm(aes)" => {
                    assert_eq!(cryptinfo.alignmask, None);
                    assert_eq!(cryptinfo.cryptoasync, false);
                    assert_eq!(cryptinfo.blocksize, Some(1));
                    assert_eq!(cryptinfo.chunksize, None);
                    assert_eq!(cryptinfo.ctzsize, None);
                    assert_eq!(cryptinfo.digestsize, None);
                    assert_eq!(
                        cryptinfo.driver,
                        "ccm_base(ctr(aes-aesni),cbcmac(aes-aesni))",
                    );
                    assert_eq!(cryptinfo.geniv, "<none>");
                    assert_eq!(cryptinfo.internal, "no");
                    assert_eq!(cryptinfo.ivsize, Some(16));
                    assert_eq!(cryptinfo.max_authsize, Some(16));
                    assert_eq!(cryptinfo.max_keysize, None);
                    assert_eq!(cryptinfo.min_keysize, None);
                    assert_eq!(cryptinfo.module, "kernel");
                    assert_eq!(cryptinfo.priority, Some(300));
                    assert_eq!(cryptinfo.refcnt, Some(3));
                    assert_eq!(cryptinfo.seedsize, None);
                    assert_eq!(cryptinfo.statesize, None);
                    assert_eq!(cryptinfo.selftest, "passed");
                    assert_eq!(cryptinfo.cryptotype, "aead");
                    assert_eq!(cryptinfo.walksize, None);
                }
                "ctr(aes)" => {
                    assert_eq!(cryptinfo.alignmask, None);
                    assert_eq!(cryptinfo.cryptoasync, false);
                    assert_eq!(cryptinfo.blocksize, Some(1));
                    assert_eq!(cryptinfo.chunksize, Some(16));
                    assert_eq!(cryptinfo.ctzsize, None);
                    assert_eq!(cryptinfo.digestsize, None);
                    assert_eq!(cryptinfo.driver, "ctr(aes-aesni)");
                    assert_eq!(cryptinfo.geniv.is_empty(), true);
                    assert_eq!(cryptinfo.internal, "no");
                    assert_eq!(cryptinfo.ivsize, Some(16));
                    assert_eq!(cryptinfo.max_authsize, None);
                    assert_eq!(cryptinfo.max_keysize, Some(32));
                    assert_eq!(cryptinfo.min_keysize, Some(16));
                    assert_eq!(cryptinfo.module, "kernel");
                    assert_eq!(cryptinfo.priority, Some(300));
                    assert_eq!(cryptinfo.refcnt, Some(3));
                    assert_eq!(cryptinfo.seedsize, None);
                    assert_eq!(cryptinfo.selftest, "passed");
                    assert_eq!(cryptinfo.cryptotype, "skcipher");
                    assert_eq!(cryptinfo.statesize, Some(0));
                    assert_eq!(cryptinfo.walksize, Some(16));
                }
                _ => panic!("invalid crypto name: {}", cryptinfo.name),
            }
        }
    }
}
