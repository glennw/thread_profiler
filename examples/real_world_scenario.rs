#[macro_use] extern crate thread_profiler;

#[cfg(feature = "thread_profiler")]
use thread_profiler::ProfileScope;

use std::string::String;
use std::thread;
use std::vec::Vec;

fn main() {
    if !cfg!(feature = "thread_profiler") {
        panic!("This example must be run with the 'thread_profiler' feature enabled");
    }
    let workers = 10;
    let mut tasks = Vec::new();
    for i in 0..workers {
        tasks.push(thread::spawn(move || {
            #[cfg(feature = "thread_profiler")]
            thread_profiler::register_thread_with_profiler();

            // Here we use the profile_scope!() macro to profile this scope.
            // This simplifies everything by automatically feature gating
            // itself.
            // It also provides a nice naming format by specifying the module
            // path in the name of the profile.
            profile_scope!(format!("worker: thread_{}", i));
            let result = {
                // Here we call ProfileScope::new() directly instead of using the macro
                // this gives us more control over the naming, however we have
                // to feature gate it ourself
                #[cfg(feature = "thread_profiler")]
                ProfileScope::new(format!("worker: thread_{}::calculation", i));
                perform_complex_calculation()
            };
            {
                #[cfg(feature = "thread_profiler")]
                ProfileScope::new(format!("worker: thread_{}::analyse", i));
                if analyse_complex_result(result) {
                    println!("Worker {} completed OK", i);
                } else {
                    println!("Worker {} did not complete OK", i);
                }
            }
        }));
    }

    for task in tasks.drain(..) {
        match task.join() {
            Ok(_) => println!("Task completed OK"),
            _ => println!("Task did not complete!"),
        }
    }
    #[cfg(feature = "thread_profiler")]
    {
        let output_file = format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "real_world_scenario.profile.json"
        );
        println!(
            "Writing profile to {}, try loading this using chome 'about:tracing'",
            output_file
        );
        thread_profiler::write_profile(output_file.as_str());
    }
}

fn perform_complex_calculation() -> String {
    "Hello I'm a worker result".to_string()
}

fn analyse_complex_result(data: String) -> bool {
    data == "Hello I'm a worker result"
}
