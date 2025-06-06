use crate::ui::*;
use druid::Color;
use druid::{
    commands, widget::Controller, AppDelegate, Command, Cursor, Data, DelegateCtx, Env, Event,
    EventCtx, Handled, ImageBuf, Lens, LocalizedString, MouseEvent, Point, Selector, Size, Target,
    TimerToken, Widget, WindowDesc, WindowState,
};
use druid_shell::keyboard_types::{Key, KeyboardEvent, Modifiers, ShortcutMatcher};
use image::{ImageBuffer, Rgba};
use num_complex::ComplexFloat;
use std::path::Path;
use std::time::Duration;
use rusttype::{Scale,Font};

pub const SHORTCUT: Selector = Selector::new("shortcut_selector");

#[derive(Clone, Data, PartialEq, Debug)]
pub enum ImageFormat {
    Jpeg,
    Png,
    Gif,
    WebP,
    Pnm,
    Tiff,
    Tga,
    Dds,
    Bmp,
    Ico,
    Hdr,
    OpenExr,
    Farbfeld,
    Avif,
    Qoi,
}

impl ImageFormat {
    pub fn to_string(&self) -> String {
        match self {
            ImageFormat::Jpeg => ".jpeg".to_string(),
            ImageFormat::Png => ".png".to_string(),
            ImageFormat::Gif => ".gif".to_string(),
            ImageFormat::WebP => ".webp".to_string(),
            ImageFormat::Pnm => ".pnm".to_string(),
            ImageFormat::Tiff => ".tiff".to_string(),
            ImageFormat::Tga => ".tga".to_string(),
            ImageFormat::Dds => ".dds".to_string(),
            ImageFormat::Bmp => ".bmp".to_string(),
            ImageFormat::Ico => ".ico".to_string(),
            ImageFormat::Hdr => ".hdr".to_string(),
            ImageFormat::OpenExr => ".openexr".to_string(),
            ImageFormat::Farbfeld => ".farbfeld".to_string(),
            ImageFormat::Avif => ".avif".to_string(),
            ImageFormat::Qoi => ".qoi".to_string(),
        }
    }
}

#[derive(Clone, Data, PartialEq, Debug)]
pub enum Timer {
    Zero,
    ThreeSeconds,
    FiveSeconds,
    TenSeconds,
    Custom,
}

impl Timer {
    pub fn set_timer(&self) -> u64 {
        match self {
            Timer::Zero => 0,
            Timer::ThreeSeconds => 3,
            Timer::FiveSeconds => 5,
            Timer::TenSeconds => 10,
            _ => 0,
        }
    }
}

#[derive(Clone, Data, PartialEq, Debug)]
pub enum Tools{
    No,
    Resize,
    Ellipse,
    HollowEllipse,
    Arrow,
    Text,
    Highlight,
    Redact,
    Random,
}

#[derive(Clone, Data, Lens, Debug)]
pub struct AppState {
    pub name: String,
    pub selected_format: ImageFormat,
    pub shortcut: String,
    pub mods: u32,
    pub key: u32,
    pub from: Point,
    pub size: Point,
    pub scale: f32,
    pub rect: SelectionRectangle,
    pub is_full_screen: bool, //true --> end of area selection
    pub selection_transparency: f64,
    pub img: ImageBuf,
    pub cursor: CursorData,
    pub path: String,
    pub delay: Timer,
    pub resize: bool,
    pub annulla: bool,
    pub tool_window: AnnotationTools,
    pub color: Color,
    pub text: String,
    pub screen_min_x: u32,
    pub screen_min_y: u32,
}


impl AppState {
    pub fn new(scale: f32, img: ImageBuf) -> Self {
        let display_info = screenshots::DisplayInfo::all().expect("Err");
        let mut min_x=0;
        let mut min_y=0;
        let mut width = display_info[0].width as f32 * display_info[0].scale_factor;
        let mut height = display_info[0].height as f32 * display_info[0].scale_factor;

        AppState {
            name: "".to_string(),
            selected_format: ImageFormat::Jpeg,
            shortcut: "".to_string(),
            mods: Modifiers::ALT.bits(),
            key: Key::Character("s".to_string()).legacy_charcode(),
            from: Point { x: 0.0, y: 0.0 },
            size: Point {
                x: width as f64,
                y: height as f64,
            },
            scale,
            rect: SelectionRectangle::default(),
            is_full_screen: false,
            selection_transparency: 0.4,
            img,
            cursor: CursorData::new(Cursor::Arrow, None, false),
            path: ".".to_string(),
            delay: Timer::Zero,
            resize: false,
            annulla: true,
            tool_window:AnnotationTools::default(),
            color:Color::rgba(0.,0.,0.,1.),
            text: "".to_string(),
            screen_min_x:0,
            screen_min_y:0,
        }
    }

    pub fn screen(&mut self, ctx: &mut EventCtx) {
        let a = screenshots::DisplayInfo::all();

        let display_info = match a {
            Err(why) => return println!("{}", why),
            Ok(info) => info,
        };
        let b = screenshots::Screen::new(&display_info[0]);

        let c;
        if self.is_full_screen {
            c = b.capture();
        } else {
            c = b.capture_area(
                (self.rect.start_point.unwrap().x as f32 * self.scale) as i32,
                (self.rect.start_point.unwrap().y as f32 * self.scale) as i32,
                (self.rect.size.width as f32 * self.scale) as u32,
                (self.rect.size.height as f32 * self.scale) as u32,
            );
            //self.rect = SelectionRectangle::default();
        }

        let image = match c {
            Err(why) => return println!("{}", why),
            Ok(info) => info,
        };

        self.img = ImageBuf::from_raw(
            image.clone().into_raw(),
            druid::piet::ImageFormat::RgbaPremul,
            image.clone().width() as usize,
            image.clone().height() as usize,
        );

        self.tool_window.img=Some(self.img.clone());

        let width = self.img.width() as f64;
        let height = self.img.height() as f64;
        println!("w: {}, h: {}", width, height);

        self.tool_window.img_size.width = self.tool_window.width;
        self.tool_window.img_size.height = height / (width / self.tool_window.width);
        if self.tool_window.img_size.height > self.tool_window.height {
            self.tool_window.img_size.height = self.tool_window.height;
            self.tool_window.img_size.width = width / (height / self.tool_window.height);
        }

        //println!("inizializzazione altezza:{},{}",self.tool_window.img_size.height,self.img.height());
        //println!("inizializzazione larghezza:{},{}",self.tool_window.img_size.width,self.img.width());

        self.tool_window.origin=druid::Point::new(
            self.tool_window.center.x-(self.tool_window.img_size.width/2.),
            self.tool_window.center.y-(self.tool_window.img_size.height/2.),
        );


        let window = WindowDesc::new(build_ui(self.img.clone()))
            .menu(make_menu)
            .title("Screen grabbing")
            .window_size((1000., 500.))
            .set_position(Point::new(0., 0.));
        ctx.new_window(window);
        //*self = AppState::new(self.scale, self.img.clone());
        self.selection_transparency = 0.4;

        //ctx.window().close();
    }

    pub fn set_default_name(&mut self) {
        let first: String;
        if self.name.is_empty() {
            first = String::from("screenshot");
        } else {
            first = self.name.clone();
        }

        let mut str3 = format!("{}{}", first, self.selected_format.to_string());

        let mut index = 0;
        loop {
            if !Path::new(&(self.path.clone() + "\\" + &str3)).exists() {
                //println!("{}", str3);
                break;
            } else {
                index += 1;
                str3 = format!(
                    "{}{}{}",
                    first,
                    index.to_string(),
                    self.selected_format.to_string()
                );
            }
        }
        if index == 0 {
            self.name = first;
        } else {
            self.name = format!("{}{}", first, index.to_string());
        }
    }

