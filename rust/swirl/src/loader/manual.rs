//! # Module        :   _Manual Loader_
//! - **Purpose**   :   Basic assets loader, which requires the user to specify
//!                     each resource to load manually along with a unique key
//!                     (which must be `Eq` and `Hash`.) Retrival of loaded
//!                     assets is done through said key.
//!                     Currently, only image loading is implemented and tested.
//!                     Font loading results remain untested.
//! - **Version**   :   0.1.0
//! - **Author**    :   Alex Feterman
//! - **Created**   :   2021-02-09 14:57
//! - **Modified**  :   Wed 25 May 2022 05:38:26 PM CST


use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_str, JsCast};
use js_sys::{Function, RegExp};
use web_sys::{
    //console,
    Document,
    HtmlHeadElement, HtmlStyleElement,
    HtmlLabelElement,
    HtmlImageElement, HtmlAudioElement, HtmlSourceElement,
    XmlHttpRequest, XmlHttpRequestResponseType
};


pub type Url = String;


/// Specifies different types of image resizing.
#[derive(Clone, Copy, Debug)]
pub enum Resize {
    NoResize,
    ScaleEqually(f32),
    Scale(f32, f32),
    PixelSize(i16, i16),
}


#[derive(Clone, Debug)]
enum Request {
    Image(Url, Resize),
    Sound(Vec<Url>),
    Font(Url),
}


#[derive(Debug)]
enum Pending {
    Image(HtmlImageElement, Resize, Closure<dyn FnMut()>),
    Sound(HtmlAudioElement, Closure<dyn FnMut()>),
    Font(HtmlStyleElement, HtmlLabelElement, Closure<dyn FnMut()>),
}


#[derive(Clone, Debug)]
pub enum Loaded {
    Image(HtmlImageElement),
    Sound(HtmlAudioElement),
    Font(String),
}


struct BasicAssetLoaderInner<K: Eq + Hash> {
    asset_requests  : Option<HashMap<K, Request>>,
    pending_assets  : HashMap<K, Pending>,
    loaded_assets   : HashMap<K, Loaded>,
    progress_cbs    : Vec<Box<dyn FnMut(f64)>>,
    loaded_cbs      : Vec<Box<dyn FnMut(&mut HashMap<K, Loaded>)>>,
}

#[derive(Clone)]
pub struct BasicAssetLoader<K: Eq + Hash>(Rc<RefCell<BasicAssetLoaderInner<K>>>);


