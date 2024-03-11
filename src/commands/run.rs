use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, spawn},
    time::Duration,
};

use active_win_pos_rs::get_active_window;
use rdev::{listen, simulate, Event, EventType, Key};
use rodio::{source::SineWave, OutputStream, Sink, Source};

#[derive(Clone, Copy)]
pub struct Settings {
    pub sys: (Key, Key),
    pub eng: (Key, Key),
    pub wep: (Key, Key),
}

pub(crate) fn run_command(settings: Settings) {
    let (_stream, audio_device) = OutputStream::try_default().unwrap();
    let audio_sink = Sink::try_new(&audio_device).unwrap();
    audio_sink.set_volume(0.1);

    let state = Arc::new((Mutex::new(KeysToSpamState::empty()), Condvar::new()));

    let state_arc_for_event_thread = Arc::clone(&state);
    let event_pushed = spawn(move || {
        loop {
            {
                // First block until an action is needed (e.g. Key Spamming)
                // This is blocking.
                let current_state = state_arc_for_event_thread.0.lock().unwrap();
                let current_state = match (*current_state).any_true() {
                    true => {
                        // No wait needed. Keep spamming
                        current_state
                    }
                    false => {
                        // Currently no key presses are needed.
                        // Get this thread to sleep until the State changes.
                        state_arc_for_event_thread.1.wait(current_state).unwrap()
                    }
                };

                if current_state.enabled {
                    if current_state.pips_sys {
                        send_key_event(&EventType::KeyPress(settings.sys.1))
                    }
                    if current_state.pips_eng {
                        send_key_event(&EventType::KeyPress(settings.eng.1))
                    }
                    if current_state.pips_wep {
                        send_key_event(&EventType::KeyPress(settings.wep.1))
                    }
                }

                thread::sleep(Duration::from_millis(10));

                if current_state.pips_sys {
                    send_key_event(&EventType::KeyRelease(settings.sys.1))
                }
                if current_state.pips_eng {
                    send_key_event(&EventType::KeyRelease(settings.eng.1))
                }
                if current_state.pips_wep {
                    send_key_event(&EventType::KeyRelease(settings.wep.1))
                }
            }

            thread::sleep(Duration::from_millis(10));
        }
    });

    if let Err(err) = listen(move |x| callback(x, Arc::clone(&state), &audio_sink, settings)) {
        println!("Err: {:?}", err);
        event_pushed.join().unwrap();
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct KeysToSpamState {
    pips_sys: bool,
    pips_eng: bool,
    pips_wep: bool,
    // A key can be used to enable / disable Pips. Useful for On-foot stuff
    enabled: bool,
}

impl KeysToSpamState {
    pub(crate) fn empty() -> Self {
        Self {
            pips_sys: false,
            pips_eng: false,
            pips_wep: false,
            enabled: true,
        }
    }

    fn any_true(&self) -> bool {
        self.pips_eng || self.pips_sys || self.pips_wep
    }
}

fn is_active_window_elite() -> bool {
    match get_active_window() {
        Ok(val) => val.app_name == "steam_app_359320",
        Err(_) => {
            eprint!("Failed to get active window. Assume Elite is not running.");
            false
        }
    }
}

fn callback(
    event: Event,
    state: Arc<(Mutex<KeysToSpamState>, Condvar)>,
    audio_sink: &Sink,
    settings: Settings,
) -> () {
    let (is_pressed_down, key) = match event.event_type {
        rdev::EventType::KeyPress(e) => (true, e),
        rdev::EventType::KeyRelease(e) => (false, e),
        _ => return,
    };

    let mut key_state = state.0.lock().unwrap();

    if !is_active_window_elite() {
        if !key_state.any_true() {
            return;
        }

        // if here: The Window is no longer Elite, but we are still pressing some keys.
        // in this instance, all keys must be unset
        (*key_state).pips_sys = false;
        (*key_state).pips_eng = false;
        (*key_state).pips_wep = false;
    } else {
        // if here: We received an event while Elite was focused.

        if key == settings.sys.0 {
            (*key_state).pips_sys = is_pressed_down;
        } else if key == settings.eng.0 {
            (*key_state).pips_eng = is_pressed_down;
        } else if key == settings.wep.0 {
            (*key_state).pips_wep = is_pressed_down;
        } else if key == Key::KpMinus {
            if is_pressed_down {
                let new_state = !(*key_state).enabled;
                (*key_state).enabled = new_state;

                let freq = |x| match x {
                    true => 800.0,
                    false => 1600.0,
                };

                audio_sink.append(
                    SineWave::new(freq(new_state)).take_duration(Duration::from_millis(100)),
                );
                audio_sink.append(
                    SineWave::new(freq(!new_state)).take_duration(Duration::from_millis(200)),
                );

                println!("New active state: {}", new_state);
            }
        } else {
            return;
        }
    }

    eprintln!("Info: Key {:?} pressed: {:?}.", key, is_pressed_down);
    state.1.notify_one();
}

fn send_key_event(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(err) => {
            println!("We could not send {:?}: {:?}", event_type, err);
        }
    }
}
