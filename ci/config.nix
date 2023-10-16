{ config, pkgs, env, lib, ... }: with pkgs; with lib; let
  wireplumber-rust = import ../. { pkgs = null; };
  inherit (wireplumber-rust) inputs checks legacyPackages packages devShells;
  wplib = wireplumber-rust.lib;
  wpexec = packages.wpexec.override {
    buildType = "debug";
  };
  versionFeature = toString (mapNullable (f: "--features ${f}") (wplib.featureForVersion wireplumber.version));
  cargo-bin = config: "${if config.enableNightly then rustChannel.buildChannel.cargo else inputs.nixpkgs.legacyPackages.${builtins.currentSystem}.cargo}/bin/cargo";
  cargo = config: name: command: args: ci.command ({
    name = "cargo-${name}";
    displayName = "cargo " + command;
    command = "${cargo-bin config} " + command;
    impure = true;
    sourceDep = legacyPackages.source;
    ${if config.enableNightly && hasPrefix "fmt" command then "RUSTFMT" else null} = "${rustChannel.buildChannel.rustfmt}/bin/rustfmt";
    inherit (wpexec) LIBCLANG_PATH BINDGEN_EXTRA_CLANG_ARGS;
    PKG_CONFIG_PATH = makeSearchPath "lib/pkgconfig" wpexec.buildInputs;
    "AR_${replaceStrings [ "-" ] [ "_" ] hostPlatform.config}" = "${stdenv.cc.bintools.bintools}/bin/${stdenv.cc.targetPrefix}ar";
  } // args);
  rustChannel = inputs.rust.legacyPackages.${builtins.currentSystem}.unstable;
  v0' = builtins.match ''^(v)?[0-9].*$'';
  v0 = v: v != null && v0' v != null;
in {
  config = {
    name = "wireplumber.rs";
    ci.version = "v0.6";
    ci.gh-actions = {
      enable = true;
      emit = true;
      checkoutOptions = {
        submodules = false;
      };
    };
    cache.cachix = {
      ci.signingKey = "";
      arc.enable = true;
    };
    channels = {
      nixpkgs = mkIf (env.platform != "impure") "23.05";
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
          wpexec.inputs = checks.wpexec.override {
            inherit wpexec;
          };
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
            (cargo config "experimental" "build -F libspa,experimental" {
              warn = true;
            })
          ];
          versions.inputs = let
            versions = init (wplib.supportedVersions wireplumber.version);
          in map (version: cargo config "build-${version}" "build -F ${wplib.versionFeatureName version}" { }) versions;
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
          readme.inputs = [
            checks.readme-github
            checks.readme-sys-github
            checks.readme-examples-github
            checks.readme-package
            checks.readme-sys-package
            checks.readme-attrs
            checks.commitlint-help
            checks.commitlintrc
          ];
          commitlint.inputs = ci.command rec {
            name = "commitlint";
            displayName = "commitlint ${baseRef}..${gitCommit}";
            impure = true;
            gitCommit = if env.git-commit or null != null then env.git-commit else "HEAD";
            baseRef = if elem env.gh-event-name or null [ "pull_request" "pull_request_target" ]
              then "origin/${env.pr-base}"
              else "HEAD~";
            command = ''
              ${legacyPackages.wpdev-commitlint}/bin/commitlint --from $baseRef --to $gitCommit --verbose
            '';
          };
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
              inherit (devShells.plain.override { enableRustdoc = true; }) RUSTDOCFLAGS;
              RELEASE_TAG = if v0 env.git-tag then env.git-tag else wplib.releaseTag;
            })
          ];
          refs.inputs = let
            checks = {
              tag = {
                message = "tag ${env.git-tag} does not match Cargo.toml version ${wplib.version}";
                check = hasPrefix "v" env.git-tag &&
                  removePrefix "v" env.git-tag == wplib.version;
              };
              branch = {
                message = "branch ${env.git-branch} does not match Cargo.toml version ${wplib.version}";
                check = hasPrefix "v" env.git-branch &&
                  hasPrefix (removeSuffix "x" (removePrefix "v" env.git-branch)) wplib.version;
              };
            };
            check =
              if env.git-tag != null && v0 env.git-tag then checks.tag
              else if env.git-branch != null && v0 env.git-branch then checks.branch
              else null;
          in ci.command {
            name = "check-ref";
            displayName = "git ref valid";
            command = optionalString (check != null && !check.check) ''
              printf %s ${escapeShellArg check.message} >&2
              exit 1
            '';
          };
          publish-docs.inputs = let
            srcBranch = findFirst (v: v != null) null [ env.git-tag env.git-branch ];
          in ci.command {
            name = "publish-docs";
            displayName = "publish docs";
            impure = true;
            skip = if env.platform != "gh-actions" || env.gh-event-name or null != "push" then env.gh-event-name or "github"
              else if env.git-tag != null && ! v0 env.git-tag then "unversioned tag"
              else if env.git-branch != null && ! (elem env.git-branch wplib.branches || v0 env.git-branch) then "feature branch"
              else if srcBranch == null then "unknown branch"
              else false;
            gitCommit = env.git-commit;
            docsBranch = "gh-pages";
            inherit srcBranch;
            releaseTag = if env.git-branch == "main" || v0 env.git-branch then wplib.releaseTag
              else if v0 env.git-tag then env.git-tag
              else "";
            docsDep = config.tasks.docs.drv;
            refsDep = config.tasks.refs.drv;
            environment = [ "CARGO_TARGET_DIR" ];
            command = ''
              DOCS_DIR=''${CARGO_TARGET_DIR:-$PWD/target}/${rustChannel.hostTarget.triple}/doc

              if [[ ! -e $DOCS_DIR ]]; then
                # skip if no docs were actually generated by this CI run
                # (generally because no source changes)
                exit
              fi

              git fetch origin
              if [[ -e $docsBranch ]]; then
                git worktree remove -f $docsBranch || true
                rm -rf ./$docsBranch || true
              fi
              git worktree add --detach $docsBranch && cd $docsBranch
              git branch -D pages || true
              git checkout --orphan pages && git rm -rf .
              git reset --hard origin/$docsBranch -- || true
              rm -rf "./$srcBranch"
              mkdir -p "./$srcBranch"
              cp -a $DOCS_DIR/* "./$srcBranch/"
              git add "$srcBranch"

              if [[ -n $releaseTag ]] && [[ $srcBranch != $releaseTag ]]; then
                ln -sfn "$srcBranch" "$releaseTag"
                git add "$releaseTag"
              fi

              if [[ -n $(git status --porcelain) ]]; then
                export GIT_{COMMITTER,AUTHOR}_EMAIL=ghost@konpaku.2hu
                export GIT_{COMMITTER,AUTHOR}_NAME=ghost
                git commit -m "$srcBranch: $gitCommit"
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
