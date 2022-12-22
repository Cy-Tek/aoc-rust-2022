use std::alloc::{alloc, handle_alloc_error, Layout};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::{alloc};

pub struct FileSystem {
    root: Option<NonNull<Item>>,
    current: Option<NonNull<Item>>,
    _marker: PhantomData<Item>,
}

impl Drop for FileSystem {
    fn drop(&mut self) {
        let layout = Layout::new::<Item>();
        if let Some(root) = self.root {
            unsafe {
                root.as_ptr().drop_in_place();
                alloc::dealloc(root.as_ptr().cast(), layout);
            };
        }
    }
}

pub enum Item {
    Directory {
        parent: Option<NonNull<Self>>,
        name: String,
        children: Vec<NonNull<Item>>,
    },
    File {
        parent: NonNull<Self>,
        name: String,
        size: usize,
    },
}

impl Drop for Item {
    fn drop(&mut self) {
        let Item::Directory { children, ..} = self else { return; };
        let layout = Layout::new::<Item>();

        while let Some(child) = children.pop() {
            unsafe {
                child.as_ptr().drop_in_place();
                alloc::dealloc(child.as_ptr().cast(), layout);
            };
        }
    }
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

    pub fn go_up(&self) -> Option<NonNull<Self>> {
        match self {
            Item::Directory { parent, .. } => parent.clone(),
            Item::File { parent, .. } => Some(parent.clone()),
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

    pub fn enter(&self, name: &str) -> Option<NonNull<Item>> {
        if let Item::Directory { children, .. } = self {
            for child in children {
                let child_ref = unsafe { child.as_ref() };
                if child_ref.name() == name && child_ref.is_dir() {
                    return Some(child.clone());
                }
            }
        }

        None
    }

    fn clear(&mut self) {
        let Item::Directory { ref mut children, .. } = self else { return };
        while let Some(child) = children.pop() {
            unsafe { child.as_ptr().drop_in_place() };
            unsafe { alloc::dealloc(child.as_ptr() as *mut u8, Layout::for_value(child.as_ref())) }
        }
    }
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            root: None,
            current: None,
            _marker: PhantomData {},
        }
    }

    pub fn cd(&mut self, name: &str) -> Result<(), String> {
        unsafe {
            let mut current = match self.current {
                Some(current) => current,
                None => return Ok(self.insert_dir(name)),
            };

            self.current = match name {
                ".." => current.as_mut().go_up(),
                _ => current.as_ref().enter(name),
            };
        }

        Ok(())
    }

    pub fn insert_dir(&mut self, name: &str) {
        let dir = Item::Directory {
            name: name.into(),
            parent: self.current.clone(),
            children: vec![],
        };

        match self.current {
            None => unsafe {
                let layout = Layout::new::<Item>();
                let Some(ptr) = NonNull::new(alloc(layout).cast::<Item>()) else {
                    handle_alloc_error(layout);
                };

                ptr.as_ptr().write(dir);
                self.current = Some(ptr);
                self.root = self.current.clone();
            },
            Some(mut current_item) => unsafe { current_item.as_mut().insert(dir) },
        }
    }

    pub fn insert_file(&mut self, name: &str, size: usize) {
        let Some(current) = self.current.as_mut() else {
            self.insert_dir("/".into());
            self.insert_file(name, size);
            return;
        };

        let item = Item::File {
            size,
            name: name.into(),
            parent: current.clone(),
        };

        unsafe {
            current.as_mut().insert(item);
        }
    }

    pub fn find_all_matching_sizes(&self, cb: impl Fn(&Item) -> bool) -> Vec<&Item> {
        let mut result: Vec<&Item> = Vec::new();

        unsafe {
            if let Some(item) = self.root.as_ref() {
                Self::search_all(item.as_ref(), &cb, &mut result);
            }
        }

        result
    }

    pub fn size(&self) -> usize {
        unsafe { self.root.as_ref().map_or(0, |root| root.as_ref().size()) }
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
