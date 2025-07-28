use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct CeremonyConfig {
    pub ceremony: CeremonyParameters,
    pub curve: CurveParameters,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CeremonyParameters {
    pub required_power: usize,
    pub compression: CompressionSetting,
    pub check_correctness: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CurveParameters {
    pub name: String,
    pub g1_uncompressed_byte_size: usize,
    pub g2_uncompressed_byte_size: usize,
    pub g1_compressed_byte_size: usize,
    pub g2_compressed_byte_size: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompressionSetting {
    Yes,
    No,
}

impl CeremonyConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::from(path.as_ref()))
            .build()?;
        
        config.try_deserialize()
    }

    pub fn from_env_with_defaults() -> Result<Self, ConfigError> {
        let mut config_builder = Config::builder()
            .set_default("ceremony.required_power", 8)?
            .set_default("ceremony.compression", "no")?
            .set_default("ceremony.check_correctness", true)?
            .set_default("curve.name", "bls12_381")?
            .set_default("curve.g1_uncompressed_byte_size", 96)?
            .set_default("curve.g2_uncompressed_byte_size", 192)?
            .set_default("curve.g1_compressed_byte_size", 48)?
            .set_default("curve.g2_compressed_byte_size", 96)?;

        // Try to load from ceremony.toml if it exists
        if Path::new("ceremony.toml").exists() {
            config_builder = config_builder.add_source(File::with_name("ceremony"));
        }

        // Override with environment variables
        config_builder = config_builder
            .add_source(config::Environment::with_prefix("CEREMONY").separator("__"));

        let config = config_builder.build()?;
        config.try_deserialize()
    }

    pub fn default_bls12_381() -> Self {
        Self {
            ceremony: CeremonyParameters {
                required_power: 8,
                compression: CompressionSetting::No,
                check_correctness: true,
            },
            curve: CurveParameters {
                name: "bls12_381".to_string(),
                g1_uncompressed_byte_size: 96,
                g2_uncompressed_byte_size: 192,
                g1_compressed_byte_size: 48,
                g2_compressed_byte_size: 96,
            },
        }
    }

    // Computed values based on configuration
    pub fn tau_powers_length(&self) -> usize {
        1 << self.ceremony.required_power
    }

    pub fn tau_powers_g1_length(&self) -> usize {
        (self.tau_powers_length() << 1) - 1
    }

    pub fn accumulator_byte_size(&self) -> usize {
        (self.tau_powers_g1_length() * self.curve.g1_uncompressed_byte_size) + // g1 tau powers
        (self.tau_powers_length() * self.curve.g2_uncompressed_byte_size) + // g2 tau powers
        (self.tau_powers_length() * self.curve.g1_uncompressed_byte_size) + // alpha tau powers
        (self.tau_powers_length() * self.curve.g1_uncompressed_byte_size) + // beta tau powers
        self.curve.g2_uncompressed_byte_size + // beta in g2
        64 // blake2b hash size
    }

    pub fn public_key_size(&self) -> usize {
        3 * self.curve.g2_uncompressed_byte_size + // tau, alpha, and beta in g2
        6 * self.curve.g1_uncompressed_byte_size // (s1, s1*tau), (s2, s2*alpha), (s3, s3*beta) in g1
    }

    pub fn contribution_byte_size(&self) -> usize {
        (self.tau_powers_g1_length() * self.curve.g1_compressed_byte_size) + // g1 tau powers
        (self.tau_powers_length() * self.curve.g2_compressed_byte_size) + // g2 tau powers
        (self.tau_powers_length() * self.curve.g1_compressed_byte_size) + // alpha tau powers
        (self.tau_powers_length() * self.curve.g1_compressed_byte_size) + // beta tau powers
        self.curve.g2_compressed_byte_size + // beta in g2
        64 + // blake2b hash of input accumulator
        self.public_key_size() // public key
    }
}

impl From<CompressionSetting> for crate::parameters::UseCompression {
    fn from(setting: CompressionSetting) -> Self {
        match setting {
            CompressionSetting::Yes => crate::parameters::UseCompression::Yes,
            CompressionSetting::No => crate::parameters::UseCompression::No,
        }
    }
}

impl From<bool> for crate::parameters::CheckForCorrectness {
    fn from(check: bool) -> Self {
        match check {
            true => crate::parameters::CheckForCorrectness::Yes,
            false => crate::parameters::CheckForCorrectness::No,
        }
    }
}