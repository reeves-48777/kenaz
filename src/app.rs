use crate::{schema, util};
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

pub struct App {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub out: PathBuf,
    groups: HashMap<String, Vec<String>>,
}

impl App {
    pub fn new(src: PathBuf, dst: PathBuf, out: Option<PathBuf>) -> Self {
        let out = out.unwrap_or_else(|| dst.clone());

        Self {
            src,
            dst,
            out,
            groups: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        // valid source theme made by zed theme, that will contain all or most used values
        let source_spec = util::parse_file(&self.src)?;
        // theme to fix
        let mut dest_spec = util::parse_file(&self.dst)?;

        self.build_groups(source_spec)?;
        self.fix_theme_spec(&mut dest_spec)?;
        self.save_file(&dest_spec)?;

        Ok(())
    }

    fn build_groups(&mut self, spec: schema::ZedThemeSpec) -> anyhow::Result<()> {
        for theme in spec.themes {
            // fancy log print
            tracing::info!(
                "Dumping color value for {} - {:?}",
                theme.name,
                theme.appearance
            );

            let style = serde_json::to_value(&theme.style)?;

            if let Value::Object(fields) = style {
                for (name, value) in fields {
                    if let Value::String(color) = value {
                        self.groups.entry(color).or_default().push(name);
                    }
                }
            } else {
                anyhow::bail!("Couldn't construct groups...");
            }
        }

        Ok(())
    }

    fn fix_theme_spec(&self, spec: &mut schema::ZedThemeSpec) -> anyhow::Result<()> {
        for theme in spec.themes.iter_mut() {
            let span = tracing::info_span!("Fixing theme", name = %theme.name, variant = %theme.appearance);
            let _enter = span.enter();

            let mut processed: HashSet<String> = HashSet::new();

            let mut style = serde_json::to_value(&theme.style)?;

            for (_, tokens) in &self.groups {
                // NOTE alone tokens cannot be mapped
                // in one case they already have a color assigned in our theme
                // on the other hand, they don't, so we cannot assign one without knowing what the user wants
                if tokens.len() < 2 {
                    // we can get the first token since the len is 1
                    let token = tokens.first().unwrap();
                    // if it is not set in the style we don't have to bother ourselves with it
                    if style.get(token).is_none() {
                        continue;
                    }
                }

                // get tokens in group to retreive their assigned color in the theme to fix
                let anchor = tokens.iter().find_map(|token| {
                    style
                        .get(token)
                        .and_then(|v| v.as_str())
                        .map(|v| v.to_string())
                });

                for token in tokens {
                    if style.get(token).is_none() || processed.contains(token) {
                        continue;
                    }

                    if let Some(color) = anchor.as_deref() {
                        style[token] = Value::String(color.to_string());
                        processed.insert(token.clone());
                    }
                }
            }

            theme.style = serde_json::from_value(style)?;
        }
        Ok(())
    }

    fn save_file(&self, spec: &schema::ZedThemeSpec) -> anyhow::Result<()> {
        if self.dst == self.out {
            let mut backup_path = self.out.clone();
            let file_name = backup_path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("path without file name"))?
                .to_string_lossy()
                .to_string();

            backup_path.set_file_name(format!("{file_name}.bak"));
            std::fs::copy(&self.out, &backup_path)?;
            tracing::info!("Backup created at: {:?}", &backup_path);
        }
        let out_json = serde_json::to_string_pretty(spec)?;
        std::fs::write(&self.out, out_json)?;
        tracing::info!("Theme file fixed and updated at: {:?}", self.out);

        Ok(())
    }
}
