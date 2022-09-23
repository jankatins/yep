use self_update;
use clap::{Arg, ArgAction, Command};


fn main() {
    let matches = Command::new("yep")
        .about("yet another (homedir) package manager")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("yep development team")
        .subcommand(
            Command::new("install")
                .about("Installs packages.")
                .allow_hyphen_values(true)
                .trailing_var_arg(true)
                .arg(
                    Arg::new("name")
                        .help("package name (github slug (default), url,...)")
                        .required(true)
                        .action(ArgAction::StoreValue),
                )
                .arg(
                    Arg::new("recipe")
                        .help("recipe to use for package")
                        .required(false)
                        .action(ArgAction::Set)
                        .takes_value(true)
                        .multiple_values(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("install", install_matches)) => {
            let package_name = install_matches
                .get_one::<String>("name")
                .expect("Need at least a package name (default: github slug, like jankatins/yep)")
                ;
            println!("Name: {}", package_name);
            let recipe: Vec<_> = install_matches
                .get_many::<String>("recipe")
                .unwrap_or_default()
                .map(|s| s.as_str())
                .collect();
            let values = recipe.join(" $#$ ");
            println!("Recipe: {}", values);
            download_github(package_name);
            return;
        }

        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}

fn download_github(slug: &String) -> Result<(), Box<dyn (::std::error::Error)>> {
    let mut parts = slug.split("/");
    let owner = parts.next().expect("Need a github slug in the form 'owner/repo'");
    let repo = parts.next().expect("Need a github slug in the form 'owner/repo'");
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner(owner)
        .repo_name(repo)
        .build()
        .expect("Bad")
        .fetch().expect("More bad things");
    let values = releases
        .iter()
        .map(|r| format!("{}, {}", r.version.as_str() , r.date.as_str() ))
        .collect::<Vec<String>>()
        .join("\n");
    println!("Releases: {}", values);
    Ok(())
}