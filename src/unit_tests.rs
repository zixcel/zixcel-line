use crate::boundary::{reference, secret_ref};

#[test]
fn channel_and_secret_values_remain_opaque() {
    assert!(reference("channel", "channel:notifications").is_ok());
    assert!(reference("channel", "channel notifications").is_err());
    assert!(secret_ref("secret://line/channel/observer").is_ok());
    assert!(secret_ref("channel-secret").is_err());
}
