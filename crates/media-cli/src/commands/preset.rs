use media_core::preset::builtin::builtin_presets;

use crate::args::{PresetAction, PresetArgs};

pub async fn run(args: PresetArgs) -> Result<(), media_core::MediaError> {
    match args.action {
        PresetAction::List => {
            let presets = builtin_presets();
            let header = format!("{:<25} {:<10} DESCRIPTION", "NAME", "TYPE");
            println!("{}", header);
            println!("{}", "-".repeat(70));
            for p in presets {
                let type_str = match p.preset_type {
                    media_core::preset::preset::PresetType::Video => "video",
                    media_core::preset::preset::PresetType::Audio => "audio",
                    media_core::preset::preset::PresetType::Image => "image",
                };
                println!("{:<25} {:<10} {}", p.name, type_str, p.description);
            }
        }
        PresetAction::Show { name } => {
            let presets = builtin_presets();
            let preset = presets.iter().find(|p| p.name.eq_ignore_ascii_case(&name));

            match preset {
                Some(p) => {
                    let json = serde_json::to_string_pretty(p)?;
                    println!("{}", json);
                }
                None => {
                    eprintln!("Preset not found: {}", name);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
