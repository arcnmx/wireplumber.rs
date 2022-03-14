{ config, channels, pkgs, env, lib, ... }: with pkgs; with lib; let
  wireplumber = pkgs.wireplumber or channels.arc.packages.wireplumber-0_4_4;
  importShell = config: writeText "shell.nix" ''
    import ${builtins.unsafeDiscardStringContext config.shell.drvPath}
  '';
  versionFeature =
    if versionAtLeast wireplumber.version "0.4.8" then "--features v0_4_8"
    else if versionAtLeast wireplumber.version "0.4.6" then "--features v0_4_6"
    else if versionAtLeast wireplumber.version "0.4.3" then "--features v0_4_3"
    else "";
  cargo = config: name: command: ci.command {
    name = "cargo-${name}";
    command = ''
      nix-shell ${importShell config} --run ${escapeShellArg ("cargo " + command)}
    '';
    impure = true;
  };
  wireplumber-gir = runCommand "wireplumber.gir" {
    girName = "share/gir-1.0/Wp-${versions.majorMinor wireplumber.version}.gir";
    wireplumber = wireplumber.dev;
    nativeBuildInputs = [ xmlstarlet ];
  } ''
    WIREPLUMBER_DIR=$wireplumber/$girName
    mkdir -p $out/$(dirname $girName)
    # note: actually a pw_permission is 2x uint32
    xmlstarlet ed \
      -i '///_:type[not(@name) and @c:type="pw_permission"]' -t attr -n name -v guint64 \
      -u '///_:constant[@c:type="WP_LOG_LEVEL_TRACE"]/@value' -v $((1<<8)) \
      -u '///_:constant[@c:type="WP_PIPEWIRE_OBJECT_FEATURES_ALL"]/@value' -v $((992|17)) \
      -i '///_:record[@c:type="WpIteratorMethods"]' -t attr -n glib:get-type -v wp_iterator_methods_get_type \
      -u '///_:record[@c:type="WpSpaPod"]/_:method[@c:identifier="wp_spa_pod_get_control"]//_:parameter[@name="ctl_type"]/@transfer-ownership' -v none \
      -u '///_:record[@c:type="WpSpaPod"]/_:method[@c:identifier="wp_spa_pod_get_property"]//_:parameter[@name="key"]/@transfer-ownership' -v none \
      -u '///_:record[@c:type="WpSpaPod"]/_:method[@c:identifier="wp_spa_pod_get_property"]//_:parameter[@name="value"]/@transfer-ownership' -v none \
      -u '///_:record[@c:type="WpSpaPod"]/_:method[@c:identifier="wp_spa_pod_get_string"]//_:parameter[@name="value"]/@transfer-ownership' -v none \
      -i '///_:record[@c:type="WpSpaJson"]' -t attr -n version -v 0.4.8 \
      -i '///_:record[@c:type="WpSpaJsonParser"]' -t attr -n version -v 0.4.8 \
      -i '///_:record[@c:type="WpSpaJsonBuilder"]' -t attr -n version -v 0.4.8 \
      -u '//_:namespace[@name="Wp"]/@shared-library' -v wireplumber-0.4.so.0 \
      -i '/_:repository/_:namespace' -t elem -n package \
      "$wireplumber/$girName" > $out/$girName
    xmlstarlet ed -L \
      -i '/_:repository/_:package' -t attr -n name -v wireplumber-0.4 \
      $out/$girName
  '';
  gir-dirs = concatMapStringsSep " " (dev: "--girs-directories ${dev}/share/gir-1.0") [ wireplumber-gir gobject-introspection.dev ];
  gir = writeShellScriptBin "gir" ''
    ${gir-rs}/bin/gir ${gir-dirs} "$@"
    if [[ $# -eq 0 ]]; then
      if [[ -d src/auto ]]; then
        sed -i -e '/^\/\/ from \/nix/d' src/auto/*.rs
      elif [[ -f tests/abi.rs ]]; then
        sed -i -e '/^\/\/ from \/nix/d' build{,_version}.rs {src,tests}/*.rs tests/*.{h,c}
      fi
    fi
  '';
  todo = writeShellScriptBin "todo" ''
    cd ${toString ./.}
    exec ${gir}/bin/gir -m not_bound
  '';
  RUSTDOCFLAGS = concatLists (mapAttrsToList (crate: url: [ "--extern-html-root-url" "${crate}=${url}" ]) rec {
    #glib = "https://gtk-rs.org/gtk-rs-core/stable/latest/docs/";
    glib = "https://gtk-rs.org/gtk-rs-core/git/docs/";
    glib_sys = glib;
    gio = glib;
    gio_sys = glib;
    gobject_sys = glib;
    pipewire = "https://pipewire.pages.freedesktop.org/pipewire-rs/";
    pipewire_sys = pipewire;
    libspa = pipewire;
    libspa_sys = pipewire;
  }) ++ [ "-Z" "unstable-options" ];
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
      nixpkgs = mkIf (env.platform != "impure") "21.11";
      rust = "master";
      arc = "master";
    };
    environment = {
      test = {
        inherit (config.rustChannel.buildChannel) cargo;
      };
    };
    tasks = {
      build.inputs = [
        (cargo config "build-sys" "build -p wireplumber-sys")
        (cargo config "test-sys" "test -p wireplumber-sys ${versionFeature}")
        (cargo config "build" "build")
        (cargo config "test-wp" "test ${versionFeature}")
        (cargo config "test" "test --workspace ${versionFeature}")
        (cargo config "workspace" "build --workspace --examples --bins")
      ];
    };
    jobs = {
      docs = { config, pkgs, ... }: let
        doc = ci.command {
          name = "cargo-doc";
          command = concatMapStringsSep "\n" (c: "nix-shell ${importShell config} --run ${escapeShellArg c}") [
            "cargo clean --doc && rm -rf \${CARGO_TARGET_DIR:-target}/${config.rustChannel.hostTarget.triple}/doc" # `cargo clean --doc` does nothing afaict?
            "cargo doc --no-deps -p glib-signal" # can't pass --features because cargo is garbage :<
            "cargo doc --no-deps --workspace --all-features"
            "cargo doc --no-deps --workspace --examples --document-private-items --all-features"
          ];
          docsDep = config.tasks.docs-all.drv;
          impure = true;
        };
      in {
        ci.gh-actions = {
          checkoutOptions = {
            fetch-depth = 0;
          };
        };
        enableDocs = true;
        tasks = mkForce {
          docs-all.inputs = [
            (cargo config "doc-all" "doc --all-features --workspace --no-deps")
          ];
          docs.inputs = doc;
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
              cp -a ''${CARGO_TARGET_DIR:-../target}/${config.rustChannel.hostTarget.triple}/doc/* ./

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
      dev = { config, ... }: {
        ci.gh-actions.emit = mkForce false;
        channels.nixpkgs = config.parentConfig.channels.nixpkgs;
        enableDocs = true;
        enableDev = true;
      };
    };
  };

  options = {
    enableDocs = mkEnableOption "docs generation";
    enableDev = mkEnableOption "dev shell generation";
    rustChannel = mkOption {
      type = types.unspecified;
      default = if config.enableDocs
        then arc.pkgs.rustPlatforms.nightly.hostChannel
        else channels.rust.stable;
    };
    shell = mkOption {
      type = types.unspecified;
      default = with pkgs; config.rustChannel.mkShell {
        rustTools = optional config.enableDev "rust-analyzer";
        buildInputs = [ wireplumber pipewire glib ];
        nativeBuildInputs = [ gir todo xmlstarlet pkg-config ];
        RUSTDOCFLAGS = optionals config.enableDocs RUSTDOCFLAGS;
        GIR_FILE = "${wireplumber-gir}/share/gir-1.0/Wp-0.4.gir";
        LIBCLANG_PATH = "${libclang.lib}/lib";
        BINDGEN_EXTRA_CLANG_ARGS = [
          "-I${stdenv.cc.cc}/lib/gcc/${stdenv.hostPlatform.config}/${stdenv.cc.cc.version}/include"
          "-I${stdenv.cc.libc.dev}/include"
        ];
      };
    };
  };
}
