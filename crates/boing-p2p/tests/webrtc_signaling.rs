//! Test WebRTC signaling types.

use boing_p2p::{
    SignalingDepositConfig, SignalingMessage, SignalingMessageKind, SignalingRateLimit,
    StunTurnMetrics, StunTurnReputation,
};

#[test]
fn test_signaling_rate_limit_default() {
    let rl = SignalingRateLimit::default();
    assert_eq!(rl.max_per_window, 5);
    assert_eq!(rl.window_secs, 60);
}

#[test]
fn test_signaling_deposit_config_default() {
    let cfg = SignalingDepositConfig::default();
    assert_eq!(cfg.min_deposit, 100);
    assert_eq!(cfg.expiration_secs, 300);
}

#[test]
fn test_stun_turn_reputation_from_metrics() {
    let m = StunTurnMetrics {
        uptime_bps: 9900,
        avg_latency_ms: 50,
        success_rate_bps: 9800,
        throughput_mb: 100,
    };
    let rep = StunTurnReputation::from_metrics(&m);
    assert!(rep.score > 0);
    assert!(rep.score <= StunTurnReputation::MAX);
}

#[test]
fn test_signaling_message_offer() {
    let msg = SignalingMessage::offer([1u8; 32], [2u8; 32], b"sdp-offer".to_vec());
    assert_eq!(msg.kind, SignalingMessageKind::Offer);
    assert!(msg.payload.is_some());
    assert!(msg.content_pointer.is_none());
}

#[test]
fn test_signaling_message_with_pointer() {
    let msg = SignalingMessage::with_pointer(
        SignalingMessageKind::Offer,
        [1u8; 32],
        [2u8; 32],
        "QmXyz...".into(),
    );
    assert_eq!(msg.kind, SignalingMessageKind::Offer);
    assert!(msg.payload.is_none());
    assert!(msg.content_pointer.is_some());
    assert_eq!(msg.content_pointer.as_ref().unwrap().cid, "QmXyz...");
}