    pub fn save(&mut self) {
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(
            self.img.width() as u32,
            self.img.height() as u32,
            self.img.raw_pixels().to_vec(),
        )
        .unwrap();
        self.set_default_name();
        image
            .save_with_format(
                self.path.clone() + "\\" + &self.name + &self.selected_format.to_string(),
                image::ImageFormat::Png,
            )
            .expect("Error saving");
        self.name = "".to_string();
        //self.rect = SelectionRectangle::default();
    }

    pub fn save_as(&mut self, path: &Path) {
        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(
            self.img.width() as u32,
            self.img.height() as u32,
            self.img.raw_pixels().to_vec(),
        )
        .unwrap();
        image
            .save_with_format(path, image::ImageFormat::Png)
            .expect("Error saving");
        //self.rect = SelectionRectangle::default();
    }
}

#[derive(Clone, Data, PartialEq, Lens, Debug)]
pub struct SelectionRectangle {
    pub start_point: Option<Point>,
    pub end_point: Option<Point>,
    pub p2: Option<Point>,
    pub p3: Option<Point>,
    pub size: Size,
}

impl Default for SelectionRectangle {
    fn default() -> Self {
        SelectionRectangle {
            start_point: None, //p1
            end_point: None,   // p4
            p2: None,
            p3: None,
            size: Size::ZERO,
        }
    }
}
/*
start = p1 .____. p2
           |    |
        p3 .____. p4 = end

 */

 #[derive(Clone, Data, PartialEq, Lens, Debug)]
 pub struct SelectionShape {
    pub start_point: Option<Point>,
    pub end_point: Option<Point>,
    pub center: Option<Point>,
    pub radii: Option<druid::Vec2>
}

impl Default for SelectionShape {
    fn default() -> Self {
        SelectionShape {
            start_point: None, //p1
            end_point: None,   // p4
            center: None,
            radii:None,
        }
    }
}

#[derive(Clone, Data, Lens, Debug)]
pub struct AnnotationTools {
    pub tool: Tools,
    pub center: druid::Point,
    pub origin: druid::Point,
    pub width:f64,
    pub height:f64,
    pub img_size: Size,
    pub rect_stroke: f64,
    pub rect_transparency: f64,
    pub shape: SelectionShape,
    pub img: Option<ImageBuf>,
    pub text: String,
    pub text_pos: Option<druid::Point>,
    pub random_point:Option<Point>,
}

impl Default for AnnotationTools {
   fn default() -> Self {
       AnnotationTools {
        tool: Tools::No,
        center: druid::Point::new(250., 156.25),
        origin: druid::Point::new(0.,0.),
        width:500.,
        height:312.5,
        img_size:Size::ZERO,
        rect_stroke:0.0,
        rect_transparency:0.0,
        shape:SelectionShape::default(),
        img:None,
        text:"".to_string(),
        text_pos:None,
        random_point:None,
        //text_font:Font::try_from_vec(Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8])).unwrap(),
       }
   }
}


#[derive(Clone, Data, PartialEq, Lens, Debug)]
pub struct CursorData {
    typ: Cursor,
    over: Option<Direction>,
    down: bool,
}

impl CursorData {
    pub fn new(typ: Cursor, over: Option<Direction>, down: bool) -> Self {
        Self { typ, over, down }
    }
}

#[derive(Clone, Data, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    All,
}

//Controller to take screen after the custom shortcut
pub struct Enter {
    pub id_t: TimerToken,
    pub id_t2: TimerToken,
    pub locks: [bool;5],
}

impl<W: Widget<AppState>> Controller<AppState, W> for Enter {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &Env,
    ) {
        ctx.request_focus();
        match event {
            Event::KeyDown(key) => {
                let mut keyboard_event = KeyboardEvent {
                    state: key.state,
                    key: key.key.clone(),
                    code: key.code,
                    location: key.location,
                    modifiers: key.mods.raw(),
                    repeat: key.repeat,
                    is_composing: true,
                };

                //println!("{:?} {:?}", keyboard_event.modifiers, keyboard_event.key);

                match keyboard_event.key {
                    druid::keyboard_types::Key::CapsLock=>{
                        println!("capslock");
                        self.locks[0]=true;
                    }
                    druid::keyboard_types::Key::FnLock=>{
                        self.locks[1]=true;
                    }
                    druid::keyboard_types::Key::NumLock=>{
                        self.locks[2]=true;
                    }
                    druid::keyboard_types::Key::ScrollLock=>{
                        self.locks[3]=true;
                    }
                    druid::keyboard_types::Key::SymbolLock=>{
                        self.locks[4]=true;
                    }
                    _=>{}
                }

                let mut ind=0;
                for i in self.locks{
                    //println!("{:?}",i);
                    match ind {
                        0=>{
                            if i{
                                keyboard_event.modifiers.insert(druid::keyboard_types::Modifiers::CAPS_LOCK);
                            }else {
                                keyboard_event.modifiers.remove(druid::keyboard_types::Modifiers::CAPS_LOCK);
                            }
                        },
                        1=>{
                            if i{
                                keyboard_event.modifiers.insert(druid::keyboard_types::Modifiers::FN_LOCK);
                            }else {
                                keyboard_event.modifiers.remove(druid::keyboard_types::Modifiers::FN_LOCK);
                            }                    
                        },
                        2=>{
                            if i{
                                keyboard_event.modifiers.insert(druid::keyboard_types::Modifiers::NUM_LOCK);
                            }else {
                                keyboard_event.modifiers.remove(druid::keyboard_types::Modifiers::NUM_LOCK);
                            }                    
                        },
                        3=>{
                            if i{
                                keyboard_event.modifiers.insert(druid::keyboard_types::Modifiers::SCROLL_LOCK);
                            }else {
                                keyboard_event.modifiers.remove(druid::keyboard_types::Modifiers::SCROLL_LOCK);
                            }                    
                        },
                        4=>{
                            if i{
                                keyboard_event.modifiers.insert(druid::keyboard_types::Modifiers::SYMBOL_LOCK);
                            }else {
                                keyboard_event.modifiers.remove(druid::keyboard_types::Modifiers::SYMBOL_LOCK);
                            }                    
                        },
                        _=>{}
                    }
                    ind=ind+1;
                }

                //println!("mods:{:?}",keyboard_event.modifiers);
                //println!("saved:{:?}",Modifiers::from_bits(data.mods));

                /*
                if keyboard_event.modifiers==druid::keyboard_types::Modifiers::from_bits(data.mods).unwrap(){
                    println!("sono uguali");
                }
                */

                let k=char::from_u32(data.key).unwrap();

                if keyboard_event.modifiers==druid::keyboard_types::Modifiers::from_bits(data.mods).unwrap() 
                && (keyboard_event.key==Key::Character(k.to_lowercase().to_string()) || keyboard_event.key==Key::Character(k.to_uppercase().to_string())){
                    println!("shortcut matcher");
                    data.is_full_screen=true;
                    self.id_t = ctx.request_timer(Duration::from_millis(100));
                    self.locks=[false;5];
                }

                /*
                ShortcutMatcher::from_event(keyboard_event).shortcut(
                    Modifiers::from_bits(data.mods).expect("Not a modifier"),
                    Key::Character(char::from_u32(data.key).expect("Not a char").to_string()),
                    || {
                        println!("shortcut matcher");
                        data.is_full_screen=true;
                        self.id_t = ctx.request_timer(Duration::from_millis(100));
                        self.locks=[false;5];
                        /*
                        ctx.window()
                            .clone()
                            .set_window_state(WindowState::Minimized);
                        */
                    },
                );
                */
            }
            Event::Timer(id) => {
                if self.id_t == *id {
                    self.id_t2 = ctx.request_timer(Duration::from_millis(100));
                    self.id_t = TimerToken::next();
                } else if self.id_t2 == *id {
                    data.screen(ctx);
                    ctx.window().close();
                }
            }
            _ => child.event(ctx, event, data, env),
        }

        child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        child.update(ctx, old_data, data, env)
    }
}

