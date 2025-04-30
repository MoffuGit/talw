{
  description = "TALW flake";

  inputs = {
       nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
       rust-overlay.url = "github:oxalica/rust-overlay";
       flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =  {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs =
            [
              mysql80
              nodejs
              openssl
              pkg-config
              gcc
              glib
              cargo-leptos
              (rust-bin.selectLatestNightlyWith (
                toolchain:
                toolchain.default.override {
                  extensions = [
                    "rust-src"
                    "rust-analyzer"
                  ];
                  targets = [ "wasm32-unknown-unknown" ];
                }
              ))
            ];

	 shellHook = ''
MYSQL_HOME="$PWD/mysql"
export MYSQL_SOCKET="$MYSQL_HOME/mysql.sock"

if [ ! -d "$MYSQL_HOME/mysql" ]; then
  echo "Initializing MySQL data directory..."
  mysqld --defaults-file=/dev/null --initialize --datadir="$MYSQL_HOME" --basedir=$(dirname $(dirname $(which mysqld)))
fi

if ! pgrep -x "mysqld" > /dev/null; then
    nohup mysqld --defaults-file=/dev/null --datadir=$MYSQL_HOME --basedir=$(dirname $(dirname $(which mysqld))) --socket=$MYSQL_SOCKET --pid-file=$MYSQL_HOME/mysql.pid --disable-mysqlx > /dev/null 2>&1 &
fi

shutdown_mysql() {
  if [ -f "$MYSQL_HOME/mysql.pid" ]; then
    pkill -u "$USER" mysqld &> /dev/null
    rm "$MYSQL_HOME/mysql.pid"
  fi
}

trap shutdown_mysql EXIT
          '';
        };
      }
    );
}
