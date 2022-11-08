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
  cargo-bin = config: "${if config.enableDocs then rustChannel.buildChannel.cargo else pkgs.cargo}/bin/cargo";
  cargo = config: name: command: args: ci.command ({
    name = "cargo-${name}";
    command = "${cargo-bin config} " + command;
    impure = true;
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
        (cargo config "test-sys" "test -p wireplumber-sys ${versionFeature}" { })
        (cargo config "build" "build" { })
        (cargo config "test-wp" "test ${versionFeature}" { })
        (cargo config "test" "test --workspace ${versionFeature}" { })
        (cargo config "workspace" "build --workspace --examples --bins" { })
      ];
      example.inputs = [ wpexec ];
    };
    jobs = {
      docs = { config, pkgs, ... }: {
        ci.gh-actions = {
          checkoutOptions = {
            fetch-depth = 0;
          };
        };
        enableDocs = true;
        tasks = mkForce {
          docs-all.inputs = [
            (cargo config "doc-all" "doc --all-features --workspace --no-deps" { })
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
              docsDep = config.tasks.docs-all.drv;
              inherit (wireplumber-rust.devShells.${system}.plain.override { enableRustdoc = true; }) RUSTDOCFLAGS;
            })
          ];
          publish-docs.inputs = ci.command {
            name = "publish";
            impure = true;
            skip = if env.platform != "gh-actions" || env.gh-event-name or null != "push" then env.gh-event-name or "github"
              else if env.git-branch != "master" then "branch"
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
    enableDocs = mkEnableOption "docs generation";
  };
}
