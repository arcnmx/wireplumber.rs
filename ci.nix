{ config, channels, pkgs, lib, ... }: with pkgs; with lib; let
  inherit (channels.arc.packages) wireplumber;
  importShell = writeText "shell.nix" ''
    import ${builtins.unsafeDiscardStringContext config.shell.drvPath}
  '';
  cargo = name: command: ci.command {
    name = "cargo-${name}";
    command = ''
      nix-shell ${importShell} --run ${escapeShellArg ("cargo " + command)}
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
      -u '//_:namespace[@name="Wp"]/@shared-library' -v wireplumber-0.4.so.0 \
      -i '/_:repository/_:namespace' -t elem -n package \
      "$wireplumber/$girName" > $out/$girName
    xmlstarlet ed -L \
      -i '/_:repository/_:package' -t attr -n name -v wireplumber-0.4 \
      $out/$girName
  '';
  gir-dirs = concatMapStringsSep " " (dev: "--girs-directories ${dev}/share/gir-1.0") [ wireplumber-gir gobject-introspection.dev ];
  gir = writeShellScriptBin "gir" ''
    exec ${gir-rs}/bin/gir ${gir-dirs} "$@"
  '';
  todo = writeShellScriptBin "todo" ''
    cd ${toString ./.}
    exec ${gir}/bin/gir -m not_bound
  '';
in {
  config = {
    name = "wireplumber.rs";
    ci.gh-actions.enable = true;
    cache.cachix.arc.enable = true;
    channels = {
      nixpkgs = "21.11";
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
        (cargo "build-sys" "build -p wireplumber-sys")
        (cargo "test-sys" "test -p wireplumber-sys")
        (cargo "build" "build")
        (cargo "test" "test")
      ];
    };
  };

  options = {
    rustChannel = mkOption {
      type = types.unspecified;
      default = channels.rust.stable;
    };
    shell = mkOption {
      type = types.unspecified;
      default = with pkgs; config.rustChannel.mkShell {
        buildInputs = [ wireplumber pipewire glib ];
        nativeBuildInputs = [ gir todo xmlstarlet pkg-config ];
        LIBCLANG_PATH = "${libclang.lib}/lib";
        BINDGEN_EXTRA_CLANG_ARGS = [
          "-I${stdenv.cc.cc}/lib/gcc/${stdenv.hostPlatform.config}/${stdenv.cc.cc.version}/include"
          "-I${stdenv.cc.libc.dev}/include"
        ];
      };
    };
  };
}
