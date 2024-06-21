///! Modified from https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/resource-manager.rs
use sdl2::rwops::RWops;
use sdl2::ttf::{Font, Sdl2TtfContext};

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;

// Generic struct to cache any resource loaded by a ResourceLoader
pub struct ResourceManager<'l, K, R, L>
where
    K: Hash + Eq,
    L: 'l + ResourceLoader<'l, R>,
{
    loader: &'l L,
    cache: HashMap<K, Rc<R>>,
}

impl<'l, K, R, L> ResourceManager<'l, K, R, L>
where
    K: Hash + Eq,
    L: ResourceLoader<'l, R>,
{
    pub fn new(loader: &'l L) -> Self {
        ResourceManager {
            cache: HashMap::new(),
            loader,
        }
    }

    // Generics magic to allow a HashMap to use String as a key
    // while allowing it to use &str for gets
    pub fn load<D>(&mut self, details: &D) -> Result<Rc<R>, String>
    where
        L: ResourceLoader<'l, R, Args = D>,
        D: Eq + Hash + ?Sized,
        K: Borrow<D> + for<'a> From<&'a D>,
    {
        self.cache.get(details).cloned().map_or_else(
            || {
                let resource = Rc::new(self.loader.load(details)?);
                self.cache.insert(details.into(), resource.clone());
                Ok(resource)
            },
            Ok,
        )
    }
}

// Font Context knows how to load Fonts
impl<'l> ResourceLoader<'l, Font<'l, 'static>> for Sdl2TtfContext {
    type Args = FontDetails;
    fn load(&'l self, details: &FontDetails) -> Result<Font<'l, 'static>, String> {
        println!("LOADED A FONT");
        self.load_font_from_rwops(RWops::from_bytes(&details.content)?, details.size)
    }
}

// Generic trait to Load any Resource Kind
pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;
    fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}

// Information needed to load a Font
#[derive(PartialEq, Eq, Hash)]
pub struct FontDetails {
    pub content: &'static [u8],
    pub size: u16,
}

impl<'a> From<&'a FontDetails> for FontDetails {
    fn from(details: &'a FontDetails) -> FontDetails {
        FontDetails {
            content: details.content,
            size: details.size,
        }
    }
}
