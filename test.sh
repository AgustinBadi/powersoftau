#!/bin/sh

# Powers of Tau Ceremony Test Script
# Uses workspace structure with powersoftau-cli package

echo "Cleaning up old ceremony files..."
rm -f challenge
rm -f response
rm -f new_challenge
rm -f challenge_old
rm -f response_old

echo "Starting ceremony simulation..."
echo "1. Generating initial challenge..."
cargo run --release --package powersoftau-cli --bin new_constrained

echo "2. Computing participant contribution..."
cargo run --release --package powersoftau-cli --bin compute_constrained

echo "3. Verifying contribution and generating new challenge..."
cargo run --release --package powersoftau-cli --bin verify_transform_constrained

echo "Moving files for beacon phase..."
mv challenge challenge_old
mv response response_old
mv new_challenge challenge

echo "4. Applying randomness beacon..."
cargo run --release --package powersoftau-cli --bin beacon_constrained

echo "5. Final verification..."
cargo run --release --package powersoftau-cli --bin verify_transform_constrained

echo "Ceremony simulation complete!"
echo "Final challenge file created."
