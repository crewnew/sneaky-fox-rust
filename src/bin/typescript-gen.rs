use std::io::prelude::*;
use track_pc_usage_rs as trbtt;
use trbtt::capture::*;
use trbtt::extract;
use trbtt::prelude::*;
use trbtt::sampler::Sampler;
use typescript_definitions::TypeScriptifyTrait;

const FS: &'static [fn() -> std::borrow::Cow<'static, str>] = &[
    // DbEvent::type_script_ify,
    Sampler::type_script_ify,
    EventData::type_script_ify,
    x11::X11EventData::type_script_ify,
    x11::X11WindowData::type_script_ify,
    x11::X11WindowGeometry::type_script_ify,
    x11::ProcessData::type_script_ify,
    util::OsInfo::type_script_ify,
    extract::properties::ExtractedInfo::type_script_ify,
    extract::properties::SoftwareDeviceType::type_script_ify,
    extract::properties::Identifier::type_script_ify,
    GeneralSoftware::type_script_ify,
    SpecificSoftware::type_script_ify,
    MediaType::type_script_ify,
];

// const all_types: Vec<
fn main() -> anyhow::Result<()> {
    let mut ofile = std::fs::File::create("frontend/src/server.d.ts")?;
    writeln!(ofile, "type DateTime<T> = string;")?;
    writeln!(ofile, "type Local = unknown;")?;
    writeln!(ofile, "type Timestamptz = string;")?;
    for i in &[10, 100, 1000, 10000, 100000] {
        writeln!(ofile, "type Text{} = string;", i)?;
    }
    if cfg!(any(debug_assertions, feature = "export-typescript")) {
        for f in FS {
            writeln!(ofile, "{}", f())?;
        }
    } else {
        println!("NOT IN DEBUG MODE, will not work!")
    }
    Ok(())
}
