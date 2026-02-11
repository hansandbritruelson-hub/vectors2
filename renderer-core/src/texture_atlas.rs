use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use guillotiere::{AtlasAllocator, Size, Allocation};

// We store the full Allocation in the deallocation queue to avoid importing specific ID types.

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct CacheKey {
    pub id: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug)]
pub struct AtlasRegion {
    pub allocation: Allocation, // Has ID and rectangle
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    // UV coordinates (0.0 - 1.0)
    pub u_min: f32,
    pub v_min: f32,
    pub u_max: f32,
    pub v_max: f32,
}

// Ensure the Region is fully self-contained for the renderer
impl AtlasRegion {
    pub fn new(allocation: Allocation, atlas_width: u32, atlas_height: u32) -> Self {
        let rect = allocation.rectangle;
        AtlasRegion {
            allocation,
            x: rect.min.x as u32,
            y: rect.min.y as u32,
            width: rect.width() as u32,
            height: rect.height() as u32,
            u_min: rect.min.x as f32 / atlas_width as f32,
            v_min: rect.min.y as f32 / atlas_height as f32,
            u_max: rect.max.x as f32 / atlas_width as f32,
            v_max: rect.max.y as f32 / atlas_height as f32,
        }
    }
}

// A handle that owns the allocation. When dropped, it queues deallocation.
#[derive(Debug)]
pub struct TextureHandle {
    pub region: AtlasRegion,
    deallocation_queue: Rc<RefCell<Vec<Allocation>>>,
}

impl Drop for TextureHandle {
    fn drop(&mut self) {
        // Notify Atlas that this allocation is free.
        // We push to the shared queue.
        self.deallocation_queue.borrow_mut().push(self.region.allocation);
        // crate::log(&format!("TextureHandle dropped: {:?}", self.region.allocation.id));
    }
}

pub struct TextureAtlas {
    pub width: u32,
    pub height: u32,
    allocator: AtlasAllocator,
    
    // Shared queue for dropped handles to report back
    deallocation_queue: Rc<RefCell<Vec<Allocation>>>,
    
    // Cache: Weak references to active handles.
    // If upgrade fails, the handle is gone, and likely in the deallocation queue (or about to be).
    cache: HashMap<CacheKey, Weak<TextureHandle>>,
    
    pub dirty: bool,
    // We store the pending upload data.
    // Ideally we would just write to GPU immediately, but we batch it here for `render()`.
    pub pending_uploads: Vec<(AtlasRegion, Vec<u8>)>,
}

impl TextureAtlas {
    pub fn new(width: u32, height: u32) -> Self {
        TextureAtlas {
            width,
            height,
            allocator: AtlasAllocator::new(Size::new(width as i32, height as i32)),
            deallocation_queue: Rc::new(RefCell::new(Vec::new())),
            cache: HashMap::new(),
            dirty: false,
            pending_uploads: Vec::new(),
        }
    }

    // Process any handles that were dropped
    pub fn process_deletions(&mut self) {
        let mut queue = self.deallocation_queue.borrow_mut();
        if queue.is_empty() { return; }
        
        for allocation in queue.drain(..) {
            self.allocator.deallocate(allocation.id);
            // crate::log(&format!("Deallocated texture region via handle drop"));
        }
    }

    pub fn allocate(&mut self, key: CacheKey, pixels: Vec<u8>) -> Option<Rc<TextureHandle>> {
        // 1. Check Cache
        if let Some(weak) = self.cache.get(&key) {
            if let Some(rc) = weak.upgrade() {
                return Some(rc);
            }
            // If dead, we'll overwrite it below.
        }
        
        // 2. Process Deletions (Free up space before alloc)
        self.process_deletions();

        let w = key.width as i32;
        let h = key.height as i32;
        
        // 3. Allocate using Guillotiere
        let allocation = match self.allocator.allocate(Size::new(w, h)) {
            Some(a) => a,
            None => {
                crate::log("Texture Atlas Full! (Allocation failed)");
                // We could try to defragment or resize here, but for now just fail.
                return None;
            }
        };
        
        let region = AtlasRegion::new(allocation, self.width, self.height);
        
        // 5. Create Handle
        let handle = Rc::new(TextureHandle {
            region: region.clone(),
            deallocation_queue: self.deallocation_queue.clone(),
        });
        
        // 6. Queue Upload
        self.pending_uploads.push((region, pixels));
        self.dirty = true;
        
        // 7. Store in Cache
        self.cache.insert(key, Rc::downgrade(&handle));
        
        Some(handle)
    }
    
    pub fn get_handle(&self, key: &CacheKey) -> Option<Rc<TextureHandle>> {
        if let Some(weak) = self.cache.get(key) {
             weak.upgrade()
        } else {
            None
        }
    }
}
