//! # Payout Lines
//! Creation, handling and hightlighting of pay lines.
//! *Version*   :   0.1.0
//! *Author*    :   Alex Feterman
//! *Created*   :   Wed 11/03/2021 02:20 PM
//! *Modified*  :   Fri 01/21/2022 02:40 PM


use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, throw_str};
use js_sys::Math;
use web_sys::{/*console,*/ HtmlImageElement};
use lazy_static::lazy_static;
use super::assets::*;
use swirl::*;




pub fn get_random_lines() -> Vec<Box<[u8; 5]>> {
    /// A list of zero-based row positions
    /// for pay lines to match.
    static PAY_LINE_LAYOUTS: [[u8; 5]; 25] = [
        /*  1 */ [1, 1, 1, 1, 1],
        /*  2 */ [0, 0, 0, 0, 0],
        /*  3 */ [2, 2, 2, 2, 2],
        /*  4 */ [0, 1, 2, 1, 0],
        /*  5 */ [2, 1, 0, 1, 2],
        /*  6 */ [0, 0, 1, 0, 0],
        /*  7 */ [2, 2, 1, 2, 2],
        /*  8 */ [1, 0, 0, 0, 1],
        /*  9 */ [1, 2, 2, 2, 1],
        /* 10 */ [0, 0, 1, 2, 2],
        /* 11 */ [2, 2, 1, 0, 0],
        /* 12 */ [1, 0, 1, 2, 1],
        /* 13 */ [1, 2, 1, 0, 1],
        /* 14 */ [0, 1, 0, 1, 0],
        /* 15 */ [2, 1, 2, 1, 2],
        /* 16 */ [0, 1, 1, 1, 0],
        /* 17 */ [2, 1, 1, 1, 2],
        /* 18 */ [1, 0, 1, 0, 1],
        /* 19 */ [1, 2, 1, 2, 1],
        /* 20 */ [1, 1, 0, 1, 1],
        /* 21 */ [1, 1, 2, 1, 1],
        /* 22 */ [0, 2, 2, 2, 0],
        /* 23 */ [2, 0, 0, 0, 2],
        /* 24 */ [0, 2, 0, 2, 0],
        /* 25 */ [2, 0, 2, 0, 2],
    ];
    //
    lazy_static!{
        pub static ref CUM_LINE_PROBS: [f64; 24] = {
            let inverse_pow_sum = (1..=25).fold(0f64, |acc, curr| acc + 1. / Math::pow(curr as f64, 1.25) as f64);
            let mut probs       = [0f64; 24];
            let mut acc         = 0f64;
            for i in 0..24u16 {
                acc                 += 1. / Math::pow((i + 1) as f64, 1.25) as f64 / inverse_pow_sum;
                probs[i as usize]    = acc;
                //console::log_1(&JsValue::from_str(&format!("CUM_LINE_PROBS[{}] = {}", i, probs[i as usize])));
            }
            probs
        };
    }
    //
    fn get_random_line_count() -> usize {
       let rnd = Math::random();
       //console::log_1(&JsValue::from_str(&format!("random to get line count from: {}", rnd)));
       for i in 0 .. CUM_LINE_PROBS.len() {
           if rnd <= CUM_LINE_PROBS[i] {
               //console::log_1(&JsValue::from_str(&format!("{} under {}, returning index {}", rnd, CUM_LINE_PROBS[i], i + 1)));
               return i + 1;
           }
           else {
               //console::log_1(&JsValue::from_str(&format!("Greater than {}", CUM_LINE_PROBS[i])));
           }
       }
       25
    }
    //
    let line_count  = get_random_line_count();
    let mut vec     = Vec::<usize>::with_capacity(line_count);
    while vec.len() < line_count {
        let rnd = (Math::random() * 25.) as usize;
        if vec.binary_search(&rnd).is_err() {
            vec.push(rnd);
        }
    }
    vec
        .iter()
        .map(|line| {
            let mut arr = Box::new([0u8; 5]);
            for i in 0..5usize {
                arr[i] = PAY_LINE_LAYOUTS[*line][i];
            }
            arr
        })
        .collect::<Vec<_>>()
}


#[derive(Clone, Debug)]
pub struct InvalidLayoutRows;

#[derive(Clone, Debug)]
pub struct PaylineAnimations {
    all_around  : [(SceneGraphNode<f32>, SpriteMovie); 5],
    left_cap    : [(SceneGraphNode<f32>, SpriteMovie); 2],
    middle      : [(SceneGraphNode<f32>, SpriteMovie); 3],
    right_cap   : [(SceneGraphNode<f32>, SpriteMovie); 2],
}

