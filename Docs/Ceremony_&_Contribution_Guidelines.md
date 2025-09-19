# Perpetual powers of tau ceremony on Cardano

## Introduction

### The perpetual powers of tau ceremony

The perpetual powers of tau (also known as the phase-1 trusted setup) is a cryptographic prelude used to derive the proving and verification keys required by Zero-Knowledge protocols. The trusted setup is important for Cardano because it lays the secure foundations for running applications that rely on Zero-Knowledge proofs. In our journey of developing applications, we found that most robust ceremonies target elliptic curves different from those used by Cardano ( the bls12-381 elliptic curve). This ceremony will construct partial zk-SNARK parameters for circuits up to depth 2^27 using the BLS12-381 elliptic curve construction, which will be practically suitable for almost circuits available out there.

### How does it works?

This ceremony is a multi-party computation process in which participants collectively generate these keys. Each contributor provides a secret input, ensuring that the final public parameters depend on randomness that no single participant fully controls. These secret inputs are private random values, which must be permanently discarded after use. Because they could compromise the system if leaked, they are often referred to as *toxic waste*. The security of the process relies on the guarantee that at least one contributor discards their toxic waste. If, however, all participants were to collude and retain their secrets, they could potentially subvert the system and produce valid but false Zero-Knowledge proofs. So, as we'll see later, make sure to erase your contribution, it is very important. 

### Ceremony Workflow

The **Perpetual Powers of Tau ceremony** is designed to generate public parameters that no single party controls, ensuring long-term security for Zero-Knowledge applications. The ceremony relies on two key roles: contributors and coordinators. Contributors inject private randomness into the multi-party computation, while coordinators manage the process by organizing contributions and verifying their correctness. The process unfolds as follows:

1. **Coordinator generates the initial challenge file**
    The ceremony begins with the coordinator producing an initial file, called the *challenge*. This file contains the starting parameters (the first set of powers of tau) that contributors will extend.

2. **Contributors add their secret randomness**
    Each contributor downloads the challenge file and mixes in their own private randomness. This randomness is combined with all previous contributions to extend the sequence of tau powers. The output is a new file, called the *response*, which includes the contributor’s randomness but hides the actual secret value.

   > ⚠️ At this stage, contributors must securely discard their private randomness (known as *toxic waste*). Keeping it would undermine the security of the system.

3. **Coordinator verifies contributions**
    After receiving each response, the coordinator verifies that the contribution was valid and correctly applied. This ensures that no one introduces malformed or malicious data that could break the setup.

4. **Final beacon application**
    Once all contributors have participated, the coordinator introduces an additional source of public randomness, called a *beacon*. Typically, this beacon is derived from a publicly verifiable and unpredictable source (such as block hashes, lottery results, or other randomness beacons). This step guarantees that even if every single contributor colluded, the beacon would still inject honest entropy into the system.

5. **Final verification and output**
    A last round of verification ensures that every contribution and the beacon have been correctly applied. The result is the **final trusted setup parameters** — a common reference string (CRS) that can be safely used by Zero-Knowledge proof systems.

### Ceremony coordination

Ceremony coordination

The coordination of the ceremony will be essential to ensure transparency and broad participation across the Cardano ecosystem.

Communication and social media
All announcements and updates will be shared through our official channels on Twitter/X, as well as a public GitHub repository. This will allow anyone interested to follow the progress in real time and access the resources needed to participate.

Registration form
A registration form will be published for participants to sign up as contributors. The form will request basic information (nickname, wallet address, technical background, and availability) to help organize the schedule and avoid overlaps.

Timeline
The ceremony will officially start on Q4 2025] and run for four weeks, with contribution slots assigned throughout this period. A detailed calendar will be made publicly available.

Contributors
Our goal is to reach at least 50 verified contributions. These will include:

Regular contributors, who will receive rewards in ADA after the ceremony is finalized.

Ad-honorem contributors, who will participate without financial rewards to further increase diversity and strengthen the contributor pool.

Rewards test transaction
Before distributing rewards, a test transaction on mainnet will be executed to verify the payment infrastructure and ensure that all participants can receive their compensation smoothly.

Regular reminders will be posted on social media, along with weekly progress updates, to keep the community informed and engaged.


## To run the ceremony on your laptop

### Hardware requisites

To participate in the trusted setup, you will need a computer **with at least 16 GB of RAM and 150 GB of available storage**. Each challenge file is about 72GB and the response files 36GB, the contribution on a regular computer should take around 48 to 72 hours to generate the contribution. 

### Software requisites

**Installing Rust**

