extern crate bellman;
extern crate memmap;

use powersoftau_core::batched_accumulator::BatchedAccumulator;
use powersoftau_core::parameters::UseCompression;
use powersoftau_core::small_bls12_381::Bls12CeremonyParameters;
use powersoftau_core::utils::blank_hash;
use powersoftau_core::RuntimeCeremonyParameters;

use bellman::pairing::bls12_381::Bls12;
use memmap::*;
use std::fs::OpenOptions;
use std::io::Write;


fn main() {
    // Load ceremony configuration
    let params = RuntimeCeremonyParameters::from_env_with_defaults()
        .expect("Failed to load ceremony configuration");
    
    params.print_info();
    
    let compress_new_challenge = params.use_compression();
    
    println!(
        "Will generate an empty accumulator for 2^{} powers of tau",
        params.required_power()
    );
    println!(
        "In total will generate up to {} powers",
        params.tau_powers_g1_length()
    );

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open("challenge")
        .expect("unable to create `./challenge`");

    let expected_challenge_length = match compress_new_challenge {
        UseCompression::Yes => {
            params.contribution_byte_size() - params.public_key_size()
        }
        UseCompression::No => params.accumulator_byte_size(),
    };

    file.set_len(expected_challenge_length as u64)
        .expect("unable to allocate large enough file");

    let mut writable_map = unsafe {
        MmapOptions::new()
            .map_mut(&file)
            .expect("unable to create a memory map")
    };

    // Write a blank BLAKE2b hash:
    let hash = blank_hash();
    (&mut writable_map[0..])
        .write(hash.as_slice())
        .expect("unable to write a default hash to mmap");
    writable_map
        .flush()
        .expect("unable to write blank hash to `./challenge`");

    println!("Blank hash for an empty challenge:");
    for line in hash.as_slice().chunks(16) {
        print!("\t");
        for section in line.chunks(4) {
            for b in section {
                print!("{:02x}", b);
            }
            print!(" ");
        }
        println!("");
    }

    BatchedAccumulator::<Bls12, Bls12CeremonyParameters>::generate_initial(
        &mut writable_map,
        compress_new_challenge,
    )
    .expect("generation of initial accumulator is successful");
    writable_map
        .flush()
        .expect("unable to flush memmap to disk");

    // Get the hash of the contribution, so the user can compare later
    let output_readonly = writable_map
        .make_read_only()
        .expect("must make a map readonly");
    let contribution_hash =
        BatchedAccumulator::<Bls12, Bls12CeremonyParameters>::calculate_hash(&output_readonly);

    println!("Empty contribution is formed with a hash:");

    for line in contribution_hash.as_slice().chunks(16) {
        print!("\t");
        for section in line.chunks(4) {
            for b in section {
                print!("{:02x}", b);
            }
            print!(" ");
        }
        println!("");
    }

    println!("Wrote a fresh accumulator to `./challenge`");
}