impl PaylineAnimations {
    pub fn new(assets: &AssetsStruct) -> Result<Self, SpriteMovieError> {
        //
        fn make_pair(images: &[HtmlImageElement]) -> Result<(SceneGraphNode<f32>, SpriteMovie), SpriteMovieError> {
            let movie       = SpriteMovie::new(super::MSPF as u16, images)?;
            let mut node    = SceneGraphLeaf::new_node(&movie);
            node.set_pivot(60, 60);
            Ok((node, movie))
        }
        //
        let anims = &assets.imgs.lightning_anim;
        Ok(PaylineAnimations {
            all_around  : [ make_pair(&anims.all_around)?
                          , make_pair(&anims.all_around)?
                          , make_pair(&anims.all_around)?
                          , make_pair(&anims.all_around)?
                          , make_pair(&anims.all_around)?
                          ],
            left_cap    : [ make_pair(&anims.left_cap)? 
                          , make_pair(&anims.left_cap)?
                          ],
            middle      : [ make_pair(&anims.middle_1)?
                          , make_pair(&anims.middle_2)?
                          , make_pair(&anims.middle_3)?
                          ],
            right_cap   : [ make_pair(&anims.right_cap)?
                          , make_pair(&anims.right_cap)?
                          ],
        })
    }

    fn line_layout_to_movies(
        &self,
        layout  : &Box<[u8; 5]>
    )
        -> Result<Vec<(SceneGraphNode<f32>, SpriteMovie)>, InvalidLayoutRows>
    {
        if layout.iter().any(|row| *row >= 3) {
            Err(InvalidLayoutRows)
        }
        else {
            let mut all_around_counter  : usize                                     = 0;
            let mut left_cap_counter    : usize                                     = 0;
            let mut middle_counter      : usize                                     = 0;
            let mut right_cap_counter   : usize                                     = 0;
            let mut vec                 : Vec<(SceneGraphNode<f32>, SpriteMovie)>   = Vec::with_capacity(layout.len());
            for i in 0..layout.len() {
                let prev    = if i == 0 { None } else { Some(layout[i - 1]) };
                let curr    = Some(layout[i]);
                let next    = if i == 4 { None } else { Some(layout[i + 1]) };
                let movie   =
                    if prev == curr {
                        if curr == next {
                            let movie = &self.middle[middle_counter];
                            middle_counter += 1;
                            movie
                        }
                        else {
                            let movie = &self.right_cap[right_cap_counter];
                            right_cap_counter += 1;
                            movie
                        }
                    }
                    else if curr == next {
                        let movie = &self.left_cap[left_cap_counter];
                        left_cap_counter += 1;
                        movie
                    }
                    else {
                        let movie = &self.all_around[all_around_counter];
                        all_around_counter += 1;
                        movie
                    };
                let mut movie = movie.clone();
                movie.0.set_pivot(60, 60);
                //console::log_1(&JsValue::from_str(&format!("electric movie transform: {:?}", movie.0.update_world_transform_from_top().1)));
                movie.1.set_blend_mode(BlendMode::Screen);
                vec.push(movie);
            }
            Ok(vec)
        }
    }

