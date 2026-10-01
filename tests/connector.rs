use zixcel_line::{build_plan, doctor, parse_config};

const CONFIG: &str = r#"
schema = "zixcel://line/config/v1"
config_id = "site-notifications"
channel_ref = "channel:site-notifications"
secret_ref = "secret://line/channels/site-observer"
data_sets = ["delivery-statistics", "webhook-event-metadata"]
"#;

#[test]
fn data_set_order_is_canonical() {
    let alternate = CONFIG.replace(
        "[\"delivery-statistics\", \"webhook-event-metadata\"]",
        "[\"webhook-event-metadata\", \"delivery-statistics\"]",
    );
    let first = build_plan(&parse_config(CONFIG).expect("config")).expect("plan");
    let second = build_plan(&parse_config(&alternate).expect("config")).expect("plan");
    assert_eq!(first, second);
}

#[test]
fn contract_rejects_credentials_unknowns_and_message_content() {
    assert!(parse_config(&format!("{CONFIG}\nclient_secret = \"no\"\n")).is_err());
    assert!(parse_config(&format!("{CONFIG}\nfuture = true\n")).is_err());
    assert!(parse_config(&" ".repeat(1_048_577)).is_err());
    let content = CONFIG.replace("delivery-statistics", "personal-message-content");
    assert!(parse_config(&content).is_err());
}

#[test]
fn doctor_proves_no_external_action_capability() {
    let report = doctor();
    assert!(!report.network_client_linked);
    assert!(!report.secret_resolution_enabled);
    assert!(!report.execution_enabled);
}
