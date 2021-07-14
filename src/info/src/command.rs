/*
    Copyright 2021 Volt Contributors
    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at
        http://www.apache.org/licenses/LICENSE-2.0
    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

//! Display info about a package.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use bat::PrettyPrinter;
use colored::Colorize;
use utils::{
    app::App,
    package::{Package, Version},
};
use volt_core::{command::Command, model::http_manager::get_package, VERSION};

pub struct Info {}

#[async_trait]
impl Command for Info {
    fn help() -> String {
        format!(
            r#"volt {}
    
Shows the information of a package 

Usage: {} {} {}

Options: 

  {} {} Output verbose messages on internal operations."#,
            VERSION.bright_green().bold(),
            "volt".bright_green().bold(),
            "deploy".bright_purple(),
            "[commit]".white(),
            "--verbose".bright_blue(),
            "(-v)".yellow()
        )
    }

    /// Execute the `volt info` command
    ///
    /// Display info about a package
    /// ## Arguments
    /// * `error` - Instance of the command (`Arc<App>`)
    /// ## Examples
    /// ```
    /// // Display info about a package
    /// // .exec() is an async call so you need to await it
    /// Info.exec(app).await;
    /// ```
    /// ## Returns
    /// * `Result<()>`
    async fn exec(app: Arc<App>) -> Result<()> {
        #[allow(unused_assignments)]
        let mut name = String::new();

        if !std::env::current_dir()?.join("package.json").exists() && app.args.len() == 1 {
            println!(
                "{}: {}\n",
                "warning".yellow().bold(),
                "Could not find a package.json file in the current directory"
            );
            name = utils::get_basename(app.current_dir.to_str().unwrap()).to_string();
        }

        let mut field: String = String::new();

        if app.args.len() > 2 {
            field = String::from(&app.args[2]);
            name = String::from(&app.args[1]);
        } else if app.args.len() == 2 {
            name = String::from(&app.args[1]);
        }

        let package: Package = get_package(&name).await?.unwrap();

        if field == String::new() {
            let latest_version = package.dist_tags.latest;
            println!("{}\n", format!("v{}", latest_version).bright_blue());

            if package.description != None {
                println!("{}\n", package.description.unwrap());
            }
            if package.keywords != None {
                print!("{}: ", "keywords".bright_blue().bold());
                for keyword in package.keywords.unwrap().iter() {
                    print!("{} ", keyword.green())
                }
                print!("\n\n")
            }

            let latestpackage: &Version = &package.versions[&latest_version];
            println!("distribution:");
            println!(
                "  tarball: {}",
                latestpackage.dist.tarball.bright_blue().underline()
            );
            println!("  shasum: {}", latestpackage.dist.shasum.bright_green());
            if latestpackage.dist.integrity != "" {
                println!(
                    "  integrity: {}",
                    latestpackage.dist.integrity.bright_blue()
                );
            }
            if latestpackage.dist.unpacked_size != 0 {
                println!(
                    "  unpackedSize: {}{}",
                    (latestpackage.dist.unpacked_size / 1024)
                        .to_string()
                        .bright_blue()
                        .bold(),
                    "kb".bright_blue().bold()
                );
            }

            let dependencies = latestpackage
                .dependencies
                .keys()
                .cloned()
                .collect::<Vec<String>>();

            if dependencies.len() != 0 {
                println!("\ndependencies:");
                for dep in dependencies.iter() {
                    println!("{}{}", "  - ".bright_magenta(), dep);
                }
            }

            println!("{}", "\nmaintainers:");
            for maintainer in latestpackage.maintainers.iter() {
                println!(
                    "  {} {}<{}>",
                    "-".bright_magenta(),
                    maintainer.email,
                    maintainer.name.yellow().bold()
                )
            }
            print!("\n");
        } else {
            match field.as_str() {
                "readme" => {
                    let text: String;

                    if package.readme.is_some() && package.readme.as_ref().unwrap().trim() != "" {
                        text = package.readme.unwrap();
                    } else {
                        let latest_version = package.dist_tags.latest;
                        let current_version = package.versions.get(&latest_version).unwrap();

                        if current_version.readme.is_some() {
                            text = current_version.readme.as_ref().unwrap().to_string();
                        } else {
                            text = String::new();
                        }
                    }
                    if text != String::new() {
                        PrettyPrinter::new()
                            .input_from_bytes(text.as_bytes())
                            .theme("Dracula")
                            .language("markdown")
                            .print()
                            .unwrap();
                    } else {
                        println!(
                            "{}: {}",
                            "error".bright_red(),
                            format!("could not find a readme for {}", name)
                        )
                    }
                }
                &_ => {}
            }
        }

        Ok(())
    }
}
