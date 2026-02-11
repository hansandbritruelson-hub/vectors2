use std::cell::RefCell;
// use std::rc::Rc;
use std::collections::{HashSet, HashMap};
use std::any::Any;

// --- Types ---

pub type SignalId = usize;
pub type EffectId = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Signal<T> {
    pub id: SignalId,
    _marker: std::marker::PhantomData<T>,
}

// --- Global Runtime State (Thread Local) ---

struct Runtime {
    current_effect: Option<EffectId>,
    signals: HashMap<SignalId, Box<dyn Any>>, // Stores the *value* of the signal
    subscribers: HashMap<SignalId, HashSet<EffectId>>,  // Who is listening to this signal?
    effects: HashMap<EffectId, Box<dyn FnMut()>>,       // The actual closure of the effect
    next_signal_id: SignalId,
    next_effect_id: EffectId,
}

impl Runtime {
    fn new() -> Self {
        Self {
            current_effect: None,
            signals: HashMap::new(),
            subscribers: HashMap::new(),
            effects: HashMap::new(),
            next_signal_id: 0,
            next_effect_id: 0,
        }
    }
}

thread_local! {
    static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::new());
}

// --- Public API ---

pub fn create_signal<T: 'static + Clone>(initial_value: T) -> (ReadSignal<T>, WriteSignal<T>) {
    let id = RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        let id = rt.next_signal_id;
        rt.next_signal_id += 1;
        rt.signals.insert(id, Box::new(initial_value));
        id
    });

    (
        ReadSignal { id, _marker: std::marker::PhantomData },
        WriteSignal { id, _marker: std::marker::PhantomData }
    )
}

pub fn create_effect<F>(effect_fn: F)
where
    F: FnMut() + 'static,
{
    let id = RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        let id = rt.next_effect_id;
        rt.next_effect_id += 1;
        // Don't insert yet, run_effect handles insertion
        rt.effects.insert(id, Box::new(effect_fn)); 
        id
    });

    run_effect(id);
}

// --- Signal Accessors ---

#[derive(Clone, Copy)]
pub struct ReadSignal<T> {
    pub id: SignalId,
    _marker: std::marker::PhantomData<T>,
}

impl<T: 'static + Clone> ReadSignal<T> {
    pub fn get(&self) -> T {
        RUNTIME.with(|rt| {
            let mut rt = rt.borrow_mut();
            
            // Dependency Tracking: If we are inside an effect, record this signal as a dependency
            if let Some(effect_id) = rt.current_effect {
                rt.subscribers.entry(self.id).or_default().insert(effect_id);
            }

            // Return the value
            rt.signals.get(&self.id)
                .expect("Signal not found")
                .downcast_ref::<T>()
                .expect("Signal type mismatch")
                .clone()
        })
    }
}

#[derive(Clone, Copy)]
pub struct WriteSignal<T> {
    pub id: SignalId,
    _marker: std::marker::PhantomData<T>,
}

impl<T: 'static + Clone> WriteSignal<T> {
    pub fn set(&self, new_value: T) {
        let effects_to_run = RUNTIME.with(|rt| {
            let mut rt = rt.borrow_mut();
            
            // 1. Update value
            rt.signals.insert(self.id, Box::new(new_value));

            // 2. Collect subscribers
            if let Some(subs) = rt.subscribers.get(&self.id) {
                subs.clone() 
            } else {
                HashSet::new()
            }
        });

        // 3. Run effects (outside the borrow to avoid recursion panic issues for now)
        for effect_id in effects_to_run {
            run_effect(effect_id);
        }
    }

    pub fn update<F>(&self, f: F) 
    where F: FnOnce(&T) -> T 
    {
        // 1. Get current value
        let current: T = RUNTIME.with(|rt| {
            let rt = rt.borrow();
             rt.signals.get(&self.id)
                .expect("Signal downcast error")
                .downcast_ref::<T>()
                .unwrap()
                .clone()
        });
        
        // 2. Compute new value
        let new_val = f(&current);
        
        // 3. Set
        self.set(new_val);
    }
}

// --- Internal ---

fn run_effect(id: EffectId) {
    // 1. Extract the effect closure (take ownership temporarily)
    let effect_opt = RUNTIME.with(|rt| {
        rt.borrow_mut().effects.remove(&id)
    });

    if let Some(mut f) = effect_opt {
        // 2. Set current_effect context
        let prev_effect = RUNTIME.with(|rt| {
            let mut rt = rt.borrow_mut();
            let prev = rt.current_effect;
            rt.current_effect = Some(id);
            prev
        });

        // 3. Run it
        f();

        // 4. Restore context and put effect back
        RUNTIME.with(|rt| {
            let mut rt = rt.borrow_mut();
            rt.current_effect = prev_effect;
            rt.effects.insert(id, f);
        });
    }
}