impl<K> BasicAssetLoader<K>
    where K: Hash + Eq + Clone + Display + 'static
{
    const INITIAL_CAPACITY: usize = 32;

    pub fn new() -> Self {
        BasicAssetLoader(Rc::new(RefCell::new(BasicAssetLoaderInner {
            asset_requests  : Some(HashMap::with_capacity(Self::INITIAL_CAPACITY)),
            pending_assets  : HashMap::with_capacity(Self::INITIAL_CAPACITY),
            loaded_assets   : HashMap::with_capacity(Self::INITIAL_CAPACITY),
            progress_cbs    : Vec::with_capacity(Self::INITIAL_CAPACITY),
            loaded_cbs      : Vec::with_capacity(Self::INITIAL_CAPACITY),
        })))
    }

    pub fn percentage_loaded(&self) -> f64 {
        let f_requested = self.0.borrow().asset_requests.as_ref().map_or(0, |reqs| reqs.len()) as f64;
        let f_pending   = self.0.borrow().pending_assets.len() as f64;
        let f_loaded    = self.0.borrow().loaded_assets.len()  as f64;
        f_loaded * 100.0 / f64::max(1.0, f_requested + f_loaded + f_pending)
    }

    fn call_progress_cbs(&mut self, percentage: f64) {
        for progress_cb in &mut self.0.borrow_mut().progress_cbs {
            progress_cb(percentage);
        }
    }

    fn call_loaded_cbs(&mut self) {
        let (mut loaded_cbs, mut loaded_assets) = RefMut::map_split(
            self.0.borrow_mut(),
            |inner| (&mut inner.loaded_cbs, &mut inner.loaded_assets)
        );
        for progress_cb in &mut *loaded_cbs {
            progress_cb(&mut *loaded_assets);
        }
    }

    fn add_request(&mut self, id: K, request: Request) -> Result<&mut Self, JsValue> {
        //console::log_1(&JsValue::from_str(&format!("add_request({:?})", request)));
        let asset_type_name = match request {
            Request::Image(_, _)    => &"image",
            Request::Font(_)        => &"font",
            Request::Sound(_)       => &"sound",
        };
        if let &mut Some(ref mut reqs) = &mut self.0.borrow_mut().asset_requests {
            if reqs.contains_key(&id) {
                Err(JsValue::from_str(&format!(
                    "loader::BasicAssetLoader::add_request: an asset with id '{}' already exists.",
                    id
                )))?
            }
            else {
                //  The list of extensions might seem too short, especially in
                //  contrast with previous TypeScript/ReasonML libraries. However,
                //  in my experience, these are all we've ever needed.
                let (valid_exts, urls)  =   match &request {
                    &Request::Image(ref url, _) => (&["png", "jpg", "jpeg", "svg"] as &[&'static str], vec![url]),
                    &Request::Sound(ref urls)   => (&["ogg", "mp3"] as &[&'static str], urls.iter().collect::<Vec<_>>()),
                    &Request::Font(ref url)     => (&["woff"] as &[&'static str], vec![url]),
                };
                let ext_re              =   "\\.(".to_string()
                                        +   &valid_exts.join("|")
                                        +   ")$";
                if urls.iter().all(|url| RegExp::new(&ext_re, "i").test(url)) {
                    reqs.insert(id, request);
                }
                else {
                    Err(JsValue::from_str(&format!(
                        "loader::BasicAssetLoader::add_request: requests for {}s must have extension(s) {}.",
                        asset_type_name,
                        valid_exts.join(", ")
                    )))?
                }
            }
        }
        else {
            Err(JsValue::from_str(&format!(
                "loader::BasicAssetLoader::add_request: cannot add {} after loading has started",
                asset_type_name
            )))?
        }
        Ok(self)
    }

    #[inline(always)]
    pub fn add_image_request(&mut self, id: K, img_url: String, resize: Resize) -> Result<&mut Self, JsValue> {
        self.add_request(id, Request::Image(img_url, resize))
    }

    #[inline(always)]
    pub fn add_font_request(&mut self, id: K, font_url: String) -> Result<&mut Self, JsValue> {
        self.add_request(id, Request::Font(font_url))
    }

    #[inline(always)]
    pub fn add_sound_request(&mut self, id: K, sound_urls: Vec<String>) -> Result<&mut Self, JsValue> {
        self.add_request(id, Request::Sound(sound_urls))
    }


    pub fn add_progress_listener<F>(&mut self, listener: F) -> Result<&mut Self, JsValue>
        where   F   : 'static + FnMut(f64)
    {
        if self.0.borrow().asset_requests.is_some() {
            self.0.borrow_mut().progress_cbs.push(Box::new(listener));
            Ok(self)
        }
        else {
            Err(JsValue::from_str(
                &"loader::BasicAssetLoader::add_progress_listener: cannot add listeners after loading has started"
            ))
        }
    }

    pub fn add_loaded_listener<F>(&mut self, listener: F) -> Result<&mut Self, JsValue>
        where   F   : 'static + FnMut(&mut HashMap<K, Loaded>)
    {
        if self.0.borrow().asset_requests.is_some() {
            self.0.borrow_mut().loaded_cbs.push(Box::new(listener));
            Ok(self)
        }
        else {
            Err(JsValue::from_str(
                &"loader::BasicAssetLoader::add_loaded_listener: cannot add listeners after loading has started"
            ))
        }
    }

    //  Notify progress listeners, and if the pending list is empty,
    //  Signal the "all loaded" listeners.
    fn notify_listeners_after_item_loaded(&mut self) {
        if self.0.borrow().pending_assets.len() == 0 {
            self.call_progress_cbs(100.0);
            self.call_loaded_cbs();
        }
        else {
            let pct = self.percentage_loaded();
            self.call_progress_cbs(pct);
        }
    }


    //  **Loading images**: we need to crate an `image` element and set the source so the
    //  browser will load it and let us know throught the `onload` callback. Fairly simply,
    //  really. *Note*: you don't need to clone a `Rc<RefCell<Self>>` to pass as parameter --
    //  the function will it as needed.
    fn load_image(&mut self, id: K, url: &str, resize: Resize) -> Result<(), JsValue> {
        let mut self_clone  = self.clone();
        let id_clone        = id.clone();
        let html_img        = HtmlImageElement::new()?;
        let closure         = {
            let url             = url.to_string();
            move || {
                let id_clone        = id_clone.clone();
                let opt_asset       = self_clone.0.borrow_mut().pending_assets.remove(&id_clone);
                if let Some(Pending::Image(html_img, resize, _)) = opt_asset {
                    html_img.set_onload(None);
                    let (new_width, new_height) =
                        match resize {
                            Resize::PixelSize(w, h) => {
                                let new_width   = w.abs() as u32;
                                let new_height  = h.abs() as u32;
                                //html_img.set_width (w.abs() as u32);
                                //html_img.set_height(h.abs() as u32);
                                html_img.set_width (new_width);
                                html_img.set_height(new_height);
                                (new_width, new_height)
                            },
                            Resize::Scale(sw, sh)        => {
                                let new_width   = (sw * html_img.width()  as f32) as u32;
                                let new_height  = (sh * html_img.height() as f32) as u32;
                                //html_img.set_width ((sw * html_img.width()  as f32) as u32);
                                //html_img.set_height((sh * html_img.height() as f32) as u32);
                                html_img.set_width (new_width);
                                html_img.set_height(new_height);
                                (new_width, new_height)
                            },
                            Resize::ScaleEqually(s)        => {
                                let new_width   = (s  * html_img.width()  as f32) as u32;
                                let new_height  = (s  * html_img.height() as f32) as u32;
                                html_img.set_width (new_width);
                                html_img.set_height(new_height);
                                (new_width, new_height)
                            },
                            Resize::NoResize        =>
                                (html_img.width(), html_img.height()),
                        };
                    assert_eq!(new_width,  html_img.width(),  "Unequal width on loaded image {}",  &url);
                    assert_eq!(new_height, html_img.height(), "Unequal height on loaded image {}", &url);
                    self_clone.0.borrow_mut().loaded_assets.insert(id_clone, Loaded::Image(html_img));
                    self_clone.notify_listeners_after_item_loaded();
                }
                else {
                    throw_str("Internal error: load-pending image lost.");
                }
            }
        };
        let closure     = Closure::wrap(Box::new(closure) as Box<dyn FnMut()>);
        html_img.set_onload(Some(closure.as_ref().unchecked_ref()));
        html_img.set_src(url);
        self.0.borrow_mut().pending_assets.insert(id, Pending::Image(html_img, resize, closure));
        Ok(())
    }

    //  **Loading sounds**: loading one of multiple possible sounds is left to the browser,
    //  as one lists the possible sources and the browser picks one. Awaiting for a playable
    //  sound is a matter of listening to the `canplaythrough` event.
    fn load_sound(&mut self, doc: &Document, id: K, urls: &[String]) -> Result<(), JsValue> {
        //console::log_1(&JsValue::from_str(&format!("load_sound(urls:{:?})", urls)));
        let audio_tag   = HtmlAudioElement::new()?;
        audio_tag.set_autoplay(false);
        audio_tag.set_muted(false);
        audio_tag.set_loop(false);
        audio_tag.set_preload("none");
        for src in urls {
            doc
                .create_element("source")
                .and_then(|element|
                    Ok(
                        (   element.dyn_into::<HtmlSourceElement>()?
                        ,   {
                                let ext = src
                                    .split('.')
                                    .last()
                                    .ok_or_else(|| JsValue::from_str(&format!(
                                        "BasicAssetLoader::load_sound: \
                                        No extension provided for file \"{}\".",
                                        src
                                    )))?
                                    .to_lowercase();
                                match &ext[..] {
                                    "mp3"   =>  r#"audio/mpeg; codecs="mp3""#,
                                    "ogg"   =>  r#"audio/ogg; codecs="vorbis""#,
                                    _       =>  {
                                        Err(JsValue::from_str(&format!(
                                            "BasicAssetLoader::load_sound: \
                                            Invalid audio extension \".{}\"",
                                            ext
                                        )))?;
                                        ""
                                    },
                                }
                            }
                        )
                    )
                )
                .and_then(|(source_tag, codec)| {
                    source_tag.set_src(src);
                    source_tag.set_type(codec);
                    audio_tag.append_with_node_1(&source_tag)
                })?;
        }
        //console::log_1(audio_tag.as_ref());
        let closure     = {
            let id          = id.clone();
            let mut this    = self.clone();
            Closure::wrap(
                Box::new(move || {
                    let removed = this.0.borrow_mut().pending_assets.remove(&id.clone());
                    if let Some(Pending::Sound(audio_tag, _)) = removed {
                        //console::log_1(&JsValue::from_str("Recovered audio tag"));
                        this.0.borrow_mut().loaded_assets.insert(id.clone(), Loaded::Sound(audio_tag.clone()));
                        this.notify_listeners_after_item_loaded();
                        //console::log_1(&JsValue::from_str("Listeners notified"));
                        audio_tag.set_oncanplaythrough(None);
                        //console::log_1(&JsValue::from_str("Leaving function"));
                    }
                    else {
                        throw_str("Internal error: load-pending sound lost.");
                    }
                }) as Box<dyn FnMut()>
            )
        };
        audio_tag.set_oncanplaythrough(Some(closure.as_ref().unchecked_ref::<Function>()));
        self.0.borrow_mut().pending_assets.insert(id, Pending::Sound(audio_tag.clone(), closure));
        audio_tag.load();
        Ok(())
    }

    //  **Loading fonts**: usually you have to create the CSS3 `@font-face` entry and use it somewhere
    //  in your document for the font to be active. However, there's no way to monitor when the
    //  font download, which happens behind the scenes. We reify the request through an `XMLHttpRequest`
    //  and *then* create the `@font-face` and (hidden) `HTMLLabelElement`.
    //  *Note*: you don't need to pass cloned `Rc` instances at all. The function creates its own
    //  clones as needed.
    fn load_font(&mut self, doc: &Document, id: K, url: &str) -> Result<(), JsValue> {
        //console::log_1(&JsValue::from_str("Starting load_font"));
        //  Create the static copies.
        let mut self_copy       = self.clone();
        let doc_copy            = doc.clone();
        let id_copy             = id.clone();
        //  Create document elements required to *name* and *activate* the font.
        let font_name           = format!("{}", &id_copy)
            .split('/')
            .last()
            .unwrap()
            .split('.')
            .nth(0)
            .unwrap()
            .split('\'')
            .collect::<Vec<_>>()
            .join("\\'");
        let label_tag   = doc_copy
            .create_element("label")?
            .unchecked_into::<HtmlLabelElement>();
        label_tag.set_inner_html(".");
        label_tag.style().set_css_text(&format!("visibility: hidden; font-family: \"{}\";", &font_name));
        let style_tag   = doc_copy
            .create_element("style")?
            .unchecked_into::<HtmlStyleElement>();
        style_tag.set_inner_html(&format!(
            r#"@font-face {{
                font-family: '{}';
                src: url('{}') format('woff');
            }}"#,
            &font_name,
            url
        ));
        //  Start the XHR request
        //console::log_1(&JsValue::from_str("Initializing XHR and closure"));
        let xhr         = Rc::new(XmlHttpRequest::new()?);
        let xhr_copy    = xhr.clone();
        xhr.open_with_async("GET", url, true)?;
        xhr.set_response_type(XmlHttpRequestResponseType::Arraybuffer);
        //  This closure will respond to the `load` event (i.e., will
        //  listen for successful download.)
        let closure     = Closure::wrap(Box::new(move || {
            //console::log_1(&JsValue::from_str("Inside font-loading closure"));
            //  First, we clear the listener to avoid drop errors.
            xhr_copy.set_onload(None);
            //  Get the document's HEAD (or create it.) We'll need it
            //  to insert the style tag in it.
            let head        = match doc_copy.head() {
                Some(head)  =>
                    head,
                None        => {
                    let head = doc_copy
                        .create_element("head")
                        .expect_throw("loader: cannot create missing HEAD element.")
                        .unchecked_into::<HtmlHeadElement>();
                    doc_copy.prepend_with_node_1(&head)
                        .expect_throw("loader: could not insert missing HEAD element.");
                    head
                },
            };
            //  Extract the "pending" entry. It doesn't
            //  belong there any more.
            //console::log_1(&JsValue::from_str("Trying to extract font resource."));
            let opt_asset   = self_copy.0.borrow_mut().pending_assets.remove(&id_copy);
            if let Some(Pending::Font(style, label, _)) = opt_asset {
                //  Insert the style declaring the font-face in the head.
                //console::log_1(&JsValue::from_str("Succeeding extracting font resource. Inserting style/label nodes."));
                head.append_with_node_1(&style)
                    .expect_throw(&format!(
                        "loader: couldn't append style declaration for font-family {} to head.",
                        font_name.clone()
                    ));
                //  And the label using it in the body.
                doc_copy
                    .body()
                    .expect_throw("No BODY tag found.")
                    .prepend_with_node_1(&label)
                    .expect_throw(&format!(
                        "loader: couldn't insert label using font-family {} in document.",
                        font_name.clone()
                    ));
                //  We need a copy of the font name not to spend the one we have.
                //  Insert the asset in the "ready" list.
                //console::log_1(&JsValue::from_str("Nodes inserted, creating id and font name copies."));
                let id_copy     = id_copy.clone();
                let font_name   = font_name.clone();
                //console::log_1(&JsValue::from_str("Inserting font name in loaded list."));
                self_copy
                    .0.borrow_mut()
                    .loaded_assets.insert(id_copy, Loaded::Font(font_name));
                //  And call the notifiers.
                //console::log_1(&JsValue::from_str("Calling notifiers."));
                self_copy.notify_listeners_after_item_loaded();
                //console::log_1(&JsValue::from_str("Notifiers called."));
            }
            else {
                //console::log_1(&JsValue::from_str("Font resource extraction failed."));
                throw_str(&format!(
                    "Internal error: pending font resource '{}' not found as such.",
                    font_name
                ));
            }
            //console::log_1(&JsValue::from_str("All done. Exiting font-loading function."));
        }) as Box<dyn FnMut()>);
        //  Final steps: we set the closure to listen.
        xhr.set_onload(Some(closure
            .as_ref()
            .dyn_ref::<Function>()
            .ok_or_else(|| JsValue::from_str(
                "Cannot set listener for XHR font download progress."
            ))?
        ));
        //  Then save the pending assets to the 
        self.0.borrow_mut().pending_assets.insert(
            id,
            Pending::Font(style_tag, label_tag, closure)
        );
        //console::log_1(&JsValue::from_str("Sending XHR request"));
        xhr.send()?;
        Ok(())
    }

    /// Loads all assets, triggering progress notifiers. If all resources
    /// load, the completion notifiers will be called as well.
    pub fn load_all(&mut self) -> Result<(), JsValue> {
        let reqs = std::mem::replace(&mut self.0.borrow_mut().asset_requests, None);
        if let Some(mut reqs) = reqs {
            if reqs.len() == 0 {
                self.call_progress_cbs(100.0);
                self.call_loaded_cbs();
                Ok(())
            }
            else {
                let doc         = crate::utils::get_document()?;
                for (id, asset_req) in reqs.drain() {
                    match asset_req {
                        Request::Image(url, resize) => self.load_image(id, &url, resize)?,
                        Request::Sound(urls)        => self.load_sound(&doc, id, &urls)?,
                        Request::Font(url)          => self.load_font(&doc, id, &url)?,
                    }
                }
                Ok(())
            }
        }
        else {
            Err(JsValue::from_str(
                "<swirl>::loader::BasicAssetLoader::load_all: loading cannot be called multiple times."
            ))
        }
    }
}
