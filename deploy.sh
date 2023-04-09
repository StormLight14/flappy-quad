read -p "Are you sure you want to deploy? (y/n) " answer
if [[ $answer != [yY] ]]; then
  echo "Aborting..."
  exit 0
fi

echo "Building application..."
cargo build --target wasm32-unknown-unknown

echo "Copying files to remote server..."
scp -r ./target/wasm32-unknown-unknown/debug/* storm@storm-dev.ddns.net:/var/www/html/games/flappy-quad
scp ./index.html storm@storm-dev.ddns.net:/var/www/html/games/flappy-quad

echo "Reloading Apache..."
ssh -t storm-dev.ddns.net sudo systemctl reload apache2