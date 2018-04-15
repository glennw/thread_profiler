#[macro_use]
extern crate thread_profiler;

// Put this in a seperate module to check that the module expansion is working
mod profile_tests {
    use std::fs::File;
    use std::io::prelude::Read;
    use std::string::String;
    use thread_profiler;

    #[test]
    fn test_profile_macro() {
        let output_file = format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "integration_test_macro_output.json"
        );
        thread_profiler::register_thread_with_profiler();
        {
            profile_scope!("MyTestProfile");
        }
        thread_profiler::write_profile(&output_file);

        // Get the profile that we wrote
        let mut f = File::open(output_file).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();

        // Test that the correct name has been used for the profile scope
        // but only if we have the feature enabled#
        let test = buffer.contains("profile_tests: MyTestProfile");
        if cfg!(feature = "thread_profiler") {
            assert!(
                test,
                "Integration test macro did not contain the correct profile name"
            );
        } else {
            assert!(
                !test,
                "Integration test macro incorrectly contained the profile name"
            );
        }
    }
}