The first step is to install *rustup* which is the toolchain manager of the Rust language, you can find the instructions to do so [here](https://www.rust-lang.org/learn/get-started).  The instruction should be something like this:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you check the version you should see something like this: 
```bash
$ rustup --version
rustup 1.28.2 (e4f3ad6f8 2025-04-28)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.87.0 (17067e9ac 2025-05-09)`
```

Then you will need to set up `cargo` and `rustc` to the 1.87.0 version doing the following command: 

```bash
rustup install 1.87.0
```

**Installing rsync**

You must install rsync to send your contribution: 
```bash
Rewardssudo apt update && sudo apt install rsync -y # Fedora & Debiand based system
sudo pacman -S rsync # Arch Linux
```

## Making the contribution

Good so now you are ready to make the contribution! 

**Clone the contribution software**

```bash
git clone https://github.com/AgustinBadi/powersoftau.git
```

**Update and build the Rust project**

```bash
rustup update
cargo build
cargo build --release
```

```bash
rustup update
cargo build
cargo build --release
```

**Download the challenge **

Download the `challenge_nnnn` file from the coordinator. The filename might be something like `challenge_0004`. Rename it to `challenge`:

```bash
mv challenge_nnnn challenge
```

Run the computation with `challenge` in your working directory:

```bash
cargo run --release --package powersoftau-cli --bin compute_constrained
```

You will see this prompt:

```
Will contribute to accumulator for 2^27 powers of tau
In total will generate up to 268435455 powers
Type some random text and press [ENTER] to provide additional entropy...
```

Make sure that it says `2^27 powers of tau`, and then enter random text as prompted. You should try to provide as much entropy as possible from sources which are truly hard to replicate. See below for examples derived below.

After a few minutes, it will write down the hash of the challenge file:

```
`challenge` file contains decompressed points and has a hash:
    4ef1fd9f f3154310 a773f3a4 fedecfa8
    14eec883 794e1e2f c7eb8ce4 3173e138
    0f2426d7 b8c6a097 4bfe3dd3 ae42d018
    6e0cf742 64b8e6ca c93b0a55 fd3b33bf
```

We recommend you keep a record of this hash.

The computation will run for about several hours on a fast machine. Please try your best to avoid electronic surveillance or tampering during this time.

When it is done, you will see something like this:

```
Finishing writing your contribution to `./response`...
Done!

Your contribution has been written to `./response`

The BLAKE2b hash of `./response` is:
        12345678 90123456 78901234 56789012
        12345678 90123456 78901234 56789012
        0b5337cd bb05970d 4045a88e 55fe5b1d
        507f5f0e 5c87d756 85b487ed 89a6fb50
Thank you for your participation, much appreciated! :)
```

Save the hash of the response in a file for your attestation. Upload the response file to the coordinator's server using this command:

```
rsync -vP -e "ssh -i $HOME/.ssh/id_rsa" response [remote_user]@[remote_host]:[remote_path]
```

> 📌 The remote user, host and path will be shared to you privately by the coordinator on the  

**Final step**

1. **After generating your response file, follow these steps to finalize your contribution safely:**
   1. **Publish your contribution hash**
      - Copy the hash from your response file output.
      - Post it in a private gist (for example: [gist example](https://gist.github.com/skywinder/c35ab03c66c6b200b33ea2f388a6df89)).
      - This acts as a public receipt of your participation and allows others to independently verify that your response was included.
   2. **Securely discard your toxic waste**
      - Reboot your laptop (or wipe any temporary files, if you used a VM or container).
      - This step is important to ensure your private randomness cannot be recovered later.
   3. **Transfer your response file**
      - Save the generated `response` file.
      - Provide it to the next participant or upload it to the coordinator (depending on the instructions on the discord channel).

## What’s Next?

We deeply appreciate your participation in this ceremony — your contribution plays an important role in strengthening the Cardano ecosystem! <3

The final results of the ceremony will be published in our official GitHub repository. There, we will also share the list of contributors and all relevant data related to the process.

### Rewards

Rewards will be distributed once the ceremony is completed and has passed approval from the Catalyst team. After that, we will announce the official date for rewards distribution.

---

## Appendix

**Look for a good entropy source**

To make the contribution you must make a random value that is hard to replicate. Now there are a couple random source you can choose: 

1. **Randomly typing**

The must obvious is to type a random string of characters using your keyword and creativity. 

2. **Using Cardano blocks**

You can add the Cardano block hashes as an input. You can check the hashes of the blocks on a Cardano explorer like [cardano scan](https://cardanoscan.io/blocks)

3. **Using the urandom device from Linux**

To read some values the `urandom` device you can use this command: 

```bash
xxd -p -l 256 /dev/urandom
```

It will display a long string random hexadecimals generated by the linux kernel. 

### FAQ

* **What will be the benefit for Cardano from this ceremony?**

This ceremony enables the secure operation of applications that rely on Zero-Knowledge proofs. Without completing this setup, such applications would face weaker security guarantees, limiting their robustness and trustworthiness.

* **What happens if I don't discard my toxic waste?**

In the context of a trusted setup, **toxic waste** refers to the private randomness that each contributor generates when making their contribution. This value must be permanently discarded after your participation. Failing to do so can compromise the security of the entire ceremony. Mallicious users could create valid but false proof, breaking the security garantees of a Zero-Knowledge protocol. 

* **Why is the ceremony is not fully permisionless?**

We decided to run this ceremony in a more **permissioned** way because previous open ceremonies have faced several challenges such as spam and sybil attacks. Implement a permisionless registration system is a complex task that doesn't provide sufficient garantees to solve these problem. In sum, our rationale was to make the process simpler and rely on trusted actors of the Cardano ecosystem such as stake pool operators, developers and other important figures; although permissioned, the ceremony remains transparent: every valid contribution will be publicly auditable, ensuring that the final output is trustworthy.















