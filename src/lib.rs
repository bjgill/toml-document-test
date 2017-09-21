extern crate toml_document;

#[cfg(test)]
mod tests {
    use toml_document::*;

    use std::fs::File;
    use std::io::Read;

    fn read_own_manifest() -> String {
        let mut f = File::open("Cargo.toml").expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        contents
    }

    #[test]
    fn parse_own_manifest() {
        Document::parse(&read_own_manifest()).expect("Failed to parse own manifest");
    }

    #[test]
    fn read_dependency_list() {
        let manifest = Document::parse(&read_own_manifest()).expect("Failed to parse own manifest");

        // println!("{}", manifest);

        let dependencies = match manifest.get("dependencies") {
            Some(EntryRef::Table(t)) => t,
            _ => panic!("Invalid or missing dependency table"),
        };

        assert_eq!(dependencies.len_children(), 1);

        let (name, details) = dependencies.iter().next().unwrap();

        assert_eq!(name, "toml_document");

        let mut version = match details {
            EntryRef::String(s) => s.get(),
            _ => panic!("Invalid version"),
        };

        assert_eq!(version, "0.1.3");

        // version.set("*".to_string());

        // println!("{}", manifest);
    }

    #[test]
    fn upgrade_dependency() {
        let mut manifest = Document::parse(&read_own_manifest()).expect("Failed to parse own manifest");

        // println!("{}", manifest);

        let mut dependencies = match manifest.get_mut("dependencies") {
            Some(EntryRefMut::Table(t)) => t.to_value(),
            _ => panic!("Invalid or missing dependency table"),
        };

        match dependencies {
            TableValueMut::Inline(t) => {
                match t.get("toml_document").unwrap().value() {
                    ValueRef::String(s) => s.set("*".to_string()),
                    _ => panic!("Invalid version"),
                }
            }
            _ => unimplemented!()
        }

        // assert_eq!(dependencies.len_children(), 1);

        // let (name, mut details) = dependencies.iter().next().unwrap();

        // assert_eq!(name, "toml_document");

        // let mut version = match details {
        //     EntryRefMut::String(s) => s.get(),
        //     _ => panic!("Invalid version"),
        // };

        // assert_eq!(version, "0.1.3");

        // version.set("*".to_string());

        // println!("{}", manifest);
    }
}
