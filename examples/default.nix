{ callPackage ? pkgs.callPackage, pkgs ? import <nixpkgs> { } }: let
  wpconfig = { wireplumber, linkFarm }: let
    wp = "${wireplumber}/share/wireplumber";
    cnf = path: {
      name = path;
      path = "${wp}/${path}";
    };
  in linkFarm "wireplumber.conf" [
    (cnf "main.conf")
    (cnf "common")
    (cnf "scripts")
    (cnf "main.lua.d/00-functions.lua")
    (cnf "main.lua.d/20-default-access.lua")
    (cnf "main.lua.d/40-device-defaults.lua")
    (cnf "main.lua.d/40-stream-defaults.lua")
    (cnf "main.lua.d/50-default-access-config.lua")
    (cnf "main.lua.d/90-enable-all.lua")
  ];
  pwconfig = { writeText }: writeText "pipewire.conf" (builtins.toJSON {
    "context.exec" = [];
    "context.modules" = [
      { name = "libpipewire-module-protocol-native"; }
      { name = "libpipewire-module-metadata"; }
      { name = "libpipewire-module-spa-device-factory"; }
      { name = "libpipewire-module-spa-node-factory"; }
      { name = "libpipewire-module-client-node"; }
      { name = "libpipewire-module-client-device"; }
      { name = "libpipewire-module-adapter"; }
      { name = "libpipewire-module-link-factory"; }
      { name = "libpipewire-module-session-manager"; }
    ];
    "context.objects" = [
      {
        factory = "adapter";
        args = {
          "audio.channels" = 1;
          "audio.position" = "MONO";
          "factory.name" = "support.null-audio-sink";
          "media.class" = "Audio/Source/Virtual";
          "node.name" = "source"; # TODO: or node.nick?
        };
      }
      {
        factory = "adapter";
        args = {
          "audio.channels" = 2;
          "audio.position" = "FL,FR";
          "factory.name" = "support.null-audio-sink";
          "media.class" = "Audio/Sink";
          "node.name" = "stream";
        };
      }
      {
        factory = "spa-node-factory";
        args = {
          "factory.name" = "support.node.driver";
          "node.name" = "Dummy-Driver";
        };
      }
    ];
    "context.properties" = {
      "core.daemon" = true;
      "log.level" = 3;
      "support.dbus" = false;
    };
    "context.spa-libs" = {
      "support.*" = "support/libspa-support";
    };
  });
  wptest = { runCommand, writeText, wpexec, wireplumber, pipewire, wpconfig, pwconfig, lib }: let
  in name: cmd: runCommand "wptest-${name}" {
    inherit wpconfig pwconfig;
  } ''
    export PIPEWIRE_RUNTIME_DIR=$PWD/.pw
    export PIPEWIRE_CORE=core
    export PIPEWIRE_REMOTE=$PIPEWIRE_RUNTIME_DIR/$PIPEWIRE_CORE
    export XDG_STATE_HOME=$PWD/.local

    mkdir $PIPEWIRE_RUNTIME_DIR
    ${pipewire}/bin/pipewire -c $pwconfig &
    PW_PID=$!
    sleep 1
    ${wireplumber}/bin/wireplumber -c $wpconfig/main.conf &
    WP_PID=$!
    sleep 1

    ${cmd}
  '';
  out = {
    wpconfig = callPackage wpconfig { };
    pwconfig = callPackage pwconfig { };
    wptest = callPackage wptest {
      inherit (out) wpconfig pwconfig;
    };
  };
in out
