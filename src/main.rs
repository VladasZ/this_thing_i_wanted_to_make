mod dep;
mod installer;
#[cfg(target_os = "linux")]
mod linux_installer;
#[cfg(target_os = "macos")]
mod mac_installer;
#[cfg(windows)]
mod windows_installer;

// use std::path::PathBuf;

// use git2::Repository;
// use home::home_dir;
use crate::{installer::Installer as InstallerTrait, mac_installer::MacInstaller};

pub type Result<T, E = &'static str> = std::result::Result<T, E>;

// struct Thing {
//     home:       PathBuf,
//     shell_path: PathBuf,
//     repo:       Repository,
// }
//
// impl Thing {
//     fn has_changes(&self) -> bool {
//         self.repo
//             .statuses(None)
//             .expect("Failed to get repo status")
//             .len()
//             > 0
//     }
//
//     fn update_repo(&self) {
//         if self.has_changes() {
//             self.push();
//         } else {
//             println!("No local changes. Pulling.");
//             self.pull();
//         }
//     }
//
//     fn push(&self) {}
//
//     fn pull(&self) {
//         // self.repo.p
//     }
// }
//
// impl Default for Thing {
//     fn default() -> Self {
//         let home = home_dir().expect("Failed to get home directory");
//
//         let shell_path = home.join(".shell");
//
//         let repo = match Repository::open(&shell_path) {
//             Ok(repo) => repo,
//             Err(_) => Repository::clone("git@github.com:VladasZ/.shell.git",
// &shell_path)                 .expect("Failed to clone .shell repository"),
//         };
//
//         Self {
//             home,
//             shell_path,
//             repo,
//         }
//     }
// }

#[cfg(target_os = "macos")]
type Installer = MacInstaller;

#[cfg(target_os = "linux")]
type Installer = LinuxInstaller;

#[cfg(windows)]
type Installer = WindowsInstaller;

fn main() {
    // let thing = Thing::default();
    // thing.update_repo();

    let installer = Installer::default();

    dbg!(installer.install("git").unwrap());
}
