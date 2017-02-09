extern crate time;
#[macro_use]
extern crate json;
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "thread_profiler")]
use std::cell::RefCell;
#[cfg(feature = "thread_profiler")]
use std::fs::File;
#[cfg(feature = "thread_profiler")]
use std::io::Write;
#[cfg(feature = "thread_profiler")]
use std::sync::mpsc::{channel, Sender, Receiver};
#[cfg(feature = "thread_profiler")]
use std::sync::Mutex;
#[cfg(feature = "thread_profiler")]
use time::precise_time_ns;

#[cfg(feature = "thread_profiler")]
#[macro_export]
macro_rules! profile_scope {
    ($string:expr) => {
        let _profile_scope = $crate::ProfileScope::new($string);
    }
}

#[cfg(not(feature = "thread_profiler"))]
#[macro_export]
macro_rules! profile_scope {
    ($string:expr) => {
    }
}

#[cfg(feature = "thread_profiler")]
lazy_static! {
    static ref GLOBAL_PROFILER: Mutex<Profiler> = Mutex::new(Profiler::new());
}

#[cfg(feature = "thread_profiler")]
thread_local!(static THREAD_PROFILER: RefCell<Option<ThreadProfiler>> = RefCell::new(None));

#[cfg(feature = "thread_profiler")]
#[derive(Copy, Clone)]
struct ThreadId(usize);

#[cfg(feature = "thread_profiler")]
struct ThreadInfo {
    name: String,
}

#[cfg(feature = "thread_profiler")]
enum Sample {
    Enter(ThreadId, &'static str, u64),
    Exit(ThreadId, u64),
}

#[cfg(feature = "thread_profiler")]
struct ThreadProfiler {
    id: ThreadId,
    tx: Sender<Sample>,
}

#[cfg(feature = "thread_profiler")]
impl ThreadProfiler {
    fn enter(&self, name: &'static str) {
        let sample = Sample::Enter(self.id, name, precise_time_ns());
        self.tx.send(sample).ok();
    }

    fn exit(&self) {
        let sample = Sample::Exit(self.id, precise_time_ns());
        self.tx.send(sample).ok();
    }
}

#[cfg(feature = "thread_profiler")]
struct Profiler {
    rx: Receiver<Sample>,
    tx: Sender<Sample>,
    threads: Vec<ThreadInfo>,
}

#[cfg(feature = "thread_profiler")]
impl Profiler {
    fn new() -> Profiler {
        let (tx, rx) = channel();

        Profiler {
            rx: rx,
            tx: tx,
            threads: Vec::new(),
        }
    }

    fn register_thread(&mut self, name: String) {
        let id = ThreadId(self.threads.len());

        self.threads.push(ThreadInfo {
            name: name,
        });

        THREAD_PROFILER.with(|profiler| {
            assert!(profiler.borrow().is_none());

            let thread_profiler = ThreadProfiler {
                id: id,
                tx: self.tx.clone(),
            };

            *profiler.borrow_mut() = Some(thread_profiler);
        });
    }

    fn write_profile(&self, filename: &str) {
        // Stop reading samples that are written after
        // write_profile() is called.
        let start_time = precise_time_ns();
        let mut data = json::JsonValue::new_array();

        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                Sample::Enter(tid, name, t) => {
                    if t > start_time {
                        break;
                    }

                    let thread_id = self.threads[tid.0].name.as_str();
                    let us = t / 1000;

                    data.push(object!{
                        "pid" => 0,
                        "tid" => thread_id,
                        "name" => name,
                        "ph" => "B",
                        "ts" => us
                    }).unwrap();
                }
                Sample::Exit(tid, t) => {
                    let thread_id = self.threads[tid.0].name.as_str();
                    let us = t / 1000;

                    data.push(object!{
                        "pid" => 0,
                        "tid" => thread_id,
                        "ph" => "E",
                        "ts" => us
                    }).unwrap();
                }
            }
        }

        let s = json::stringify_pretty(data, 2);
        let mut f = File::create(filename).unwrap();
        f.write_all(s.as_bytes()).unwrap();
    }
}

#[cfg(feature = "thread_profiler")]
pub struct ProfileScope;

#[cfg(feature = "thread_profiler")]
impl ProfileScope {
    pub fn new(name: &'static str) -> ProfileScope {
        THREAD_PROFILER.with(|profiler| {
            match *profiler.borrow() {
                Some(ref profiler) => {
                    profiler.enter(name);
                }
                None => {
                    println!("ERROR: ProfileScope {} on unregistered thread!", name);
                }
            }
        });

        ProfileScope
    }
}

#[cfg(feature = "thread_profiler")]
impl Drop for ProfileScope {
    fn drop(&mut self) {
        THREAD_PROFILER.with(|profiler| {
            if let Some(ref profiler) = *profiler.borrow() {
                profiler.exit();
            }
        });
    }
}

#[cfg(feature = "thread_profiler")]
pub fn write_profile(filename: &str) {
    GLOBAL_PROFILER.lock()
                   .unwrap()
                   .write_profile(filename);
}

#[cfg(feature = "thread_profiler")]
pub fn register_thread_with_profiler(thread_name: String) {
    GLOBAL_PROFILER.lock()
                   .unwrap()
                   .register_thread(thread_name);
}

#[cfg(not(feature = "thread_profiler"))]
pub fn write_profile(_filename: &str) {
    println!("WARN: write_profile was called when the thread profiler is disabled!");
}

#[cfg(not(feature = "thread_profiler"))]
pub fn register_thread_with_profiler(_thread_name: String) {
}
