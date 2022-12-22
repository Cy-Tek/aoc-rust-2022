use std::alloc::{alloc, Layout};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::{alloc, ptr};

pub struct FileSystem {
    root: *mut Item,
    current: *mut Item,
    _marker: PhantomData<Item>,
}

impl Drop for FileSystem {
    fn drop(&mut self) {
        let root = match unsafe { self.root.as_mut() } {
            Some(item) => item,
            None => return,
        };

        root.clear();

        unsafe {
            alloc::dealloc(self.root as *mut u8, Layout::for_value(root))
        }

        println!("I've been dropped");
    }
}

pub enum Item {
    Directory {
        parent: *mut Item,
        name: String,
        children: Vec<NonNull<Item>>,
    },
    File {
        parent: *mut Item,
        name: String,
        size: usize,
    },
}

impl Item {
    pub fn size(&self) -> usize {
        match self {
            Item::Directory { children, .. } => children
                .iter()
                .map(|child| unsafe { child.as_ref() }.size())
                .sum(),
            Item::File { size, .. } => *size,
        }
    }

    pub fn name(&self) -> &String {
        match self {
            Item::Directory { name, .. } => name,
            Item::File { name, .. } => name,
        }
    }

    pub fn is_dir(&self) -> bool {
        match self {
            Item::Directory { .. } => true,
            Item::File { .. } => false,
        }
    }

    pub fn go_up(&self) -> *mut Item {
        match self {
            Item::Directory { parent, .. } => parent.clone(),
            Item::File { parent, .. } => parent.clone(),
        }
    }

    pub fn insert(&mut self, item: Item) {
        let children = match self {
            Item::Directory { children, .. } => children,
            Item::File { .. } => return,
        };

        unsafe {
            let layout = Layout::for_value(&item);
            let new_ptr = alloc(layout) as *mut Item;
            new_ptr.write(item);
            let non_null = NonNull::new(new_ptr);

            match non_null {
                Some(ptr) => children.push(ptr),
                None => alloc::handle_alloc_error(layout),
            }
        }
    }

    pub fn enter(&self, name: &str) -> *mut Item {
        if let Item::Directory { children, .. } = self {
            for child in children {
                let child_ref = unsafe { child.as_ref() };
                if child_ref.name() == name && child_ref.is_dir() {
                    return child.as_ptr();
                }
            }
        }

        ptr::null_mut()
    }

    fn clear(self) {
        let Item::Directory { mut children, .. } = self else { return };
        while let Some(mut child) = children.pop() {
            child.clear();
            unsafe { alloc::dealloc(child.as_ptr() as *mut u8, Layout::for_value(child.as_ref())) }
        }
    }
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            root: ptr::null_mut(),
            current: ptr::null_mut(),
            _marker: PhantomData {},
        }
    }

    pub fn cd(&mut self, name: &str) -> Result<(), String> {
        unsafe {
            let current = match self.current.as_mut() {
                Some(current) => current,
                None => return Ok(self.insert_dir(name)),
            };

            if name == ".." {
                let new_ptr = current.go_up();
                if !new_ptr.is_null() {
                    self.current = new_ptr;
                }
            }

            let new_ptr = current.enter(name);
            if !new_ptr.is_null() {
                self.current = new_ptr;
            }
        }

        Ok(())
    }

    pub fn insert_dir(&mut self, name: &str) {
        let current = unsafe { self.current.as_mut() };

        match current {
            None => unsafe {
                let dir = Item::Directory {
                    name: name.into(),
                    parent: ptr::null_mut(),
                    children: vec![],
                };

                let layout = Layout::for_value(&dir);
                self.current = alloc(layout) as *mut Item;
                self.current.write(dir);
                self.root = self.current.clone();
            },
            Some(current_item) => {
                let dir = Item::Directory {
                    name: name.into(),
                    parent: self.current,
                    children: vec![],
                };
                current_item.insert(dir)
            }
        }
    }

    pub fn insert_file(&mut self, name: &str, size: usize) {
        let current = unsafe { self.current.as_mut() };
        let item = Item::File {
            size,
            name: name.into(),
            parent: self.current,
        };

        match current {
            None => unsafe {
                self.insert_dir("/".into());
                (*self.current).insert(item);
            },
            Some(current_item) => current_item.insert(item),
        }
    }

    pub fn find_all_matching_sizes(&self, cb: impl Fn(&Item) -> bool) -> Vec<&Item> {
        let mut result: Vec<&Item> = Vec::new();

        unsafe {
            if let Some(item) = self.root.as_ref() {
                Self::search_all(item, &cb, &mut result);
            }
        }

        result
    }

    pub fn size(&self) -> usize {
        unsafe { self.root.as_ref().map_or(0, |root| root.size()) }
    }

    fn search_all<'a>(root: &'a Item, cb: &impl Fn(&Item) -> bool, out: &mut Vec<&'a Item>) {
        if cb(root) {
            out.push(root);
        }

        if let Item::Directory { children, .. } = root {
            for child in children {
                unsafe {
                    Self::search_all(child.as_ref(), cb, out);
                }
            }
        }
    }
}
