{
  description = "wireplumber rust bindings";
  inputs = {
    flakelib.url = "github:flakelib/fl";
    nixpkgs = { };
    rust = {
      url = "github:arcnmx/nixexprs-rust";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, flakelib, nixpkgs, rust, ... }@inputs: let
    nixlib = nixpkgs.lib;
  in flakelib {
    inherit inputs;
    systems = [ "x86_64-linux" "aarch64-linux" ];
    devShells = {
      plain = {
        mkShell, writeShellScriptBin, wpexec
      , wireplumber, pipewire, glib
      , pkg-config
      , wireplumber-gir, wpdev-gir, wpdev-todo, wpdev-commitlint
      , enableRustdoc ? false
      , enableRust ? true, cargo
      , rustTools ? [ ]
      }: let
      in mkShell {
        inherit rustTools;
        buildInputs = [ wireplumber pipewire glib ];
        nativeBuildInputs = [
          pkg-config
          wpdev-commitlint wpdev-gir wpdev-todo
          (writeShellScriptBin "generate" "nix run .#wpdev-generate")
        ] ++ nixlib.optional enableRust cargo;
        RUSTDOCFLAGS = rust.lib.rustdocFlags {
          inherit (self.lib) crate;
          enableUnstableRustdoc = enableRustdoc;
          extern = rec {
            glib = let
              version = nixlib.versions.majorMinor self.lib.crate.dependencies.glib.version;
            in if self.lib.crate.dependencies.glib ? git
              then "https://gtk-rs.org/gtk-rs-core/git/docs/"
              else "https://gtk-rs.org/gtk-rs-core/stable/${version}/docs/";
            glib-sys = glib;
            gio = glib;
            gio-sys = glib;
            gobject-sys = glib;
            pipewire = "https://pipewire.pages.freedesktop.org/pipewire-rs/";
            pipewire-sys = pipewire;
            libspa = pipewire;
            libspa-sys = pipewire;
          };
        };
        GIR_FILE = "${wireplumber-gir}/share/gir-1.0/Wp-0.4.gir";
        inherit (wpexec) LIBCLANG_PATH BINDGEN_EXTRA_CLANG_ARGS;
      };
      stable = { rust'stable, outputs'devShells'plain }: outputs'devShells'plain.override {
        inherit (rust'stable) mkShell;
        enableRust = false;
      };
      dev = { rust'unstable, outputs'devShells'plain }: outputs'devShells'plain.override {
        inherit (rust'unstable) mkShell;
        enableRust = false;
        enableRustdoc = true;
        rustTools = [ "rust-analyzer" ];
      };
      default = { outputs'devShells }: outputs'devShells.plain;
    };
    packages = {
      wpexec = {
        stdenv, rustPlatform, lib
      , wireplumber, pipewire, glib
      , pkg-config, libclang
      , buildType ? "release"
      , source
      }: with lib; rustPlatform.buildRustPackage rec {
        pname = "wpexec-rs";
        version = if buildType == "release"
          then self.lib.version
          else self.lastModifiedDate or self.lib.version;

        src = source;
        inherit (self.lib.crate) cargoLock;

        buildInputs = [ wireplumber pipewire glib ];
        nativeBuildInputs = [ pkg-config ];

        cargoBuildFlags = "--workspace --bin wpexec";
        buildFeatures = mapNullable singleton (self.lib.featureForVersion wireplumber.version);

        inherit buildType;
        doCheck = false;

        LIBCLANG_PATH = "${libclang.lib}/lib";
        BINDGEN_EXTRA_CLANG_ARGS = [
          "-I${stdenv.cc.cc}/lib/gcc/${stdenv.hostPlatform.config}/${stdenv.cc.cc.version}/include"
          "-I${stdenv.cc.libc.dev}/include"
        ];

        meta = with lib; {
          description = "A WirePlumber utility ported to Rust";
          homepage = "https://github.com/arcnmx/wireplumber.rs";
          license = lib.licenses.mit;
          maintainers = [ maintainers.arcnmx ];
          platforms = platforms.linux;
          mainProgram = "wpexec";
        };
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
          -i '///_:class[@c:type="WpMetadata"]/_:method[@name="find"]/_:return-value' -t attr -n nullable -v 1 \
          -i '///_:record[@c:type="WpSpaJsonParser"]/_:method[@name="get_json"]/_:return-value' -t attr -n nullable -v 1 \
          -i '///_:record[@c:type="WpSpaPodParser"]/_:method[@name="get_pod"]/_:return-value' -t attr -n nullable -v 1 \
          -i '///_:class[@name="SpaType"]/_:method[@name="get_values_table"]/_:return-value' -t attr -n nullable -v 1 \
          -i '///_:class[@name="SpaType"]/_:method[@name="name"]/_:return-value' -t attr -n nullable -v 1 \
          -i '///_:function[@name="find_file"]/_:return-value' -t attr -n nullable -v 1 \
          -i '///_:function[@name="spa_id_value_from_name"]/_:return-value' -t attr -n nullable -v 1 \
          -i '///_:function[@name="spa_id_value_from_number"]/_:return-value' -t attr -n nullable -v 1 \
          -i '///_:function[@name="spa_id_value_from_short_name"]/_:return-value' -t attr -n nullable -v 1 \
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
          -i '/_:repository/_:package[not(@name)]' -t attr -n name -v wireplumber-0.4 \
          $out/$girName
      '';
    };
    checks = {
      wpexec = { wptest, writeText, wpexec, lib }: let
        key = placeholder "wpexec";
        arg = builtins.toJSON [ key ];
        script = writeText "wpexec.lua" ''
          Debug.dump_table(...)
          Core.quit()
        '';
      in wptest "wpexec" ''
        set +o pipefail
        timeout 5 ${wpexec}/bin/wpexec --json ${lib.escapeShellArg arg} ${script} | grep -F ${key}
        touch $out
      '';
      rustfmt = { rust'builders, source }: rust'builders.check-rustfmt-unstable {
        src = source;
        config = ./.rustfmt.toml;
        cargoFmtArgs = [
          "-p" "wireplumber"
          "-p" "wp-examples"
        ];
      };
      docs = { docs }: docs;
      readme = { rust'builders, wpdev-readme }: rust'builders.check-generate {
        expected = wpdev-readme;
        src = ./src/README.md;
        meta.name = "diff src/README.md (nix run .#wpdev-generate)";
      };
      readme-sys = { rust'builders, wpdev-sys-readme }: rust'builders.check-generate {
        expected = wpdev-sys-readme;
        src = ./sys/src/README.md;
        meta.name = "diff sys/src/README.md (nix run .#wpdev-generate)";
      };
      commitlint-help = { rust'builders, wpdev-commitlint-help }: rust'builders.check-generate {
        expected = wpdev-commitlint-help;
        src = ./.github/commitlint.adoc;
        meta.name = "diff .github/commitlint.adoc (nix run .#wpdev-generate)";
      };
      release-branch = { rust'builders, source }: let
        inherit (self.lib) releaseTag;
        docs'rs = {
          inherit (self.lib.crate.package) name;
          version = releaseTag;
          baseUrl = rust.lib.escapePattern self.lib.pagesRoot;
        };
        cargo'docs = {
          inherit (docs'rs) name version baseUrl;
        };
      in rust'builders.check-contents {
        name = "wireplumber-release-check" ;
        patterns = [
          { path = "src/lib.rs"; inherit docs'rs; }
          { path = "sys/src/lib.rs"; inherit docs'rs; }
          { path = "src/README.md"; plain = "/tree/${releaseTag}/examples"; }
          { path = "Cargo.toml"; inherit cargo'docs; }
          { path = "sys/Cargo.toml"; cargo'docs = cargo'docs // { name = "wireplumber-sys"; }; }
        ];
        src = source;
      };
    };
    legacyPackages = { callPackageSet }: callPackageSet {
      source = { rust'builders }: rust'builders.wrapSource self.lib.crate.src;

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

      docs = { rust'builders, outputs'devShells'plain, wpexec, source }: let
        shell = outputs'devShells'plain.override { enableRust = false; enableRustdoc = true; };
      in rust'builders.cargoDoc {
        inherit (self.lib) crate;
        src = source;
        enableUnstableRustdoc = true;
        rustdocFlags = shell.RUSTDOCFLAGS;
        cargoDocFlags = [ "--no-deps" "--workspace" ];
        postBuild = ''
          cargo doc --frozen \
            ''${cargoDocFeaturesFlag} \
            ''${cargoDocFlags} \
            --examples --document-private-items
        '';

        inherit (wpexec)
          buildInputs nativeBuildInputs
          LIBCLANG_PATH BINDGEN_EXTRA_CLANG_ARGS;
      };

      wptest = { callPackage }: (import ./examples { inherit callPackage; }).wptest;

      wpdev-commitlintrc = { writeText, commitlint, nodePackages }: writeText "wireplumber-rust.commitlintrc.json" (builtins.toJSON
        (self.lib.commitlint.commitlintrc // {
          extends = [ "${nodePackages."@commitlint/config-conventional"}/lib/node_modules/@commitlint/config-conventional/." ];
        })
      );
      wpdev-commitlint = { writeShellScriptBin, commitlint, wpdev-commitlintrc }: writeShellScriptBin "commitlint" ''
        exec ${commitlint}/bin/commitlint --config ${wpdev-commitlintrc} "$@"
      '';
      wpdev-todo = { writeShellScriptBin, wpdev-gir }: writeShellScriptBin "todo" ''
        cd ${toString ./.}
        exec ${nixlib.getExe wpdev-gir} -m not_bound
      '';
      wpdev-fmt = { writeShellScriptBin }: writeShellScriptBin "wpfmt" ''
        cargo fmt -p wireplumber -p wp-examples
      '';
      wpdev-generate = {
        rust'builders
      , wpdev-readme, wpdev-sys-readme
      , wpdev-commitlint-help
      , outputHashes
      }: rust'builders.generateFiles {
        name = "readmes";
        paths = {
          "src/README.md" = wpdev-readme;
          "sys/src/README.md" = wpdev-sys-readme;
          ".github/commitlint.adoc" = wpdev-commitlint-help;
          "lock.nix" = outputHashes;
        };
      };
      wpdev-readme = { rust'builders }: rust'builders.adoc2md {
        src = ./README.adoc;
        attributes = let
          inherit (self.lib.crate.package) repository;
        in rec {
          release = self.lib.releaseTag;
          relative-tree = "${repository}/tree/${release}/";
          relative-blob = "${repository}/blob/${release}/";
        };
      };
      wpdev-sys-readme = { rust'builders, wpdev-readme }: rust'builders.adoc2md {
        src = ./sys/README.adoc;
        inherit (wpdev-readme) attributes;
      };
      wpdev-commitlint-help = { writeText }: writeText "commitlint.adoc" self.lib.commitlint.help-adoc;
      outputHashes = { rust'builders }: rust'builders.cargoOutputHashes {
        inherit (self.lib) crate;
      };
    } { };
    lib = with nixlib; {
      featureVersions = [
        "0.4.3" "0.4.5"
        "0.4.6"
        "0.4.8" "0.4.10"
        "0.4.11" "0.4.12"
      ];
      supportedVersions = version: filter (versionAtLeast version) self.lib.featureVersions;
      versionFeatureName = version: "v" + replaceStrings [ "." ] [ "_" ] version;
      featureForVersion = version: let
        features = self.lib.supportedVersions version;
      in if features == [ ] then null else self.lib.versionFeatureName (last features);
      crate = rust.lib.importCargo {
        path = ./Cargo.toml;
        inherit (import ./lock.nix) outputHashes;
      };
      inherit (self.lib.crate.package) version;
      inherit (self.lib.crate.package.metadata) branches;
      owner = "arcnmx";
      repo = "wireplumber.rs";
      pagesRoot = rust.lib.ghPages {
        inherit (self.lib) owner repo;
      };
      releaseTag = "v${self.lib.version}";

      commitlint = import ./ci/commitlint.nix {
        inherit self;
        lib = nixlib;
      };
    };
    config = rec {
      name = "wireplumber-rust";
      packages.namespace = [ name ];
    };
  };
}
