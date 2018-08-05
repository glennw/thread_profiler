[![Build Status](https://travis-ci.org/glennw/thread_profiler.svg)](https://travis-ci.org/glennw/thread_profiler) 
[![](http://meritbadge.herokuapp.com/thread_profiler)](https://crates.io/crates/thread_profiler)
[![Documentation](https://docs.rs/thread_profiler/badge.svg)](https://docs.rs/thread_profiler)

# Thread Profiler

This is a simple CPU profiler for [WebRender](). It can write out the resutls in [Trace Event Format](https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/edit).

Read more about the usage and associated tools at https://aras-p.info/blog/2017/01/23/Chrome-Tracing-as-Profiler-Frontend/

## Hookup

Call `register_thread_with_profiler()` for each thread.

Call `profile_scope!("what you are measuring")` for each scope to profile.  Will introduce a hidden variable that stops the profiling of the section when going out of scope.

Call `write_profile(file_path)` when you need to save the results.  Make sure all profiles introduced with `profile_scope!` are actually out of scope.

## Example

```rust
thread::spawn(|| {
    register_thread_with_profiler();
    
    {
        profile_scope!(format!("Thread {:?}, section {:?}", 1, "init"));
        // your "init" stuff goes here
    } // profiling scope ends here
    
    {
        profile_scope!(...);
        // and so on
    }
});

// at the very end, after joining all threads
write_profile("/some/place/for/your/profile.json");
```

## View results

With Chrome: go to `chrome://tracing` and click on "Load".

Standalone: check out and compile [catapult](https://github.com/catapult-project/catapult/tree/master/tracing), then call `trace2html`.
