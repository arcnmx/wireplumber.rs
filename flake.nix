{
  description = "wireplumber rust bindings";
  inputs = {
    flakelib.url = "github:flakelib/fl";
    nixpkgs = { };
    rust = {
      url = "github:arcnmx/nixexprs-rust";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    arc = {
      url = "github:arcnmx/nixexprs";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { flakelib, nixpkgs, ... }@inputs: let
    nixlib = nixpkgs.lib;
  in flakelib {
    inherit inputs;
    devShells = {
      plain = {
        mkShell, wpexec
      , wireplumber, pipewire, glib
      , pkg-config, wireplumber-gir, wpdev-gir, wpdev-todo
      , enableRustdoc ? false
      , enableRust ? true, cargo
      , rustTools ? [ ]
      }: let
        RUSTDOCFLAGS = nixlib.concatLists (nixlib.mapAttrsToList (crate: url:
          [ "--extern-html-root-url" "${crate}=${url}" ]
        ) rec {
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
      in mkShell {
        inherit rustTools;
        buildInputs = [ wireplumber pipewire glib ];
        nativeBuildInputs = [ pkg-config wpdev-gir wpdev-todo ] ++ nixlib.optional enableRust cargo;
        RUSTDOCFLAGS = nixlib.optionals enableRustdoc RUSTDOCFLAGS;
        GIR_FILE = "${wireplumber-gir}/share/gir-1.0/Wp-0.4.gir";
        inherit (wpexec) LIBCLANG_PATH BINDGEN_EXTRA_CLANG_ARGS;
      };
      stable = { rust'stable, outputs'devShells'plain }: outputs'devShells'plain.override {
        inherit (rust'stable) mkShell;
        enableRust = false;
      };
      dev = { arc'rustPlatforms, outputs'devShells'plain }: outputs'devShells'plain.override {
        inherit (arc'rustPlatforms.nightly.hostChannel) mkShell;
        enableRust = false;
        enableRustdoc = true;
        rustTools = [ "rust-analyzer" ];
      };
      default = { outputs'devShells }: outputs'devShells.plain;
    };
    packages = {
      wpexec = { stdenv, rustPlatform, lib, wireplumber, pipewire, glib, pkg-config, libclang }: rustPlatform.buildRustPackage rec {
        pname = "wpexec-rs";
        version = inputs.self.lastModifiedDate or "0";

        src = inputs.self;
        cargoLock = {
          lockFile = ./Cargo.lock;
          outputHashes = {
            "glib-signal-0.1.0" = "sha256-6awaofRnQcU5j3IWVH8Vo08FvS/fjVAHClnTFYMC9vY=";
          };
        };

        buildInputs = [ wireplumber pipewire glib ];
        nativeBuildInputs = [ pkg-config ];

        cargoBuildFlags = "-p wp-examples --bin wpexec";
        cargoBuildFeatures = [ (lib.featureForVersion wireplumber.version) ];

        buildType = "debug";
        doCheck = false;

        LIBCLANG_PATH = "${libclang.lib}/lib";
        BINDGEN_EXTRA_CLANG_ARGS = [
          "-I${stdenv.cc.cc}/lib/gcc/${stdenv.hostPlatform.config}/${stdenv.cc.cc.version}/include"
          "-I${stdenv.cc.libc.dev}/include"
        ];
      };
      gir-rs-0_16 = { rustPlatform, gir-rs, fetchFromGitHub }: rustPlatform.buildRustPackage rec {
        inherit (gir-rs) postPatch meta pname;
        version = "0.16-2022-10-27";

        src = fetchFromGitHub {
          owner = "gtk-rs";
          repo = "gir";
          rev = "f92952f3f7ea3c880558d57668129747ee1bec90";
          sha256 = "sha256-G1h72zVpxOE6JXbZSgAp68wjI75hzU+uhDDku7437D8=";
        };

        cargoSha256 = "sha256-JQNtvLywnxzC4h9ATzNCxpM5erOeLVu0veRIkhLV470=";
        buildType = "debug";
        doCheck = false;
      };
      wireplumber-gir = { runCommand, xmlstarlet, wireplumber }: runCommand "wireplumber-${wireplumber.version}.gir" {
        girName = "share/gir-1.0/Wp-${nixlib.versions.majorMinor wireplumber.version}.gir";
        wireplumber = wireplumber.dev;
        nativeBuildInputs = [ xmlstarlet ];
        # note: a pw_permission is actually 2x uint32
      } ''
        mkdir -p $out/$(dirname $girName)
        xmlstarlet ed \
          -i '///_:type[not(@name) and @c:type="pw_permission"]' -t attr -n name -v guint64 \
          -u '///_:constant[@c:type="WP_LOG_LEVEL_TRACE"]/@value' -v $((1<<8)) \
          -u '///_:constant[@c:type="WP_PIPEWIRE_OBJECT_FEATURES_ALL"]/@value' -v $((992|17)) \
          -i '///_:record[@c:type="WpIteratorMethods"]' -t attr -n glib:get-type -v wp_iterator_methods_get_type \
          -u '///_:record[@c:type="WpSpaPod"]/_:method[@c:identifier="wp_spa_pod_get_control"]//_:parameter[@name="ctl_type"]/@transfer-ownership' -v none \
          -u '///_:record[@c:type="WpSpaPod"]/_:method[@c:identifier="wp_spa_pod_get_property"]//_:parameter[@name="key"]/@transfer-ownership' -v none \
          -u '///_:record[@c:type="WpSpaPod"]/_:method[@c:identifier="wp_spa_pod_get_property"]//_:parameter[@name="value"]/@transfer-ownership' -v none \
          -u '///_:record[@c:type="WpSpaPod"]/_:method[@c:identifier="wp_spa_pod_get_string"]//_:parameter[@name="value"]/@transfer-ownership' -v none \
          -i '///_:class[@c:type="WpFactory"]' -t attr -n version -v 0.4.5 \
          -i '///_:record[@c:type="WpFactoryClass"]' -t attr -n version -v 0.4.5 \
          -i '///_:record[@c:type="WpSpaJson"]' -t attr -n version -v 0.4.8 \
          -i '///_:record[@c:type="WpSpaJsonParser"]' -t attr -n version -v 0.4.8 \
          -i '///_:record[@c:type="WpSpaJsonBuilder"]' -t attr -n version -v 0.4.8 \
          -i '///_:record[@c:type="WpSpaJson"]/_:constructor[@name="new_from_stringn"]' -t attr -n version -v 0.4.10 \
          -i '///_:enumeration[@c:type="WpSiAdapterPortsState"]' -t attr -n version -v 0.4.10 \
          -i '///_:interface[@c:type="WpSiAdapter"]/glib:signal[@name="adapter-ports-state-changed"]' -t attr -n version -v 0.4.10 \
          -i '///_:class[@c:type="WpDbus"]' -t attr -n version -v 0.4.11 \
          -i '///_:enumeration[@c:type="WpLinkState"]' -t attr -n version -v 0.4.11 \
          -i '///_:enumeration[@c:type="WpDBusState"]' -t attr -n version -v 0.4.11 \
          -i '///_:bitfield[@c:type="WpDbusFeatures"]' -t attr -n version -v 0.4.11 \
          -i '///_:bitfield[@c:type="WpLinkFeatures"]' -t attr -n version -v 0.4.11 \
          -i '///_:class[@c:type="WpCore"]/_:method[@name="get_vm_type"]' -t attr -n version -v 0.4.11 \
          -i '///_:class[@c:type="WpLink"]/_:property[@name="state"]' -t attr -n version -v 0.4.11 \
          -i '///_:class[@c:type="WpLink"]/glib:signal[@name="state-changed"]' -t attr -n version -v 0.4.11 \
          -i '///_:function[@name="get_library_version"]' -t attr -n version -v 0.4.12 \
          -i '///_:function[@name="get_library_api_version"]' -t attr -n version -v 0.4.12 \
          -u '//_:namespace[@name="Wp"]/@shared-library' -v wireplumber-0.4.so.0 \
          -i '/_:repository/_:namespace' -t elem -n package \
          "$wireplumber/$girName" > $out/$girName
        xmlstarlet ed -L \
          -i '/_:repository/_:package' -t attr -n name -v wireplumber-0.4 \
          $out/$girName
      '';
    };
    legacyPackages = { callPackageSet }: callPackageSet {
      wpdev-gir = { writeShellScriptBin, gir-rs-0_16, wireplumber-gir, gobject-introspection }: let
        gir-dirs = nixlib.concatMapStringsSep " " (dev:
          "--girs-directories ${dev}/share/gir-1.0"
        ) [ wireplumber-gir gobject-introspection.dev ];
      in writeShellScriptBin "gir" ''
        ${nixlib.getExe gir-rs-0_16} ${gir-dirs} "$@"
        if [[ $# -eq 0 ]]; then
          if [[ -d src/auto ]]; then
            sed -i -e '/^\/\/ from \/nix/d' src/auto/*.rs
          elif [[ -f tests/abi.rs ]]; then
            sed -i -e '/^\/\/ from \/nix/d' build{,_version}.rs {src,tests}/*.rs tests/*.{h,c}
          fi
        fi
      '';

      wpdev-todo = { writeShellScriptBin, wpdev-gir }: writeShellScriptBin "todo" ''
        cd ${toString ./.}
        exec ${nixlib.getExe wpdev-gir} -m not_bound
      '';
    } { };
    lib = with nixlib; {
      featureVersions = [
        "0.4.3" "0.4.5"
        "0.4.6"
        "0.4.8" "0.4.10"
        "0.4.11" "0.4.12"
      ];
      featureForVersion = version: mapNullable (v:
        "v" + replaceStrings [ "." ] [ "_" ] v
      ) (findFirst (versionAtLeast version) null (reverseList inputs.self.lib.featureVersions));
    };
    config = rec {
      name = "wireplumber-rust";
      packages.namespace = [ name ];
      inputs.arc = {
        lib.namespace = [ "arc" ];
        packages.namespace = [ "arc" ];
      };
    };
  };
}
