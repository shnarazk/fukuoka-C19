{
  description = "A Fukuoka COVID-19 stats viewer in Rust/Dioxus";
  inputs.nixpkgs.url = github:NixOS/nixpkgs;
  outputs = { self, nixpkgs }:
  {
    packages = builtins.listToAttrs
      (map
        (system:
          with import nixpkgs { system = "${system}"; };
          {
            name = system;
            value = {
              default =
                stdenv.mkDerivation rec {
                  name = "fukuoka-c19-${version}";
                  pname = "fukuoka-c19";
                  version = "0.4.0-20220408-1";
                  src = self;
                  buildInputs = rustc.buildInputs ++ [ cargo rustc ] ++
                                lib.optionals stdenv.isDarwin (
                                  with darwin.apple_sdk.frameworks; [ AppKit Carbon WebKit ]);
                  buildPhase = "cargo build --release";
                  installPhase = ''
                      mkdir -p $out/bin;
                      cp target/release/fukuoka-c19 $out/bin/fukuoka-c19
                  '';
                }
              ;
          };}
        )
        [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ]
      )
    ;
  };
}
