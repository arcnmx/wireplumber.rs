{ config, channels, pkgs, env, lib, ... }: with pkgs; with lib; let
  lockData = builtins.fromJSON (builtins.readFile ./flake.lock);
  sourceInfo = lockData.nodes.std.locked;
  src = fetchTarball {
    url = "https://github.com/${sourceInfo.owner}/${sourceInfo.repo}/archive/${sourceInfo.rev}.tar.gz";
    sha256 = sourceInfo.narHash;
  };
  inherit (import src) Flake;
  wireplumber-rust = Flake.CallDir ./. (Flake.Lock.Node.inputs (Flake.Lock.root (Flake.Lock.New (lockData // {
    override.sources.nixpkgs = pkgs.path;
  }))));
  inherit (wireplumber-rust.packages.${system}) wpexec;
  versionFeature = toString (mapNullable (f: "--features ${f}") (wireplumber-rust.lib.featureForVersion wireplumber.version));
  cargo-bin = config: "${if config.enableNightly then rustChannel.buildChannel.cargo else pkgs.cargo}/bin/cargo";
  cargo = config: name: command: args: ci.command ({
    name = "cargo-${name}";
    displayName = "cargo " + command;
    command = "${cargo-bin config} " + command;
    impure = true;
    ${if config.enableNightly && hasPrefix "fmt" command then "RUSTFMT" else null} = "${rustChannel.buildChannel.rustfmt}/bin/rustfmt";
    inherit (wpexec) LIBCLANG_PATH BINDGEN_EXTRA_CLANG_ARGS;
    PKG_CONFIG_PATH = makeSearchPath "lib/pkgconfig" wpexec.buildInputs;
    "AR_${replaceStrings [ "-" ] [ "_" ] hostPlatform.config}" = "${stdenv.cc.bintools.bintools}/bin/${stdenv.cc.targetPrefix}ar";
  } // args);
  rustChannel = channels.rust.nightly;
in {
  config = {
    name = "wireplumber.rs";
    ci.version = "nix2.4-broken"; # for checkout@v2
    ci.gh-actions = {
      enable = true;
      emit = true;
    };
    cache.cachix.arc.enable = true;
    channels = {
      nixpkgs = mkIf (env.platform != "impure") "22.11";
      rust = "master";
    };
    environment = {
      test = {
        inherit (pkgs) pkg-config;
        inherit (stdenv) cc;
      };
    };
    tasks = {
      build.inputs = [
        (cargo config "build-sys" "build -p wireplumber-sys" { })
        (cargo config "build" "build" { })
      ];
      test.inputs = cargo config "test" "test" {
        buildDep = config.tasks.build.drv;
      };
    };
    jobs = {
      example-wpexec = { config, ... }: {
        ci.gh-actions.name = "wpexec";
        tasks = mkForce {
          example.inputs = wpexec;
        };
      };
      test = { config, ... }: {
        ci.gh-actions.name = "cargo test";
        tasks = mkForce {
          sys.inputs = cargo config "test-sys" "test -p wireplumber-sys ${versionFeature}" { };
          test.inputs = cargo config "test-wp" "test ${versionFeature}" { };
        };
      };
      features = { config, ... }: {
        ci.gh-actions.name = "features";
        tasks = mkForce {
          features.inputs = [
            (cargo config "libspa" "build -F libspa,futures" { })
            (cargo config "glib-signal-sans-futures" "build -F glib-signal" { })
            (cargo config "experimental" "build -F experimental" {
              warn = true;
            })
          ];
          versions.inputs = let
            versions = init (wireplumber-rust.lib.supportedVersions wireplumber.version);
          in map (version: cargo config "build-${version}" "build -F ${wireplumber-rust.lib.versionFeatureName version}" { }) versions;
        };
      };
      examples = { config, ... }: {
        ci.gh-actions.name = "cargo build --examples";
        tasks = mkForce {
          build.inputs = cargo config "workspace" "build --workspace --examples --bins" { };
        };
      };
      docs = { config, ... }: {
        ci.gh-actions.name = "cargo doc --workspace";
        enableNightly = true;
        tasks = mkForce {
          docs-all.inputs = cargo config "doc" "doc --all-features --workspace --no-deps" { };
        };
      };
      nightly = { config, ... }: {
        ci.gh-actions.name = "cargo doc+fmt";
        ci.gh-actions = {
          checkoutOptions = {
            fetch-depth = 0;
          };
        };
        enableNightly = true;
        tasks = mkForce {
          fmt.inputs = [
            (cargo config "fmt" "fmt --check" {
              cache = false;
            })
            (cargo config "fmt-examples" "fmt -p wp-examples --check" {
              cache = false;
            })
          ];
          docs.inputs = [
            (cargo config "doc" ("clean --doc && rm -rf \${CARGO_TARGET_DIR:-target}/${rustChannel.hostTarget.triple}/doc"
              # `cargo clean --doc` does nothing afaict?
              + "\n" + concatStringsSep "\n" [
                "${cargo-bin config} doc --no-deps -p glib-signal" # can't pass --features because cargo is garbage :<
                "${cargo-bin config} doc --no-deps --workspace --all-features"
                "${cargo-bin config} doc --no-deps --workspace --examples --document-private-items --all-features"
              ]
            ) {
              displayName = "cargo doc";
              inherit (wireplumber-rust.devShells.${system}.plain.override { enableRustdoc = true; }) RUSTDOCFLAGS;
            })
          ];
          publish-docs.inputs = ci.command {
            name = "publish";
            displayName = "publish docs";
            impure = true;
            skip = if env.platform != "gh-actions" || env.gh-event-name or null != "push" then env.gh-event-name or "github"
              else if env.git-branch != "main" then "branch"
              else false;
            gitCommit = env.git-commit;
            docsBranch = "gh-pages";
            docsDep = config.tasks.docs.drv;
            environment = [ "CARGO_TARGET_DIR" ];
            command = ''
              git fetch origin
              if [[ -e $docsBranch ]]; then
                git worktree remove -f $docsBranch || true
                rm -rf ./$docsBranch || true
              fi
              git worktree add --detach $docsBranch && cd $docsBranch
              git branch -D pages || true
              git checkout --orphan pages && git rm -rf .
              git reset --hard origin/$docsBranch -- || true
              rm -rf ./*
              cp -a ''${CARGO_TARGET_DIR:-../target}/${rustChannel.hostTarget.triple}/doc/* ./

              git add -A
              if [[ -n $(git status --porcelain) ]]; then
                export GIT_{COMMITTER,AUTHOR}_EMAIL=ghost@konpaku.2hu
                export GIT_{COMMITTER,AUTHOR}_NAME=ghost
                git commit -m "$gitCommit"
                git push origin HEAD:$docsBranch
              fi
            '';
          };
        };
      };
    };
  };

  options = {
    enableNightly = mkEnableOption "unstable rust";
  };
}
