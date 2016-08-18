use common::print_status;
use constants;
use std::collections::HashSet;

// Install the listed packages and their dependencies
pub fn install_packages<'a>(packages_to_install: HashSet<&'a str>) -> Vec<i32> {
    let packages = if let Some(x) = opt_packages {
        x
    } else if let Some(x) = cmd_packages {
        x.values_of(constants::PACKAGES).unwrap()
    } else {
        return vec![0];
    };
    print_status("Installing", &packages);
    vec![0]
}
