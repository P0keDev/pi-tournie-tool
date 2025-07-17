use std::{fs::File, io, path::{Path, PathBuf}};

use kiss_xml::dom::Node;
use sysinfo::Disks;

pub struct SDHandler {
    path: PathBuf,
    
}

impl SDHandler {
    pub fn find_sd() -> Option<Self> {
        let disks = Disks::new_with_refreshed_list();
        for disk in disks.list() {
            if disk.mount_point().starts_with("/media/") {
                return Some(Self { path: disk.mount_point().to_path_buf() });
            }
        }
        None
    }

    pub fn get_slippi_version(&self) -> Option<String> {
        let path = self.path.join("apps");
        for entry in path.read_dir().ok()? {
            let _: Option<()> = try {
                let mut p = entry.ok()?.path();
                if !p.is_dir() { continue; }

                p.push("meta.xml");
                let xml = kiss_xml::parse_filepath(p).ok()?;

                let name = xml.root_element().first_element_by_name("name").ok()?.text();
                if name != "Slippi Nintendont" { continue; }

                let version = xml.root_element().first_element_by_name("version").ok()?.text();
                return Some(version.to_string());
            };
        }

        None
    }

}