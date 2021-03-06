use std::path::{Path, PathBuf};

use colored::*;
use convert_case::{Case, Casing};

mod scoop;
use scoop::ScoopJson;

#[derive(Debug, Clone)]
pub struct Manager {
    pub bin: Option<String>,
    pub channel: String,
    pub features: Option<String>,
    pub name: String,
    pub description: String,
    pub homepage: String,
    pub repository: String,
    pub license: String,
    pub homebrew_tap_path: PathBuf,
}

impl Manager {
    pub fn write_homebrewtap_workflows_update_formula(&self) -> anyhow::Result<()> {
        let tap_github_workflows_update_formula_fpath = {
            let path = self
                .homebrew_tap_path
                .join(".github/workflows/update-formula-cargodist.yml");
            std::fs::create_dir_all(&path.parent().unwrap())?;
            path
        };
        let tap_github_workflows_update_formula_contnet =
            include_str!("templates/update-formula-cargodist.yml");

        std::fs::write(
            &tap_github_workflows_update_formula_fpath,
            tap_github_workflows_update_formula_contnet,
        );

        println!(
            "✔️ {} was written",
            tap_github_workflows_update_formula_fpath
                .into_os_string()
                .into_string()
                .unwrap()
                .green()
        );
        Ok(())
    }

    pub fn write_homebrewtap_templates_formula(&self) -> anyhow::Result<()> {
        let context = {
            let mut context = tera::Context::new();
            context.insert("name", &self.name);
            context.insert("name_pascal_case", &self.name.to_case(Case::Pascal));
            context.insert("description", &self.description);
            context.insert("homepage", &self.homepage);
            context.insert("repository", &self.repository);
            context.insert("repository_url", &self.repository.trim_end_matches(".git"));
            context
        };

        match tera::Tera::one_off(include_str!("templates/formula.rb"), &context, false) {
            Ok(formula_content) => {
                let tap_templates_formula_fpath = {
                    let path = self
                        .homebrew_tap_path
                        .join("templates")
                        .join(format!("{}.rb", &self.name));
                    std::fs::create_dir_all(&path.parent().unwrap())?;
                    path
                };
                std::fs::write(&tap_templates_formula_fpath, formula_content);

                println!(
                    "✔️ {} was written",
                    tap_templates_formula_fpath
                        .into_os_string()
                        .into_string()
                        .unwrap()
                        .green()
                );
            }
            Err(err) => eprintln!("{}", err),
        }
        Ok(())
    }

    pub fn write_scoop_bucket(&self) -> anyhow::Result<()> {
        let scoop = ScoopJson::from(self.to_owned());
        let content = serde_json::to_string_pretty(&scoop)?;
        let fpath = {
            let path = self
                .homebrew_tap_path
                .join("bucket")
                .join(format!("{}.json", &self.name));
            std::fs::create_dir_all(&path.parent().unwrap())?;
            path
        };
        std::fs::write(&fpath, &content)?;
        println!(
            "✔️ {} was written",
            fpath.into_os_string().into_string().unwrap().green()
        );

        Ok(())
    }

    pub fn write_project_templates_formula(&self) -> anyhow::Result<()> {
        let github_workflows_release_content = {
            let bin_option = self
                .bin
                .to_owned()
                .map(|bin_name| format!("--bin {}", &bin_name))
                .unwrap_or_default();
            let features_option = self
                .features
                .to_owned()
                .map(|bin_name| format!(r#"--features "{}""#, &bin_name))
                .unwrap_or_default();
            let release_template = include_str!("templates/release.yml");
            release_template
                .replace("{% channel %}", &self.channel)
                .replace("{% name %}", &self.name)
                .replace("{% bin_option %}", &bin_option)
                .replace("{% features_option %}", &features_option)
        };

        let github_workflows_release_fpath = {
            let s = format!(".github/workflows/release-{}-cargodist.yml", &self.name);
            let path = std::env::current_dir()?.join(s);
            std::fs::create_dir_all(&path.parent().unwrap())?;
            path
        };

        std::fs::write(
            &github_workflows_release_fpath,
            github_workflows_release_content,
        );

        println!(
            "✔️ {} was written",
            github_workflows_release_fpath
                .to_owned()
                .into_os_string()
                .into_string()
                .unwrap()
                .green()
        );
        Ok(())
    }
}
