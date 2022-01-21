{ config, channels, pkgs, env, lib, ... }: with pkgs; with lib; let
  wireplumber = pkgs.wireplumber or channels.arc.packages.wireplumber-0_4_4;
  importShell = config: writeText "shell.nix" ''
    import ${builtins.unsafeDiscardStringContext config.shell.drvPath}
  '';
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
in {
  config = {
    name = "wireplumber.rs";
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
        (cargo config "test-sys" "test -p wireplumber-sys")
        (cargo config "build" "build")
        (cargo config "test" "test")
        (cargo config "workspace" "build --workspace")
      ];
    };
    jobs = {
      docs = { config, ... }: {
        enableDocs = true;
        tasks = mkForce {
          docs.inputs = [
            (cargo config "doc" "doc --workspace --features dox")
            (cargo config "doc-all" "doc --all-features")
          ];
        };
      };
      dev = { config, ... }: {
        channels.nixpkgs = config.parentConfig.channels.nixpkgs;
        enableDocs = true;
        enableDev = true;
        ci.gh-actions.emit = mkForce false;
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