//Controller to save custom shortcut

pub struct ShortcutController{
    pub locks:[bool;5],
}

impl<W: Widget<AppState>> Controller<AppState, W> for ShortcutController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &Env,
    ) {
        if let Event::KeyDown(key) = event {
            let mut keyboard_event = KeyboardEvent {
                state: key.state,
                key: key.key.clone(),
                code: key.code,
                location: key.location,
                modifiers: key.mods.raw(),
                repeat: key.repeat,
                is_composing: true,
            };
            
            println!("{:?}", keyboard_event.key);
            /*println!("{:?}",keyboard_event.modifiers);
            println!("{:?}",keyboard_event.state);
            println!("{:?}",keyboard_event.repeat);
            println!("{:?}",self.locks);*/

            match keyboard_event.key {
                druid::keyboard_types::Key::CapsLock=>{
                    println!("capslock");
                    self.locks[0]=true;
                }
                druid::keyboard_types::Key::FnLock=>{
                    self.locks[1]=true;
                }
                druid::keyboard_types::Key::NumLock=>{
                    self.locks[2]=true;
                }
                druid::keyboard_types::Key::ScrollLock=>{
                    self.locks[3]=true;
                }
                druid::keyboard_types::Key::SymbolLock=>{
                    self.locks[4]=true;
                }
                _=>{}
            }

            println!("prima:{:?}",keyboard_event.modifiers);
            let mut ind=0;
            for i in self.locks{
                //println!("{:?}",i);
                match ind {
                    0=>{
                        if i{
                            keyboard_event.modifiers.insert(druid::keyboard_types::Modifiers::CAPS_LOCK);
                        }else {
                            keyboard_event.modifiers.remove(druid::keyboard_types::Modifiers::CAPS_LOCK);
                        }
                    },
                    1=>{
                        if i{
                            keyboard_event.modifiers.insert(druid::keyboard_types::Modifiers::FN_LOCK);
                        }else {
                            keyboard_event.modifiers.remove(druid::keyboard_types::Modifiers::FN_LOCK);
                        }                    
                    },
                    2=>{
                        if i{
                            keyboard_event.modifiers.insert(druid::keyboard_types::Modifiers::NUM_LOCK);
                        }else {
                            keyboard_event.modifiers.remove(druid::keyboard_types::Modifiers::NUM_LOCK);
                        }                    
                    },
                    3=>{
                        if i{
                            keyboard_event.modifiers.insert(druid::keyboard_types::Modifiers::SCROLL_LOCK);
                        }else {
                            keyboard_event.modifiers.remove(druid::keyboard_types::Modifiers::SCROLL_LOCK);
                        }                    
                    },
                    4=>{
                        if i{
                            keyboard_event.modifiers.insert(druid::keyboard_types::Modifiers::SYMBOL_LOCK);
                        }else {
                            keyboard_event.modifiers.remove(druid::keyboard_types::Modifiers::SYMBOL_LOCK);
                        }                    
                    },
                    _=>{}
                }
                ind=ind+1;
            }

            println!("dopo:{:?}",keyboard_event.modifiers);

            data.mods = keyboard_event.modifiers.bits();
            data.key = keyboard_event.key.legacy_charcode();
        }

        child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        child.update(ctx, old_data, data, env)
    }
}

//Controller for the click and drag motion
pub struct AreaController {
    pub id_t: TimerToken,
    pub id_t2: TimerToken,
    pub flag: bool,
}

impl<W: Widget<AppState>> Controller<AppState, W> for AreaController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &Env,
    ) {
        //if !data.resize {
        if !self.flag {
            data.is_full_screen = false;
            match event {
                Event::MouseDown(mouse_button) => {
                    let mouse_down = MouseEvent {
                        pos: mouse_button.pos,
                        window_pos: mouse_button.window_pos,
                        buttons: mouse_button.buttons,
                        mods: mouse_button.mods,
                        count: mouse_button.count,
                        focus: mouse_button.focus,
                        button: mouse_button.button,
                        wheel_delta: mouse_button.wheel_delta,
                    };
                    data.from = mouse_down.pos;
                    data.rect.start_point = Some(mouse_button.pos);
                    data.rect.end_point = Some(mouse_button.pos);
                }
                Event::MouseUp(mouse_button) => {
                    let _mouse_up = MouseEvent {
                        pos: mouse_button.pos,
                        window_pos: mouse_button.window_pos,
                        buttons: mouse_button.buttons,
                        mods: mouse_button.mods,
                        count: mouse_button.count,
                        focus: mouse_button.focus,
                        button: mouse_button.button,
                        wheel_delta: mouse_button.wheel_delta,
                    };
                    //data.rect.end_point = Some(mouse_button.pos);
                    let r = druid::Rect::from_points(data.from, mouse_button.pos);

                    // aggiusto i punti
                    data.rect.start_point = Some(r.origin());
                    data.rect.p2 = Some(Point::new(r.max_x(), r.min_y()));
                    data.rect.p3 = Some(Point::new(r.min_x(), r.max_y()));
                    data.rect.end_point = Some(Point::new(r.max_x(), r.max_y()));
                    data.rect.size = r.size();
                    data.selection_transparency = 0.0;

                    match data.delay {
                        Timer::Zero => self.id_t = ctx.request_timer(Duration::from_millis(100)),
                        _ => {
                            self.id_t =ctx.request_timer(Duration::from_secs(data.delay.set_timer()))
                        }
                    }

                    ctx.window().clone().hide();
                }
                Event::MouseMove(mouse_button) => {
                    if !data.rect.start_point.is_none() {
                        data.rect.end_point = Some(mouse_button.pos);
                    }
                }
                Event::Timer(id) => {
                    if self.id_t == *id {
                        ctx.window().clone().show();
                        self.id_t2 = ctx.request_timer(Duration::from_millis(100));
                        self.id_t = TimerToken::next();
                    } else if self.id_t2 == *id {
                        data.screen(ctx);
                        //data.resize = true;
                        ctx.window().close();
                    }
                }
                _ => child.event(ctx, event, data, env),
            }
        } else {
            match event {
                Event::Timer(id) => {
                    if self.id_t == *id {
                        //ctx.window().clone().set_window_state(WindowState::Restored);
                        ctx.window().clone().show();
                        self.id_t2 = ctx.request_timer(Duration::from_millis(100));
                        self.id_t = TimerToken::next();
                    } else if self.id_t2 == *id {
                        data.screen(ctx);
                        ctx.window().close();
                    }
                }
                _ => {
                    data.is_full_screen = true;
                    match data.delay {
                        Timer::Zero => self.id_t = ctx.request_timer(Duration::from_millis(100)),
                        _ => {
                            self.id_t =
                                ctx.request_timer(Duration::from_secs(data.delay.set_timer()))
                        }
                    }

                    ctx.window().clone().hide();
                    /*ctx.window()
                        .clone()
                        .set_window_state(WindowState::Minimized)
                        */
                }
            }
        }
        //}
        child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        child.update(ctx, old_data, data, env)
    }
}

