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
    current_scope: Option<ScopeId>,
    signals: HashMap<SignalId, Box<dyn Any>>,
    subscribers: HashMap<SignalId, HashSet<EffectId>>,
    effects: HashMap<EffectId, Box<dyn FnMut()>>,
    effect_scopes: HashMap<EffectId, ScopeId>, // Which scope does this effect belong to?
    scopes: HashMap<ScopeId, ScopeData>,
    next_signal_id: SignalId,
    next_effect_id: EffectId,
    next_scope_id: ScopeId,
}

pub type ScopeId = usize;

struct ScopeData {
    signals: Vec<SignalId>,
    effects: Vec<EffectId>,
    sub_scopes: Vec<ScopeId>,
    parent: Option<ScopeId>,
    cleanups: Vec<Box<dyn FnOnce()>>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            current_effect: None,
            current_scope: None,
            signals: HashMap::new(),
            subscribers: HashMap::new(),
            effects: HashMap::new(),
            effect_scopes: HashMap::new(),
            scopes: HashMap::new(),
            next_signal_id: 0,
            next_effect_id: 0,
            next_scope_id: 0,
        }
    }

    fn dispose_scope(&mut self, id: ScopeId) {
        if let Some(data) = self.scopes.remove(&id) {
            // 1. Dispose sub-scopes
            for sub_id in data.sub_scopes {
                self.dispose_scope(sub_id);
            }

            // 2. Run cleanups
            for cleanup in data.cleanups {
                cleanup();
            }

            // 3. Remove effects
            for effect_id in data.effects {
                self.effects.remove(&effect_id);
                self.effect_scopes.remove(&effect_id);
                // Also remove from subscribers
                for subs in self.subscribers.values_mut() {
                    subs.remove(&effect_id);
                }
            }

            // 4. Remove signals
            for signal_id in data.signals {
                self.signals.remove(&signal_id);
                self.subscribers.remove(&signal_id);
            }
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
        
        if let Some(scope_id) = rt.current_scope {
            if let Some(scope) = rt.scopes.get_mut(&scope_id) {
                scope.signals.push(id);
            }
        }
        
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
        rt.effects.insert(id, Box::new(effect_fn)); 

        if let Some(scope_id) = rt.current_scope {
            if let Some(scope) = rt.scopes.get_mut(&scope_id) {
                scope.effects.push(id);
                rt.effect_scopes.insert(id, scope_id);
            }
        }

        id
    });

    run_effect(id);
}

pub fn create_root<F, T>(f: F) -> T 
where F: FnOnce(Scope) -> T
{
    let (id, parent) = RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        let id = rt.next_scope_id;
        rt.next_scope_id += 1;
        
        let parent = rt.current_scope;
        rt.scopes.insert(id, ScopeData {
            signals: Vec::new(),
            effects: Vec::new(),
            sub_scopes: Vec::new(),
            parent,
            cleanups: Vec::new(),
        });
        
        if let Some(p) = parent {
            if let Some(p_data) = rt.scopes.get_mut(&p) {
                p_data.sub_scopes.push(id);
            }
        }
        
        rt.current_scope = Some(id);
        (id, parent)
    });

    let result = f(Scope { id });

    RUNTIME.with(|rt| {
        rt.borrow_mut().current_scope = parent;
    });

    result
}

pub struct Scope {
    pub id: ScopeId,
}

impl Scope {
    pub fn dispose(self) {
        RUNTIME.with(|rt| {
            rt.borrow_mut().dispose_scope(self.id);
        });
    }
}

pub fn on_cleanup<F>(f: F) 
where F: FnOnce() + 'static
{
    RUNTIME.with(|rt| {
        let mut rt = rt.borrow_mut();
        if let Some(scope_id) = rt.current_scope {
            if let Some(scope) = rt.scopes.get_mut(&scope_id) {
                scope.cleanups.push(Box::new(f));
            }
        }
    });
}
pub fn create_memo<T: 'static + Clone, F: Fn() -> T + 'static>(f: F) -> ReadSignal<T> {
    let (read, write) = create_signal(f());
    create_effect(move || {
        write.set(f());
    });
    read
}

// --- Signal Accessors ---

pub struct ReadSignal<T> {
    pub id: SignalId,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Clone for ReadSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ReadSignal<T> {}

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

pub struct WriteSignal<T> {
    pub id: SignalId,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Clone for WriteSignal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for WriteSignal<T> {}

impl<T: 'static + Clone> WriteSignal<T> {
    pub fn set<U: Into<T>>(&self, new_value: U) {
        let val: T = new_value.into();
        crate::log(&format!("Signal set: id={}", self.id));
        let effects_to_run = RUNTIME.with(|rt| {
            let mut rt = rt.borrow_mut();
            
            // 1. Update value
            rt.signals.insert(self.id, Box::new(val));

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
        // 2. Set current_effect context and scope context
        let (prev_effect, prev_scope, _scope_id) = RUNTIME.with(|rt| {
            let mut rt = rt.borrow_mut();
            let prev_e = rt.current_effect;
            let prev_s = rt.current_scope;
            let sid = rt.effect_scopes.get(&id).cloned();
            rt.current_effect = Some(id);
            if let Some(sid) = sid {
                rt.current_scope = Some(sid);
            }
            (prev_e, prev_s, sid)
        });

        // 3. Run it
        f();

        // 4. Restore context and put effect back
        RUNTIME.with(|rt| {
            let mut rt = rt.borrow_mut();
            rt.current_effect = prev_effect;
            rt.current_scope = prev_scope;
            rt.effects.insert(id, f);
        });
    }
}

// --- Conversion Trait ---

pub trait ToReactiveString {
    fn to_reactive_string(&self) -> String;
}

impl<T: std::fmt::Display + Clone + 'static> ToReactiveString for ReadSignal<T> {
    fn to_reactive_string(&self) -> String { self.get().to_string() }
}

impl ToReactiveString for String {
    fn to_reactive_string(&self) -> String { self.clone() }
}

impl ToReactiveString for &str {
    fn to_reactive_string(&self) -> String { self.to_string() }
}

impl ToReactiveString for i32 {
    fn to_reactive_string(&self) -> String { self.to_string() }
}

impl ToReactiveString for f32 {
    fn to_reactive_string(&self) -> String { self.to_string() }
}

pub trait ToBool {
    fn to_bool(&self) -> bool;
}

impl ToBool for bool {
    fn to_bool(&self) -> bool { *self }
}

impl ToBool for ReadSignal<bool> {
    fn to_bool(&self) -> bool { self.get() }
}

impl<T, Rhs> PartialEq<Rhs> for ReadSignal<T>
where
    T: PartialEq<Rhs> + Clone + 'static,
{
    fn eq(&self, other: &Rhs) -> bool {
        self.get() == *other
    }
}


