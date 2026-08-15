use ipwho_sdk::IPWhoClient;

#[tokio::main]
async fn main() {
    let key = std::env::var("IPWHO_API_KEY").expect("IPWHO_API_KEY");
    let mut pass = 0u32;
    let mut fail = 0u32;
    macro_rules! ok {
        ($c:expr, $m:expr) => {
            if $c { pass += 1; println!("  PASS {}", $m); }
            else  { fail += 1; println!("  FAIL {}", $m); }
        };
    }

    let client = IPWhoClient::new(key).expect("client");

    // 1. lookup
    let r = client.lookup("8.8.8.8", None, None).await.expect("lookup");
    let d = r.data.as_ref().expect("data");
    let gl = d.geo_location.as_ref().expect("geo");
    let tz = d.timezone.as_ref().expect("tz");
    let fl = d.flag.as_ref().expect("flag");
    let cu = d.currency.as_ref().expect("currency");
    let cn = d.connection.as_ref().expect("connection");
    ok!(d.ip == "8.8.8.8", format!("lookup ip == 8.8.8.8 (got {})", d.ip));
    ok!(gl.country.as_deref() == Some("United States"), format!("country == United States (got {:?})", gl.country));
    ok!(cn.asn_number == Some(15169), format!("asn_number == 15169 (got {:?})", cn.asn_number));
    ok!(gl.dial_code.is_some(), format!("dial_code captured ({:?})", gl.dial_code));
    ok!(gl.is_in_eu.is_some(), "is_in_eu captured");
    ok!(tz.time_zone.is_some(), format!("time_zone captured ({:?})", tz.time_zone));
    ok!(fl.flag_icon.is_some(), format!("flag_Icon captured ({:?})", fl.flag_icon));
    ok!(fl.flag_unicode.is_some(), format!("flag_unicode captured ({:?})", fl.flag_unicode));
    ok!(cu.name_plural.is_some(), format!("name_plural captured ({:?})", cu.name_plural));
    ok!(cn.asn_org.is_some(), format!("asn_org captured ({:?})", cn.asn_org));
    ok!(cn.connection_type.is_some(), format!("connection_type captured ({:?})", cn.connection_type));

    // 2. me
    let me = client.me(None, None).await.expect("me");
    ok!(me.data.map(|d| !d.ip.is_empty()).unwrap_or(false), "me ip captured");

    // 3. bulk
    let b = client.bulk(&["8.8.8.8", "1.1.1.1"]).await.expect("bulk");
    let n = b.data.and_then(|d| d.response_array).map(|v| v.len()).unwrap_or(0);
    ok!(n == 2, format!("bulk returns 2 (got {})", n));

    // 4. bad key
    let bc = IPWhoClient::new("sk.invalid_test_key").expect("bad client");
    let res = bc.lookup("8.8.8.8", None, None).await;
    ok!(res.is_err(), format!("bad key raised error ({:?})", res.err()));

    println!("\nRUST RESULT: {} passed, {} failed", pass, fail);
    if fail > 0 { std::process::exit(1); }
}
