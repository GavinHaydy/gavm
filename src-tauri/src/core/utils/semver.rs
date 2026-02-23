// semver.rs
// Simple semantic version sorting utility

pub fn sort_versions_desc(versions: &mut Vec<String>) {
    versions.sort_by(|a, b| {
        let pa: Vec<u32> = a.split('.')
            .filter_map(|x| x.parse().ok())
            .collect();

        let pb: Vec<u32> = b.split('.')
            .filter_map(|x| x.parse().ok())
            .collect();

        pb.cmp(&pa) // descending
    });
}