//Controller to resize screening area

pub struct ResizeController{
    pub points:Vec<Point>,
}

impl<W: Widget<AppState>> Controller<AppState, W> for ResizeController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &druid::Event,
        data: &mut AppState,
        env: &Env,
    ) {

        match data.tool_window.tool {
            Tools::Resize=>{
                if let Event::MouseDown(_mouse_button) = event {
                    match data.cursor.over {
                        Some(_) => {
                            data.cursor.down = true; // è sopra il bordo, sto premendo
                        }
                        None => return, // non è sopra il bordo
                    }
                } else if let Event::MouseUp(_mouse_button) = event {
                    //data.selection_transparency = 0.0;
                    data.cursor.down = false;
                    data.rect.size = druid::Rect::from_points(
                        data.rect.start_point.unwrap(),
                        data.rect.end_point.unwrap(),
                    )
                    .size();
                } else if let Event::MouseMove(mouse_button) = event {
                    let mouse_button = MouseEvent {
                        pos: mouse_button.pos,
                        window_pos: mouse_button.window_pos,
                        buttons: mouse_button.buttons,
                        mods: mouse_button.mods,
                        count: mouse_button.count,
                        focus: mouse_button.focus,
                        button: mouse_button.button,
                        wheel_delta: mouse_button.wheel_delta,
                    };
        
                    let rect = druid::Rect::from_points(
                        data.rect.start_point.unwrap(),
                        data.rect.end_point.unwrap(),
                    );
                    /* let rect = druid::Rect::from_center_size(
                        Point::new(250., 156.25),
                        Size::new(data.rect.size.width, data.rect.size.height),
                    ); */
                    //sposto senza premere
                    if data.cursor.down == false {
                        // cambia cursore -> diagonale
                        if (mouse_button.pos.x - rect.min_x()).abs() <= 10.
                            && (mouse_button.pos.y - rect.min_y()).abs() <= 10.
                        {
                            data.cursor.typ = Cursor::Crosshair;
                            data.cursor.over = Some(Direction::UpLeft);
                        } else if (mouse_button.pos.x - rect.max_x()).abs() <= 10.
                            && (mouse_button.pos.y - rect.max_y()).abs() <= 10.
                        {
                            data.cursor.typ = Cursor::Crosshair;
                            data.cursor.over = Some(Direction::DownRight);
                        }
                        // cambia cursore -> antidiagonale
                        else if (mouse_button.pos.x - rect.min_x()).abs() <= 10.
                            && (mouse_button.pos.y - rect.max_y()).abs() <= 10.
                        {
                            data.cursor.typ = Cursor::Crosshair;
                            data.cursor.over = Some(Direction::DownLeft);
                        } else if (mouse_button.pos.y - rect.min_y()).abs() <= 10.
                            && (mouse_button.pos.x - rect.max_x()).abs() <= 10.
                        {
                            data.cursor.typ = Cursor::Crosshair;
                            data.cursor.over = Some(Direction::UpRight);
                        } else if (mouse_button.pos.x - rect.min_x()).abs() <= 10. {
                            if mouse_button.pos.y > rect.min_y() && mouse_button.pos.y < rect.max_y() {
                                // cambia cursore -> sinistra
                                data.cursor.typ = Cursor::ResizeLeftRight;
                                data.cursor.over = Some(Direction::Left);
                            }
                        } else if (mouse_button.pos.x - rect.max_x()).abs() <= 10. {
                            if mouse_button.pos.y > rect.min_y() && mouse_button.pos.y < rect.max_y() {
                                // cambia cursore -> destra
                                data.cursor.typ = Cursor::ResizeLeftRight;
                                data.cursor.over = Some(Direction::Right);
                            }
                        } else if (mouse_button.pos.y - rect.max_y()).abs() <= 10. {
                            if mouse_button.pos.x > rect.min_x() && mouse_button.pos.x < rect.max_x() {
                                // cambia cursore -> verticale
                                data.cursor.typ = Cursor::ResizeUpDown;
                                data.cursor.over = Some(Direction::Down);
                            }
                        } else if (mouse_button.pos.y - rect.min_y()).abs() <= 10. {
                            if mouse_button.pos.x > rect.min_x() && mouse_button.pos.x < rect.max_x() {
                                // cambia cursore -> verticale
                                data.cursor.typ = Cursor::ResizeUpDown;
                                data.cursor.over = Some(Direction::Up);
                            }
                        } else if (mouse_button.pos.x<=(rect.min_x()+(rect.max_x()-rect.min_x())*0.6))
                            && (mouse_button.pos.x>=(rect.min_x()+(rect.max_x()-rect.min_x())*0.4))
                            && (mouse_button.pos.y<=(rect.min_y()+(rect.max_y()-rect.min_y())*0.6))
                            && (mouse_button.pos.y>=(rect.min_y()+(rect.max_y()-rect.min_y())*0.4)){
                                data.cursor.typ=Cursor::Crosshair;
                                data.cursor.over=Some(Direction::All);
                        } else {
                            data.cursor.typ = Cursor::Arrow;
                            data.cursor.over = None;
                        }
                        ctx.set_cursor(&data.cursor.typ);
                    }
                    //sposto premendo
                    else if data.cursor.down == true {
                        match data.cursor.over {
                            Some(Direction::Up) => {
                                if mouse_button.pos.y < data.rect.p3.unwrap().y - 10. && mouse_button.pos.y>(156.25-data.tool_window.img_size.height/2.){
                                    data.rect.start_point.replace(
                                        Point::new(
                                            data.rect.start_point.unwrap().x,
                                            mouse_button.pos.y,
                                        ));
                                    data.rect
                                        .p2
                                        .replace(Point::new(data.rect.p2.unwrap().x, mouse_button.pos.y));
                                }
                            }
                            Some(Direction::Down) => {
                                if mouse_button.pos.y > data.rect.start_point.unwrap().y + 10. && mouse_button.pos.y<(156.25+data.tool_window.img_size.height/2.){
                                    data.rect.end_point.replace(
                                        Point::new(
                                            data.rect.end_point.unwrap().x,
                                            mouse_button.pos.y,
                                        ));
                                    data.rect
                                        .p3
                                        .replace(Point::new(data.rect.p3.unwrap().x, mouse_button.pos.y));
                                }
                            }
                            Some(Direction::Left) => {
                                if mouse_button.pos.x < data.rect.p2.unwrap().x - 10. && mouse_button.pos.x>(250.-data.tool_window.img_size.width/2.) {
                                    data.rect.start_point.replace(Point::new(
                                        mouse_button.pos.x,
                                        data.rect.start_point.unwrap().y,
                                    ));
                                    data.rect
                                        .p3
                                        .replace(Point::new(mouse_button.pos.x, data.rect.p3.unwrap().y));
                                }
                            }
                            Some(Direction::Right) => {
                                if mouse_button.pos.x > data.rect.start_point.unwrap().x + 10. && mouse_button.pos.x<(250.+data.tool_window.img_size.width/2.){
                                    data.rect.end_point.replace(Point::new(
                                        mouse_button.pos.x,
                                        data.rect.end_point.unwrap().y,
                                    ));
                                    data.rect
                                        .p2
                                        .replace(Point::new(mouse_button.pos.x, data.rect.p2.unwrap().y));
                                }
                            }
                            Some(Direction::UpLeft) => {
                                if mouse_button.pos.x < data.rect.end_point.unwrap().x - 10. && mouse_button.pos.x>(250.-data.tool_window.img_size.width/2.){
                                    data.rect.start_point.replace(Point::new(
                                        mouse_button.pos.x,
                                        data.rect.start_point.unwrap().y,
                                    ));
                                    data.rect
                                        .p3
                                        .replace(Point::new(mouse_button.pos.x, data.rect.p3.unwrap().y));
                                }
                                if mouse_button.pos.y < data.rect.end_point.unwrap().y - 10. && mouse_button.pos.y>(156.25-data.tool_window.img_size.height/2.) {
                                    data.rect.start_point.replace(Point::new(
                                        data.rect.start_point.unwrap().x,
                                        mouse_button.pos.y,
                                    ));
                                    data.rect
                                        .p2
                                        .replace(Point::new(data.rect.p2.unwrap().x, mouse_button.pos.y));
                                }
                            }
                            Some(Direction::UpRight) => {
                                if mouse_button.pos.x > data.rect.p3.unwrap().x + 10. && mouse_button.pos.x<(250.+data.tool_window.img_size.width/2.){
                                    data.rect
                                        .p2
                                        .replace(Point::new(mouse_button.pos.x, data.rect.p2.unwrap().y));
                                    data.rect.end_point.replace(Point::new(
                                        mouse_button.pos.x,
                                        data.rect.end_point.unwrap().y,
                                    ));
                                }
                                if mouse_button.pos.y < data.rect.end_point.unwrap().y - 10. && mouse_button.pos.y>(156.25-data.tool_window.img_size.height/2.){
                                    data.rect
                                        .p2
                                        .replace(Point::new(data.rect.p2.unwrap().x, mouse_button.pos.y));
                                    data.rect.start_point.replace(Point::new(
                                        data.rect.start_point.unwrap().x,
                                        mouse_button.pos.y,
                                    ));
                                }
                            }
                            Some(Direction::DownLeft) => {
                                if mouse_button.pos.x < data.rect.end_point.unwrap().x - 10. && mouse_button.pos.x>(250.-data.tool_window.img_size.width/2.){
                                    data.rect
                                        .p3
                                        .replace(Point::new(mouse_button.pos.x, data.rect.p3.unwrap().y));
                                    data.rect.start_point.replace(Point::new(
                                        mouse_button.pos.x,
                                        data.rect.start_point.unwrap().y,
                                    ));
                                }
                                if mouse_button.pos.y > data.rect.start_point.unwrap().y + 10. && mouse_button.pos.y<(156.25+data.tool_window.img_size.height/2.){
                                    data.rect
                                        .p3
                                        .replace(Point::new(data.rect.p3.unwrap().x, mouse_button.pos.y));
                                    data.rect.end_point.replace(Point::new(
                                        data.rect.end_point.unwrap().x,
                                        mouse_button.pos.y,
                                    ));
                                }
                            }
                            Some(Direction::DownRight) => {
                                if mouse_button.pos.x > data.rect.p3.unwrap().x + 10. && mouse_button.pos.x<(250.+data.tool_window.img_size.width/2.){
                                    data.rect.end_point.replace(Point::new(
                                        mouse_button.pos.x,
                                        data.rect.end_point.unwrap().y,
                                    ));
                                    data.rect
                                        .p2
                                        .replace(Point::new(mouse_button.pos.x, data.rect.p2.unwrap().y));
                                }
                                if mouse_button.pos.y > data.rect.start_point.unwrap().y + 10. && mouse_button.pos.y<(156.25+data.tool_window.img_size.height/2.) {
                                    data.rect.end_point.replace(Point::new(
                                        data.rect.end_point.unwrap().x,
                                        mouse_button.pos.y,
                                    ));
                                    data.rect
                                        .p3
                                        .replace(Point::new(data.rect.p3.unwrap().x, mouse_button.pos.y));
                                }
                            }
                            Some(Direction::All)=>{
                                if mouse_button.pos.x>(data.tool_window.origin.x+data.rect.size.width/2.) 
                                && (mouse_button.pos.x<data.tool_window.origin.x+data.tool_window.img_size.width-data.rect.size.width/2.)
                                && mouse_button.pos.y>(data.tool_window.origin.y+data.rect.size.height/2.) 
                                && (mouse_button.pos.y<data.tool_window.origin.y+data.tool_window.img_size.height-data.rect.size.height/2.){
                                    let rect2=druid::Rect::from_center_size(mouse_button.pos, data.rect.size);

                                    data.rect.start_point.replace(rect2.origin());
                                    data.rect.end_point.replace(Point::new(rect2.max_x(), rect2.max_y()));
                                    data.rect.p2=Some(Point::new(0., rect2.max_y()));
                                    data.rect.p3=Some(Point::new(rect2.max_x(), 0.));
                                }
                            }
                            None => return, // non è sopra il bordo
                        }
                    }
                }
            },

            Tools::Ellipse=>{/////////////////////// per creare l'ellisse si rischia di andare in overflow
                match event {
                    Event::MouseDown(mouse_button) => {
                        let mouse_down = MouseEvent {
                            pos: mouse_button.pos,
                            window_pos: mouse_button.window_pos,
                            buttons: mouse_button.buttons,
                            mods: mouse_button.mods,
                            count: mouse_button.count,
                            focus: mouse_button.focus,
                            button: mouse_button.button,
                            wheel_delta: mouse_button.wheel_delta,
                        };
                        data.tool_window.shape.start_point=Some(mouse_down.pos);
                        data.tool_window.shape.end_point=Some(mouse_down.pos);

                        //println!("{:?}",data.tool_window.ellipse.end_point);
                        
                        data.selection_transparency=1.;
                    }
                    Event::MouseMove(mouse_button)=>{
                        let mouse_move = MouseEvent {
                            pos: mouse_button.pos,
                            window_pos: mouse_button.window_pos,
                            buttons: mouse_button.buttons,
                            mods: mouse_button.mods,
                            count: mouse_button.count,
                            focus: mouse_button.focus,
                            button: mouse_button.button,
                            wheel_delta: mouse_button.wheel_delta,
                        };
        
                        data.tool_window.shape.end_point=Some(mouse_move.pos);
                        if let (Some(start), Some(end)) = (data.tool_window.shape.start_point, data.tool_window.shape.end_point) {
                            let radius1 = (start.x - end.x) / 2.;
                            let radius2 = (start.y - end.y) / 2.;
                            let c1 = end.x + radius1;
                            let c2 = end.y + radius2;
                            data.tool_window.shape.center = Some(druid::Point::new(c1, c2));
                            data.tool_window.shape.radii = Some(druid::Vec2::new(radius1.abs(), radius2.abs()));
                        }
                    }
                    Event::MouseUp(mouse_button)=>{
                        let mouse_up = MouseEvent {
                            pos: mouse_button.pos,
                            window_pos: mouse_button.window_pos,
                            buttons: mouse_button.buttons,
                            mods: mouse_button.mods,
                            count: mouse_button.count,
                            focus: mouse_button.focus,
                            button: mouse_button.button,
                            wheel_delta: mouse_button.wheel_delta,
                        };

                        data.selection_transparency=0.;
                        
                        let mut image: ImageBuffer<Rgba<u8>, Vec<u8>>=ImageBuffer::from_vec(
                            data.img.width() as u32,
                            data.img.height() as u32,
                        data.tool_window.img.clone().unwrap().raw_pixels().to_vec()).unwrap();

                        println!("qui ok");

                        println!("{:?}  {:?}",(data.tool_window.shape.radii.unwrap().x*(data.img.width() as f64/data.tool_window.img_size.width)) as i32,(data.tool_window.shape.radii.unwrap().y*(data.img.height() as f64/data.tool_window.img_size.height)) as i32);

                        /*
                        let (x0, y0) = (((data.tool_window.shape.center.unwrap().x-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as i32,
                        ((data.tool_window.shape.center.unwrap().y-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height))  as i32);
    let w2 = (data.tool_window.shape.radii.unwrap().x*(data.img.width() as f64/data.tool_window.img_size.width)) as i32*(data.tool_window.shape.radii.unwrap().x*(data.img.width() as f64/data.tool_window.img_size.width)) as i32;
    let h2 = (data.tool_window.shape.radii.unwrap().y*(data.img.height() as f64/data.tool_window.img_size.height)) as i32 * (data.tool_window.shape.radii.unwrap().y*(data.img.height() as f64/data.tool_window.img_size.height)) as i32;
    let mut x = 0;
    let mut y = (data.tool_window.shape.radii.unwrap().y*(data.img.height() as f64/data.tool_window.img_size.height)) as i32;
    let mut px = 0;
    let mut py = 2 * w2 * y;

    //render_func(x0, y0, x, y);

    // Top and bottom regions.
    let mut p = (h2 - (w2 * (data.tool_window.shape.radii.unwrap().y*(data.img.height() as f64/data.tool_window.img_size.height)) as i32)) as f32 + (0.25 * w2 as f32);
    while px < py {
        x += 1;
        px += 2 * h2;
        if p < 0.0 {
            p += (h2 + px) as f32;
        } else {
            y -= 1;
            py += -2 * w2;
            p += (h2 + px - py) as f32;
        }

        //render_func(x0, y0, x, y);
    }

    
    // Left and right regions.
    let a=(h2 as f32) * (x as f32 + 0.5).powi(2);
    let d=(y - 1).pow(2);
    println!("{:?}  {:?}",w2,(y - 1).pow(2));
    let b=(w2 * (y - 1).pow(2)) as f32;
    let c=(w2 * h2) as f32;
    p = (h2 as f32) * (x as f32 + 0.5).powi(2) + (w2 * (y - 1).pow(2)) as f32 - (w2 * h2) as f32;
    
*/
                        let color = data.color.as_rgba8();
                        let prova=imageproc::drawing::draw_filled_ellipse(
                            &mut image,
                            (((data.tool_window.shape.center.unwrap().x-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as i32,
                                ((data.tool_window.shape.center.unwrap().y-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height))  as i32),
                            (data.tool_window.shape.radii.unwrap().x*(data.img.width() as f64/data.tool_window.img_size.width)) as i32,
                            (data.tool_window.shape.radii.unwrap().y*(data.img.height() as f64/data.tool_window.img_size.height)) as i32,
                            Rgba([color.0, color.1, color.2, 255]));

                            println!("qui ok");


                        data.tool_window.img=Some(ImageBuf::from_raw(
                            prova.clone().into_raw(),
                            druid::piet::ImageFormat::RgbaPremul,
                            prova.clone().width() as usize,
                            prova.clone().height() as usize,
                        ));

                        println!("qui ok");


                        data.tool_window.shape.start_point=None;
                        data.tool_window.shape.end_point=None;
                        data.tool_window.shape.center=None;
                        data.tool_window.shape.radii=None;
        /*
                        data.tool_window.ellipse.end_point=Some(mouse_up.pos);
                        //let a=image_canvas::layout::CanvasLayout::with_plane(data.img);
                        let mut image2: ImageBuffer<Rgba<u8>, Vec<u8>>=ImageBuffer::from_vec(
                            data.img.width() as u32,
                            data.img.height() as u32,
                        data.img.raw_pixels().to_vec()).unwrap();
        
                        let prova=imageproc::drawing::draw_filled_ellipse(
                            &mut image2,
                            (mouse_up.pos.x as i32,mouse_up.pos.y as i32),
                            100,
                            100,
                            Rgba([255,0,0,255]));
        
                        
                        
                        data.img=ImageBuf::from_raw(
                            prova.clone().into_raw(),
                            druid::piet::ImageFormat::RgbaPremul,
                            prova.clone().width() as usize,
                            prova.clone().height() as usize,
                        );
*/
                        //data.img.to_image(ctx);
                        //data.tool_window.tool=Tools::No;
                    },
                    _=>{}
                }
            }
            Tools::HollowEllipse=>{
                match event {
                    Event::MouseDown(mouse_button) => {
                        let mouse_down = MouseEvent {
                            pos: mouse_button.pos,
                            window_pos: mouse_button.window_pos,
                            buttons: mouse_button.buttons,
                            mods: mouse_button.mods,
                            count: mouse_button.count,
                            focus: mouse_button.focus,
                            button: mouse_button.button,
                            wheel_delta: mouse_button.wheel_delta,
                        };
                        data.tool_window.shape.start_point=Some(mouse_down.pos);
                        data.tool_window.shape.end_point=Some(mouse_down.pos);
                        
                        data.selection_transparency=1.;
                    }
                    Event::MouseMove(mouse_button)=>{
                        let mouse_move = MouseEvent {
                            pos: mouse_button.pos,
                            window_pos: mouse_button.window_pos,
                            buttons: mouse_button.buttons,
                            mods: mouse_button.mods,
                            count: mouse_button.count,
                            focus: mouse_button.focus,
                            button: mouse_button.button,
                            wheel_delta: mouse_button.wheel_delta,
                        };
        
                        data.tool_window.shape.end_point=Some(mouse_move.pos);
                        if let (Some(start), Some(end)) = (data.tool_window.shape.start_point, data.tool_window.shape.end_point) {
                            let radius1 = (start.x - end.x) / 2.;
                            let radius2 = (start.y - end.y) / 2.;
                            let c1 = end.x + radius1;
                            let c2 = end.y + radius2;
                            data.tool_window.shape.center = Some(druid::Point::new(c1, c2));
                            data.tool_window.shape.radii = Some(druid::Vec2::new(radius1.abs(), radius2.abs()));
                        }
                    }
                    Event::MouseUp(mouse_button)=>{
                        let mouse_up = MouseEvent {
                            pos: mouse_button.pos,
                            window_pos: mouse_button.window_pos,
                            buttons: mouse_button.buttons,
                            mods: mouse_button.mods,
                            count: mouse_button.count,
                            focus: mouse_button.focus,
                            button: mouse_button.button,
                            wheel_delta: mouse_button.wheel_delta,
                        };

                        data.selection_transparency=0.;
                        
                        let mut image: ImageBuffer<Rgba<u8>, Vec<u8>>=ImageBuffer::from_vec(
                            data.img.width() as u32,
                            data.img.height() as u32,
                        data.tool_window.img.clone().unwrap().raw_pixels().to_vec()).unwrap();

                        let color = data.color.as_rgba8();

                        println!("qui ok");
                        for i in 0..50{
                            imageproc::drawing::draw_hollow_ellipse_mut(
                                &mut image,
                                (((data.tool_window.shape.center.unwrap().x-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as i32,
                                    ((data.tool_window.shape.center.unwrap().y-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height))  as i32),
                                ((data.tool_window.shape.radii.unwrap().x-i as f64/20.)*(data.img.width() as f64/data.tool_window.img_size.width)) as i32,
                                ((data.tool_window.shape.radii.unwrap().y-i as f64/20.)*(data.img.height() as f64/data.tool_window.img_size.height)) as i32,
                                Rgba([color.0, color.1, color.2, 255]));
                        println!("{:?}",i);
                            imageproc::drawing::draw_hollow_ellipse_mut(
                                &mut image,
                                (((data.tool_window.shape.center.unwrap().x-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as i32,
                                    ((data.tool_window.shape.center.unwrap().y-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height))  as i32),
                                ((data.tool_window.shape.radii.unwrap().x+i as f64/20.)*(data.img.width() as f64/data.tool_window.img_size.width)) as i32,
                                ((data.tool_window.shape.radii.unwrap().y+i as f64/20.)*(data.img.height() as f64/data.tool_window.img_size.height)) as i32,
                                Rgba([color.0, color.1, color.2, 255]));
                        }

                        println!("qui ok");


                        data.tool_window.img=Some(ImageBuf::from_raw(
                            image.clone().into_raw(),
                            druid::piet::ImageFormat::RgbaPremul,
                            image.clone().width() as usize,
                            image.clone().height() as usize,
                        ));

                        data.tool_window.shape.start_point=None;
                        data.tool_window.shape.end_point=None;
                        data.tool_window.shape.center=None;
                        data.tool_window.shape.radii=None;
                    },
                    _=>{}
                }
            }
            Tools::Arrow=>{
                match event {
                    Event::MouseDown(mouse_button) => {
                        let mouse_down = MouseEvent {
                            pos: mouse_button.pos,
                            window_pos: mouse_button.window_pos,
                            buttons: mouse_button.buttons,
                            mods: mouse_button.mods,
                            count: mouse_button.count,
                            focus: mouse_button.focus,
                            button: mouse_button.button,
                            wheel_delta: mouse_button.wheel_delta,
                        };
                        data.tool_window.shape.start_point=Some(mouse_down.pos);
                        data.tool_window.shape.end_point=Some(mouse_down.pos);
                        
                        data.tool_window.rect_transparency=1.;
                    }
                    Event::MouseMove(mouse_button)=>{
                        let mouse_move = MouseEvent {
                            pos: mouse_button.pos,
                            window_pos: mouse_button.window_pos,
                            buttons: mouse_button.buttons,
                            mods: mouse_button.mods,
                            count: mouse_button.count,
                            focus: mouse_button.focus,
                            button: mouse_button.button,
                            wheel_delta: mouse_button.wheel_delta,
                        };
        
                        data.tool_window.shape.end_point=Some(mouse_move.pos);
                        
                    }
                    
                    Event::MouseUp(mouse_button)=>{
                        let mouse_up = MouseEvent {
                            pos: mouse_button.pos,
                            window_pos: mouse_button.window_pos,
                            buttons: mouse_button.buttons,
                            mods: mouse_button.mods,
                            count: mouse_button.count,
                            focus: mouse_button.focus,
                            button: mouse_button.button,
                            wheel_delta: mouse_button.wheel_delta,
                        };

                        data.tool_window.rect_transparency=0.;
                        data.tool_window.shape.end_point=Some(mouse_up.pos);

                        /*
                        ctx.stroke(
                            body,
                            &Color::rgba(color.0, color.1, color.2, data.tool_window.rect_transparency),
                            10.,
                        );*/

                        let end=data.tool_window.shape.end_point.unwrap();
                        let start=data.tool_window.shape.start_point.unwrap();

                        let cos = 0.866;
                        let sin = 0.500;
                        let dx=end.x-start.x;
                        let dy=end.y-start.y;
                        let end1=druid::Point::new(end.x-(dx*cos+dy*-sin)*2./5.,end.y-(dx*sin+dy*cos)*2./5.);
                        let end2=druid::Point::new(end.x-(dx*cos+dy*sin)*2./5.,end.y-(dx*-sin+dy*cos)*2./5.);
/*
                        ctx.stroke(
                            druid::kurbo::Line::new(end,end1),
                            &Color::rgba(color.0, color.1, color.2, data.tool_window.rect_transparency),
                            10.,
                        );

                        ctx.stroke(
                            druid::kurbo::Line::new(end,end2),
                            &Color::rgba(color.0, color.1, color.2, data.tool_window.rect_transparency),
                            10.,
                        );*/
                        
                        let mut image: ImageBuffer<Rgba<u8>, Vec<u8>>=ImageBuffer::from_vec(
                            data.img.width() as u32,
                            data.img.height() as u32,
                        data.tool_window.img.clone().unwrap().raw_pixels().to_vec()).unwrap();

                        let color = data.color.as_rgba8();

                        for i in -85..=85{  /* loop */

                            imageproc::drawing::draw_line_segment_mut(
                                &mut image,
                                (((start.x+i as f64/20.-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as f32,
                                ((start.y+i as f64/20.-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height)) as f32),
                                (((end.x+i as f64/20.-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as f32,
                                ((end.y+i as f64/20.-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height)) as f32),
                                Rgba([color.0, color.1, color.2, 255])
                            );

                            imageproc::drawing::draw_line_segment_mut(
                                &mut image,
                                (((end.x+i as f64/20.-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as f32,
                                ((end.y+i as f64/20.-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height)) as f32),
                                (((end1.x+i as f64/20.-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as f32,
                                ((end1.y+i as f64/20.-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height)) as f32),
                                Rgba([color.0, color.1, color.2, 255])
                            );

                            imageproc::drawing::draw_line_segment_mut(
                                &mut image,
                                (((end.x+i as f64/20.-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as f32,
                                ((end.y+i as f64/20.-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height)) as f32),
                                (((end2.x+i as f64/20.-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as f32,
                                ((end2.y+i as f64/20.-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height)) as f32),
                                Rgba([color.0, color.1, color.2, 255])
                            );

                        }
                        

                        data.tool_window.img=Some(ImageBuf::from_raw(
                            image.clone().into_raw(),
                            druid::piet::ImageFormat::RgbaPremul,
                            image.clone().width() as usize,
                            image.clone().height() as usize,
                        ));



                        data.tool_window.shape.start_point=None;
                        data.tool_window.shape.end_point=None;
                        data.tool_window.shape.center=None;
                        data.tool_window.shape.radii=None;
                        
                    },
                    _=>{}
                }
            
            }
            Tools::Text=>{
                match event {
                    Event::MouseDown(mouse_button) => {
                        let mouse_down = MouseEvent {
                            pos: mouse_button.pos,
                            window_pos: mouse_button.window_pos,
                            buttons: mouse_button.buttons,
                            mods: mouse_button.mods,
                            count: mouse_button.count,
                            focus: mouse_button.focus,
                            button: mouse_button.button,
                            wheel_delta: mouse_button.wheel_delta,
                        };

                        data.tool_window.text_pos=Some(mouse_down.pos);
                    }
                    
                    _=>{}
                }
            }
            Tools::Highlight=>{
                match event {
                    Event::MouseDown(mouse_button) => {
                        let mouse_down = MouseEvent {
                            pos: mouse_button.pos,
                            window_pos: mouse_button.window_pos,
                            buttons: mouse_button.buttons,
                            mods: mouse_button.mods,
                            count: mouse_button.count,
                            focus: mouse_button.focus,
                            button: mouse_button.button,
                            wheel_delta: mouse_button.wheel_delta,
                        };

                        data.color = data.color.with_alpha(1.);
                        data.tool_window.shape.start_point=Some(mouse_down.pos);

                    }
                    Event::MouseMove(mouse_button)=>{
                        let mouse_move = MouseEvent {
                            pos: mouse_button.pos,
                            window_pos: mouse_button.window_pos,
                            buttons: mouse_button.buttons,
                            mods: mouse_button.mods,
                            count: mouse_button.count,
                            focus: mouse_button.focus,
                            button: mouse_button.button,
                            wheel_delta: mouse_button.wheel_delta,
                        };

                        data.tool_window.shape.end_point=Some(mouse_move.pos);

                    }
                    Event::MouseUp(_mouse_button)=>{

                        //opencv::imgproc::line();
                        
                        let mut image: ImageBuffer<Rgba<u8>, Vec<u8>>=ImageBuffer::from_vec(
                            data.img.width() as u32,
                            data.img.height() as u32,
                        data.tool_window.img.clone().unwrap().raw_pixels().to_vec()).unwrap();

                        let color = data.color.as_rgba8();
                        let mut start=data.tool_window.shape.start_point.unwrap();
                        let mut end=data.tool_window.shape.end_point.unwrap();

                        let dx =  (end.x-start.x).abs();
                        let sx;
                        if start.x<end.x{
                            sx=1;
                        }else {
                            sx=-1;
                        }
                        let dy = -(end.y-start.y).abs();
                        let sy;
                        if start.y<end.y{
                            sy=1;
                        }else {
                            sy=-1;
                        }
                        let mut err = dx+dy;
                        let mut e2; /* error value e_xy */
                        
                        if data.tool_window.shape.start_point.unwrap().x==data.tool_window.shape.end_point.unwrap().x &&
                            data.tool_window.shape.start_point.unwrap().y==data.tool_window.shape.end_point.unwrap().y {
                                imageproc::drawing::draw_line_segment_mut(
                                    &mut image,
                                    (((start.x+5.-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as f32,
                                     ((start.y-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height)) as f32),
                                     (((start.x-5.-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as f32,
                                     ((start.y-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height)) as f32),
                                    Rgba([color.0, color.1, color.2, 255])
                                );
                            }else {
                                for _i in 0..=425{  /* loop */
                                    e2 = err*2.;
                                    if e2 >= dy {
                                        err =err+dy; 
                                        start.y = start.y-sy as f64/100.; 
                                        end.y = end.y-sy as f64/100.;
                                        
                                    } /* e_xy+e_x > 0 */
                                    if e2 <= dx {
                                        err =err+dx; 
                                        start.x = start.x+sx as f64/100.; 
                                        end.x = end.x+sx as f64/100.;
                                    } /* e_xy+e_y < 0 */

                                    imageproc::drawing::draw_line_segment_mut(
                                        &mut image,
                                        (((start.x-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as f32,
                                        ((start.y-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height)) as f32),
                                        (((end.x-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as f32,
                                        ((end.y-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height)) as f32),
                                        Rgba([color.0, color.1, color.2, 255])
                                    );
                                }

                                start=data.tool_window.shape.start_point.unwrap();
                                end=data.tool_window.shape.end_point.unwrap();



                                for _i in 0..=425{  /* loop */
                                    e2 = err*2.;
                                    if e2 >= dy {
                                        err =err+dy; 
                                        start.y = start.y+sy as f64/100.; 
                                        end.y = end.y+sy as f64/100.;
                                        
                                    } /* e_xy+e_x > 0 */
                                    if e2 <= dx {
                                        err =err+dx; 
                                        start.x = start.x-sx as f64/100.; 
                                        end.x = end.x-sx as f64/100.;
                                    } /* e_xy+e_y < 0 */

                                    imageproc::drawing::draw_line_segment_mut(
                                        &mut image,
                                        (((start.x-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as f32,
                                        ((start.y-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height)) as f32),
                                        (((end.x-data.tool_window.origin.x)*(data.img.width() as f64/data.tool_window.img_size.width)) as f32,
                                        ((end.y-data.tool_window.origin.y)*(data.img.height() as f64/data.tool_window.img_size.height)) as f32),
                                        Rgba([color.0, color.1, color.2, 255])
                                    );
                                }
                            }

                        data.tool_window.img=Some(ImageBuf::from_raw(
                            image.clone().into_raw(),
                            druid::piet::ImageFormat::RgbaPremul,
                            image.clone().width() as usize,
                            image.clone().height() as usize,
                        ));
                        
                        data.tool_window.shape.start_point=None;
                        data.tool_window.shape.end_point=None;
                    },
                    _=>{}
                }
            
            }
            Tools::Random=>{/////////////da non usare
                match event {
                    Event::MouseDown(mouse_button) => {
                        let mouse_down = MouseEvent {
                            pos: mouse_button.pos,
                            window_pos: mouse_button.window_pos,
                            buttons: mouse_button.buttons,
                            mods: mouse_button.mods,
                            count: mouse_button.count,
                            focus: mouse_button.focus,
                            button: mouse_button.button,
                            wheel_delta: mouse_button.wheel_delta,
                        };

                        data.color = data.color.with_alpha(1.);
                    }
                    Event::MouseMove(mouse_button)=>{
                        let mouse_move = MouseEvent {
                            pos: mouse_button.pos,
                            window_pos: mouse_button.window_pos,
                            buttons: mouse_button.buttons,
                            mods: mouse_button.mods,
                            count: mouse_button.count,
                            focus: mouse_button.focus,
                            button: mouse_button.button,
                            wheel_delta: mouse_button.wheel_delta,
                        };

                        self.points.push(mouse_move.pos);
                        data.tool_window.random_point=Some(mouse_move.pos);

                    }
                    Event::MouseUp(_mouse_button)=>{
                        data.color=data.color.with_alpha(1.);

                        let mut image: ImageBuffer<Rgba<u8>, Vec<u8>>=ImageBuffer::from_vec(
                            data.img.width() as u32,
                            data.img.height() as u32,
                        data.tool_window.img.clone().unwrap().raw_pixels().to_vec()).unwrap();

                        let color = data.color.as_rgba8();
                        
                        for p in self.points.clone(){  /* loop */
                            imageproc::drawing::draw_filled_circle_mut(
                                &mut image,
                                (p.x as i32,p.y as i32),
                                10,
                                Rgba([color.0, color.1, color.2, 255])
                            );
                        }


                        data.tool_window.img=Some(ImageBuf::from_raw(
                            image.clone().into_raw(),
                            druid::piet::ImageFormat::RgbaPremul,
                            image.clone().width() as usize,
                            image.clone().height() as usize,
                        ));

                        data.tool_window.random_point=None;
                        data.color=data.color.with_alpha(0.0);
                    },
                    _=>{}
                }
            
            }
            _=>{}
        }
        

        if !ctx.is_hot(){
            data.cursor.over=None;
            data.cursor.down=false;
        }
        if ctx.is_focused() && data.tool_window.tool!=Tools::Text{
            ctx.resign_focus();
            //ctx.focus_prev();
        }
        //}
        child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut druid::UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        child.update(ctx, old_data, data, env)
    }
}
pub struct Delegate; //vedi main.rs
impl AppDelegate<AppState> for Delegate {
    //mi permette di gestire i comandi di show_save_panel e show_open_panel, rispettivamente infatti chiamano SAVE_FILE e OPEN_FILE
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            if let Err(e) = std::fs::write(file_info.path(), &data.name[..]) {
                println!("Error writing file: {e}");
            } else {
                data.save_as(file_info.path());
                return Handled::Yes;
            }
        }
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match std::fs::read_dir(file_info.path()) {
                Ok(_s) => {
                    data.path = file_info.path().to_str().unwrap().to_string();
                }
                Err(e) => {
                    println!("Error opening folder: {e}");
                }
            }
            return Handled::Yes;
        }
        if cmd.is(SHORTCUT) {
            let new_win = WindowDesc::new(shortcut_ui())
                .title(LocalizedString::new("Shortcut"))
                .window_size((300.0, 200.0));
            ctx.new_window(new_win);
        }
        Handled::No
    }
}
