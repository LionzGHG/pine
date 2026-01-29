use std::{any::TypeId, collections::HashMap, path::Path, sync::{Arc, LazyLock, Mutex}};

use crate::basic_attributes::Texture2D;

/// ## Description
/// The [Asset] trait is associated with every [Attribute](crate::Attribute) that has the ability to be **up- or downloaded**
/// into an **external file**, for example [Textures](crate::basic_attributes::Texture2D).
pub trait Asset: std::any::Any {
    fn clone_box(&self) -> Box<dyn Asset>;
    fn as_any(&self) -> &dyn std::any::Any;
}

impl Clone for Box<dyn Asset> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait AsAny: std::any::Any {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Cast<T> {
    fn cast(&self) -> Option<&T>;
}

pub trait Safecast {
    fn safecast<T: 'static + Asset>(&self) -> Option<&T>;
}

impl Safecast for Box<dyn Asset> {
    fn safecast<T: 'static + Asset>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

impl<T: 'static + Asset> Cast<T> for Box<dyn Asset> {
    fn cast(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}


pub(in crate) trait AssetExt: Asset {
    fn is<T: std::any::Any>(&self) -> bool {
        self.as_any().type_id() == TypeId::of::<T>()
    }

    fn to<T: std::any::Any>(&self) -> &T {
        if let Some(any) = self.as_any().downcast_ref::<T>() {
            return any;
        }
        panic!("Failed to cast Asset.");
    }

    fn try_to<T: std::any::Any>(&self) -> Option<&T> {
        if let Some(any) = self.as_any().downcast_ref::<T>() {
            return Some(any);
        }
        return None;
    }
}

static mut TEXTURES: LazyLock<Arc<Mutex<HashMap<String, Texture2D>>>> = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

/// ## Description
/// The [Assets] struct is Pine's **asset browser**. You can use `Assets::add` to add an asset to the asset browser
/// (or manually: the path of the asset browser is always 'root/assets/...') or (more common) use `Assets::get` to retrieve
/// an asset from out of the asset browser and *load* it as an actual [Attribute](crate::Attribute).
/// 
/// ## Example
/// ```
/// fn __start__(commands: &mut Commands) {
///     let texture = Assets::get::<Texture2D>("my_texture.png").unwrap();
///     println!("successfully loaded texture {texture:?}!");
/// }
/// ```
pub struct Assets;

impl Assets {
    pub(in crate) unsafe fn get_texture_2d(res_name: &str) -> Option<Texture2D> {
        let mut res = res_name.to_string();

        if !res.contains(".png") || !res.contains(".jpg") {
            res.push_str(".png");
        }

        let name = format!("assets/textures/{res}");
        let file = Path::new(&name);

        if !TEXTURES.lock().unwrap().contains_key(file.to_str().unwrap()) {
            if file.exists() {
                Assets::add(res_name, Texture2D::new(name.clone()));
            } else {
                return None;
            }
        }

        return Some(TEXTURES.lock().unwrap().get(file.to_str().unwrap()).unwrap().clone());
    }

    /// ## Description
    /// `Assets::get` tries to download an [Asset] from a file into an actual [Attribute](crate::Attribute) that
    /// you can work with in your code.
    /// ## Example
    /// ```
    /// Assets::get::<Texture2D>("name_of_my_texture.png").unwrap();
    /// ```
    pub fn get<T: Asset + 'static + Clone>(res_name: &str) -> Option<T> {
        unsafe {
            if let Some(texture) = Assets::get_texture_2d(res_name) {
                return Some((Box::new(texture) as Box<dyn Asset>).safecast::<T>()?.clone());
            }
        }

        None
    }

    /// ## Description
    /// `Assets::add` is used to **upload** code representations of [Attributes](crate::Attribute) into a file
    /// format for storage and saving. You can upload every attribute that implements the [Asset] trait.
    pub fn add(res_name: &str, asset: impl Asset + AssetExt) {
        if asset.is::<Texture2D>() {
            Assets::add_texture2d(res_name, asset.to::<Texture2D>().clone());
        }
    }

    fn add_texture2d(res_name: &str, texture: Texture2D) {
        let mut res = res_name.to_string();

        if !res_name.contains(".jpg") || !res_name.contains(".png") {
            res.push_str(".png");
        }

        let name = format!("assets/textures/{res}");
        let file = Path::new(&name);

        unsafe {
            if !TEXTURES.lock().unwrap().contains_key(file.to_str().unwrap()) {
                TEXTURES.lock().unwrap().insert(file.to_str().unwrap().to_string(), texture);
            } else {
                println!("WARNING: you tried to add texture '{res_name}' but this texture is already uploaded!");
            }
        }
    }
}