    pub fn play_random_line_movie_sets(
        &self,
        scene   : &mut SceneGraphFrame::<f32>,
        start_x : i16,
        start_y : i16
    )
        -> Result<Box<dyn FnOnce()>, JsValue>
    {
        let line_layouts        = get_random_lines();
        //console::log_1(&JsValue::from_str(&format!("Playing random lines: {} sets.", line_layouts.len())));
        let mut line_movie_sets = Vec::<Vec<RefCell<(SceneGraphNode<f32>, SpriteMovie)>>>::with_capacity(line_layouts.len());
        for layout in line_layouts.iter() {
            //console::log_1(&JsValue::from_str(&format!("{:?}", &layout)));
            line_movie_sets.push(
                self
                    .line_layout_to_movies(layout)
                    .map_err(|_| JsValue::from_str("Invalid layout row"))?
                    .into_iter()
                    .map(|mov| RefCell::new(mov))
                    .collect::<Vec<_>>()
            );
            //console::log_1(&JsValue::from_str(&format!("{} node+movie entries.", line_movie_sets[line_movie_sets.len() - 1].len())));
        }
        let stopped             = Rc::new(Cell::new(false));
        let line_movie_sets     = Rc::new(line_movie_sets);
        let current_set         = Rc::new(Cell::new(0usize));
        let play_closure_mem: Rc<RefCell<Option<Closure<dyn FnMut()>>>>
                                = Rc::new(RefCell::new(None));
        let play_closure        = {
            let current_set         = current_set.clone();
            let line_movie_sets     = line_movie_sets.clone();
            let mut scene           = scene.clone();
            let play_closure_mem    = play_closure_mem.clone();
            let stopped             = stopped.clone();
            Closure::wrap(Box::new(move || {
                //console::log_1(&JsValue::from_str(&format!("Starting on movie set {}", current_set.get())));
                let movies_completed    = Rc::new(Cell::new(0usize));
                for (mov_idx, movie) in line_movie_sets[current_set.get()].iter().enumerate() {
                    movie.borrow_mut().0.set_x_and_y(
                        start_x + mov_idx as i16                                    * super::ICON_WIDTH  as i16,
                        start_y + line_layouts[current_set.get()][mov_idx] as i16   * super::ICON_HEIGHT as i16
                    );
                    scene.append_child(movie.borrow().0.clone()).unwrap_throw();
                    debug_assert_eq!(movie.borrow().0.parent(), &Some(scene.clone()));
                    let listener_mem: Rc<Cell<Option<usize>>> = Rc::new(Cell::new(None));
                    let listener    = {
                        let mut scene           = scene.clone();
                        let mut movie1          = movie.borrow().1.clone();
                        let current_set         = current_set.clone();
                        let movies_completed    = movies_completed.clone();
                        let line_movie_sets     = line_movie_sets.clone();
                        let play_closure_mem    = play_closure_mem.clone();
                        let stopped             = stopped.clone();
                        let listener_mem        = listener_mem.clone();
                        move |_ev: &SpriteMovieEvent| {
                            if !stopped.get() {
                                let new_movies_completed = movies_completed.get() + 1;
                                movies_completed.set(new_movies_completed);
                                if new_movies_completed == line_movie_sets[current_set.get()].len() {
                                    //console::log_1(&JsValue::from_str(&format!("Stopping movie set {}", current_set.get())));
                                    for cell in line_movie_sets[current_set.get()].iter() {
                                        scene.remove_child(&cell.borrow().0.clone()).expect_throw("Couldn't remove lightning anim from scene");
                                    }
                                    let new_current_set = (current_set.get() + 1) % line_movie_sets.len();
                                    current_set.set(new_current_set);
                                    if let Some(ref mut closure) = *play_closure_mem.borrow_mut() {
                                        //console::log_1(&JsValue::from_str("Iterating on next set"));
                                        get_window()
                                            .unwrap_throw()
                                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                closure.as_ref().unchecked_ref(),
                                                0
                                            )
                                            .unwrap_throw();
                                    }
                                    else {
                                        throw_str("Failure to find closure");
                                    }
                                }
                            }
                            else {
                                //console::log_1(&JsValue::from_str("Can't iterate to next anim set: already stopped."));
                            }
                            movie1.unsubscribe(SpriteMovieEventType::Stop, listener_mem.get().unwrap());
                            ActionAfterEvent::NoAction
                        }
                    };
                    listener_mem.set(Some(movie.borrow_mut().1.subscribe(SpriteMovieEventType::Stop, listener).expect_throw("Couldn't add movie-stop event listener")));
                    movie.borrow_mut().1.play(false).unwrap_throw();
                }
            }) as Box<dyn FnMut()>)
        };
        get_window()?
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                play_closure.as_ref().unchecked_ref(),
                0
            )?;
        play_closure_mem.replace(Some(play_closure));
        let interrupt = {
            let mut scene = scene.clone();
            Box::new(move || {
                if !stopped.get() {
                    //console::log_1(&JsValue::from_str("Not stopped, stopping."));
                    stopped.set(true);
                    for movie in line_movie_sets[current_set.get()].iter() {
                        movie.borrow_mut().1.stop().unwrap_throw();
                        scene.remove_child(&movie.borrow().0.clone()).unwrap_throw();
                    }
                    play_closure_mem.replace(None);
                }
                else {
                    //console::log_1(&JsValue::from_str("Already stopped."));
                }
            }) as Box<dyn FnOnce()>
        };
        Ok(interrupt)
    }
}
