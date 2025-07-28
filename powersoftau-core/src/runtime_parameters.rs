use crate::config::CeremonyConfig;
use crate::parameters::{CheckForCorrectness, UseCompression};

#[derive(Clone, Debug)]
pub struct RuntimeCeremonyParameters {
    config: CeremonyConfig,
}

impl RuntimeCeremonyParameters {
    pub fn new(config: CeremonyConfig) -> Self {
        Self { config }
    }

    pub fn from_file(path: &str) -> Result<Self, config::ConfigError> {
        let config = CeremonyConfig::from_file(path)?;
        Ok(Self::new(config))
    }

    pub fn from_env_with_defaults() -> Result<Self, config::ConfigError> {
        let config = CeremonyConfig::from_env_with_defaults()?;
        Ok(Self::new(config))
    }

    pub fn default_bls12_381() -> Self {
        Self::new(CeremonyConfig::default_bls12_381())
    }

    // Accessors that match the old trait interface
    pub fn required_power(&self) -> usize {
        self.config.ceremony.required_power
    }

    pub fn g1_uncompressed_byte_size(&self) -> usize {
        self.config.curve.g1_uncompressed_byte_size
    }

    pub fn g2_uncompressed_byte_size(&self) -> usize {
        self.config.curve.g2_uncompressed_byte_size
    }

    pub fn g1_compressed_byte_size(&self) -> usize {
        self.config.curve.g1_compressed_byte_size
    }

    pub fn g2_compressed_byte_size(&self) -> usize {
        self.config.curve.g2_compressed_byte_size
    }

    pub fn tau_powers_length(&self) -> usize {
        self.config.tau_powers_length()
    }

    pub fn tau_powers_g1_length(&self) -> usize {
        self.config.tau_powers_g1_length()
    }

    pub fn accumulator_byte_size(&self) -> usize {
        self.config.accumulator_byte_size()
    }

    pub fn public_key_size(&self) -> usize {
        self.config.public_key_size()
    }

    pub fn contribution_byte_size(&self) -> usize {
        self.config.contribution_byte_size()
    }

    pub fn hash_size(&self) -> usize {
        64 // Blake2b hash size
    }

    pub fn empirical_batch_size(&self) -> usize {
        1 << 21
    }

    pub fn use_compression(&self) -> UseCompression {
        self.config.ceremony.compression.clone().into()
    }

    pub fn check_for_correctness(&self) -> CheckForCorrectness {
        self.config.ceremony.check_correctness.into()
    }

    // Print configuration summary
    pub fn print_info(&self) {
        println!("Ceremony Configuration:");
        println!("  Required Power: {} (2^{} = {} powers)", 
                 self.required_power(), 
                 self.required_power(), 
                 self.tau_powers_length());
        println!("  Curve: {}", self.config.curve.name);
        println!("  Compression: {:?}", self.config.ceremony.compression);
        println!("  Accumulator size: {} bytes", self.accumulator_byte_size());
        println!("  Contribution size: {} bytes", self.contribution_byte_size());
    }
}