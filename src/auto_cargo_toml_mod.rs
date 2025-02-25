// auto_cargo_toml_mod

//! functions to get data from Cargo.toml

use lazy_static::lazy_static;
use regex::*;

lazy_static! {
    /// remove email from author
    static ref REGEX_REMOVE_EMAIL: Regex = Regex::new(r#"( <.+?>)"#).unwrap();
}

/// Read data from Cargo.toml
pub struct CargoToml {
    /// the first Cargo.toml is maybe a workspace. It has a different structure.
    cargo_toml_workspace_maybe: cargo_toml::Manifest,
    /// the main Cargo.toml is different for single project or for workspace
    /// for workspace is the first `main` member
    _cargo_toml_main: cargo_toml::Manifest,
    /// the package is read from the main Cargo.toml
    package: cargo_toml::Package,
}

impl crate::public_api_mod::CargoTomlPublicApiMethods for CargoToml {
    /// read Cargo.toml, for workspaces it is the Cargo.toml of the first member
    fn read() -> Self {
        let absolute_path = std::path::absolute("Cargo.toml").unwrap();
        let cargo_toml_workspace_maybe = cargo_toml::Manifest::from_path(absolute_path).unwrap();
        let cargo_toml_main = match &cargo_toml_workspace_maybe.workspace {
            None => cargo_toml_workspace_maybe.clone(),
            Some(workspace) => {
                let main_member = &workspace.members[0];
                let absolute_path = std::path::absolute(&format!("{}/Cargo.toml", main_member)).unwrap();
                let cargo_main = cargo_toml::Manifest::from_path(absolute_path).unwrap();
                //return
                cargo_main
            }
        };
        let package = cargo_toml_main.package.as_ref().unwrap().to_owned();
        CargoToml {
            cargo_toml_workspace_maybe,
            _cargo_toml_main: cargo_toml_main,
            package,
        }
    }

    /// Cargo.toml package name
    fn package_name(&self) -> String {
        self.package.name.to_string()
    }

    /// Cargo.toml package version
    fn package_version(&self) -> String {
        self.package.version().to_string()
    }

    /// Cargo.toml package authors as string
    fn package_authors_string(&self) -> String {
        let authors = crate::utils_mod::concatenate_vec_to_string(self.package.authors(), ", ");
        authors
    }

    /// Cargo.toml package authors as string without emails
    fn package_author_name(&self) -> String {
        let author = self.package_authors_string();
        let author = REGEX_REMOVE_EMAIL.replace_all(&author, "").to_string();
        author
    }

    /// Cargo.toml package repository
    fn package_repository(&self) -> Option<String> {
        self.package.repository().map(|x| x.to_string())
    }

    /// Cargo.toml package repository
    fn package_description(&self) -> Option<String> {
        self.package.description().map(|x| x.to_string())
    }

    /// Cargo.toml package homepage
    fn package_homepage(&self) -> String {
        match self.package.homepage() {
            None => String::new(),
            Some(x) => x.to_string(),
        }
    }

    /// Cargo.toml workspace members
    fn workspace_members(&self) -> Option<Vec<String>> {
        self.cargo_toml_workspace_maybe.workspace.as_ref().map(|workspace| workspace.members.clone())
    }

    /// github_owner from package_repository
    fn github_owner(&self) -> Option<String> {
        match self.package_repository() {
            Some(repository) => {
                let splitted: Vec<&str> = repository.trim_start_matches("https://").split("/").collect();
                Some(splitted[1].to_string())
            }
            None => None,
        }
    }
    /// Cargo.toml package keywords
    fn package_keywords(&self) -> Vec<String> {
        self.package.keywords().to_owned()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn test_cargo_toml() {
        use crate::public_api_mod::CargoTomlPublicApiMethods;
        let cargo_toml = CargoToml::read();
        assert_eq!(cargo_toml.package_author_name(), "Bestia.dev");
        assert_eq!(cargo_toml.package_homepage(), "https://bestia.dev");
    }
}
