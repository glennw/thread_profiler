/// A simple example showing how
/// to use a thread_profiler
/// to profile a long running function
/// in a single thread.
///
/// The outputted profile will be something like
///
/// 'simple: covering function'
///     |  
/// ____V________________________
/// |____________________________|         
///  |____________|_____________|
///     ^             ^
///     |           'simple: section 2'
/// 'simple: section_1'

#[macro_use]
extern crate thread_profiler;

use std::vec::Vec;

fn main() {
    println!(
        "Running with thread_profler. Names prepended by '{}'",
        module_path!()
    );

    // Register this thread with the profiler.
    thread_profiler::register_thread_with_profiler();
    long_complex_function();
    // Write the profile to a file
    let output = "./profile.json";
    println!("Writing output to {}", output);
    thread_profiler::write_profile(output);
}

fn long_complex_function() {
    // This will create a profile scope called
    // 'simple' as this is the name of the module.
    profile_scope!("covering function");
    let mut v = Vec::new();
    {
        // This will create a profile scope called
        // 'simple: section_1', i.e. the name of the
        // module with a comment describing the profile.

        profile_scope!("section 1");
        // Do complex work
        v.push(1);
        // Drop is called on
        // 'simple: section_1'
    }
    {
        // This will create a profile scope called
        // 'simple: section_2'
        profile_scope!("section 2");
        // Do complex work
        v.push(2);
        // Drop is called on
        // 'simple: section_2'
    }
    v.push(3)
    // Drop is called on
    // 'simple'
}
