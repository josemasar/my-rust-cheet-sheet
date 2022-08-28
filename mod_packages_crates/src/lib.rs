pub mod mod_packages_crates {
    pub fn learn_modpackagescrates() {
        /* How to organize your code in files, modules, and packages.
        How those concepts interact.
        How to use third-party packages from the Crates.io repository. */
        use regex::Regex;
        // mod authentication;

        // let mut user = authentication::User::new("jeremy", "super-secret");

        // println!("The username is: {}", user.get_username());
        // user.set_password("even-more-secret");

        let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
        println!("Did our date match? {}", re.is_match("2014-01-01"));
    }
}
