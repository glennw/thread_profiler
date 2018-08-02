[![Build Status](https://travis-ci.org/glennw/thread_profiler.svg)](https://travis-ci.org/glennw/thread_profiler) 
[![](http://meritbadge.herokuapp.com/thread_profiler)](https://crates.io/crates/thread_profiler)
[![Documentation](https://docs.rs/thread_profiler/badge.svg)](https://docs.rs/thread_profiler)

# Thread Profiler

This is a simple CPU profiler for [WebRender](). It can write out the results in [Trace Event Format](https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/edit).

Read more about the usage and associated tools at https://aras-p.info/blog/2017/01/23/Chrome-Tracing-as-Profiler-Frontend/

## Hookup

Call `register_thread_with_profiler` for each thread.

Call `write_profile` when you need to save the results.

## View results

With Chrome: go to `chrome://tracing` and click on "Load".

Standalone: check out and compile [catapult](https://github.com/catapult-project/catapult/tree/master/tracing), then call `trace2html`.
