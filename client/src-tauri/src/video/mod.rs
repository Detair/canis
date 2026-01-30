//! Video Encoding and RTP Module
//!
//! VP9 software encoding and RTP packetization for screen sharing.

pub mod encoder;
pub mod rtp;

use thiserror::Error;

/// Video encoding errors.
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum VideoError {
    #[error("Encoder initialization failed: {0}")]
    InitFailed(String),
    #[error("Encoding error: {0}")]
    EncodeFailed(String),
    #[error("RTP send error: {0}")]
    RtpSendFailed(String),
    #[error("Unsupported codec: {0}")]
    UnsupportedCodec(String),
}

/// An encoded video packet ready for RTP packetization.
pub struct EncodedPacket {
    /// Encoded data.
    pub data: Vec<u8>,
    /// Whether this is a keyframe.
    pub is_keyframe: bool,
    /// Presentation timestamp in 90kHz clock units.
    pub pts: u64,
}

/// Quality tier for screen sharing.
#[derive(Debug, Clone, Copy)]
pub struct QualityParams {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub bitrate_kbps: u32,
}

impl QualityParams {
    pub fn from_tier(tier: &str) -> Self {
        match tier {
            "low" => Self {
                width: 854,
                height: 480,
                fps: 15,
                bitrate_kbps: 500,
            },
            "high" => Self {
                width: 1920,
                height: 1080,
                fps: 30,
                bitrate_kbps: 3000,
            },
            "premium" => Self {
                width: 1920,
                height: 1080,
                fps: 60,
                bitrate_kbps: 5000,
            },
            // "medium" and default
            _ => Self {
                width: 1280,
                height: 720,
                fps: 30,
                bitrate_kbps: 1500,
            },
        }
    }
}
