// This file was generated by gir (https://github.com/gtk-rs/gir)
// DO NOT EDIT

use wp_sys::*;
use std::mem::{align_of, size_of};
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["wireplumber-0.4"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG")
        .unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}",
                           &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
#[cfg(target_os = "linux")]
#[cfg(feature = "v0_4_6")]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let value = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse value");
        c_constants.push((name, value));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_value, &c_value
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
#[cfg(target_os = "linux")]
#[cfg(feature = "v0_4_6")]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let size = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse size");
        let alignment = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse alignment");
        c_layouts.push((name, Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in
        RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_layout, &c_layout
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    ("WpClientClass", Layout {size: size_of::<WpClientClass>(), alignment: align_of::<WpClientClass>()}),
    ("WpComponentLoader", Layout {size: size_of::<WpComponentLoader>(), alignment: align_of::<WpComponentLoader>()}),
    ("WpComponentLoaderClass", Layout {size: size_of::<WpComponentLoaderClass>(), alignment: align_of::<WpComponentLoaderClass>()}),
    ("WpConstraintType", Layout {size: size_of::<WpConstraintType>(), alignment: align_of::<WpConstraintType>()}),
    ("WpConstraintVerb", Layout {size: size_of::<WpConstraintVerb>(), alignment: align_of::<WpConstraintVerb>()}),
    ("WpCoreClass", Layout {size: size_of::<WpCoreClass>(), alignment: align_of::<WpCoreClass>()}),
    ("WpDeviceClass", Layout {size: size_of::<WpDeviceClass>(), alignment: align_of::<WpDeviceClass>()}),
    ("WpDirection", Layout {size: size_of::<WpDirection>(), alignment: align_of::<WpDirection>()}),
    ("WpEndpoint", Layout {size: size_of::<WpEndpoint>(), alignment: align_of::<WpEndpoint>()}),
    ("WpEndpointClass", Layout {size: size_of::<WpEndpointClass>(), alignment: align_of::<WpEndpointClass>()}),
    ("WpFactoryClass", Layout {size: size_of::<WpFactoryClass>(), alignment: align_of::<WpFactoryClass>()}),
    ("WpFeatureActivationTransitionClass", Layout {size: size_of::<WpFeatureActivationTransitionClass>(), alignment: align_of::<WpFeatureActivationTransitionClass>()}),
    ("WpGlobalProxy", Layout {size: size_of::<WpGlobalProxy>(), alignment: align_of::<WpGlobalProxy>()}),
    ("WpGlobalProxyClass", Layout {size: size_of::<WpGlobalProxyClass>(), alignment: align_of::<WpGlobalProxyClass>()}),
    ("WpImplEndpointClass", Layout {size: size_of::<WpImplEndpointClass>(), alignment: align_of::<WpImplEndpointClass>()}),
    ("WpImplMetadataClass", Layout {size: size_of::<WpImplMetadataClass>(), alignment: align_of::<WpImplMetadataClass>()}),
    ("WpImplModuleClass", Layout {size: size_of::<WpImplModuleClass>(), alignment: align_of::<WpImplModuleClass>()}),
    ("WpImplNodeClass", Layout {size: size_of::<WpImplNodeClass>(), alignment: align_of::<WpImplNodeClass>()}),
    ("WpInitFlags", Layout {size: size_of::<WpInitFlags>(), alignment: align_of::<WpInitFlags>()}),
    ("WpInterestMatch", Layout {size: size_of::<WpInterestMatch>(), alignment: align_of::<WpInterestMatch>()}),
    ("WpInterestMatchFlags", Layout {size: size_of::<WpInterestMatchFlags>(), alignment: align_of::<WpInterestMatchFlags>()}),
    ("WpIteratorMethods", Layout {size: size_of::<WpIteratorMethods>(), alignment: align_of::<WpIteratorMethods>()}),
    ("WpLibraryErrorEnum", Layout {size: size_of::<WpLibraryErrorEnum>(), alignment: align_of::<WpLibraryErrorEnum>()}),
    ("WpLinkClass", Layout {size: size_of::<WpLinkClass>(), alignment: align_of::<WpLinkClass>()}),
    ("WpLookupDirs", Layout {size: size_of::<WpLookupDirs>(), alignment: align_of::<WpLookupDirs>()}),
    ("WpMetadata", Layout {size: size_of::<WpMetadata>(), alignment: align_of::<WpMetadata>()}),
    ("WpMetadataClass", Layout {size: size_of::<WpMetadataClass>(), alignment: align_of::<WpMetadataClass>()}),
    ("WpMetadataFeatures", Layout {size: size_of::<WpMetadataFeatures>(), alignment: align_of::<WpMetadataFeatures>()}),
    ("WpNodeClass", Layout {size: size_of::<WpNodeClass>(), alignment: align_of::<WpNodeClass>()}),
    ("WpNodeFeatures", Layout {size: size_of::<WpNodeFeatures>(), alignment: align_of::<WpNodeFeatures>()}),
    ("WpNodeState", Layout {size: size_of::<WpNodeState>(), alignment: align_of::<WpNodeState>()}),
    ("WpObject", Layout {size: size_of::<WpObject>(), alignment: align_of::<WpObject>()}),
    ("WpObjectClass", Layout {size: size_of::<WpObjectClass>(), alignment: align_of::<WpObjectClass>()}),
    ("WpObjectFeatures", Layout {size: size_of::<WpObjectFeatures>(), alignment: align_of::<WpObjectFeatures>()}),
    ("WpObjectManagerClass", Layout {size: size_of::<WpObjectManagerClass>(), alignment: align_of::<WpObjectManagerClass>()}),
    ("WpPipewireObjectInterface", Layout {size: size_of::<WpPipewireObjectInterface>(), alignment: align_of::<WpPipewireObjectInterface>()}),
    ("WpPlugin", Layout {size: size_of::<WpPlugin>(), alignment: align_of::<WpPlugin>()}),
    ("WpPluginClass", Layout {size: size_of::<WpPluginClass>(), alignment: align_of::<WpPluginClass>()}),
    ("WpPluginFeatures", Layout {size: size_of::<WpPluginFeatures>(), alignment: align_of::<WpPluginFeatures>()}),
    ("WpPortClass", Layout {size: size_of::<WpPortClass>(), alignment: align_of::<WpPortClass>()}),
    ("WpProxy", Layout {size: size_of::<WpProxy>(), alignment: align_of::<WpProxy>()}),
    ("WpProxyClass", Layout {size: size_of::<WpProxyClass>(), alignment: align_of::<WpProxyClass>()}),
    ("WpProxyFeatures", Layout {size: size_of::<WpProxyFeatures>(), alignment: align_of::<WpProxyFeatures>()}),
    ("WpSessionItem", Layout {size: size_of::<WpSessionItem>(), alignment: align_of::<WpSessionItem>()}),
    ("WpSessionItemClass", Layout {size: size_of::<WpSessionItemClass>(), alignment: align_of::<WpSessionItemClass>()}),
    ("WpSessionItemFeatures", Layout {size: size_of::<WpSessionItemFeatures>(), alignment: align_of::<WpSessionItemFeatures>()}),
    ("WpSiAcquisitionInterface", Layout {size: size_of::<WpSiAcquisitionInterface>(), alignment: align_of::<WpSiAcquisitionInterface>()}),
    ("WpSiAdapterInterface", Layout {size: size_of::<WpSiAdapterInterface>(), alignment: align_of::<WpSiAdapterInterface>()}),
    ("WpSiAdapterPortsState", Layout {size: size_of::<WpSiAdapterPortsState>(), alignment: align_of::<WpSiAdapterPortsState>()}),
    ("WpSiEndpointInterface", Layout {size: size_of::<WpSiEndpointInterface>(), alignment: align_of::<WpSiEndpointInterface>()}),
    ("WpSiFactory", Layout {size: size_of::<WpSiFactory>(), alignment: align_of::<WpSiFactory>()}),
    ("WpSiFactoryClass", Layout {size: size_of::<WpSiFactoryClass>(), alignment: align_of::<WpSiFactoryClass>()}),
    ("WpSiLinkInterface", Layout {size: size_of::<WpSiLinkInterface>(), alignment: align_of::<WpSiLinkInterface>()}),
    ("WpSiLinkableInterface", Layout {size: size_of::<WpSiLinkableInterface>(), alignment: align_of::<WpSiLinkableInterface>()}),
    ("WpSpaDeviceClass", Layout {size: size_of::<WpSpaDeviceClass>(), alignment: align_of::<WpSpaDeviceClass>()}),
    ("WpSpaDeviceFeatures", Layout {size: size_of::<WpSpaDeviceFeatures>(), alignment: align_of::<WpSpaDeviceFeatures>()}),
    ("WpSpaIdTable", Layout {size: size_of::<WpSpaIdTable>(), alignment: align_of::<WpSpaIdTable>()}),
    ("WpSpaIdValue", Layout {size: size_of::<WpSpaIdValue>(), alignment: align_of::<WpSpaIdValue>()}),
    ("WpStateClass", Layout {size: size_of::<WpStateClass>(), alignment: align_of::<WpStateClass>()}),
    ("WpTransition", Layout {size: size_of::<WpTransition>(), alignment: align_of::<WpTransition>()}),
    ("WpTransitionClass", Layout {size: size_of::<WpTransitionClass>(), alignment: align_of::<WpTransitionClass>()}),
    ("WpTransitionStep", Layout {size: size_of::<WpTransitionStep>(), alignment: align_of::<WpTransitionStep>()}),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) WP_CONSTRAINT_TYPE_G_PROPERTY", "3"),
    ("(gint) WP_CONSTRAINT_TYPE_NONE", "0"),
    ("(gint) WP_CONSTRAINT_TYPE_PW_GLOBAL_PROPERTY", "1"),
    ("(gint) WP_CONSTRAINT_TYPE_PW_PROPERTY", "2"),
    ("(gint) WP_CONSTRAINT_VERB_EQUALS", "61"),
    ("(gint) WP_CONSTRAINT_VERB_IN_LIST", "99"),
    ("(gint) WP_CONSTRAINT_VERB_IN_RANGE", "126"),
    ("(gint) WP_CONSTRAINT_VERB_IS_ABSENT", "45"),
    ("(gint) WP_CONSTRAINT_VERB_IS_PRESENT", "43"),
    ("(gint) WP_CONSTRAINT_VERB_MATCHES", "35"),
    ("(gint) WP_CONSTRAINT_VERB_NOT_EQUALS", "33"),
    ("(gint) WP_DIRECTION_INPUT", "0"),
    ("(gint) WP_DIRECTION_OUTPUT", "1"),
    ("(guint) WP_INIT_ALL", "15"),
    ("(guint) WP_INIT_PIPEWIRE", "1"),
    ("(guint) WP_INIT_SET_GLIB_LOG", "8"),
    ("(guint) WP_INIT_SET_PW_LOG", "4"),
    ("(guint) WP_INIT_SPA_TYPES", "2"),
    ("WP_INTEREST_MATCH_ALL", "15"),
    ("(guint) WP_INTEREST_MATCH_FLAGS_CHECK_ALL", "1"),
    ("(guint) WP_INTEREST_MATCH_FLAGS_NONE", "0"),
    ("(guint) WP_INTEREST_MATCH_GTYPE", "1"),
    ("(guint) WP_INTEREST_MATCH_G_PROPERTIES", "8"),
    ("(guint) WP_INTEREST_MATCH_NONE", "0"),
    ("(guint) WP_INTEREST_MATCH_PW_GLOBAL_PROPERTIES", "2"),
    ("(guint) WP_INTEREST_MATCH_PW_PROPERTIES", "4"),
    ("WP_ITERATOR_METHODS_VERSION", "0"),
    ("(gint) WP_LIBRARY_ERROR_INVALID_ARGUMENT", "1"),
    ("(gint) WP_LIBRARY_ERROR_INVARIANT", "0"),
    ("(gint) WP_LIBRARY_ERROR_OPERATION_FAILED", "2"),
    ("WP_LOG_LEVEL_TRACE", "256"),
    ("(guint) WP_LOOKUP_DIR_ENV_CONFIG", "1"),
    ("(guint) WP_LOOKUP_DIR_ENV_DATA", "2"),
    ("(guint) WP_LOOKUP_DIR_ETC", "2048"),
    ("(guint) WP_LOOKUP_DIR_PREFIX_SHARE", "4096"),
    ("(guint) WP_LOOKUP_DIR_XDG_CONFIG_HOME", "1024"),
    ("(guint) WP_METADATA_FEATURE_DATA", "65536"),
    ("(guint) WP_NODE_FEATURE_PORTS", "65536"),
    ("(gint) WP_NODE_STATE_CREATING", "0"),
    ("(gint) WP_NODE_STATE_ERROR", "-1"),
    ("(gint) WP_NODE_STATE_IDLE", "2"),
    ("(gint) WP_NODE_STATE_RUNNING", "3"),
    ("(gint) WP_NODE_STATE_SUSPENDED", "1"),
    ("WP_OBJECT_FEATURES_ALL", "4294967295"),
    ("WP_OBJECT_FORMAT", "<%s:%p>"),
    ("WP_PIPEWIRE_OBJECT_FEATURES_ALL", "1009"),
    ("WP_PIPEWIRE_OBJECT_FEATURES_MINIMAL", "17"),
    ("(guint) WP_PIPEWIRE_OBJECT_FEATURE_INFO", "16"),
    ("(guint) WP_PIPEWIRE_OBJECT_FEATURE_PARAM_FORMAT", "64"),
    ("(guint) WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PORT_CONFIG", "256"),
    ("(guint) WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PROFILE", "128"),
    ("(guint) WP_PIPEWIRE_OBJECT_FEATURE_PARAM_PROPS", "32"),
    ("(guint) WP_PIPEWIRE_OBJECT_FEATURE_PARAM_ROUTE", "512"),
    ("(guint) WP_PLUGIN_FEATURE_ENABLED", "1"),
    ("(guint) WP_PROXY_FEATURE_BOUND", "1"),
    ("(guint) WP_SESSION_ITEM_FEATURE_ACTIVE", "1"),
    ("(guint) WP_SESSION_ITEM_FEATURE_EXPORTED", "2"),
    ("(gint) WP_SI_ADAPTER_PORTS_STATE_CONFIGURED", "2"),
    ("(gint) WP_SI_ADAPTER_PORTS_STATE_CONFIGURING", "1"),
    ("(gint) WP_SI_ADAPTER_PORTS_STATE_NONE", "0"),
    ("(guint) WP_SPA_DEVICE_FEATURE_ENABLED", "65536"),
    ("WP_SPA_TYPE_INVALID", "4294967295"),
    ("(gint) WP_TRANSITION_STEP_CUSTOM_START", "16"),
    ("(gint) WP_TRANSITION_STEP_ERROR", "1"),
    ("(gint) WP_TRANSITION_STEP_NONE", "0"),
];

